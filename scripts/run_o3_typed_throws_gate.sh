#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/o3-typed-throws"
FIXTURE_DIR="$ROOT/target/runtime-probe/resilient-fixtures"
MD_OUT="$OUT_DIR/o3-typed-throws-summary.md"
JSON_OUT="$OUT_DIR/o3-typed-throws-summary.json"

mkdir -p "$OUT_DIR"
cd "$ROOT"

bash ./scripts/build_runtime_thunks.sh >/dev/null
mkdir -p "$FIXTURE_DIR"
swiftc -emit-library -emit-module \
  -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
  -enable-library-evolution -g \
  -module-name ResilientFixtures \
  -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
  examples/ResilientFixtures.swift
swiftc -emit-library -g \
  -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures \
  -o libRustBridge.dylib examples/RustBridge.swift

cargo build --example runtime_o3_typed_throws_probe >/dev/null
cargo build --release --example runtime_o3_typed_throws_probe >/dev/null

DEBUG_LOG="$OUT_DIR/o3-typed-throws-debug.log"
RELEASE_LOG="$OUT_DIR/o3-typed-throws-release.log"

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_o3_typed_throws_probe" | tee "$DEBUG_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_o3_typed_throws_probe" | tee "$RELEASE_LOG"

debug_passed="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_passed="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"

if [[ -z "$debug_passed" || -z "$release_passed" ]]; then
  echo "Missing summary output from O.3 probe" >&2
  exit 1
fi

status="PASS"
if [[ "$debug_passed" != "$release_passed" ]]; then
  status="FAIL"
fi

cat > "$JSON_OUT" <<EOF
{
  "gate": "o3_typed_throws_abi_coverage",
  "status": "$status",
  "debug_pass": "$debug_passed",
  "release_pass": "$release_passed",
  "debug_log": "$(basename "$DEBUG_LOG")",
  "release_log": "$(basename "$RELEASE_LOG")"
}
EOF

cat > "$MD_OUT" <<EOF
# O.3 Typed-Throws ABI Coverage Gate

- status: **$status**
- debug passed: $debug_passed
- release passed: $release_passed
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")

Gate passes when debug and release probe pass counts are identical.
EOF

echo "Wrote $JSON_OUT"
echo "Wrote $MD_OUT"

if [[ "$status" != "PASS" ]]; then
  echo "O.3 typed-throws gate FAILED" >&2
  exit 1
fi

echo "O.3 typed-throws gate PASSED"
