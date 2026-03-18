#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

MODE="verbose"
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  cat <<'EOF'
Usage: scripts/run_tracks_g_to_m_tmux.sh [mode]

Modes:
  (default)        Verbose per-track output + summary
  --summary, -s    Compact per-track PASS/FAIL + duration + summary
  --json, -j       JSON output for CI parsing
  --help, -h       Show this help
EOF
  exit 0
elif [[ "${1:-}" == "--summary" || "${1:-}" == "-s" ]]; then
  MODE="summary"
elif [[ "${1:-}" == "--json" || "${1:-}" == "-j" ]]; then
  MODE="json"
fi

SUMMARY_ONLY=0
JSON_ONLY=0
if [[ "$MODE" == "summary" ]]; then
  SUMMARY_ONLY=1
fi
if [[ "$MODE" == "json" ]]; then
  SUMMARY_ONLY=1
  JSON_ONLY=1
fi

SCRIPTS=(
  "scripts/run_track_g_full_tmux.sh"
  "scripts/run_track_h1_tmux.sh"
  "scripts/run_track_h2_tmux.sh"
  "scripts/run_track_h3_tmux.sh"
  "scripts/run_track_i_tmux.sh"
  "scripts/run_track_j_tmux.sh"
  "scripts/run_track_k_tmux.sh"
  "scripts/run_track_l_tmux.sh"
  "scripts/run_track_m_tmux.sh"
)

summary_ok=0
summary_fail=0
start_epoch="$(date +%s)"
results_json=()

if [[ "$JSON_ONLY" -eq 0 ]]; then
  echo "=== Consolidated Regression: Tracks G -> M ==="
fi
if [[ "$SUMMARY_ONLY" -eq 1 && "$JSON_ONLY" -eq 0 ]]; then
  echo "mode=summary"
fi

for script in "${SCRIPTS[@]}"; do
  script_start="$(date +%s)"
  script_name="$(basename "$script")"

  if [[ "$JSON_ONLY" -eq 0 ]]; then
    echo
    echo ">>> Running ${script}"
  fi
  chmod +x "$script"

  out_file="/tmp/$(basename "$script").aggregate.out"
  rm -f "$out_file"

  attempt=1
  max_attempts=2
  while true; do
    if [[ "$SUMMARY_ONLY" -eq 1 ]]; then
      if "$script" > "$out_file" 2>&1; then
        break
      fi
    else
      if "$script" | tee "$out_file"; then
        break
      fi
    fi

    if grep -q "input file 'examples/RustBridge.swift' was modified during the build" "$out_file" && [[ "$attempt" -lt "$max_attempts" ]]; then
      if [[ "$SUMMARY_ONLY" -eq 1 && "$JSON_ONLY" -eq 0 ]]; then
        echo "note=${script_name}: transient_swift_rebuild_race_retrying"
      elif [[ "$JSON_ONLY" -eq 0 ]]; then
        echo "note=transient_swift_rebuild_race_retrying"
      fi
      attempt=$((attempt + 1))
      sleep 1
      continue
    fi

    elapsed=$(( $(date +%s) - script_start ))

    if [[ "$SUMMARY_ONLY" -eq 1 && "$JSON_ONLY" -eq 0 ]]; then
      echo "result=${script_name}: script_error (${elapsed}s)"
    elif [[ "$JSON_ONLY" -eq 0 ]]; then
      echo "result=script_error"
    fi
    retries=$((attempt - 1))
    results_json+=("{\"script\":\"${script_name}\",\"status\":\"script_error\",\"tmux_exit\":null,\"duration_seconds\":${elapsed},\"retries\":${retries}}")
    summary_fail=$((summary_fail + 1))
    continue 2
  done

  tmux_exit="$(grep -E '^tmux_exit=' "$out_file" | tail -n 1 | cut -d'=' -f2 || true)"
  elapsed=$(( $(date +%s) - script_start ))
  retries=$((attempt - 1))
  if [[ -z "$tmux_exit" ]]; then
    if [[ "$SUMMARY_ONLY" -eq 1 && "$JSON_ONLY" -eq 0 ]]; then
      echo "result=${script_name}: invalid_output_missing_tmux_exit (${elapsed}s)"
    elif [[ "$JSON_ONLY" -eq 0 ]]; then
      echo "result=invalid_output (missing tmux_exit)"
    fi
    results_json+=("{\"script\":\"${script_name}\",\"status\":\"invalid_output\",\"tmux_exit\":null,\"duration_seconds\":${elapsed},\"retries\":${retries}}")
    summary_fail=$((summary_fail + 1))
    continue
  fi

  if [[ "$tmux_exit" == "0" ]]; then
    if [[ "$SUMMARY_ONLY" -eq 1 && "$JSON_ONLY" -eq 0 ]]; then
      echo "result=${script_name}: PASS (${elapsed}s)"
    elif [[ "$JSON_ONLY" -eq 0 ]]; then
      echo "result=PASS"
    fi
    results_json+=("{\"script\":\"${script_name}\",\"status\":\"pass\",\"tmux_exit\":0,\"duration_seconds\":${elapsed},\"retries\":${retries}}")
    summary_ok=$((summary_ok + 1))
  else
    if [[ "$SUMMARY_ONLY" -eq 1 && "$JSON_ONLY" -eq 0 ]]; then
      echo "result=${script_name}: FAIL tmux_exit=${tmux_exit} (${elapsed}s)"
    elif [[ "$JSON_ONLY" -eq 0 ]]; then
      echo "result=FAIL (tmux_exit=${tmux_exit})"
    fi
    results_json+=("{\"script\":\"${script_name}\",\"status\":\"fail\",\"tmux_exit\":${tmux_exit},\"duration_seconds\":${elapsed},\"retries\":${retries}}")
    summary_fail=$((summary_fail + 1))
  fi

done

total_elapsed=$(( $(date +%s) - start_epoch ))

if [[ "$JSON_ONLY" -eq 1 ]]; then
  printf '{"mode":"json","scripts_passed":%d,"scripts_failed":%d,"total_duration_seconds":%d,"results":[' "$summary_ok" "$summary_fail" "$total_elapsed"
  for i in "${!results_json[@]}"; do
    if [[ "$i" -gt 0 ]]; then
      printf ','
    fi
    printf '%s' "${results_json[$i]}"
  done
  printf ']}'
  echo
else
  echo
  echo "=== Consolidated Summary ==="
  echo "scripts_passed=${summary_ok}"
  echo "scripts_failed=${summary_fail}"
  echo "total_duration_seconds=${total_elapsed}"
fi

if [[ "$summary_fail" -ne 0 ]]; then
  exit 1
fi
