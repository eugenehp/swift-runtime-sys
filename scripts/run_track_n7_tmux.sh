#!/usr/bin/env bash
set -euo pipefail

SESSION="track_n7"
LOG="/tmp/run_track_n7.log"
EXITF="/tmp/run_track_n7.exit"

rm -f "$LOG" "$EXITF"
tmux kill-session -t "$SESSION" 2>/dev/null || true

tmux new-session -d -s "$SESSION" \
  "cd /Users/Shared/swift-runtime-sys && mkdir -p target/runtime-probe/resilient-fixtures && ./scripts/build_runtime_thunks.sh >/dev/null 2>&1 && swiftc -emit-library -emit-module -emit-module-path target/runtime-probe/resilient-fixtures/ResilientFixtures.swiftmodule -enable-library-evolution -g -module-name ResilientFixtures -o target/runtime-probe/resilient-fixtures/libResilientFixtures.dylib examples/ResilientFixtures.swift && swiftc -emit-library -g -I target/runtime-probe/resilient-fixtures -L target/runtime-probe/resilient-fixtures -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift && cargo build --example runtime_binary_contract_derivation_probe && DYLD_LIBRARY_PATH=target/runtime-probe/resilient-fixtures:. ./target/debug/examples/runtime_binary_contract_derivation_probe > \"$LOG\" 2>&1; echo $? > \"$EXITF\""

echo "started $SESSION"
