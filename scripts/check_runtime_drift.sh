#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
CURRENT_JSON="$OUT_DIR/ap2-private-surface-inventory.json"
BASELINE_JSON="${BASELINE_JSON:-$ROOT/target/runtime-probe/ap2-runtime-surface-baseline.json}"
UPDATE_BASELINE="${UPDATE_BASELINE:-0}"
OUT_MD="$OUT_DIR/ap2-runtime-drift-report.md"

mkdir -p "$OUT_DIR"

"$ROOT/scripts/inventory_runtime_touchpoints.sh" >/dev/null

if [[ ! -f "$BASELINE_JSON" ]]; then
  if [[ "$UPDATE_BASELINE" == "1" ]]; then
    cp "$CURRENT_JSON" "$BASELINE_JSON"
    cat > "$OUT_MD" <<EOF
# AP.2 Runtime Drift Report

Result: BASELINE_INITIALIZED

- baseline: ${BASELINE_JSON}
- current: ${CURRENT_JSON}
- note: baseline did not exist and was initialized from current inventory.
EOF
    echo "Wrote $OUT_MD"
    exit 0
  fi

  cat > "$OUT_MD" <<EOF
# AP.2 Runtime Drift Report

Result: FAIL

- baseline: ${BASELINE_JSON}
- current: ${CURRENT_JSON}
- reason: baseline missing (set UPDATE_BASELINE=1 to initialize).
EOF
  echo "Wrote $OUT_MD"
  exit 1
fi

python3 - "$BASELINE_JSON" "$CURRENT_JSON" "$OUT_MD" <<'PY'
import json
import pathlib
import sys

baseline_path = pathlib.Path(sys.argv[1])
current_path = pathlib.Path(sys.argv[2])
report_path = pathlib.Path(sys.argv[3])

base = json.loads(baseline_path.read_text())
cur = json.loads(current_path.read_text())

def removed(key):
    b = set(base["symbols"].get(key, []))
    c = set(cur["symbols"].get(key, []))
    return sorted(b - c)

removed_contract = removed("contract_exports")
removed_thunks = removed("thunk_exports")
removed_high_risk = removed("high_risk_touchpoints")

status = "PASS"
issues = []
if removed_contract:
    status = "FAIL"
    issues.append("contract export removals detected")
if removed_thunks:
    status = "FAIL"
    issues.append("runtime thunk removals detected")
if removed_high_risk:
    status = "FAIL"
    issues.append("high-risk touchpoint removals detected")

lines = [
    "# AP.2 Runtime Drift Report",
    "",
    f"Result: {status}",
    "",
    f"- baseline: {baseline_path}",
    f"- current: {current_path}",
    "",
    "## Summary",
    "",
    f"- baseline symbols: {base['summary']['all_symbols']}",
    f"- current symbols: {cur['summary']['all_symbols']}",
    f"- removed contract exports: {len(removed_contract)}",
    f"- removed thunk exports: {len(removed_thunks)}",
    f"- removed high-risk touchpoints: {len(removed_high_risk)}",
]

if issues:
    lines += ["", "## Issues"]
    lines += [f"- {issue}" for issue in issues]

for title, entries in [
    ("Removed Contract Exports", removed_contract),
    ("Removed Thunk Exports", removed_thunks),
    ("Removed High-Risk Touchpoints", removed_high_risk),
]:
    lines += ["", f"## {title}"]
    if entries:
        lines += [f"- {entry}" for entry in entries]
    else:
        lines.append("- none")

report_path.write_text("\n".join(lines) + "\n")
print(f"Wrote {report_path}")

if status != "PASS":
    sys.exit(1)
PY
