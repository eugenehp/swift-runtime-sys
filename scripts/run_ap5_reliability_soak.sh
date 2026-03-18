#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
SOAK_DIR="$OUT_DIR/ap5-soak"
HISTORY_DIR="$SOAK_DIR/history"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
SUMMARY_JSON="$SOAK_DIR/ap5-soak-summary.json"
SUMMARY_MD="$SOAK_DIR/ap5-soak-summary.md"
TREND_MD="$SOAK_DIR/ap5-stability-trend.md"
HISTORY_JSON="$HISTORY_DIR/${STAMP}.json"
SOAK_RUNS="${AP5_SOAK_RUNS:-5}"
HISTORY_WINDOW="${AP5_HISTORY_WINDOW:-5}"
FLAKE_BUDGET="${AP5_FLAKE_BUDGET:-0}"

mkdir -p "$SOAK_DIR" "$HISTORY_DIR"
cd "$ROOT"

run_gate() {
  local gate="$1"
  case "$gate" in
    parity_debug)
      ./scripts/run_parity_matrix.sh
      ;;
    protocol_dispatch)
      ./scripts/run_protocol_dispatch_matrix.sh
      ;;
    abi_shape_debug)
      ./scripts/run_abi_shape_closure.sh
      ;;
    ap4_differential)
      ./scripts/run_ap4_differential_oracle.sh
      ;;
    contract_debug)
      ./scripts/run_contract_parity.sh
      ;;
    parity_release)
      PROFILE=release ./scripts/run_parity_matrix.sh
      ;;
    contract_release)
      PROFILE=release ./scripts/run_contract_parity.sh
      ;;
    *)
      echo "unknown gate: $gate" >&2
      return 2
      ;;
  esac
}

gates=(
  parity_debug
  protocol_dispatch
  abi_shape_debug
  ap4_differential
  contract_debug
  parity_release
  contract_release
)

tmp_tsv="$(mktemp)"
trap 'rm -f "$tmp_tsv"' EXIT

flake_count=0
fail_count=0
pass_count=0

for iteration in $(seq 1 "$SOAK_RUNS"); do
  for gate in "${gates[@]}"; do
    first_log="$SOAK_DIR/${STAMP}-${iteration}-${gate}-attempt1.log"
    retry_log="$SOAK_DIR/${STAMP}-${iteration}-${gate}-attempt2.log"
    retry_used=0
    status="PASS"
    reason="stable-pass"

    if run_gate "$gate" >"$first_log" 2>&1; then
      :
    else
      retry_used=1
      if run_gate "$gate" >"$retry_log" 2>&1; then
        status="FLAKY"
        reason="failed-then-passed-on-retry"
        flake_count=$((flake_count + 1))
      else
        status="FAIL"
        reason="failed-on-both-attempts"
        fail_count=$((fail_count + 1))
      fi
    fi

    if [[ "$status" == "PASS" ]]; then
      pass_count=$((pass_count + 1))
    fi

    printf '%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$iteration" "$gate" "$status" "$retry_used" "$first_log" "$reason" >> "$tmp_tsv"
  done
done

python3 - "$tmp_tsv" "$SUMMARY_JSON" "$SUMMARY_MD" "$HISTORY_JSON" "$STAMP" "$SOAK_RUNS" "$HISTORY_WINDOW" "$FLAKE_BUDGET" <<'PY'
import json
import pathlib
import sys

tsv_path = pathlib.Path(sys.argv[1])
summary_json = pathlib.Path(sys.argv[2])
summary_md = pathlib.Path(sys.argv[3])
history_json = pathlib.Path(sys.argv[4])
stamp = sys.argv[5]
soak_runs = int(sys.argv[6])
history_window = int(sys.argv[7])
flake_budget = int(sys.argv[8])

rows = []
for line in tsv_path.read_text(encoding="utf-8").splitlines():
    iteration, gate, status, retry_used, first_log, reason = line.split("\t")
    rows.append(
        {
            "iteration": int(iteration),
            "gate": gate,
            "status": status,
            "retry_used": retry_used == "1",
            "log": first_log,
            "reason": reason,
        }
    )

pass_count = sum(1 for row in rows if row["status"] == "PASS")
flake_count = sum(1 for row in rows if row["status"] == "FLAKY")
fail_count = sum(1 for row in rows if row["status"] == "FAIL")
result = "PASS" if flake_count <= flake_budget and fail_count == 0 else "FAIL"

payload = {
    "version": 1,
    "timestamp": stamp,
    "soak_runs": soak_runs,
    "gate_count": len(rows),
    "pass_count": pass_count,
    "flake_count": flake_count,
    "fail_count": fail_count,
    "flake_budget": flake_budget,
    "result": result,
    "rows": rows,
}
summary_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
history_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

lines = [
    "# AP.5 Reliability Soak Summary",
    "",
    f"- timestamp: {stamp}",
    f"- soak_runs: {soak_runs}",
    f"- total_gate_executions: {len(rows)}",
    f"- pass_count: {pass_count}",
    f"- flake_count: {flake_count}",
    f"- fail_count: {fail_count}",
    f"- flake_budget: {flake_budget}",
    f"- result: {result}",
    "",
    "| Iteration | Gate | Status | Retry Used | Reason |",
    "|---:|---|---|---|---|",
]
for row in rows:
    lines.append(
        f"| {row['iteration']} | {row['gate']} | {row['status']} | {'yes' if row['retry_used'] else 'no'} | {row['reason']} |"
    )
summary_md.write_text("\n".join(lines) + "\n", encoding="utf-8")

history_dir = history_json.parent
history_files = sorted(history_dir.glob("*.json"), reverse=True)[:history_window]
trend_lines = [
    "# AP.5 Stability Trend",
    "",
    f"- history_window: {history_window}",
    f"- flake_budget: {flake_budget}",
    "",
    "| Timestamp | Result | Flakes | Failures | Gate Executions |",
    "|---|---|---:|---:|---:|",
]
window_flakes = 0
for path in history_files:
    data = json.loads(path.read_text(encoding="utf-8"))
    window_flakes += int(data.get("flake_count", 0))
    trend_lines.append(
        f"| {data.get('timestamp', path.stem)} | {data.get('result', 'FAIL')} | {data.get('flake_count', 0)} | {data.get('fail_count', 0)} | {data.get('gate_count', 0)} |"
    )
trend_lines += [
    "",
    f"- rolling_window_flakes: {window_flakes}",
    f"- rolling_window_result: {'PASS' if window_flakes <= flake_budget else 'FAIL'}",
]
pathlib.Path(sys.argv[3]).parent.joinpath("ap5-stability-trend.md").write_text(
    "\n".join(trend_lines) + "\n", encoding="utf-8"
)

if fail_count > 0 or window_flakes > flake_budget:
    raise SystemExit(1)
PY

echo "Wrote $SUMMARY_JSON"
echo "Wrote $SUMMARY_MD"
echo "Wrote $TREND_MD"
echo "AP.5 reliability soak PASS"