#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="$ROOT/target/runtime-probe"
LOG_FILE="$LOG_DIR/lldb.log"

mkdir -p "$LOG_DIR"

cd "$ROOT"
swiftc -emit-library -g -o libRustBridge.dylib examples/RustBridge.swift
./scripts/build_runtime_thunks.sh
cargo build --example runtime_raw_probe

DYLD_LIBRARY_PATH='.' lldb -b -s scripts/lldb_runtime_cmds.txt "target/debug/examples/runtime_raw_probe" > "$LOG_FILE" 2>&1 || true

echo "Wrote $LOG_FILE"
