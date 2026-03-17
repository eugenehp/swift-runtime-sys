#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SESSION="swift_runtime_lldb"
LOG_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$LOG_DIR/resilient-fixtures"
LOG_FILE="$LOG_DIR/lldb_tmux.log"

mkdir -p "$LOG_DIR"
mkdir -p "$FIXTURE_DIR"

cd "$ROOT"
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
./scripts/build_runtime_thunks.sh
cargo build --example runtime_raw_probe

if tmux has-session -t "$SESSION" 2>/dev/null; then
  tmux kill-session -t "$SESSION"
fi

TRY_INCREMENT="${RUNTIME_TRY_INCREMENT:-0}"
CMD="cd '$ROOT' && DYLD_LIBRARY_PATH='$FIXTURE_DIR:.' RUNTIME_TRY_INCREMENT='$TRY_INCREMENT' lldb -b -s scripts/lldb_runtime_cmds.txt 'target/debug/examples/runtime_raw_probe' > '$LOG_FILE' 2>&1"
tmux new-session -d -s "$SESSION" "$CMD"

for _ in {1..60}; do
  if ! tmux has-session -t "$SESSION" 2>/dev/null; then
    break
  fi
  sleep 1
done

if tmux has-session -t "$SESSION" 2>/dev/null; then
  tmux kill-session -t "$SESSION"
fi

echo "Wrote $LOG_FILE"
