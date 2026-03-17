#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$LOG_DIR/resilient-fixtures"
LOG_FILE="$LOG_DIR/lldb.log"

mkdir -p "$LOG_DIR"
mkdir -p "$FIXTURE_DIR"

cd "$ROOT"
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
./scripts/build_runtime_thunks.sh
cargo build --example runtime_raw_probe

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." lldb -b -s scripts/lldb_runtime_cmds.txt "target/debug/examples/runtime_raw_probe" > "$LOG_FILE" 2>&1 || true

echo "Wrote $LOG_FILE"
