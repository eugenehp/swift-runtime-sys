#!/usr/bin/env bash
set -euo pipefail

# ---------------------------------------------------------------------------
# Reliability gate for runtime parity.
#
# Release floor (required for a production parity claim):
#   FUZZ_CASES=128 ./scripts/run_parity_stress.sh 100
#
# CI defaults:
#   PR push  : FUZZ_CASES=64  ./scripts/run_parity_stress.sh 3
#   main push: FUZZ_CASES=128 ./scripts/run_parity_stress.sh 10
#
# On any failure the seed and log path are captured in the summary file
# so failures are fully reproducible:
#   RUNTIME_FUZZ_SEED=<seed> RUNTIME_FUZZ_CASES=<n> ./scripts/run_parity_matrix.sh
# ---------------------------------------------------------------------------

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/stress"
RUNS="${1:-20}"
STOP_ON_FAIL="${STOP_ON_FAIL:-0}"
FUZZ_CASES="${FUZZ_CASES:-64}"

if ! [[ "$RUNS" =~ ^[0-9]+$ ]] || [[ "$RUNS" -le 0 ]]; then
  echo "usage: $0 [positive-run-count]"
  exit 2
fi

mkdir -p "$OUT_DIR"

STAMP="$(date -u +"%Y-%m-%dT%H_%M_%SZ")"
SUMMARY_FILE="$OUT_DIR/stress-summary-${STAMP}.md"

passes=0
fails=0
failed_runs=""

echo "Running parity stress: ${RUNS} iterations"

for i in $(seq 1 "$RUNS"); do
  run_log="$OUT_DIR/run-${STAMP}-${i}.log"
  echo "[$i/$RUNS] running matrix..."
  seed=$((i * 7919))

  if (cd "$ROOT" && RUNTIME_FUZZ_SEED="$seed" RUNTIME_FUZZ_CASES="$FUZZ_CASES" ./scripts/run_parity_matrix.sh) >"$run_log" 2>&1; then
    record_line="$(grep -E "^History record:" "$run_log" | tail -n 1 || true)"
    passed="$(echo "$record_line" | sed -nE 's/.*\(([0-9]+)\/([0-9]+) PASS\).*/\1/p')"
    total="$(echo "$record_line" | sed -nE 's/.*\(([0-9]+)\/([0-9]+) PASS\).*/\2/p')"

    if [[ -n "$passed" && -n "$total" && "$passed" == "$total" ]]; then
      passes=$((passes + 1))
      echo "[$i/$RUNS] PASS (${passed}/${total})"
    else
      fails=$((fails + 1))
      failed_runs+="- run ${i}: incomplete pass ratio; seed=${seed} FUZZ_CASES=${FUZZ_CASES}; reproduce: RUNTIME_FUZZ_SEED=${seed} RUNTIME_FUZZ_CASES=${FUZZ_CASES} ./scripts/run_parity_matrix.sh; log: ${run_log}\n"
      echo "[$i/$RUNS] FAIL (unexpected summary parsing) seed=${seed}"
      if [[ "$STOP_ON_FAIL" == "1" ]]; then break; fi
    fi
  else
    fails=$((fails + 1))
    failed_runs+="- run ${i}: command failed; seed=${seed} FUZZ_CASES=${FUZZ_CASES}; reproduce: RUNTIME_FUZZ_SEED=${seed} RUNTIME_FUZZ_CASES=${FUZZ_CASES} ./scripts/run_parity_matrix.sh; log: ${run_log}\n"
    echo "[$i/$RUNS] FAIL (command failed) seed=${seed}"
    if [[ "$STOP_ON_FAIL" == "1" ]]; then break; fi
  fi
done

completed=$((passes + fails))

cat > "$SUMMARY_FILE" <<MD
# Runtime Parity Stress Summary

- timestamp: ${STAMP}
- requested_runs: ${RUNS}
- completed_runs: ${completed}
- passed_runs: ${passes}
- failed_runs: ${fails}
- fuzz_cases_per_run: ${FUZZ_CASES}
- release_floor: FUZZ_CASES=128 ./scripts/run_parity_stress.sh 100

## Failures

${failed_runs:-None}
MD

echo "Wrote ${SUMMARY_FILE}"
echo "Stress result: ${passes}/${completed} runs fully passed"

if [[ "$fails" -gt 0 ]]; then
  exit 1
fi
