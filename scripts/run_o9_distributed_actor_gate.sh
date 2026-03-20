#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WATCH_JSON="$ROOT/target/runtime-probe/o9-distributed-watch/o9-distributed-watch-summary.json"
OUT_DIR="$ROOT/target/runtime-probe/o9-distributed-actor"
FIXTURE_DIR="$ROOT/target/runtime-probe/resilient-fixtures"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
SUMMARY_JSON="$OUT_DIR/o9-distributed-actor-summary.json"
SUMMARY_MD="$OUT_DIR/o9-distributed-actor-summary.md"
DEBUG_LOG="$OUT_DIR/o9-distributed-actor-debug.log"
RELEASE_LOG="$OUT_DIR/o9-distributed-actor-release.log"

mkdir -p "$OUT_DIR"
mkdir -p "$FIXTURE_DIR"

cd "$ROOT"

bash ./scripts/build_runtime_thunks.sh >/dev/null
swiftc -emit-library -emit-module \
  -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
  -enable-library-evolution -g \
  -module-name ResilientFixtures \
  -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
  examples/ResilientFixtures.swift
swiftc -emit-library -g \
  -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures \
  -o libRustBridge.dylib examples/RustBridge.swift

cargo build --example runtime_o9_distributed_probe >/dev/null
cargo build --release --example runtime_o9_distributed_probe >/dev/null

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_o9_distributed_probe" | tee "$DEBUG_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_o9_distributed_probe" | tee "$RELEASE_LOG"

debug_passed="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_passed="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"
debug_flags="$(grep -E '^o9 distributed scaffold parity =>' "$DEBUG_LOG" | tail -1 | sed 's/^o9 distributed scaffold parity => //')"
release_flags="$(grep -E '^o9 distributed scaffold parity =>' "$RELEASE_LOG" | tail -1 | sed 's/^o9 distributed scaffold parity => //')"

if [[ -z "$debug_passed" || -z "$release_passed" ]]; then
  echo "Missing summary output from O.9 scaffold probe" >&2
  exit 1
fi

scaffold_probe_status="PASS"
if [[ "$debug_passed" != "$release_passed" || "$debug_flags" != "$release_flags" ]]; then
  scaffold_probe_status="FAIL"
fi

watch_status="UNKNOWN"
watch_reason="watch artifact missing"
host_support_blocker="watch_artifact_missing"
readiness_phase="watch-missing"
binding_export_constant_count=0

if [[ -f "$WATCH_JSON" ]]; then
  read -r watch_status host_support_blocker readiness_phase binding_export_constant_count watch_reason <<EOF
$(python3 - "$WATCH_JSON" <<'PY'
import json
import sys

with open(sys.argv[1], 'r', encoding='utf-8') as fh:
    data = json.load(fh)

status = data.get('watch_status', 'UNKNOWN')
blocker = data.get('host_support_blocker', 'unknown')
phase = data.get('readiness_phase', 'watch-missing')
binding_count = data.get('binding_export_constant_count', 0)
reason = data.get('watch_reason', 'not provided').replace('"', '\\"')
print(status, blocker, phase, binding_count, reason)
PY
)
EOF
fi

# O13 gate skeleton policy:
# - If host is not fully supported, skip (non-blocking) and emit deterministic artifact.
# - If host is supported, emit transitional "PENDING_IMPLEMENTATION" state until O14 probes exist.
if [[ "$scaffold_probe_status" != "PASS" ]]; then
  gate_status="FAIL_SCAFFOLD"
  result="FAIL"
  detail="O9 scaffold probe failed (debug_pass=$debug_passed, release_pass=$release_passed)"
elif [[ "$watch_status" != "SUPPORTED" ]]; then
  gate_status="SKIPPED_UNSUPPORTED"
  result="PASS"
  detail="Distributed runtime not fully supported on host; O9 execution skipped after scaffold validation (blocker=$host_support_blocker, readiness_phase=$readiness_phase, binding_export_constant_count=$binding_export_constant_count)"
elif [[ "${O9_ENABLE_IMPLEMENTATION:-0}" != "1" ]]; then
  gate_status="PENDING_IMPLEMENTATION"
  result="PASS"
  detail="Host is SUPPORTED and scaffold validated, but real O9 probes are deferred to Wave O14; set O9_ENABLE_IMPLEMENTATION=1 when probes are implemented"
else
  gate_status="PENDING_IMPLEMENTATION"
  result="PASS"
  detail="O9 implementation flag enabled, scaffold validated, but the real O14 probe suite is not implemented yet"
fi

cat > "$SUMMARY_JSON" <<EOF
{
  "timestamp": "$STAMP",
  "gate": "o9_distributed_actor_surface",
  "result": "$result",
  "gate_status": "$gate_status",
  "watch_status": "$watch_status",
  "watch_reason": "$watch_reason",
  "host_support_blocker": "$host_support_blocker",
  "readiness_phase": "$readiness_phase",
  "binding_export_constant_count": $binding_export_constant_count,
  "scaffold_probe_status": "$scaffold_probe_status",
  "debug_pass": "$debug_passed",
  "release_pass": "$release_passed",
  "debug_flags": "$debug_flags",
  "release_flags": "$release_flags",
  "debug_log": "$(basename "$DEBUG_LOG")",
  "release_log": "$(basename "$RELEASE_LOG")",
  "detail": "$detail",
  "implementation_ready": false,
  "o14_probe_scaffold_present": true,
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
- host_support_blocker: $host_support_blocker
- readiness_phase: $readiness_phase
- binding_export_constant_count: $binding_export_constant_count
- scaffold_probe_status: $scaffold_probe_status
- debug passed: $debug_passed
- release passed: $release_passed
- debug flags: $debug_flags
- release flags: $release_flags
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")
- detail: $detail

This gate now validates the dormant O14b scaffold path on every host cell, while
still deferring real distributed-actor runtime execution until host support reaches SUPPORTED.
EOF

echo "Wrote $SUMMARY_JSON"
echo "Wrote $SUMMARY_MD"

if [[ "$result" != "PASS" ]]; then
  echo "O.9 distributed actor gate FAILED" >&2
  exit 1
fi

echo "O.9 distributed actor gate PASS"
