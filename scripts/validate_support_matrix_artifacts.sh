#!/usr/bin/env bash
set -euo pipefail

# Validate parity artifacts across required support-matrix cells.
#
# Usage:
#   ./scripts/validate_support_matrix_artifacts.sh <artifact-root>
#
# Expected artifact layout (from actions/download-artifact):
#   <artifact-root>/parity-report-macos-14-<sha>/parity-report.json
#   <artifact-root>/parity-report-macos-15-<sha>/parity-report.json
#   <artifact-root>/contract-parity-macos-14-<sha>/contract-dispatch.log
#   <artifact-root>/contract-parity-macos-15-<sha>/contract-dispatch.log

ARTIFACT_ROOT="${1:-target/ci/parity-artifacts}"
REQUIRED_CELLS="${REQUIRED_CELLS:-macos-14 macos-15}"
OUT_DIR="target/runtime-probe"
OUT_MD="$OUT_DIR/support-matrix-signoff.md"

mkdir -p "$OUT_DIR"

if [[ ! -d "$ARTIFACT_ROOT" ]]; then
  echo "artifact root not found: $ARTIFACT_ROOT" >&2
  exit 1
fi

if ! ls "$ARTIFACT_ROOT"/parity-report-*/parity-report.json >/dev/null 2>&1; then
  echo "no parity-report.json files found under $ARTIFACT_ROOT" >&2
  exit 1
fi

same_total=""
status_ok=1

cat > "$OUT_MD" <<MD
# Support Matrix Signoff

Required cells: ${REQUIRED_CELLS}

## Parity Matrix

| Cell | Pass Ratio | Artifact JSON | Result |
|---|---:|---|---|
MD

for cell in $REQUIRED_CELLS; do
  json="$(ls "$ARTIFACT_ROOT"/parity-report-${cell}-*/parity-report.json 2>/dev/null | head -n 1 || true)"
  if [[ -z "$json" || ! -f "$json" ]]; then
    echo "| ${cell} | missing | missing | FAIL |" >> "$OUT_MD"
    status_ok=0
    continue
  fi

  passed="$(python3 -c "import json; d=json.load(open('$json')); s=d.get('status', {}); print(int(d.get('passed', sum(1 for v in s.values() if int(v)==1))))")"
  total="$(python3 -c "import json; d=json.load(open('$json')); s=d.get('status', {}); print(int(d.get('total', len(s))))")"

  if [[ "$same_total" == "" ]]; then
    same_total="$total"
  elif [[ "$same_total" != "$total" ]]; then
    status_ok=0
  fi

  if [[ "$passed" -eq "$total" && "$total" -gt 0 ]]; then
    echo "| ${cell} | ${passed}/${total} | ${json} | PASS |" >> "$OUT_MD"
  else
    echo "| ${cell} | ${passed}/${total} | ${json} | FAIL |" >> "$OUT_MD"
    status_ok=0
  fi
done

cat >> "$OUT_MD" <<MD

## Contract Parity

| Cell | Dispatch Artifact | Result |
|---|---|---|
MD

for cell in $REQUIRED_CELLS; do
  dispatch_log="$(ls "$ARTIFACT_ROOT"/contract-parity-${cell}-*/contract-dispatch.log 2>/dev/null | head -n 1 || true)"
  if [[ -z "$dispatch_log" || ! -f "$dispatch_log" ]]; then
    echo "| ${cell} | missing | FAIL |" >> "$OUT_MD"
    status_ok=0
    continue
  fi

  if grep -q "normalized=true" "$dispatch_log" && \
     grep -q "metadata_registry=true" "$dispatch_log" && \
     grep -q "protocol_registry=true" "$dispatch_log" && \
     grep -q "generic_metadata=Supported" "$dispatch_log" && \
     grep -q "protocol_registry=Supported" "$dispatch_log"; then
    echo "| ${cell} | ${dispatch_log} | PASS |" >> "$OUT_MD"
  else
    echo "| ${cell} | ${dispatch_log} | FAIL |" >> "$OUT_MD"
    status_ok=0
  fi
done

echo "Wrote $OUT_MD"

if [[ "$status_ok" -ne 1 ]]; then
  echo "support matrix signoff failed" >&2
  exit 1
fi

echo "support matrix signoff passed"
