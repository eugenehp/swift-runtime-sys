#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
INVENTORY_JSON="$OUT_DIR/ap2-private-surface-inventory.json"
POLICY_JSON="${POLICY_JSON:-$ROOT/scripts/ap2-killswitch-policy.json}"
OUT_MD="$OUT_DIR/ap2-killswitch-policy-report.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

./scripts/inventory_runtime_touchpoints.sh >/dev/null

if [[ ! -f "$POLICY_JSON" ]]; then
  cat > "$OUT_MD" <<EOF
# AP.2 Kill-Switch Policy Report

Result: FAIL

- policy: ${POLICY_JSON}
- reason: policy file missing
EOF
  echo "Wrote $OUT_MD"
  exit 1
fi

python3 - "$INVENTORY_JSON" "$POLICY_JSON" "$OUT_MD" <<'PY'
import json
import pathlib
import re
import sys

inventory_path = pathlib.Path(sys.argv[1])
policy_path = pathlib.Path(sys.argv[2])
report_path = pathlib.Path(sys.argv[3])

inventory = json.loads(inventory_path.read_text())
policy = json.loads(policy_path.read_text())

high_risk = set(inventory.get("symbols", {}).get("high_risk_touchpoints", []))
entries = policy.get("entries", [])
by_symbol = {entry.get("symbol"): entry for entry in entries}

env_pattern = re.compile(r"^[A-Z0-9_]+$")
missing = sorted(high_risk - set(by_symbol))
invalid_entries = []

for symbol, entry in by_symbol.items():
    if symbol not in high_risk:
        continue
    risk = entry.get("risk")
    env = entry.get("kill_switch_env", "")
    default = entry.get("default")
    fallback = entry.get("fallback", "")
    if risk != "high":
        invalid_entries.append((symbol, "risk must be high"))
    if not env_pattern.match(env):
        invalid_entries.append((symbol, "kill_switch_env must be uppercase snake case"))
    if default != "deny":
        invalid_entries.append((symbol, "default must be deny"))
    if not fallback:
        invalid_entries.append((symbol, "fallback must be non-empty"))

status = "PASS" if not missing and not invalid_entries else "FAIL"

lines = [
    "# AP.2 Kill-Switch Policy Report",
    "",
    f"Result: {status}",
    "",
    f"- inventory: {inventory_path}",
    f"- policy: {policy_path}",
    f"- high-risk touchpoints: {len(high_risk)}",
    f"- policy entries: {len(entries)}",
    f"- missing mappings: {len(missing)}",
    f"- invalid mappings: {len(invalid_entries)}",
    "",
    "## Mapped Touchpoints",
]

for symbol in sorted(high_risk):
    entry = by_symbol.get(symbol)
    if not entry:
        lines.append(f"- {symbol}: MISSING")
        continue
    lines.append(
        "- {} => env={} default={} fallback={}".format(
            symbol,
            entry.get("kill_switch_env", ""),
            entry.get("default", ""),
            entry.get("fallback", ""),
        )
    )

lines += ["", "## Validation Issues"]
if not missing and not invalid_entries:
    lines.append("- none")
else:
    for symbol in missing:
        lines.append(f"- {symbol}: missing policy mapping")
    for symbol, msg in invalid_entries:
        lines.append(f"- {symbol}: {msg}")

report_path.write_text("\n".join(lines) + "\n")
print(f"Wrote {report_path}")

if status != "PASS":
    sys.exit(1)
PY
