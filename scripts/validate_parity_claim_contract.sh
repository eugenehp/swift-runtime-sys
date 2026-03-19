#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
CONTRACT_JSON="${CLAIM_CONTRACT_JSON:-$ROOT/scripts/parity_claim_contract.json}"
REPRO_JSON="${REPRO_INPUTS_JSON:-$OUT_DIR/repro-inputs.json}"
OUT_MD="$OUT_DIR/parity-claim-signoff.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

if [[ ! -f "$CONTRACT_JSON" ]]; then
  echo "claim contract missing: $CONTRACT_JSON" >&2
  exit 1
fi

if [[ ! -f "$REPRO_JSON" ]]; then
  echo "repro inputs missing: $REPRO_JSON" >&2
  exit 1
fi

python3 - "$CONTRACT_JSON" "$REPRO_JSON" "$OUT_MD" "$ROOT" <<'PY'
import json
import pathlib
import re
import sys

contract_path = pathlib.Path(sys.argv[1])
repro_path = pathlib.Path(sys.argv[2])
out_md = pathlib.Path(sys.argv[3])
root = pathlib.Path(sys.argv[4])

contract = json.loads(contract_path.read_text(encoding="utf-8"))
repro = json.loads(repro_path.read_text(encoding="utf-8"))

issues = []

def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))

def require_file(rel_path):
    p = root / rel_path
    if not p.exists():
        issues.append(f"missing artifact: {rel_path}")
        return None
    return p

parity_json_p = require_file("target/runtime-probe/parity-report.json")
ap4_json_p = require_file("target/runtime-probe/ap4-differential-campaign/campaign-summary.json")
ap5_json_p = require_file("target/runtime-probe/ap5-soak/ap5-soak-summary.json")
support_md_p = require_file("target/runtime-probe/support-matrix-signoff.md")
promotion_md_p = require_file("target/runtime-probe/promotion-policy-signoff.md")
plan_md_p = require_file("target/runtime-probe/plan-completion-signoff.md")
phase_c_md_p = require_file("target/runtime-probe/phase-c-signoff/phase-c-signoff.md")

if parity_json_p:
    parity = read_json(parity_json_p)
    status = parity.get("status", {})
    passed = int(parity.get("passed", parity.get("pass_count", sum(1 for v in status.values() if int(v) == 1))))
    total = int(parity.get("total", parity.get("total_checks", len(status))))
    if total <= 0 or passed != total:
        issues.append("parity report not fully green")

if ap4_json_p:
    ap4 = read_json(ap4_json_p)
    min_ap4_seeds = int(contract["minimum_budgets"]["ap4_seed_count"])
    min_fragments = int(contract["minimum_budgets"]["ap4_fragment_count"])
    if int(ap4.get("mismatches", -1)) != 0:
        issues.append("AP.4 mismatches must be zero")
    seeds = ap4.get("seeds", [])
    if len(seeds) < min_ap4_seeds:
        issues.append(f"AP.4 seed count below minimum ({len(seeds)} < {min_ap4_seeds})")
    if int(ap4.get("fragment_count", -1)) != min_fragments:
        issues.append("AP.4 fragment count mismatch")
    oracles = sorted(ap4.get("oracles", []))
    if oracles != sorted(["native_swift", "native_swift_replay", "rust_runtime"]):
        issues.append("AP.4 oracle set mismatch")

if ap5_json_p:
    ap5 = read_json(ap5_json_p)
    flake_budget = int(contract["minimum_budgets"]["ap5_flake_budget"])
    min_soak_runs = int(contract["minimum_budgets"]["ap5_soak_runs"])
    if int(ap5.get("flake_count", 0)) > flake_budget:
        issues.append("AP.5 flake budget exceeded")
    if int(ap5.get("fail_count", 0)) != 0:
        issues.append("AP.5 has hard failures")
    if int(ap5.get("soak_runs", 0)) < min_soak_runs:
        issues.append("AP.5 soak runs below minimum")

def require_signoff_contains(path, needle):
    if path is None:
        return
    text = path.read_text(encoding="utf-8")
    if needle not in text:
        issues.append(f"signoff missing '{needle}': {path}")

require_signoff_contains(support_md_p, "PASS")
require_signoff_contains(promotion_md_p, "PASS")
require_signoff_contains(phase_c_md_p, "PASS")
if plan_md_p is not None:
    text = plan_md_p.read_text(encoding="utf-8")
    if "Result: PASS" not in text and "passed" not in text.lower():
        issues.append(f"plan completion signoff is not PASS: {plan_md_p}")

pin = contract["toolchain_pins"]
rust_channel = pin["rust_toolchain_channel"]
swift_prefix = pin["swift_version_prefix"]

rustc_version = repro.get("toolchains", {}).get("rustc", "")
swift_line1 = repro.get("toolchains", {}).get("swift_line1", "")
cargo_lock_sha = repro.get("pins", {}).get("cargo_lock_sha256", "")
if not rustc_version.startswith(f"rustc {rust_channel}"):
    issues.append("rustc version does not match claim pin")
if not swift_line1.startswith(swift_prefix):
    issues.append("swift version does not match claim pin")
if contract["toolchain_pins"].get("cargo_lock_required", False) and not cargo_lock_sha:
    issues.append("cargo lock hash missing in repro inputs")

status = "PASS" if not issues else "FAIL"
lines = [
    "# Parity Claim Signoff",
    "",
    f"- contract: {contract_path}",
    f"- repro_inputs: {repro_path}",
    f"- scope: {contract.get('scope', {}).get('id', 'unknown')}",
    f"- result: {status}",
    "",
    "## Validation",
]
if not issues:
    lines.append("- all checks passed")
else:
    lines.extend([f"- {issue}" for issue in issues])

out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(f"Wrote {out_md}")

if issues:
    sys.exit(1)
PY

echo "parity claim signoff passed"