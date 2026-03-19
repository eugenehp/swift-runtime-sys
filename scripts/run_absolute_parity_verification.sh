#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TARGETS_JSON="${TARGETS_JSON:-$ROOT/scripts/upstream_conformance_targets.json}"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_MD="$OUT_DIR/absolute-parity-signoff.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

echo "[absolute] stage=phase_c_signoff start"
bash ./scripts/run_phase_c_signoff.sh
echo "[absolute] stage=phase_c_signoff done"

echo "[absolute] stage=ap6_claim_verifier start"
./scripts/run_ap6_claim_verifier.sh
echo "[absolute] stage=ap6_claim_verifier done"

refs="$(python3 - "$TARGETS_JSON" <<'PY'
import json
import sys

targets = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
print(' '.join(targets.get('tracked_refs', [])))
PY
)"

for ref in $refs; do
  echo "[absolute] stage=upstream_conformance ref=${ref} start"
  ./scripts/run_upstream_conformance.sh "$ref"
  echo "[absolute] stage=upstream_conformance ref=${ref} done"
done

echo "[absolute] stage=upstream_promotion_policy start"
./scripts/validate_upstream_promotion_policy.sh
echo "[absolute] stage=upstream_promotion_policy done"

timestamp_utc="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

cat > "$OUT_MD" <<MD
# Absolute Parity Signoff

- timestamp_utc: ${timestamp_utc}
- phase_c_signoff: PASS
- ap6_claim_verifier: PASS
- upstream_refs: ${refs}
- upstream_promotion_policy: PASS
- result: PASS
MD

echo "Wrote $OUT_MD"
echo "Absolute parity verification PASS"
