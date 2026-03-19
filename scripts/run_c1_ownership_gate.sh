#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"
MD_OUT="$OUT_DIR/c1-ownership-gate.md"
JSON_OUT="$OUT_DIR/c1-ownership-gate.json"

mkdir -p "$OUT_DIR" "$FIXTURE_DIR"
cd "$ROOT"

./scripts/build_runtime_thunks.sh >/dev/null
swiftc -emit-library -emit-module \
  -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
  -enable-library-evolution -g \
  -module-name ResilientFixtures \
  -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
  examples/ResilientFixtures.swift
swiftc -emit-library -g \
  -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures \
  -o libRustBridge.dylib examples/RustBridge.swift

cargo build --example runtime_c1_ownership_probe >/dev/null
cargo build --release --example runtime_c1_ownership_probe >/dev/null

DEBUG_LOG="$OUT_DIR/c1-ownership-debug.log"
RELEASE_LOG="$OUT_DIR/c1-ownership-release.log"

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_c1_ownership_probe" | tee "$DEBUG_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_c1_ownership_probe" | tee "$RELEASE_LOG"

debug_summary="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_summary="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"

if [[ -z "$debug_summary" || -z "$release_summary" ]]; then
  echo "Missing summary output from C.1 probe" >&2
  exit 1
fi

status="PASS"
if [[ "$debug_summary" != "$release_summary" ]]; then
  status="FAIL"
fi

cat > "$JSON_OUT" <<EOF
{
  "gate": "c1_ownership_hardening",
  "status": "$status",
  "debug_pass": "$debug_summary",
  "release_pass": "$release_summary",
  "debug_log": "$(basename "$DEBUG_LOG")",
  "release_log": "$(basename "$RELEASE_LOG")"
}
EOF

cat > "$MD_OUT" <<EOF
# C.1 Ownership Hardening Gate

- status: **$status**
- debug passed: $debug_summary
- release passed: $release_summary
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")

Semantics are considered stable when debug and release pass counts are identical.
EOF

echo "Wrote $JSON_OUT"
echo "Wrote $MD_OUT"

if [[ "$status" != "PASS" ]]; then
  echo "C.1 ownership gate FAILED" >&2
  exit 1
fi

echo "C.1 ownership gate PASSED"
