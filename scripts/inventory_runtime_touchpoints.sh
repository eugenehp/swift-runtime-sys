#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_JSON="$OUT_DIR/ap2-private-surface-inventory.json"
OUT_MD="$OUT_DIR/ap2-private-surface-inventory.md"
TMP_SYMBOLS="$OUT_DIR/.ap2-symbols.tmp"

mkdir -p "$OUT_DIR"
cd "$ROOT"

extract_symbols() {
  if command -v rg >/dev/null 2>&1; then
    rg -oN '"(swift_[A-Za-z0-9_]+|runtime_thunk_[A-Za-z0-9_]+|\$s[^"\\]+|_\$s[^"\\]+)"' src examples \
      | sed -E 's/^"|"$//g'
  else
    grep -RhoE '"(swift_[A-Za-z0-9_]+|runtime_thunk_[A-Za-z0-9_]+|\$s[^"\\]+|_\$s[^"\\]+)"' src examples \
      | sed -E 's/^"|"$//g'
  fi
}

extract_symbols | sort -u > "$TMP_SYMBOLS"

python3 - "$TMP_SYMBOLS" "$OUT_JSON" <<'PY'
import json
import pathlib
import sys

symbols_path = pathlib.Path(sys.argv[1])
out_path = pathlib.Path(sys.argv[2])
all_symbols = [line.strip() for line in symbols_path.read_text().splitlines() if line.strip()]

contract_exports = sorted([s for s in all_symbols if s.startswith("swift_contract_")])
runtime_exports = sorted([s for s in all_symbols if s.startswith("swift_") and not s.startswith("swift_contract_")])
thunk_exports = sorted([s for s in all_symbols if s.startswith("runtime_thunk_")])
mangled_symbols = sorted([s for s in all_symbols if s.startswith("$s") or s.startswith("_$s")])

high_risk_patterns = [
    "swift_allocObject",
    "swift_deallocClassInstance",
    "swift_release",
    "swift_retain",
    "swift_getTypeByMangledNameInContext",
    "swift_getTypeByMangledNameInEnvironment",
]

high_risk_touchpoints = sorted(
    {s for s in all_symbols for p in high_risk_patterns if p in s}
)

data = {
    "summary": {
        "all_symbols": len(all_symbols),
        "contract_exports": len(contract_exports),
        "runtime_exports": len(runtime_exports),
        "thunk_exports": len(thunk_exports),
        "mangled_symbols": len(mangled_symbols),
        "high_risk_touchpoints": len(high_risk_touchpoints),
    },
    "risk_policy": {
        "high": ["high_risk_touchpoints"],
        "medium": ["mangled_symbols", "thunk_exports"],
        "low": ["contract_exports", "runtime_exports"],
    },
    "symbols": {
        "contract_exports": contract_exports,
        "runtime_exports": runtime_exports,
        "thunk_exports": thunk_exports,
        "mangled_symbols": mangled_symbols,
        "high_risk_touchpoints": high_risk_touchpoints,
    },
}

out_path.write_text(json.dumps(data, indent=2))
print(f"Wrote {out_path}")
PY

python3 - "$OUT_JSON" "$OUT_MD" <<'PY'
import json
import pathlib
import sys

json_path = pathlib.Path(sys.argv[1])
md_path = pathlib.Path(sys.argv[2])
obj = json.loads(json_path.read_text())
summary = obj["summary"]
symbols = obj["symbols"]

lines = [
    "# AP.2 Runtime Touchpoint Inventory",
    "",
    "## Summary",
    "",
    f"- total symbols: {summary['all_symbols']}",
    f"- contract exports: {summary['contract_exports']}",
    f"- runtime exports: {summary['runtime_exports']}",
    f"- thunk exports: {summary['thunk_exports']}",
    f"- mangled symbols: {summary['mangled_symbols']}",
    f"- high-risk touchpoints: {summary['high_risk_touchpoints']}",
    "",
    "## High-Risk Touchpoints",
]

if symbols["high_risk_touchpoints"]:
    for entry in symbols["high_risk_touchpoints"]:
        lines.append(f"- {entry}")
else:
    lines.append("- none")

lines += [
    "",
    "## Risk Classification Policy",
    "",
    "- high: raw runtime allocation/refcount/type lookup touchpoints",
    "- medium: runtime thunk usage and mangled symbol dependence",
    "- low: contract-scoped stable bridge exports",
]

md_path.write_text("\n".join(lines) + "\n")
print(f"Wrote {md_path}")
PY

rm -f "$TMP_SYMBOLS"
