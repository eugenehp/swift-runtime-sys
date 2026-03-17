#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_FILE="$OUT_DIR/protocol-dispatch-matrix.md"

mkdir -p "$OUT_DIR"
cd "$ROOT"

./scripts/build_runtime_thunks.sh
swiftc -emit-library -g -o libRustBridge.dylib examples/RustBridge.swift
cargo build --example runtime_protocol_probe > /dev/null

variants=(x20 x0 x20x0 x0x1 x20x1 existential)

{
  echo "# Protocol Dispatch Matrix"
  echo
  echo "| Variant | Exit | Dispatch | Direct | Semantic |"
  echo "|---|---:|---:|---:|---|"

  for v in "${variants[@]}"; do
    set +e
    output=$(RUNTIME_PROTOCOL_VARIANT="$v" DYLD_LIBRARY_PATH=. target/debug/examples/runtime_protocol_probe 2>&1)
    code=$?
    set -e

    dispatch=$(echo "$output" | sed -nE 's/.*dispatch=([-0-9]+).*/\1/p' | head -n1)
    direct=$(echo "$output" | sed -nE 's/.*direct=([-0-9]+).*/\1/p' | head -n1)

    semantic="FAIL"
    if echo "$output" | grep -q "semantic=PASS"; then
      semantic="PASS"
    fi

    if [[ -z "$dispatch" ]]; then dispatch="-"; fi
    if [[ -z "$direct" ]]; then direct="-"; fi

    echo "| $v | $code | $dispatch | $direct | $semantic |"
  done
} > "$OUT_FILE"

echo "Wrote $OUT_FILE"
cat "$OUT_FILE"
