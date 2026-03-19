#!/bin/bash
set -e

WORKSPACE=/Users/Shared/swift-runtime-sys
ARTIFACT_DIR="$WORKSPACE/target/runtime-probe"
FIXTURE_DIR="$ARTIFACT_DIR/resilient-fixtures"

mkdir -p "$FIXTURE_DIR"

echo "=== Building C.4 Bridge & Thunks ==="
cd "$WORKSPACE"
bash scripts/build_runtime_thunks.sh >/dev/null 2>&1 || true

swiftc -emit-library -emit-module \
  -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
  -enable-library-evolution -g \
  -module-name ResilientFixtures \
  -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
  examples/ResilientFixtures.swift

swiftc -emit-library -g \
  -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures \
  -o libRustBridge.dylib examples/RustBridge.swift

cargo build --release 2>&1 | grep -E "Compiling swift-runtime-sys|Finished" | head -5

echo ""
echo "=== Phase C.4: Closure & Async-Capture Semantics ==="
echo ""

echo "Building C.4 probe (debug)..."
cargo build --example runtime_c4_closure_probe 2>&1 | grep -E "Compiling|Finished" | head -3

echo "Running C.4 probe (debug)..."
DEBUG_OUTPUT=$(DYLD_LIBRARY_PATH="$FIXTURE_DIR:." cargo run --example runtime_c4_closure_probe 2>&1 | tee /tmp/c4_debug.log)
DEBUG_PASSED=$(echo "$DEBUG_OUTPUT" | grep "Passed:" | head -1 | grep -oE "[0-9]+/[0-9]+" || echo "0/0")
DEBUG_STATUS=$(echo "$DEBUG_OUTPUT" | grep "Status:" | head -1 || echo "UNKNOWN")

echo ""
echo "Building C.4 probe (release)..."
cargo build --example runtime_c4_closure_probe --release 2>&1 | grep -E "Compiling|Finished" | head -3

echo "Running C.4 probe (release)..."
RELEASE_OUTPUT=$(DYLD_LIBRARY_PATH="$FIXTURE_DIR:." cargo run --example runtime_c4_closure_probe --release 2>&1 | tee /tmp/c4_release.log)
RELEASE_PASSED=$(echo "$RELEASE_OUTPUT" | grep "Passed:" | head -1 | grep -oE "[0-9]+/[0-9]+" || echo "0/0")
RELEASE_STATUS=$(echo "$RELEASE_OUTPUT" | grep "Status:" | head -1 || echo "UNKNOWN")

echo ""
echo "=== C.4 Comparative Summary ==="
echo "Debug:   $DEBUG_PASSED ($DEBUG_STATUS)"
echo "Release: $RELEASE_PASSED ($RELEASE_STATUS)"

mkdir -p "$ARTIFACT_DIR"

{
  echo "{"
  echo '  "phase": "C.4",'
  echo '  "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",'
  echo '  "test_type": "closure_async",'
  echo '  "debug_passed": "'$DEBUG_PASSED'",'
  echo '  "release_passed": "'$RELEASE_PASSED'",'
  echo '  "equivalence": "'$([ "$DEBUG_PASSED" = "$RELEASE_PASSED" ] && echo "EQUIV" || echo "DIVERGE")'"'
  echo "}"
} > "$ARTIFACT_DIR/c4-closure-gate.json"

{
  echo "# Phase C.4: Closure & Async Gate Results"
  echo ""
  echo "**Debug Build**: $DEBUG_PASSED ($DEBUG_STATUS)"
  echo ""
  echo "**Release Build**: $RELEASE_PASSED ($RELEASE_STATUS)"
  echo ""
  if [ "$DEBUG_PASSED" = "$RELEASE_PASSED" ]; then
    echo "**Equivalence**: ✓ PASS (debug and release identical)"
  else
    echo "**Equivalence**: ✗ FAIL (divergence detected)"
  fi
  echo ""
  echo "## Full Output"
  echo ""
  echo "### Debug"
  echo '```'
  cat /tmp/c4_debug.log
  echo '```'
  echo ""
  echo "### Release"
  echo '```'
  cat /tmp/c4_release.log
  echo '```'
} > "$ARTIFACT_DIR/c4-closure-gate.md"

echo ""
echo "Wrote $ARTIFACT_DIR/c4-closure-gate.json"
echo "Wrote $ARTIFACT_DIR/c4-closure-gate.md"

if echo "$DEBUG_STATUS" | grep -q "ALL TESTS PASSED" && echo "$RELEASE_STATUS" | grep -q "ALL TESTS PASSED"; then
  echo ""
  echo "C.4 closure gate PASSED"
  exit 0
else
  echo ""
  echo "C.4 closure gate FAILED"
  exit 1
fi
