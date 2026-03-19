#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
BUNDLE_ROOT="$OUT_DIR/claim-bundle"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
BUNDLE_DIR="$BUNDLE_ROOT/$STAMP"
EVIDENCE_DIR="$BUNDLE_DIR/evidence"
MANIFEST_JSON="$BUNDLE_DIR/manifest.json"
MANIFEST_SIG="$BUNDLE_DIR/manifest.sha256"
ARCHIVE="$BUNDLE_ROOT/claim-evidence-bundle-$STAMP.tar.gz"
ARCHIVE_SHA="$ARCHIVE.sha256"

mkdir -p "$EVIDENCE_DIR"
cd "$ROOT"

required=(
  scripts/parity_claim_contract.json
  target/runtime-probe/repro-inputs.json
  target/runtime-probe/parity-claim-signoff.md
  target/runtime-probe/parity-report.json
  target/runtime-probe/parity-report.md
  target/runtime-probe/support-matrix-signoff.md
  target/runtime-probe/promotion-policy-signoff.md
  target/runtime-probe/plan-completion-signoff.md
  target/runtime-probe/abi-shape-closure.json
  target/runtime-probe/abi-shape-closure.md
  target/runtime-probe/ap4-differential-oracle.md
  target/runtime-probe/ap4-differential-campaign/campaign-summary.json
  target/runtime-probe/ap5-soak/ap5-soak-summary.json
  target/runtime-probe/ap5-soak/ap5-soak-summary.md
  target/runtime-probe/ap5-soak/ap5-stability-trend.md
)

for rel in "${required[@]}"; do
  if [[ ! -f "$rel" ]]; then
    echo "missing required evidence file: $rel" >&2
    exit 1
  fi
  mkdir -p "$EVIDENCE_DIR/$(dirname "$rel")"
  cp "$rel" "$EVIDENCE_DIR/$rel"
done

python3 - "$EVIDENCE_DIR" "$MANIFEST_JSON" <<'PY'
import hashlib
import json
import pathlib
import sys

evidence_dir = pathlib.Path(sys.argv[1])
manifest_path = pathlib.Path(sys.argv[2])

entries = []
for path in sorted(p for p in evidence_dir.rglob("*") if p.is_file()):
    rel = path.relative_to(evidence_dir).as_posix()
    h = hashlib.sha256(path.read_bytes()).hexdigest()
    entries.append({"path": rel, "sha256": h})

manifest = {
    "version": 1,
    "file_count": len(entries),
    "entries": entries,
}
manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
print(f"Wrote {manifest_path}")
PY

shasum -a 256 "$MANIFEST_JSON" | awk '{print $1}' > "$MANIFEST_SIG"

tar -czf "$ARCHIVE" -C "$BUNDLE_ROOT" "$STAMP"
shasum -a 256 "$ARCHIVE" | awk '{print $1}' > "$ARCHIVE_SHA"

echo "Wrote $ARCHIVE"
echo "Wrote $ARCHIVE_SHA"
