#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_FILE="$OUT_DIR/protocol-dispatch-matrix.md"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"

mkdir -p "$OUT_DIR"
mkdir -p "$FIXTURE_DIR"
cd "$ROOT"

./scripts/build_runtime_thunks.sh
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
cargo build --example runtime_protocol_probe > /dev/null

variants=(x20 x0 x20x0 x0x1 x20x1 existential)
required_variants=(${RUNTIME_PROTOCOL_REQUIRED_VARIANTS:-x20 x0 x20x0 x0x1 x20x1 existential})
required_failures=0

is_required_variant() {
  local needle="$1"
  for v in "${required_variants[@]}"; do
    if [[ "$v" == "$needle" ]]; then
      return 0
    fi
  done
  return 1
}

{
  echo "# Protocol Dispatch Matrix"
  echo
  echo "Required variants: ${required_variants[*]}"
  echo
  echo "| Variant | Exit | Dispatch | Direct | Semantic |"
  echo "|---|---:|---:|---:|---|"

  for v in "${variants[@]}"; do
    set +e
    output=$(RUNTIME_PROTOCOL_VARIANT="$v" DYLD_LIBRARY_PATH="$FIXTURE_DIR:." target/debug/examples/runtime_protocol_probe 2>&1)
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

    if is_required_variant "$v"; then
      if [[ "$code" != "0" || "$semantic" != "PASS" ]]; then
        required_failures=$((required_failures + 1))
      fi
    fi
  done
} > "$OUT_FILE"

echo "Wrote $OUT_FILE"
cat "$OUT_FILE"

if [[ "$required_failures" -gt 0 ]]; then
  echo "Required protocol variants failed: $required_failures"
  exit 1
fi
