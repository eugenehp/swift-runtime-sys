#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG="/tmp/run_track_h2.log"
EXITF="/tmp/run_track_h2.exit"
SESSION="track_h2"

rm -f "$LOG" "$EXITF"
tmux kill-session -t "$SESSION" 2>/dev/null || true

tmux new-session -d -s "$SESSION" "bash -lc 'set -euo pipefail; cd \"$ROOT\"; mkdir -p target/runtime-probe/resilient-fixtures; ./scripts/build_runtime_thunks.sh >/dev/null 2>&1; swiftc -emit-library -emit-module -emit-module-path target/runtime-probe/resilient-fixtures/ResilientFixtures.swiftmodule -enable-library-evolution -g -module-name ResilientFixtures -o target/runtime-probe/resilient-fixtures/libResilientFixtures.dylib examples/ResilientFixtures.swift; swiftc -emit-library -g -I target/runtime-probe/resilient-fixtures -L target/runtime-probe/resilient-fixtures -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift; cargo build --example runtime_generic_protocol_probe; DYLD_LIBRARY_PATH=target/runtime-probe/resilient-fixtures:. ./target/debug/examples/runtime_generic_protocol_probe' > \"$LOG\" 2>&1; echo \$? > \"$EXITF\""

for _ in $(seq 1 420); do
  if [[ -f "$EXITF" ]]; then
    break
  fi
  sleep 1
done

if [[ ! -f "$EXITF" ]]; then
  echo "tmux_status=timeout"
  tmux has-session -t "$SESSION" 2>/dev/null && echo "tmux_session=alive" || echo "tmux_session=not_found"
  echo "---log_tail---"
  tail -n 120 "$LOG" || true
  exit 2
fi

echo "tmux_status=completed"
echo "tmux_exit=$(cat "$EXITF")"
echo "---log_tail---"
tail -n 220 "$LOG" || true
