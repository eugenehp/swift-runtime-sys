#!/usr/bin/env bash
set -euo pipefail

# Validate promotion policy for moving compiler-feature-dependent paths to required.
#
# Usage:
#   ./scripts/validate_promotion_policy.sh <artifact-root>
#
# Environment:
#   REQUIRED_CELLS="macos-14 macos-15"
#   PROMOTION_HISTORY_WINDOW=2
#
# Policy checks:
# - required cells have full parity pass and contract parity pass
# - required cells are represented in README deviation ledger (no undocumented cells)
# - each required cell has a green parity history window of N snapshots

ARTIFACT_ROOT="${1:-target/ci/parity-artifacts}"
REQUIRED_CELLS="${REQUIRED_CELLS:-macos-14 macos-15}"
PROMOTION_HISTORY_WINDOW="${PROMOTION_HISTORY_WINDOW:-2}"
OUT_DIR="target/runtime-probe"
OUT_MD="$OUT_DIR/promotion-policy-signoff.md"
README_FILE="README.md"

mkdir -p "$OUT_DIR"

if [[ ! -d "$ARTIFACT_ROOT" ]]; then
  echo "artifact root not found: $ARTIFACT_ROOT" >&2
  exit 1
fi

if [[ ! -f "$README_FILE" ]]; then
  echo "README not found: $README_FILE" >&2
  exit 1
fi

status_ok=1

cat > "$OUT_MD" <<MD
# Promotion Policy Signoff

Required cells: ${REQUIRED_CELLS}
History window: ${PROMOTION_HISTORY_WINDOW}

| Cell | Deviation Ledger Entry | Parity | Contract | Green Window | Result |
|---|---|---:|---|---:|---|
MD

is_green_snapshot() {
  local json_file="$1"
  python3 - <<'PY' "$json_file"
import json,sys
p=sys.argv[1]
with open(p,'r',encoding='utf-8') as f:
    d=json.load(f)
status=d.get('status',{})
passed=int(d.get('passed', sum(1 for v in status.values() if int(v)==1)))
total=int(d.get('total', len(status)))
print('1' if total>0 and passed==total else '0')
PY
}

for cell in $REQUIRED_CELLS; do
  ledger_ok="no"
  parity_ratio="missing"
  contract_ok="FAIL"
  window_ratio="0/${PROMOTION_HISTORY_WINDOW}"
  row_result="FAIL"

  if grep -Fq -- "- \`${cell}\`:" "$README_FILE"; then
    ledger_ok="yes"
  else
    status_ok=0
  fi

  parity_json="$(ls "$ARTIFACT_ROOT"/parity-report-${cell}-*/parity-report.json 2>/dev/null | head -n 1 || true)"
  if [[ -n "$parity_json" && -f "$parity_json" ]]; then
    passed="$(python3 -c "import json; d=json.load(open('$parity_json')); s=d.get('status', {}); print(int(d.get('passed', sum(1 for v in s.values() if int(v)==1))))")"
    total="$(python3 -c "import json; d=json.load(open('$parity_json')); s=d.get('status', {}); print(int(d.get('total', len(s))))")"
    parity_ratio="${passed}/${total}"
    if [[ "$passed" -ne "$total" || "$total" -le 0 ]]; then
      status_ok=0
    fi
  else
    status_ok=0
  fi

  dispatch_log="$(ls "$ARTIFACT_ROOT"/contract-parity-${cell}-*/contract-dispatch.log 2>/dev/null | head -n 1 || true)"
  if [[ -n "$dispatch_log" && -f "$dispatch_log" ]] && \
     grep -q "normalized=true" "$dispatch_log" && \
     grep -q "metadata_registry=true" "$dispatch_log" && \
     grep -q "protocol_registry=true" "$dispatch_log"; then
    contract_ok="PASS"
  else
    status_ok=0
  fi

  green_count=0
  history_files=()
  while IFS= read -r line; do
    history_files+=("$line")
  done < <(ls -t "$ARTIFACT_ROOT"/parity-report-${cell}-*/history/*.json 2>/dev/null || true)

  if [[ ${#history_files[@]} -eq 0 && -n "$parity_json" && -f "$parity_json" ]]; then
    history_files+=("$parity_json")
  fi

  if [[ ${#history_files[@]} -ge "$PROMOTION_HISTORY_WINDOW" ]]; then
    for ((i=0; i<PROMOTION_HISTORY_WINDOW; i++)); do
      if [[ "$(is_green_snapshot "${history_files[$i]}")" == "1" ]]; then
        green_count=$((green_count + 1))
      fi
    done
  fi

  window_ratio="${green_count}/${PROMOTION_HISTORY_WINDOW}"
  if [[ "$green_count" -ne "$PROMOTION_HISTORY_WINDOW" ]]; then
    status_ok=0
  fi

  if [[ "$ledger_ok" == "yes" && "$contract_ok" == "PASS" && "$green_count" -eq "$PROMOTION_HISTORY_WINDOW" ]]; then
    if [[ "$parity_ratio" != "missing" ]] && [[ "${parity_ratio%%/*}" == "${parity_ratio##*/}" ]]; then
      row_result="PASS"
    fi
  fi

  echo "| ${cell} | ${ledger_ok} | ${parity_ratio} | ${contract_ok} | ${window_ratio} | ${row_result} |" >> "$OUT_MD"
done

echo "Wrote $OUT_MD"

if [[ "$status_ok" -ne 1 ]]; then
  echo "promotion policy signoff failed" >&2
  exit 1
fi

echo "promotion policy signoff passed"
