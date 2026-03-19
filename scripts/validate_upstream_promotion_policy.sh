#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARTIFACT_ROOT="${1:-$ROOT/target/runtime-probe/upstream-conformance/current}"
TARGETS_JSON="${TARGETS_JSON:-$ROOT/scripts/upstream_conformance_targets.json}"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_MD="$OUT_DIR/upstream-promotion-signoff.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

python3 - "$ARTIFACT_ROOT" "$TARGETS_JSON" "$OUT_MD" <<'PY'
import glob
import json
import pathlib
import sys

artifact_root = pathlib.Path(sys.argv[1])
targets_path = pathlib.Path(sys.argv[2])
out_md = pathlib.Path(sys.argv[3])

targets = json.loads(targets_path.read_text(encoding="utf-8"))
tracked_refs = targets.get("tracked_refs", [])

rows = []
issues = []

for ref in tracked_refs:
    candidates = list(artifact_root.glob(f"{ref}.json"))
    if not candidates:
        candidates = [pathlib.Path(p) for p in glob.glob(str(artifact_root / f"**/{ref}.json"), recursive=True)]
    if not candidates:
        rows.append((ref, "missing", "missing", "FAIL"))
        issues.append(f"missing conformance artifact for {ref}")
        continue

path = candidates[0]
data = json.loads(path.read_text(encoding="utf-8"))
result = data.get("result", "FAIL")
sha = data.get("upstream_sha", "")
status = "PASS" if result == "PASS" and sha else "FAIL"
if status != "PASS":
    issues.append(f"non-converged ref {ref}: result={result} sha_present={bool(sha)}")
    if not sha:
        sha = "missing"
    rows.append((ref, sha, result, status))
else:
    rows.append((ref, sha, result, status))

overall = "PASS" if not issues else "FAIL"
lines = [
    "# Upstream Promotion Signoff",
    "",
    f"- targets: {targets_path}",
    f"- artifact_root: {artifact_root}",
    f"- require_all_refs_green: {targets.get('promotion_policy', {}).get('require_all_refs_green', True)}",
    f"- result: {overall}",
    "",
    "| Upstream Ref | Upstream SHA | Result | Status |",
    "|---|---|---|---|",
]
for row in rows:
    lines.append(f"| {row[0]} | {row[1]} | {row[2]} | {row[3]} |")
lines += ["", "## Validation Issues"]
if not issues:
    lines.append("- none")
else:
    lines.extend([f"- {issue}" for issue in issues])

out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(f"Wrote {out_md}")

if issues:
    sys.exit(1)
PY

echo "upstream promotion signoff passed"