#!/bin/bash
# Dictionary parity probe driver for Track C.3
# Tests: empty dict, insert/get, upsert, remove, missing-key, hash collision safety (50 keys),
#        Dict<Int32,OpaqueRef> basic insert/get/contains, Dict<Int32,OpaqueRef> remove

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

echo "Running dictionary parity probe..."
export DYLD_LIBRARY_PATH="$FIXTURE_DIR:$BUILD_DIR"
cargo run --example runtime_dict_probe --release -- 2>&1

echo ""
echo "Dictionary parity probe PASS"
