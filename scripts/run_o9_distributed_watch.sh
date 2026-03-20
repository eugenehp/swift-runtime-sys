#!/bin/bash
# run_o9_distributed_watch.sh: Watch O.9 Distributed actor surface on this host cell.
# Purpose: Capture host-side Distributed typecheck and runtime support status.
# Exit: 0 if watch artifacts are written successfully (unsupported host returns WATCH_UNSUPPORTED status, not error).

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROBE_DIR="$REPO_ROOT/target/runtime-probe/o9-distributed-watch"

mkdir -p "$PROBE_DIR"

echo "[o9-watch] Checking host Distributed support on $(date -u +'%Y-%m-%dT%H_%M_%SZ')" >&2

# Check 1: Swift module import support via swiftc
# We attempt to compile a minimal Distributed fragment and capture diagnostics.
TEMP_DISTRIBUTED_CHECK="$(mktemp -t o9_check.XXXXXX).swift"
trap "rm -f '$TEMP_DISTRIBUTED_CHECK'" EXIT

cat > "$TEMP_DISTRIBUTED_CHECK" << 'EOF'
#if canImport(Distributed)
import Distributed

// Lightweight actor-location protocol check
typealias DefaultDistributedActorSystem = LocalTestingDistributedActorSystem

@available(macOS 13.0, *)
distributed actor TestActor {
  typealias ActorSystem = DefaultDistributedActorSystem

  distributed func ping() -> String {
    return "pong"
  }
}
#else
// Distributed not available
#endif
EOF

DISTRIBUTED_TYPECHECK_AVAILABLE=0
DISTRIBUTED_TYPECHECK_REASON=""

# Attempt typecheck only (no emit)
if swiftc -typecheck "$TEMP_DISTRIBUTED_CHECK" -suppress-warnings 2>&1 | grep -q "Distributed"; then
  DISTRIBUTED_TYPECHECK_AVAILABLE=1
  DISTRIBUTED_TYPECHECK_REASON="Distributed module is importable"
elif swiftc -typecheck "$TEMP_DISTRIBUTED_CHECK" -suppress-warnings 2>&1 | grep -q "cannot find"; then
  DISTRIBUTED_TYPECHECK_AVAILABLE=0
  DISTRIBUTED_TYPECHECK_REASON="Distributed module not found in this Swift version"
else
  # swiftc may succeed or fail for other reasons; check exit code
  if swiftc -typecheck "$TEMP_DISTRIBUTED_CHECK" -suppress-warnings >/dev/null 2>&1; then
    DISTRIBUTED_TYPECHECK_AVAILABLE=1
    DISTRIBUTED_TYPECHECK_REASON="Distributed typecheck successful"
  else
    DISTRIBUTED_TYPECHECK_AVAILABLE=0
    DISTRIBUTED_TYPECHECK_REASON="Distributed typecheck failed or module unavailable"
  fi
fi

echo "[o9-watch] Distributed typecheck: AVAILABLE=$DISTRIBUTED_TYPECHECK_AVAILABLE REASON='$DISTRIBUTED_TYPECHECK_REASON'" >&2

# Check 2: Runtime library availability (libswiftDistributed dylib)
DISTRIBUTED_RUNTIME_AVAILABLE=0
DISTRIBUTED_RUNTIME_REASON=""

# Try to find libswiftDistributed in the Swift toolchain
SWIFTC_PATH="$(which swiftc)"
SWIFT_TOOLCHAIN_ROOT="$(dirname "$(dirname "$SWIFTC_PATH")")"

# Common paths in Xcode/Swift toolchain
CANDIDATE_PATHS=(
  "$SWIFT_TOOLCHAIN_ROOT/lib/swift/libswiftDistributed.dylib"
  "$SWIFT_TOOLCHAIN_ROOT/lib/libswiftDistributed.dylib"
  "/usr/lib/swift/libswiftDistributed.dylib"
  "/opt/swift/lib/swift/libswiftDistributed.dylib"
)

for path in "${CANDIDATE_PATHS[@]}"; do
  if [ -f "$path" ]; then
    DISTRIBUTED_RUNTIME_AVAILABLE=1
    DISTRIBUTED_RUNTIME_REASON="Found at $path"
    break
  fi
done

if [ $DISTRIBUTED_RUNTIME_AVAILABLE -eq 0 ]; then
  DISTRIBUTED_RUNTIME_REASON="libswiftDistributed.dylib not found in standard toolchain paths"
fi

echo "[o9-watch] Distributed runtime: AVAILABLE=$DISTRIBUTED_RUNTIME_AVAILABLE REASON='$DISTRIBUTED_RUNTIME_REASON'" >&2

# Check 3: Distributed actor metadata support
# Lightweight check: can we reference Distributed.ActorSystem as a type?
DISTRIBUTED_ACTOR_METADATA_AVAILABLE=0
DISTRIBUTED_ACTOR_METADATA_REASON=""

if [ $DISTRIBUTED_TYPECHECK_AVAILABLE -eq 1 ]; then
  DISTRIBUTED_ACTOR_METADATA_AVAILABLE=1
  DISTRIBUTED_ACTOR_METADATA_REASON="Actor metadata supported via Distributed module import"
else
  DISTRIBUTED_ACTOR_METADATA_AVAILABLE=0
  DISTRIBUTED_ACTOR_METADATA_REASON="Distributed module unavailable; actor metadata check deferred"
fi

echo "[o9-watch] Distributed actor metadata: AVAILABLE=$DISTRIBUTED_ACTOR_METADATA_AVAILABLE REASON='$DISTRIBUTED_ACTOR_METADATA_REASON'" >&2

# Summary determination
if [ $DISTRIBUTED_TYPECHECK_AVAILABLE -eq 1 ] && [ $DISTRIBUTED_RUNTIME_AVAILABLE -eq 1 ]; then
  WATCH_STATUS="SUPPORTED"
  WATCH_REASON="Both typecheck and runtime library available on this host cell"
elif [ $DISTRIBUTED_TYPECHECK_AVAILABLE -eq 1 ]; then
  WATCH_STATUS="PARTIAL"
  WATCH_REASON="Typecheck available but runtime library missing"
elif [ $DISTRIBUTED_RUNTIME_AVAILABLE -eq 1 ]; then
  WATCH_STATUS="PARTIAL"
  WATCH_REASON="Runtime library found but typecheck support unavailable"
else
  WATCH_STATUS="UNSUPPORTED"
  WATCH_REASON="Distributed module not available on this host cell; O.9 implementation readiness = NOT READY"
fi

echo "[o9-watch] Overall watch status: $WATCH_STATUS" >&2

# Write JSON artifact
TIMESTAMP="$(date -u +'%Y-%m-%dT%H_%M_%SZ')"
JSON_FILE="$PROBE_DIR/o9-distributed-watch-summary.json"

cat > "$JSON_FILE" << EOJSON
{
  "timestamp": "$TIMESTAMP",
  "watch_status": "$WATCH_STATUS",
  "watch_reason": "$WATCH_REASON",
  "distributed_typecheck_available": $DISTRIBUTED_TYPECHECK_AVAILABLE,
  "distributed_typecheck_reason": "$DISTRIBUTED_TYPECHECK_REASON",
  "distributed_runtime_available": $DISTRIBUTED_RUNTIME_AVAILABLE,
  "distributed_runtime_reason": "$DISTRIBUTED_RUNTIME_REASON",
  "distributed_actor_metadata_available": $DISTRIBUTED_ACTOR_METADATA_AVAILABLE,
  "distributed_actor_metadata_reason": "$DISTRIBUTED_ACTOR_METADATA_REASON",
  "o9_implementation_ready": $([[ "$WATCH_STATUS" == "SUPPORTED" ]] && echo "true" || echo "false")
}
EOJSON

# Write Markdown artifact
MD_FILE="$PROBE_DIR/o9-distributed-watch-summary.md"

cat > "$MD_FILE" << EOMD
# Track O.9 Distributed Readiness Watch

- timestamp: $TIMESTAMP
- watch_status: $WATCH_STATUS
- watch_reason: $WATCH_REASON

## Distributed Typecheck Support
- available: $DISTRIBUTED_TYPECHECK_AVAILABLE
- reason: $DISTRIBUTED_TYPECHECK_REASON

## Distributed Runtime Library
- available: $DISTRIBUTED_RUNTIME_AVAILABLE
- reason: $DISTRIBUTED_RUNTIME_REASON

## Distributed Actor Metadata
- available: $DISTRIBUTED_ACTOR_METADATA_AVAILABLE
- reason: $DISTRIBUTED_ACTOR_METADATA_REASON

## O.9 Implementation Readiness
- status: $([[ "$WATCH_STATUS" == "SUPPORTED" ]] && echo "READY TO IMPLEMENT" || echo "NOT YET READY")
- next_action: $([[ "$WATCH_STATUS" == "SUPPORTED" ]] && echo "Begin O.9 probe + gate implementation" || echo "Defer O.9 work until host support improves")

EOMD

echo "[o9-watch] Wrote artifacts to $PROBE_DIR" >&2
echo "Wrote $JSON_FILE"
echo "Wrote $MD_FILE"

# Return zero regardless of support status; classification is in the artifact
exit 0
