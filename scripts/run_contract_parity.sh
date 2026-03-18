#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"
OUT_MD="$OUT_DIR/contract-parity.md"
DESCRIPTOR_LOG="$OUT_DIR/contract-descriptor.log"
DISPATCH_LOG="$OUT_DIR/contract-dispatch.log"
PROFILE="${PROFILE:-debug}"

if [[ "$PROFILE" == "release" ]]; then
  BIN_PREFIX="target/release/examples"
else
  BIN_PREFIX="target/debug/examples"
fi

mkdir -p "$OUT_DIR"
mkdir -p "$FIXTURE_DIR"
cd "$ROOT"

./scripts/build_runtime_thunks.sh
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
if [[ "$PROFILE" == "release" ]]; then
  cargo build --release --example runtime_contract_probe --example runtime_contract_dispatch_probe > /dev/null
else
  cargo build --example runtime_contract_probe --example runtime_contract_dispatch_probe > /dev/null
fi

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$BIN_PREFIX/runtime_contract_probe" > "$DESCRIPTOR_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$BIN_PREFIX/runtime_contract_dispatch_probe" > "$DISPATCH_LOG"

descriptor_line=$(head -n 1 "$DESCRIPTOR_LOG")
dispatch_line=$(head -n 1 "$DISPATCH_LOG")

if ! grep -q "version=1" "$DESCRIPTOR_LOG"; then
  echo "Contract descriptor probe did not report version=1"
  exit 1
fi

if ! grep -q "normalized=true" "$DISPATCH_LOG"; then
  echo "Contract dispatch probe did not report normalized=true"
  exit 1
fi

if ! grep -q "metadata_registry=true" "$DISPATCH_LOG"; then
  echo "Contract dispatch probe did not report metadata registry success"
  exit 1
fi

if ! grep -q "protocol_registry=true" "$DISPATCH_LOG"; then
  echo "Contract dispatch probe did not report protocol registry success"
  exit 1
fi

if ! grep -q "generic_metadata=Supported" "$DISPATCH_LOG"; then
  echo "Contract dispatch probe did not report supported generic metadata registry"
  exit 1
fi

if ! grep -q "protocol_registry=Supported" "$DISPATCH_LOG"; then
  echo "Contract dispatch probe did not report supported protocol registry"
  exit 1
fi

cat > "$OUT_MD" <<EOF
# Contract Parity

- Descriptor: $descriptor_line
- Dispatch: $dispatch_line
EOF

echo "Wrote $OUT_MD"
cat "$OUT_MD"
