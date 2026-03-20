#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WATCH_JSON="$ROOT/target/runtime-probe/o9-distributed-watch/o9-distributed-watch-summary.json"
OUT_DIR="$ROOT/target/runtime-probe/o9-distributed-actor"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
SUMMARY_JSON="$OUT_DIR/o9-distributed-actor-summary.json"
SUMMARY_MD="$OUT_DIR/o9-distributed-actor-summary.md"

mkdir -p "$OUT_DIR"

watch_status="UNKNOWN"
watch_reason="watch artifact missing"

if [[ -f "$WATCH_JSON" ]]; then
  read -r watch_status watch_reason <<EOF
$(python3 - "$WATCH_JSON" <<'PY'
import json
import sys

with open(sys.argv[1], 'r', encoding='utf-8') as fh:
    data = json.load(fh)

status = data.get('watch_status', 'UNKNOWN')
reason = data.get('watch_reason', 'not provided').replace('"', '\\"')
print(status, reason)
PY
)
EOF
fi

# O13 gate skeleton policy:
# - If host is not fully supported, skip (non-blocking) and emit deterministic artifact.
# - If host is supported, emit transitional "PENDING_IMPLEMENTATION" state until O14 probes exist.
if [[ "$watch_status" != "SUPPORTED" ]]; then
  gate_status="SKIPPED_UNSUPPORTED"
  result="PASS"
  detail="Distributed runtime not fully supported on host; O9 gate intentionally skipped"
elif [[ "${O9_ENABLE_IMPLEMENTATION:-0}" != "1" ]]; then
  gate_status="PENDING_IMPLEMENTATION"
  result="PASS"
  detail="Host is SUPPORTED but O9 probes are deferred to Wave O14; set O9_ENABLE_IMPLEMENTATION=1 when probes are implemented"
else
  gate_status="PENDING_IMPLEMENTATION"
  result="PASS"
  detail="O9 implementation flag enabled, but O14 probe suite is not implemented yet"
fi

cat > "$SUMMARY_JSON" <<EOF
{
  "timestamp": "$STAMP",
  "gate": "o9_distributed_actor_surface",
  "result": "$result",
  "gate_status": "$gate_status",
  "watch_status": "$watch_status",
  "watch_reason": "$watch_reason",
  "detail": "$detail",
  "implementation_ready": false,
  "o14_probe_suite_present": false
}
EOF

cat > "$SUMMARY_MD" <<EOF
# O.9 Distributed Actor Gate (Wave O13 Skeleton)

- timestamp: $STAMP
- result: **$result**
- gate_status: $gate_status
- watch_status: $watch_status
- watch_reason: $watch_reason
- detail: $detail

This is a Wave O13 skeleton gate. It records host capability and transitional status,
and intentionally defers real distributed-actor probe execution to Wave O14.
EOF

echo "Wrote $SUMMARY_JSON"
echo "Wrote $SUMMARY_MD"
echo "O.9 distributed actor gate skeleton PASS"
