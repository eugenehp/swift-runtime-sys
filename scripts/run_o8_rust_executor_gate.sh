#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/o8-rust-executor"
FIXTURE_DIR="$ROOT/target/runtime-probe/resilient-fixtures"
MD_OUT="$OUT_DIR/o8-rust-executor-summary.md"
JSON_OUT="$OUT_DIR/o8-rust-executor-summary.json"

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

cargo build --example runtime_o8_rust_executor_probe >/dev/null
cargo build --release --example runtime_o8_rust_executor_probe >/dev/null

DEBUG_LOG="$OUT_DIR/o8-rust-executor-debug.log"
RELEASE_LOG="$OUT_DIR/o8-rust-executor-release.log"

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_o8_rust_executor_probe" | tee "$DEBUG_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_o8_rust_executor_probe" | tee "$RELEASE_LOG"

debug_passed="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_passed="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"
debug_flags="$(grep -E '^o8 rust executor parity =>' "$DEBUG_LOG" | tail -1 | sed 's/^o8 rust executor parity => //')"
release_flags="$(grep -E '^o8 rust executor parity =>' "$RELEASE_LOG" | tail -1 | sed 's/^o8 rust executor parity => //')"

if [[ -z "$debug_passed" || -z "$release_passed" ]]; then
  echo "Missing summary output from O.8 probe" >&2
  exit 1
fi

status="PASS"
if [[ "$debug_passed" != "$release_passed" || "$debug_flags" != "$release_flags" ]]; then
  status="FAIL"
fi

cat > "$JSON_OUT" <<EOF
{
  "gate": "o8_rust_owned_executor",
  "status": "$status",
  "debug_pass": "$debug_passed",
  "release_pass": "$release_passed",
  "debug_flags": "$debug_flags",
  "release_flags": "$release_flags",
  "debug_log": "$(basename "$DEBUG_LOG")",
  "release_log": "$(basename "$RELEASE_LOG")"
}
EOF

cat > "$MD_OUT" <<EOF
# O.8 Rust-Owned Executor Gate

- status: **$status**
- debug passed: $debug_passed
- release passed: $release_passed
- debug flags: $debug_flags
- release flags: $release_flags
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")

Gate passes when debug and release probe pass counts and parity flags are identical.
EOF

echo "Wrote $JSON_OUT"
echo "Wrote $MD_OUT"

if [[ "$status" != "PASS" ]]; then
  echo "O.8 rust-owned executor gate FAILED" >&2
  exit 1
fi

echo "O.8 rust-owned executor gate PASSED"