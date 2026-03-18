#!/bin/bash
# Dynamic cast parity probe driver for Track D.1
# Tests: metatype identity, successful narrow cast, failed narrow cast,
#        metatype comparison (two objects same type), round-trip cast + dispatch

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${REPO_ROOT}/target/runtime-probe"
FIXTURE_DIR="${BUILD_DIR}/resilient-fixtures"
mkdir -p "$BUILD_DIR"
mkdir -p "$FIXTURE_DIR"

cd "$REPO_ROOT"

echo "Building runtime thunks..."
./scripts/build_runtime_thunks.sh > /dev/null 2>&1

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

echo "Building RustBridge.dylib..."
swiftc \
    -emit-library \
    -g \
    -I "$FIXTURE_DIR" \
    -L "$FIXTURE_DIR" \
    -lResilientFixtures \
    -o "$BUILD_DIR/libRustBridge.dylib" \
    examples/RustBridge.swift

echo "Running dynamic cast parity probe..."
export DYLD_LIBRARY_PATH="$FIXTURE_DIR:$BUILD_DIR"
cargo run --example runtime_cast_probe --release -- 2>&1

echo ""
echo "Dynamic cast parity probe PASS"
