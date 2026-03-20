#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/p6-formatting"
FIXTURE_DIR="$ROOT/target/runtime-probe/resilient-fixtures"
MD_OUT="$OUT_DIR/p6-formatting-summary.md"
JSON_OUT="$OUT_DIR/p6-formatting-summary.json"

mkdir -p "$OUT_DIR"
mkdir -p "$FIXTURE_DIR"
cd "$ROOT"

bash ./scripts/build_runtime_thunks.sh >/dev/null
swiftc -emit-library -emit-module \
  -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" \
  -enable-library-evolution -g \
  -module-name ResilientFixtures \
  -o "$FIXTURE_DIR/libResilientFixtures.dylib" \
  examples/ResilientFixtures.swift
swiftc -emit-library -g \
  -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures \
  -o libRustBridge.dylib examples/RustBridge.swift

cargo build --example runtime_p6_formatting_probe >/dev/null
cargo build --release --example runtime_p6_formatting_probe >/dev/null

DEBUG_LOG="$OUT_DIR/p6-formatting-debug.log"
RELEASE_LOG="$OUT_DIR/p6-formatting-release.log"

DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/debug/examples/runtime_p6_formatting_probe" | tee "$DEBUG_LOG"
DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "$ROOT/target/release/examples/runtime_p6_formatting_probe" | tee "$RELEASE_LOG"

debug_passed="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_passed="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"
debug_flags="$(grep -E '^p6 formatting parity =>' "$DEBUG_LOG" | tail -1 | sed 's/^p6 formatting parity => //')"
release_flags="$(grep -E '^p6 formatting parity =>' "$RELEASE_LOG" | tail -1 | sed 's/^p6 formatting parity => //')"

if [[ -z "$debug_passed" || -z "$release_passed" ]]; then
  echo "Missing summary output from P.6 probe" >&2
  exit 1
fi

status="PASS"
if [[ "$debug_passed" != "$release_passed" || "$debug_flags" != "$release_flags" ]]; then
  status="FAIL"
fi

cat > "$JSON_OUT" <<EOF
{
  "gate": "p6_number_formatting",
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
# P.6 Number Formatting Gate

- status: **$status**
- debug passed: $debug_passed
- release passed: $release_passed
- debug flags: $debug_flags
- release flags: $release_flags
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")

Gate passes when debug and release pass counts and parity flags are identical.
EOF

echo "Wrote $JSON_OUT"
echo "Wrote $MD_OUT"

if [[ "$status" != "PASS" ]]; then
  echo "P.6 formatting gate FAILED" >&2
  exit 1
fi

echo "P.6 formatting gate PASSED"
