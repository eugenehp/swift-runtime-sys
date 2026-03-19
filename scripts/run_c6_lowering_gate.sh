#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"

mkdir -p "$OUT_DIR" "$FIXTURE_DIR"
cd "$ROOT"

echo "=== Building C.6 Bridge & Thunks ==="
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
cargo build --release >/dev/null

echo ""
echo "=== Phase C.6: Dynamic Call-Lowering Expansion ==="
echo ""

echo "Building C.6 probe (debug)..."
cargo build --example runtime_c6_lowering_probe >/dev/null

echo "Running C.6 probe (debug)..."
DEBUG_LOG="$OUT_DIR/c6-lowering-debug.log"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_c6_lowering_probe" | tee "$DEBUG_LOG"
DEBUG_PASSED="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
DEBUG_STATUS="$(grep -E '^Status:' "$DEBUG_LOG" | tail -1)"

echo ""
echo "Building C.6 probe (release)..."
cargo build --release --example runtime_c6_lowering_probe >/dev/null

echo "Running C.6 probe (release)..."
RELEASE_LOG="$OUT_DIR/c6-lowering-release.log"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_c6_lowering_probe" | tee "$RELEASE_LOG"
RELEASE_PASSED="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"
RELEASE_STATUS="$(grep -E '^Status:' "$RELEASE_LOG" | tail -1)"

echo ""
echo "=== C.6 Comparative Summary ==="
echo "Debug:   $DEBUG_PASSED ($DEBUG_STATUS)"
echo "Release: $RELEASE_PASSED ($RELEASE_STATUS)"

{
  echo "{"
  echo '  "phase": "C.6",'
  echo '  "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",'
  echo '  "test_type": "dynamic_lowering",'
  echo '  "debug_passed": "'$DEBUG_PASSED'",'
  echo '  "release_passed": "'$RELEASE_PASSED'",'
  echo '  "equivalence": "'$([ "$DEBUG_PASSED" = "$RELEASE_PASSED" ] && echo "EQUIV" || echo "DIVERGE")'"'
  echo "}"
} > "$OUT_DIR/c6-lowering-gate.json"

{
  echo "# Phase C.6: Dynamic Lowering Gate Results"
  echo ""
  echo "**Debug Build**: $DEBUG_PASSED ($DEBUG_STATUS)"
  echo ""
  echo "**Release Build**: $RELEASE_PASSED ($RELEASE_STATUS)"
  echo ""
  if [[ "$DEBUG_PASSED" = "$RELEASE_PASSED" ]]; then
    echo "**Equivalence**: PASS"
  else
    echo "**Equivalence**: FAIL"
  fi
} > "$OUT_DIR/c6-lowering-gate.md"

if [[ "$DEBUG_PASSED" != "$RELEASE_PASSED" ]]; then
  echo "C.6 gate failed: debug/release divergence" >&2
  exit 1
fi

echo ""
echo "C.6 gate PASSED: debug and release are equivalent ($DEBUG_PASSED)."
