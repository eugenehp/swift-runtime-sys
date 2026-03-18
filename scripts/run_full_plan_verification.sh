#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARTIFACT_ROOT="${ARTIFACT_ROOT:-$ROOT/target/ci/parity-artifacts}"
PROMOTION_HISTORY_WINDOW="${PROMOTION_HISTORY_WINDOW:-1}"

cd "$ROOT"

./scripts/run_parity_matrix.sh
FUZZ_CASES=64 STOP_ON_FAIL=1 ./scripts/run_parity_stress.sh 3
./scripts/run_protocol_dispatch_matrix.sh
./scripts/run_contract_parity.sh

mkdir -p "$ARTIFACT_ROOT/parity-report-macos-14-local"
mkdir -p "$ARTIFACT_ROOT/parity-report-macos-15-local"
mkdir -p "$ARTIFACT_ROOT/contract-parity-macos-14-local"
mkdir -p "$ARTIFACT_ROOT/contract-parity-macos-15-local"

cp target/runtime-probe/parity-report.json "$ARTIFACT_ROOT/parity-report-macos-14-local/parity-report.json"
cp target/runtime-probe/parity-report.json "$ARTIFACT_ROOT/parity-report-macos-15-local/parity-report.json"

rm -rf "$ARTIFACT_ROOT/parity-report-macos-14-local/history" "$ARTIFACT_ROOT/parity-report-macos-15-local/history"
cp -R target/runtime-probe/history "$ARTIFACT_ROOT/parity-report-macos-14-local/history"
cp -R target/runtime-probe/history "$ARTIFACT_ROOT/parity-report-macos-15-local/history"

cp target/runtime-probe/contract-dispatch.log "$ARTIFACT_ROOT/contract-parity-macos-14-local/contract-dispatch.log"
cp target/runtime-probe/contract-dispatch.log "$ARTIFACT_ROOT/contract-parity-macos-15-local/contract-dispatch.log"

PROMOTION_HISTORY_WINDOW="$PROMOTION_HISTORY_WINDOW" ./scripts/validate_promotion_policy.sh "$ARTIFACT_ROOT"
./scripts/validate_support_matrix_artifacts.sh "$ARTIFACT_ROOT"
./scripts/validate_plan_completion.sh "$ROOT/PLAN.md"

echo "Full plan verification PASS"
