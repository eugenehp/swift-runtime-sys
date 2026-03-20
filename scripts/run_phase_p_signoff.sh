#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
RUN_DIR="$OUT_DIR/phase-p-signoff"
SUMMARY_JSON="$RUN_DIR/phase-p-signoff.json"
SUMMARY_MD="$RUN_DIR/phase-p-signoff.md"

mkdir -p "$RUN_DIR"
cd "$ROOT"

# Phase P gates in execution order.
gates=(
  "p1:scripts/run_p1_string_gate.sh"
  "p2:scripts/run_p2_url_gate.sh"
  "p3:scripts/run_p3_codable_gate.sh"
  "p4:scripts/run_p4_collections_gate.sh"
  "p5:scripts/run_p5_temporal_gate.sh"
  "p6:scripts/run_p6_formatting_gate.sh"
)

tmp_tsv="$(mktemp)"
trap 'rm -f "$tmp_tsv"' EXIT

for item in "${gates[@]}"; do
  key="${item%%:*}"
  script="${item#*:}"
  log="$RUN_DIR/${STAMP}-${key}.log"
  echo "[phase-p] running ${key} -> ${script}"
  if bash "$script" >"$log" 2>&1; then
    status="PASS"
  else
    status="FAIL"
  fi
  printf '%s\t%s\t%s\n' "$key" "$script" "$status" >> "$tmp_tsv"
  if [[ "$status" != "PASS" ]]; then
    echo "[phase-p] gate ${key} failed; see ${log}" >&2
    break
  fi
done

# Run parity matrix for baseline regression guard.
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

required = ["p1", "p2", "p3", "p4", "p5", "p6"]
status_map = {r["gate"]: r["status"] for r in rows}
missing = [g for g in required if g not in status_map]
failed = [g for g in required if status_map.get(g) != "PASS"]

result = "PASS" if (not missing and not failed and parity_status == "PASS") else "FAIL"

payload = {
    "timestamp": stamp,
    "phase": "P",
    "required_gates": required,
    "rows": rows,
    "missing_gates": missing,
    "failed_gates": failed,
    "parity_status": parity_status,
    "result": result,
}
out_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

lines = [
    "# Phase P Signoff",
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
echo "Phase P signoff PASS"
