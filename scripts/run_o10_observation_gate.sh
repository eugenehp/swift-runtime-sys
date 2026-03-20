#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/o10-observation"
FIXTURE_DIR="$ROOT/target/runtime-probe/resilient-fixtures"
MD_OUT="$OUT_DIR/o10-observation-summary.md"
JSON_OUT="$OUT_DIR/o10-observation-summary.json"
HISTORY_DIR="$OUT_DIR/history"

mkdir -p "$OUT_DIR"
mkdir -p "$HISTORY_DIR"
cd "$ROOT"

bash ./scripts/build_runtime_thunks.sh >/dev/null
mkdir -p "$FIXTURE_DIR"
swiftc -emit-library -emit-module \
  -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
  -enable-library-evolution -g \
  -module-name ResilientFixtures \
  -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
  examples/ResilientFixtures.swift
swiftc -emit-library -g \
  -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures \
  -o libRustBridge.dylib examples/RustBridge.swift

cargo build --example runtime_o10_observation_probe >/dev/null
cargo build --release --example runtime_o10_observation_probe >/dev/null

DEBUG_LOG="$OUT_DIR/o10-observation-debug.log"
RELEASE_LOG="$OUT_DIR/o10-observation-release.log"

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_o10_observation_probe" | tee "$DEBUG_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_o10_observation_probe" | tee "$RELEASE_LOG"

debug_passed="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_passed="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"

if [[ -z "$debug_passed" || -z "$release_passed" ]]; then
  echo "Missing summary output from O.10 probe" >&2
  exit 1
fi

status="PASS"
if [[ "$debug_passed" != "$release_passed" ]]; then
  status="FAIL"
fi

cat > "$JSON_OUT" <<EOF
{
  "gate": "o10_observation_runtime_surface",
  "status": "$status",
  "debug_pass": "$debug_passed",
  "release_pass": "$release_passed",
  "debug_log": "$(basename "$DEBUG_LOG")",
  "release_log": "$(basename "$RELEASE_LOG")"
}
EOF

RUN_TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
GIT_HASH="$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown")"
HISTORY_FILE="$HISTORY_DIR/${RUN_TS//[: ]/_}_${GIT_HASH}.json"
cat > "$HISTORY_FILE" <<EOF
{
  "timestamp": "${RUN_TS}",
  "gate": "o10_observation_runtime_surface",
  "status": "${status}",
  "debug_pass": "${debug_passed}",
  "release_pass": "${release_passed}"
}
EOF

cat > "$MD_OUT" <<EOF
# O.10 Observation Runtime Surface Gate

- status: **$status**
- debug passed: $debug_passed
- release passed: $release_passed
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")

Gate passes when debug and release probe pass counts are identical.
EOF

echo "Wrote $JSON_OUT"
echo "Wrote $MD_OUT"
echo "History record: $HISTORY_FILE"

if [[ "$status" != "PASS" ]]; then
  echo "O.10 observation gate FAILED" >&2
  exit 1
fi

echo "O.10 observation gate PASSED"
