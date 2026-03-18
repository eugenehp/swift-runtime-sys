#!/bin/bash
# String parity probe driver for Track C.1
# Tests: empty strings, ASCII, UTF-8 multibyte, null-termination safety

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${REPO_ROOT}/target/runtime-probe"
FIXTURE_DIR="${BUILD_DIR}/resilient-fixtures"
mkdir -p "$BUILD_DIR"
mkdir -p "$FIXTURE_DIR"

cd "$REPO_ROOT"

# Build runtime thunks
echo "Building runtime thunks..."
./scripts/build_runtime_thunks.sh > /dev/null 2>&1

# Build ResilientFixtures
echo "Building ResilientFixtures module..."
swiftc \
    -emit-library \
    -emit-module \
    -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
    -enable-library-evolution \
    -g \
    -module-name ResilientFixtures \
    -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
    examples/ResilientFixtures.swift

# Build RustBridge
echo "Building RustBridge.dylib..."
swiftc \
    -emit-library \
    -g \
    -I "$FIXTURE_DIR" \
    -L "$FIXTURE_DIR" \
    -lResilientFixtures \
    -o "$BUILD_DIR/libRustBridge.dylib" \
    examples/RustBridge.swift

# Build and run probe using cargo
echo "Running string parity probe..."
export DYLD_LIBRARY_PATH="$FIXTURE_DIR:$BUILD_DIR"
cargo run --example runtime_string_probe --release -- 2>&1

echo ""
echo "String parity probe PASS"
