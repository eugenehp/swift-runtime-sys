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
REQUIRED_PROFILES="${REQUIRED_PROFILES:-debug release}"
MATRIX_CONTRACT="${MATRIX_CONTRACT:-scripts/support_matrix_contract.json}"
USE_MATRIX_CONTRACT="${USE_MATRIX_CONTRACT:-1}"
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

if [[ "$USE_MATRIX_CONTRACT" == "1" ]]; then
  if [[ ! -f "$MATRIX_CONTRACT" ]]; then
    echo "matrix contract not found: $MATRIX_CONTRACT" >&2
    exit 1
  fi
  REQUIRED_CELLS="$(python3 - "$MATRIX_CONTRACT" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cells = [entry['cell'] for entry in data.get('required_cells', [])]
print(' '.join(cells))
PY
)"
fi

same_total=""
status_ok=1

cat > "$OUT_MD" <<MD
# Support Matrix Signoff

Required cells: ${REQUIRED_CELLS}
Required profiles: ${REQUIRED_PROFILES}
Matrix contract: ${MATRIX_CONTRACT} (enabled=${USE_MATRIX_CONTRACT})

## Parity Matrix

| Cell | Profile | Pass Ratio | Artifact JSON | Manifest | Result |
|---|---|---:|---|---|---|
MD

for cell in $REQUIRED_CELLS; do
  profiles_for_cell="$REQUIRED_PROFILES"
  expected_arch=""
  expected_swift_prefix=""
  if [[ "$USE_MATRIX_CONTRACT" == "1" ]]; then
    profiles_for_cell="$(python3 - "$MATRIX_CONTRACT" "$cell" <<'PY'
import json
import sys

contract = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cell = sys.argv[2]
for entry in contract.get('required_cells', []):
    if entry.get('cell') == cell:
        print(' '.join(entry.get('profiles', [])))
        break
PY
)"
    expected_arch="$(python3 - "$MATRIX_CONTRACT" "$cell" <<'PY'
import json
import sys

contract = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cell = sys.argv[2]
for entry in contract.get('required_cells', []):
    if entry.get('cell') == cell:
        print(entry.get('arch', ''))
        break
PY
)"
    expected_swift_prefix="$(python3 - "$MATRIX_CONTRACT" "$cell" <<'PY'
import json
import sys

contract = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cell = sys.argv[2]
for entry in contract.get('required_cells', []):
    if entry.get('cell') == cell:
        print(entry.get('swift_version_prefix', ''))
        break
PY
)"
  fi

  for profile in $profiles_for_cell; do
    json="$(ls "$ARTIFACT_ROOT"/parity-report-${cell}-${profile}-*/parity-report.json 2>/dev/null | head -n 1 || true)"
    if [[ -z "$json" || ! -f "$json" ]]; then
      echo "| ${cell} | ${profile} | missing | missing | missing | FAIL |" >> "$OUT_MD"
      status_ok=0
      continue
    fi

    parity_dir="$(dirname "$json")"
    manifest_json="$parity_dir/support-cell-manifest.json"
    manifest_result="PASS"
    if [[ ! -f "$manifest_json" ]]; then
      manifest_result="missing"
      status_ok=0
    else
      if ! python3 - "$manifest_json" "$cell" "$profile" "$expected_arch" "$expected_swift_prefix" <<'PY'
import json
import sys

manifest_path, exp_cell, exp_profile, exp_arch, exp_swift_prefix = sys.argv[1:]
m = json.load(open(manifest_path, 'r', encoding='utf-8'))

ok = True
if m.get('gate') != 'parity':
    ok = False
if m.get('cell') != exp_cell:
    ok = False
if m.get('profile') != exp_profile:
    ok = False
if exp_arch and m.get('arch') != exp_arch:
    ok = False
if exp_swift_prefix and not str(m.get('swift_version', '')).startswith(exp_swift_prefix):
    ok = False

if not ok:
    raise SystemExit(1)
PY
      then
        manifest_result="mismatch"
        status_ok=0
      fi
    fi

    passed="$(python3 -c "import json; d=json.load(open('$json')); s=d.get('status', {}); print(int(d.get('passed', sum(1 for v in s.values() if int(v)==1))))")"
    total="$(python3 -c "import json; d=json.load(open('$json')); s=d.get('status', {}); print(int(d.get('total', len(s))))")"

    if [[ "$same_total" == "" ]]; then
      same_total="$total"
    elif [[ "$same_total" != "$total" ]]; then
      status_ok=0
    fi

    if [[ "$passed" -eq "$total" && "$total" -gt 0 ]]; then
      if [[ "$manifest_result" == "PASS" ]]; then
        echo "| ${cell} | ${profile} | ${passed}/${total} | ${json} | ${manifest_json} | PASS |" >> "$OUT_MD"
      else
        echo "| ${cell} | ${profile} | ${passed}/${total} | ${json} | ${manifest_result} | FAIL |" >> "$OUT_MD"
        status_ok=0
      fi
    else
      echo "| ${cell} | ${profile} | ${passed}/${total} | ${json} | ${manifest_result} | FAIL |" >> "$OUT_MD"
      status_ok=0
    fi
  done
done

cat >> "$OUT_MD" <<MD

## Contract Parity

| Cell | Dispatch Artifact | Manifest | Result |
|---|---|---|---|
MD

for cell in $REQUIRED_CELLS; do
  profiles_for_cell="$REQUIRED_PROFILES"
  expected_arch=""
  expected_swift_prefix=""
  if [[ "$USE_MATRIX_CONTRACT" == "1" ]]; then
    profiles_for_cell="$(python3 - "$MATRIX_CONTRACT" "$cell" <<'PY'
import json
import sys

contract = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cell = sys.argv[2]
for entry in contract.get('required_cells', []):
    if entry.get('cell') == cell:
        print(' '.join(entry.get('profiles', [])))
        break
PY
)"
    expected_arch="$(python3 - "$MATRIX_CONTRACT" "$cell" <<'PY'
import json
import sys

contract = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cell = sys.argv[2]
for entry in contract.get('required_cells', []):
    if entry.get('cell') == cell:
        print(entry.get('arch', ''))
        break
PY
)"
    expected_swift_prefix="$(python3 - "$MATRIX_CONTRACT" "$cell" <<'PY'
import json
import sys

contract = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
cell = sys.argv[2]
for entry in contract.get('required_cells', []):
    if entry.get('cell') == cell:
        print(entry.get('swift_version_prefix', ''))
        break
PY
)"
  fi

  for profile in $profiles_for_cell; do
    dispatch_log="$(ls "$ARTIFACT_ROOT"/contract-parity-${cell}-${profile}-*/contract-dispatch.log 2>/dev/null | head -n 1 || true)"
    if [[ -z "$dispatch_log" || ! -f "$dispatch_log" ]]; then
      echo "| ${cell}/${profile} | missing | missing | FAIL |" >> "$OUT_MD"
      status_ok=0
      continue
    fi

    contract_dir="$(dirname "$dispatch_log")"
    manifest_json="$contract_dir/support-cell-manifest.json"
    manifest_result="PASS"
    if [[ ! -f "$manifest_json" ]]; then
      manifest_result="missing"
      status_ok=0
    else
      if ! python3 - "$manifest_json" "$cell" "$profile" "$expected_arch" "$expected_swift_prefix" <<'PY'
import json
import sys

manifest_path, exp_cell, exp_profile, exp_arch, exp_swift_prefix = sys.argv[1:]
m = json.load(open(manifest_path, 'r', encoding='utf-8'))

ok = True
if m.get('gate') != 'contract':
    ok = False
if m.get('cell') != exp_cell:
    ok = False
if m.get('profile') != exp_profile:
    ok = False
if exp_arch and m.get('arch') != exp_arch:
    ok = False
if exp_swift_prefix and not str(m.get('swift_version', '')).startswith(exp_swift_prefix):
    ok = False

if not ok:
    raise SystemExit(1)
PY
      then
        manifest_result="mismatch"
        status_ok=0
      fi
    fi

    if grep -q "normalized=true" "$dispatch_log" && \
       grep -q "metadata_registry=true" "$dispatch_log" && \
       grep -q "protocol_registry=true" "$dispatch_log" && \
       grep -q "generic_metadata=Supported" "$dispatch_log" && \
       grep -q "protocol_registry=Supported" "$dispatch_log"; then
      if [[ "$manifest_result" == "PASS" ]]; then
        echo "| ${cell}/${profile} | ${dispatch_log} | ${manifest_json} | PASS |" >> "$OUT_MD"
      else
        echo "| ${cell}/${profile} | ${dispatch_log} | ${manifest_result} | FAIL |" >> "$OUT_MD"
        status_ok=0
      fi
    else
      echo "| ${cell}/${profile} | ${dispatch_log} | ${manifest_result} | FAIL |" >> "$OUT_MD"
      status_ok=0
    fi
  done
done

echo "Wrote $OUT_MD"

if [[ "$status_ok" -ne 1 ]]; then
  echo "support matrix signoff failed" >&2
  exit 1
fi

echo "support matrix signoff passed"
