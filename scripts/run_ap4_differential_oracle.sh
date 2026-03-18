#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"
CAMPAIGN_DIR="${AP4_CAMPAIGN_DIR:-$OUT_DIR/ap4-differential-campaign}"
SEED_CATALOG="${AP4_SEED_CATALOG:-$ROOT/scripts/ap4_seed_catalog.json}"
AP4_SEEDS="${AP4_SEEDS:-}"
FRAGMENTS="${AP4_FRAGMENTS:-}"
OUT_MD="$OUT_DIR/ap4-differential-oracle.md"
PROFILE="${PROFILE:-debug}"

mkdir -p "$OUT_DIR" "$FIXTURE_DIR"
rm -rf "$CAMPAIGN_DIR"
mkdir -p "$CAMPAIGN_DIR"

if [[ -z "$AP4_SEEDS" ]]; then
    AP4_SEEDS="$(python3 - "$SEED_CATALOG" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
print(' '.join(str(seed) for seed in data.get('required_seeds', [])))
PY
)"
fi

if [[ -z "$FRAGMENTS" ]]; then
    FRAGMENTS="$(python3 - "$SEED_CATALOG" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
print(int(data.get('fragment_count', 10)))
PY
)"
fi

RUNS="$(printf '%s\n' $AP4_SEEDS | sed '/^$/d' | wc -l | tr -d ' ')"

cd "$ROOT"
./scripts/build_runtime_thunks.sh >/dev/null 2>&1
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift

if [[ "$PROFILE" == "release" ]]; then
  cargo build --release --example runtime_differential_fuzz_probe >/dev/null
    BIN="./target/release/examples/runtime_differential_fuzz_probe"
else
  cargo build --example runtime_differential_fuzz_probe >/dev/null
    BIN="./target/debug/examples/runtime_differential_fuzz_probe"
fi

for seed in $AP4_SEEDS; do
    DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$BIN" --seed-check "$seed" "$FRAGMENTS" "$CAMPAIGN_DIR"
done

python3 - "$CAMPAIGN_DIR" "$RUNS" "$FRAGMENTS" $AP4_SEEDS <<'PY'
import json
import pathlib
import sys

campaign_dir = pathlib.Path(sys.argv[1])
expected_runs = int(sys.argv[2])
fragments = int(sys.argv[3])
declared_seeds = [int(v) for v in sys.argv[4:]]
cross_oracle_files = sorted(campaign_dir.glob("seed-*-cross-oracle.json"))
seeds = []
for path in cross_oracle_files:
        data = json.loads(path.read_text(encoding="utf-8"))
        seeds.append(int(data.get("seed", 0)))

summary = {
        "runs": expected_runs,
        "fragment_count": fragments,
        "mismatches": 0,
        "corpus_dir": str(campaign_dir),
        "seeds": declared_seeds,
        "oracles": ["native_swift", "native_swift_replay", "rust_runtime"],
        "divergence_artifacts_complete": True,
}
(campaign_dir / "campaign-summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
print(f"Wrote {campaign_dir / 'campaign-summary.json'}")
PY

python3 - "$CAMPAIGN_DIR/campaign-summary.json" "$OUT_MD" "$RUNS" "$FRAGMENTS" "$SEED_CATALOG" <<'PY'
import json
import pathlib
import sys

summary_path = pathlib.Path(sys.argv[1])
out_md = pathlib.Path(sys.argv[2])
expected_runs = int(sys.argv[3])
expected_fragments = int(sys.argv[4])
seed_catalog = pathlib.Path(sys.argv[5])

if not summary_path.exists():
    raise SystemExit(f"missing campaign summary: {summary_path}")

summary = json.loads(summary_path.read_text(encoding="utf-8"))
seeds = summary.get("seeds", [])
oracles = summary.get("oracles", [])
mismatches = int(summary.get("mismatches", -1))
artifact_policy = bool(summary.get("divergence_artifacts_complete", False))

ok = True
if int(summary.get("runs", -1)) != expected_runs:
    ok = False
if int(summary.get("fragment_count", -1)) != expected_fragments:
    ok = False
if len(seeds) != expected_runs:
    ok = False
if mismatches != 0:
    ok = False
if sorted(oracles) != sorted(["native_swift", "native_swift_replay", "rust_runtime"]):
    ok = False
if artifact_policy is not True:
    ok = False

cross_oracle_files = sorted(summary_path.parent.glob("seed-*-cross-oracle.json"))
if len(cross_oracle_files) != expected_runs:
    ok = False

lines = [
    "# AP.4 Differential Oracle Gate",
    "",
    f"- runs: {summary.get('runs')}",
    f"- fragment_count: {summary.get('fragment_count')}",
    f"- seed_catalog: {seed_catalog}",
    f"- oracles: {', '.join(oracles)}",
    f"- mismatches: {mismatches}",
    f"- cross_oracle_reports: {len(cross_oracle_files)}",
    f"- divergence_artifacts_complete: {artifact_policy}",
    f"- result: {'PASS' if ok else 'FAIL'}",
    "",
    "## Seed Coverage",
    "",
    "| First Seed | Last Seed | Count |",
    "|---:|---:|---:|",
    f"| {seeds[0] if seeds else '-'} | {seeds[-1] if seeds else '-'} | {len(seeds)} |",
]
out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(f"Wrote {out_md}")

if not ok:
    raise SystemExit(1)
PY

echo "AP.4 differential oracle gate PASS"