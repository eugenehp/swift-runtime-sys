#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PLAN_FILE="${1:-$ROOT/PLAN.md}"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_MD="$OUT_DIR/plan-completion-signoff.md"

mkdir -p "$OUT_DIR"

if [[ ! -f "$PLAN_FILE" ]]; then
  echo "plan file not found: $PLAN_FILE" >&2
  exit 1
fi

scope_lines="$(awk '
  /^## Absolute Parity Closure Program \(Post-N\.8\)/ { exit }
  { print }
' "$PLAN_FILE")"

unchecked_count="$(printf '%s\n' "$scope_lines" | grep -Ec '^- \[ \] ' || true)"
checked_count="$(printf '%s\n' "$scope_lines" | grep -Ec '^- \[x\] ' || true)"

required_evidence_files=(
  "$ROOT/target/runtime-probe/contract-parity.md"
  "$ROOT/target/runtime-probe/support-matrix-signoff.md"
  "$ROOT/target/runtime-probe/promotion-policy-signoff.md"
  "$ROOT/scripts/run_full_plan_verification.sh"
)

missing_count=0
missing_paths=()
for path in "${required_evidence_files[@]}"; do
  if [[ ! -e "$path" ]]; then
    missing_count=$((missing_count + 1))
    missing_paths+=("$path")
  fi
done

cat > "$OUT_MD" <<MD
# Plan Completion Signoff

- Plan file: ${PLAN_FILE}
- Checked items: ${checked_count}
- Unchecked items: ${unchecked_count}
- Required evidence files present: $(( ${#required_evidence_files[@]} - missing_count ))/${#required_evidence_files[@]}
MD

if [[ "$unchecked_count" -ne 0 ]]; then
  echo "\n## Unchecked Items\n" >> "$OUT_MD"
  printf '%s\n' "$scope_lines" | grep -E '^- \[ \] ' >> "$OUT_MD" || true
  echo "Wrote $OUT_MD"
  echo "plan completion signoff failed" >&2
  exit 1
fi

if [[ "$missing_count" -ne 0 ]]; then
  echo "\n## Missing Evidence Files\n" >> "$OUT_MD"
  for path in "${missing_paths[@]}"; do
    echo "- ${path}" >> "$OUT_MD"
  done
  echo "Wrote $OUT_MD"
  echo "plan completion signoff failed" >&2
  exit 1
fi

echo "\nResult: PASS" >> "$OUT_MD"
echo "Wrote $OUT_MD"
echo "plan completion signoff passed"
