#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/o9-distributed-watch"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
WATCH_JSON="$OUT_DIR/o9-distributed-watch-summary.json"
GATE_JSON="$ROOT/target/runtime-probe/o9-distributed-actor/o9-distributed-actor-summary.json"
ELIG_JSON="$OUT_DIR/o9-promotion-eligibility-summary.json"
ELIG_MD="$OUT_DIR/o9-promotion-eligibility-summary.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

# Refresh watch and gate skeleton artifacts each run.
bash ./scripts/run_o9_distributed_watch.sh >/dev/null
bash ./scripts/run_o9_distributed_actor_gate.sh >/dev/null

watch_status="UNKNOWN"
watch_reason="watch artifact missing"
gate_status="UNKNOWN"

if [[ -f "$WATCH_JSON" ]]; then
  read -r watch_status watch_reason <<EOF
$(python3 - "$WATCH_JSON" <<'PY'
import json
import sys

with open(sys.argv[1], 'r', encoding='utf-8') as fh:
    data = json.load(fh)

print(
    data.get('watch_status', 'UNKNOWN'),
    data.get('watch_reason', 'not provided').replace('"', '\\"')
)
PY
)
EOF
fi

if [[ -f "$GATE_JSON" ]]; then
  gate_status="$(python3 - "$GATE_JSON" <<'PY'
import json
import sys

with open(sys.argv[1], 'r', encoding='utf-8') as fh:
    data = json.load(fh)

print(data.get('gate_status', 'UNKNOWN'))
PY
)"
fi

promotion_eligible=false
eligibility_reason=""

if [[ "$watch_status" == "SUPPORTED" && "$gate_status" == "PASS" ]]; then
  promotion_eligible=true
  eligibility_reason="Host support and O9 gate both PASS; promotion can be considered"
elif [[ "$watch_status" == "SUPPORTED" ]]; then
  promotion_eligible=false
  eligibility_reason="Host support available, but O9 gate is not PASS (transitional/O14 implementation pending)"
else
  promotion_eligible=false
  eligibility_reason="Host support not yet complete; wait for watch_status=SUPPORTED"
fi

cat > "$ELIG_JSON" <<EOF
{
  "timestamp": "$STAMP",
  "watch_status": "$watch_status",
  "watch_reason": "$watch_reason",
  "gate_status": "$gate_status",
  "promotion_eligible": $promotion_eligible,
  "eligibility_reason": "$eligibility_reason"
}
EOF

cat > "$ELIG_MD" <<EOF
# O.9 Promotion Eligibility Watch

- timestamp: $STAMP
- watch_status: $watch_status
- watch_reason: $watch_reason
- gate_status: $gate_status
- promotion_eligible: $promotion_eligible
- eligibility_reason: $eligibility_reason

Promotion requires both conditions:
1. watch_status == SUPPORTED
2. o9 gate_status == PASS
EOF

echo "Wrote $ELIG_JSON"
echo "Wrote $ELIG_MD"
