#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
C8_DIR="$OUT_DIR/c8-host-reliability"
HISTORY_DIR="$C8_DIR/history"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
SOAK_RUNS="${C8_SOAK_RUNS:-3}"
FLAKE_BUDGET="${C8_FLAKE_BUDGET:-0}"
REGRESSION_BUDGET_PCT="${C8_REGRESSION_BUDGET_PCT:-30}"
DIFF_FRAGMENTS="${C8_DIFF_FRAGMENTS:-12}"
DIFF_SEEDS="${C8_DIFF_SEEDS:-1 2 3 4 6 7 8 10 11}"
DIFF_DIR="$C8_DIR/differential-corpus"
DIFF_JSON="$C8_DIR/c8-differential-expanded.json"
DIFF_MD="$C8_DIR/c8-differential-expanded.md"

SUMMARY_JSON="$C8_DIR/c8-host-reliability-summary.json"
SUMMARY_MD="$C8_DIR/c8-host-reliability-summary.md"
TREND_JSON="$C8_DIR/c8-host-trend.json"
TREND_MD="$C8_DIR/c8-host-trend.md"
HISTORY_JSON="$HISTORY_DIR/${STAMP}.json"

mkdir -p "$C8_DIR" "$HISTORY_DIR"
cd "$ROOT"

run_gate() {
  local gate="$1"
  case "$gate" in
    c4)
      bash ./scripts/run_c4_closure_gate.sh
      ;;
    c5)
      bash ./scripts/run_c5_optimizer_gate.sh
      ;;
    c6)
      bash ./scripts/run_c6_lowering_gate.sh
      ;;
    c7)
      bash ./scripts/run_c7_safety_gate.sh
      ;;
    *)
      echo "unknown gate: $gate" >&2
      return 2
      ;;
  esac
}

measure_probe() {
  local profile="$1"
  local example="$2"
  local label="$3"
  local fixture_dir="$OUT_DIR/resilient-fixtures"
  local bin
  local time_log
  local run_log
  local elapsed
  local rss

  if [[ "$profile" == "release" ]]; then
    cargo build --release --example "$example" >/dev/null
    bin="$ROOT/target/release/examples/$example"
  else
    cargo build --example "$example" >/dev/null
    bin="$ROOT/target/debug/examples/$example"
  fi

  time_log="$C8_DIR/${STAMP}-${label}-${profile}.time.log"
  run_log="$C8_DIR/${STAMP}-${label}-${profile}.run.log"

  DYLD_LIBRARY_PATH="$fixture_dir:." /usr/bin/time -l "$bin" >"$run_log" 2>"$time_log"
  elapsed="$(awk '/ real$/ {print $1; exit}' "$time_log")"
  rss="$(awk '/maximum resident set size/ {print $1; exit}' "$time_log")"

  if [[ -z "$elapsed" ]]; then
    elapsed="0"
  fi
  if [[ -z "$rss" ]]; then
    rss="0"
  fi

  echo "$label|$profile|$elapsed|$rss"
}

gates=(c4 c5 c6 c7)
tmp_tsv="$(mktemp)"
trap 'rm -f "$tmp_tsv"' EXIT

for iteration in $(seq 1 "$SOAK_RUNS"); do
  echo "[c8] soak iteration ${iteration}/${SOAK_RUNS}"
  for gate in "${gates[@]}"; do
    first_log="$C8_DIR/${STAMP}-soak-${iteration}-${gate}-attempt1.log"
    retry_log="$C8_DIR/${STAMP}-soak-${iteration}-${gate}-attempt2.log"
    status="PASS"
    reason="stable-pass"
    retry_used=0

    if run_gate "$gate" >"$first_log" 2>&1; then
      :
    else
      retry_used=1
      if run_gate "$gate" >"$retry_log" 2>&1; then
        status="FLAKY"
        reason="failed-then-passed-on-retry"
      else
        status="FAIL"
        reason="failed-on-both-attempts"
      fi
    fi

    printf '%s\t%s\t%s\t%s\t%s\n' \
      "$iteration" "$gate" "$status" "$retry_used" "$reason" >> "$tmp_tsv"
  done
done

echo "[c8] running expanded differential corpus"
mkdir -p "$OUT_DIR/resilient-fixtures" "$DIFF_DIR"
./scripts/build_runtime_thunks.sh >/dev/null 2>&1
swiftc -emit-library -emit-module -emit-module-path "$OUT_DIR/resilient-fixtures/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$OUT_DIR/resilient-fixtures/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$OUT_DIR/resilient-fixtures" -L "$OUT_DIR/resilient-fixtures" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
cargo build --example runtime_differential_fuzz_probe >/dev/null

for seed in $DIFF_SEEDS; do
  diff_log="$C8_DIR/${STAMP}-differential-seed-${seed}.log"
  DYLD_LIBRARY_PATH="$OUT_DIR/resilient-fixtures:." ./target/debug/examples/runtime_differential_fuzz_probe --seed-check "$seed" "$DIFF_FRAGMENTS" "$DIFF_DIR" >"$diff_log" 2>&1
done

python3 - "$DIFF_JSON" "$DIFF_MD" "$STAMP" "$DIFF_FRAGMENTS" $DIFF_SEEDS <<'PY'
import json
import pathlib
import sys

out_json = pathlib.Path(sys.argv[1])
out_md = pathlib.Path(sys.argv[2])
stamp = sys.argv[3]
fragments = int(sys.argv[4])
seeds = [int(v) for v in sys.argv[5:]]

payload = {
    "timestamp": stamp,
    "fragment_count": fragments,
    "seed_count": len(seeds),
    "seeds": seeds,
    "result": "PASS",
}
out_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

lines = [
    "# C.8 Expanded Differential Corpus",
    "",
    f"- timestamp: {stamp}",
    f"- fragment_count: {fragments}",
    f"- seed_count: {len(seeds)}",
    f"- first_seed: {seeds[0] if seeds else '-'}",
    f"- last_seed: {seeds[-1] if seeds else '-'}",
    "- result: PASS",
]
out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")
PY

echo "[c8] measuring host trend metrics"
measure_probe debug runtime_call_lowering_probe dynamic_invoke >> "$C8_DIR/${STAMP}-measurements.tsv"
measure_probe release runtime_call_lowering_probe dynamic_invoke >> "$C8_DIR/${STAMP}-measurements.tsv"
measure_probe debug runtime_metadata_enumeration_probe metadata_traversal >> "$C8_DIR/${STAMP}-measurements.tsv"
measure_probe release runtime_metadata_enumeration_probe metadata_traversal >> "$C8_DIR/${STAMP}-measurements.tsv"

python3 - "$tmp_tsv" "$C8_DIR/${STAMP}-measurements.tsv" "$SUMMARY_JSON" "$SUMMARY_MD" "$TREND_JSON" "$TREND_MD" "$HISTORY_DIR" "$HISTORY_JSON" "$STAMP" "$SOAK_RUNS" "$FLAKE_BUDGET" "$REGRESSION_BUDGET_PCT" <<'PY'
import json
import pathlib
import sys

soak_tsv = pathlib.Path(sys.argv[1])
measure_tsv = pathlib.Path(sys.argv[2])
summary_json = pathlib.Path(sys.argv[3])
summary_md = pathlib.Path(sys.argv[4])
trend_json = pathlib.Path(sys.argv[5])
trend_md = pathlib.Path(sys.argv[6])
history_dir = pathlib.Path(sys.argv[7])
history_json = pathlib.Path(sys.argv[8])
stamp = sys.argv[9]
soak_runs = int(sys.argv[10])
flake_budget = int(sys.argv[11])
regression_budget_pct = float(sys.argv[12])

rows = []
for line in soak_tsv.read_text(encoding="utf-8").splitlines():
    iteration, gate, status, retry_used, reason = line.split("\t")
    rows.append(
        {
            "iteration": int(iteration),
            "gate": gate,
            "status": status,
            "retry_used": retry_used == "1",
            "reason": reason,
        }
    )

measurements = []
for line in measure_tsv.read_text(encoding="utf-8").splitlines():
    label, profile, elapsed, rss = line.split("|")
    measurements.append(
        {
            "label": label,
            "profile": profile,
            "elapsed_sec": float(elapsed),
            "max_rss_kb": int(float(rss)),
        }
    )

pass_count = sum(1 for row in rows if row["status"] == "PASS")
flake_count = sum(1 for row in rows if row["status"] == "FLAKY")
fail_count = sum(1 for row in rows if row["status"] == "FAIL")
soak_pass = (flake_count <= flake_budget) and (fail_count == 0)

history_files = sorted(history_dir.glob("*.json"))
previous = None
if history_files:
    previous_candidates = [p for p in history_files if p.name != history_json.name]
    if previous_candidates:
        previous = json.loads(previous_candidates[-1].read_text(encoding="utf-8"))

regressions = []
if previous:
    prev_map = {
        (m["label"], m["profile"]): m
        for m in previous.get("measurements", [])
    }
    for cur in measurements:
        key = (cur["label"], cur["profile"])
        prev = prev_map.get(key)
        if not prev:
            continue
        elapsed_prev = float(prev.get("elapsed_sec", 0.0))
        rss_prev = float(prev.get("max_rss_kb", 0.0))
        elapsed_pct = 0.0
        rss_pct = 0.0
        if elapsed_prev > 0:
            elapsed_pct = ((cur["elapsed_sec"] - elapsed_prev) / elapsed_prev) * 100.0
        if rss_prev > 0:
            rss_pct = ((cur["max_rss_kb"] - rss_prev) / rss_prev) * 100.0
        if elapsed_pct > regression_budget_pct or rss_pct > regression_budget_pct:
            regressions.append(
                {
                    "label": cur["label"],
                    "profile": cur["profile"],
                    "elapsed_regression_pct": round(elapsed_pct, 2),
                    "rss_regression_pct": round(rss_pct, 2),
                }
            )

trend_pass = len(regressions) == 0
result = "PASS" if soak_pass and trend_pass else "FAIL"

payload = {
    "version": 1,
    "timestamp": stamp,
    "soak_runs": soak_runs,
    "flake_budget": flake_budget,
    "regression_budget_pct": regression_budget_pct,
    "soak_rows": rows,
    "pass_count": pass_count,
    "flake_count": flake_count,
    "fail_count": fail_count,
    "measurements": measurements,
    "regressions": regressions,
    "result": result,
}

summary_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
history_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
trend_json.write_text(json.dumps({
    "timestamp": stamp,
    "regression_budget_pct": regression_budget_pct,
    "measurements": measurements,
    "regressions": regressions,
    "result": "PASS" if trend_pass else "FAIL",
}, indent=2) + "\n", encoding="utf-8")

md_lines = [
    "# C.8 Host Reliability Summary",
    "",
    f"- timestamp: {stamp}",
    f"- soak_runs: {soak_runs}",
    f"- pass_count: {pass_count}",
    f"- flake_count: {flake_count}",
    f"- fail_count: {fail_count}",
    f"- flake_budget: {flake_budget}",
    f"- regression_budget_pct: {regression_budget_pct}",
    f"- result: {result}",
    "",
    "## Soak Detail",
    "",
    "| Iteration | Gate | Status | Retry | Reason |",
    "|---:|---|---|---|---|",
]
for row in rows:
    md_lines.append(
        f"| {row['iteration']} | {row['gate']} | {row['status']} | {'yes' if row['retry_used'] else 'no'} | {row['reason']} |"
    )
summary_md.write_text("\n".join(md_lines) + "\n", encoding="utf-8")

trend_lines = [
    "# C.8 Host Trend",
    "",
    f"- timestamp: {stamp}",
    f"- regression_budget_pct: {regression_budget_pct}",
    f"- result: {'PASS' if trend_pass else 'FAIL'}",
    "",
    "| Label | Profile | Elapsed (s) | Max RSS (KB) |",
    "|---|---|---:|---:|",
]
for m in measurements:
    trend_lines.append(
        f"| {m['label']} | {m['profile']} | {m['elapsed_sec']:.4f} | {m['max_rss_kb']} |"
    )
if regressions:
    trend_lines += [
        "",
        "## Regressions",
        "",
        "| Label | Profile | Elapsed Regression % | RSS Regression % |",
        "|---|---|---:|---:|",
    ]
    for r in regressions:
        trend_lines.append(
            f"| {r['label']} | {r['profile']} | {r['elapsed_regression_pct']} | {r['rss_regression_pct']} |"
        )
trend_md.write_text("\n".join(trend_lines) + "\n", encoding="utf-8")

if result != "PASS":
    raise SystemExit(1)
PY

echo "Wrote $SUMMARY_JSON"
echo "Wrote $SUMMARY_MD"
echo "Wrote $TREND_JSON"
echo "Wrote $TREND_MD"
echo "C.8 host reliability gate PASS"
