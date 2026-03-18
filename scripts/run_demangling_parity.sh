#!/bin/bash
# Symbol demangling parity probe driver for Track D.2
# Tests: demangling of Contract AnyBox exports, String, Array, Dictionary operations, and Release

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$REPO_ROOT"

echo "Running symbol demangling parity probe..."
cargo run --example runtime_demangling_probe --release -- 2>&1

echo ""
echo "Symbol demangling parity probe PASS"
