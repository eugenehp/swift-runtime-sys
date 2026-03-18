#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="${1:-$ROOT/target/runtime-probe/n6-corpus}"
RUNS="${2:-25}"
FRAGMENTS="${3:-12}"
FIXTURE_DIR="$ROOT/target/runtime-probe/resilient-fixtures"

mkdir -p "$FIXTURE_DIR"

cd "$ROOT"
./scripts/build_runtime_thunks.sh >/dev/null 2>&1
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
cargo build --example runtime_differential_fuzz_probe
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." ./target/debug/examples/runtime_differential_fuzz_probe --campaign "$RUNS" "$FRAGMENTS" "$OUT_DIR"