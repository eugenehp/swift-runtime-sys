#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
RUN_DIR="$OUT_DIR/phase-c-signoff"
SUMMARY_JSON="$RUN_DIR/phase-c-signoff.json"
SUMMARY_MD="$RUN_DIR/phase-c-signoff.md"

mkdir -p "$RUN_DIR"
cd "$ROOT"

# Phase C host-cell gates in promotion order.
gates=(
  "c1:scripts/run_c1_ownership_gate.sh"
  "c2:scripts/run_c2_existentials_gate.sh"
  "c3:scripts/run_c3_enum_gate.sh"
  "c4:scripts/run_c4_closure_gate.sh"
  "c5:scripts/run_c5_optimizer_gate.sh"
  "c6:scripts/run_c6_lowering_gate.sh"
  "c7:scripts/run_c7_safety_gate.sh"
  "c8:scripts/run_c8_host_reliability_gate.sh"
  "c9:scripts/run_c9_host_promotion_gate.sh"
)

tmp_tsv="$(mktemp)"
trap 'rm -f "$tmp_tsv"' EXIT

for item in "${gates[@]}"; do
  key="${item%%:*}"
  script="${item#*:}"
  log="$RUN_DIR/${STAMP}-${key}.log"
  echo "[phase-c] running ${key} -> ${script}"
  if bash "$script" >"$log" 2>&1; then
    status="PASS"
  else
    status="FAIL"
  fi
  printf '%s\t%s\t%s\n' "$key" "$script" "$status" >> "$tmp_tsv"
  if [[ "$status" != "PASS" ]]; then
    echo "[phase-c] gate ${key} failed; see ${log}" >&2
    break
  fi
done

# Always run parity matrix at the end for final baseline evidence.
PARITY_LOG="$RUN_DIR/${STAMP}-parity.log"
if bash scripts/run_parity_matrix.sh >"$PARITY_LOG" 2>&1; then
  PARITY_STATUS="PASS"
else
  PARITY_STATUS="FAIL"
fi

python3 - "$tmp_tsv" "$SUMMARY_JSON" "$SUMMARY_MD" "$STAMP" "$PARITY_STATUS" <<'PY'
import json
import pathlib
import sys

rows_path = pathlib.Path(sys.argv[1])
out_json = pathlib.Path(sys.argv[2])
out_md = pathlib.Path(sys.argv[3])
stamp = sys.argv[4]
parity_status = sys.argv[5]

rows = []
for line in rows_path.read_text(encoding="utf-8").splitlines():
    key, script, status = line.split("\t")
    rows.append({"gate": key, "script": script, "status": status})

required = ["c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9"]
status_map = {r["gate"]: r["status"] for r in rows}
missing = [g for g in required if g not in status_map]
failed = [g for g in required if status_map.get(g) != "PASS"]

result = "PASS" if (not missing and not failed and parity_status == "PASS") else "FAIL"

payload = {
    "timestamp": stamp,
    "phase": "C",
    "required_gates": required,
    "rows": rows,
    "missing_gates": missing,
    "failed_gates": failed,
    "parity_status": parity_status,
    "result": result,
}
out_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

lines = [
    "# Phase C Host-Cell Signoff",
    "",
    f"- timestamp: {stamp}",
    f"- parity_status: {parity_status}",
    f"- result: {result}",
    "",
    "| Gate | Script | Status |",
    "|---|---|---|",
]
for row in rows:
    lines.append(f"| {row['gate']} | {row['script']} | {row['status']} |")
if missing:
    lines += ["", "## Missing Gates"]
    lines += [f"- {m}" for m in missing]
if failed:
    lines += ["", "## Failed Gates"]
    lines += [f"- {f}" for f in failed]
out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")

if result != "PASS":
    raise SystemExit(1)
PY

echo "Wrote $SUMMARY_JSON"
echo "Wrote $SUMMARY_MD"
echo "Phase C signoff PASS"
