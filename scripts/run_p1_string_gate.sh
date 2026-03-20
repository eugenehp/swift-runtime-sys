#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/p1-string"
MD_OUT="$OUT_DIR/p1-string-summary.md"
JSON_OUT="$OUT_DIR/p1-string-summary.json"

mkdir -p "$OUT_DIR"
cd "$ROOT"

# Build and run P.1 String probe in debug and release modes
cargo build --example runtime_p1_string_probe >/dev/null 2>&1 || {
  echo "Failed to build P.1 string probe (debug)" >&2
  exit 1
}
cargo build --release --example runtime_p1_string_probe >/dev/null 2>&1 || {
  echo "Failed to build P.1 string probe (release)" >&2
  exit 1
}

DEBUG_LOG="$OUT_DIR/p1-string-debug.log"
RELEASE_LOG="$OUT_DIR/p1-string-release.log"

# Run debug and release probes, capturing output
"$ROOT/target/debug/examples/runtime_p1_string_probe" | tee "$DEBUG_LOG"
"$ROOT/target/release/examples/runtime_p1_string_probe" | tee "$RELEASE_LOG"

# Extract pass counts and parity flags from logs
debug_passed="$(grep -E '^Passed:' "$DEBUG_LOG" | tail -1 | awk '{print $2}')"
release_passed="$(grep -E '^Passed:' "$RELEASE_LOG" | tail -1 | awk '{print $2}')"
debug_flags="$(grep -E '^p1 string parity =>' "$DEBUG_LOG" | tail -1 | sed 's/^p1 string parity => //')"
release_flags="$(grep -E '^p1 string parity =>' "$RELEASE_LOG" | tail -1 | sed 's/^p1 string parity => //')"

if [[ -z "$debug_passed" || -z "$release_passed" ]]; then
  echo "Missing summary output from P.1 probe" >&2
  exit 1
fi

# Gate passes if debug and release are identical
status="PASS"
if [[ "$debug_passed" != "$release_passed" || "$debug_flags" != "$release_flags" ]]; then
  status="FAIL"
fi

# Write JSON artifact
cat > "$JSON_OUT" <<EOF
{
  "gate": "p1_string_bridging",
  "status": "$status",
  "debug_pass": "$debug_passed",
  "release_pass": "$release_passed",
  "debug_flags": "$debug_flags",
  "release_flags": "$release_flags",
  "debug_log": "$(basename "$DEBUG_LOG")",
  "release_log": "$(basename "$RELEASE_LOG")",
  "timestamp": "$(date -u +%Y-%m-%dT%H_%M_%SZ)"
}
EOF

# Write markdown artifact
cat > "$MD_OUT" <<EOF
# P.1 String / ByteString Probe

- status: **$status**
- debug passed: $debug_passed
- release passed: $release_passed
- debug flags: $debug_flags
- release flags: $release_flags
- debug log: $(basename "$DEBUG_LOG")
- release log: $(basename "$RELEASE_LOG")

## Test Coverage
- ASCII string construction (5 bytes)
- UTF-8 multi-byte (emoji, accented chars)
- Null-termination boundary semantics
- String normalization (NFC form)
- Case-folding (upper/lower)
- Empty and non-empty string capacity

Gate passes when debug and release probe pass counts and parity flags are identical.
EOF

echo "Wrote $JSON_OUT"
echo "Wrote $MD_OUT"

if [[ "$status" != "PASS" ]]; then
  echo "P.1 string bridging gate FAILED" >&2
  exit 1
else
  echo "P.1 string bridging gate PASSED"
fi
