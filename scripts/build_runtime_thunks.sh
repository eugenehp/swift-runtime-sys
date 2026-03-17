#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

./scripts/generate_runtime_thunks.sh
clang -dynamiclib -g -o libRuntimeThunks.dylib \
	examples/runtime_swiftcall_thunks.generated.c \
	examples/runtime_thunk_utils.c

echo "Wrote $ROOT/libRuntimeThunks.dylib"
