#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARTIFACT_ROOT="${ARTIFACT_ROOT:-$ROOT/target/ci/parity-artifacts}"
PROMOTION_HISTORY_WINDOW="${PROMOTION_HISTORY_WINDOW:-1}"

cd "$ROOT"

# Debug profile gates
./scripts/run_parity_matrix.sh
FUZZ_CASES=64 STOP_ON_FAIL=1 ./scripts/run_parity_stress.sh 3
./scripts/run_protocol_dispatch_matrix.sh
./scripts/run_contract_parity.sh
./scripts/validate_runtime_killswitch_policy.sh

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

cp target/runtime-probe/parity-report.json "$TMP_DIR/parity-debug.json"
cp target/runtime-probe/parity-report.md "$TMP_DIR/parity-debug.md"
rm -rf "$TMP_DIR/history-debug"
cp -R target/runtime-probe/history "$TMP_DIR/history-debug"
cp target/runtime-probe/contract-dispatch.log "$TMP_DIR/contract-dispatch-debug.log"
cp target/runtime-probe/contract-descriptor.log "$TMP_DIR/contract-descriptor-debug.log"

# Release profile gates
PROFILE=release ./scripts/run_parity_matrix.sh
PROFILE=release ./scripts/run_contract_parity.sh

cp target/runtime-probe/parity-report.json "$TMP_DIR/parity-release.json"
cp target/runtime-probe/parity-report.md "$TMP_DIR/parity-release.md"
rm -rf "$TMP_DIR/history-release"
cp -R target/runtime-probe/history "$TMP_DIR/history-release"
cp target/runtime-probe/contract-dispatch.log "$TMP_DIR/contract-dispatch-release.log"
cp target/runtime-probe/contract-descriptor.log "$TMP_DIR/contract-descriptor-release.log"

for cell in macos-14 macos-15; do
	for profile in debug release; do
		mkdir -p "$ARTIFACT_ROOT/parity-report-${cell}-${profile}-local"
		mkdir -p "$ARTIFACT_ROOT/contract-parity-${cell}-${profile}-local"
	done
done

for cell in macos-14 macos-15; do
	cp "$TMP_DIR/parity-debug.json" "$ARTIFACT_ROOT/parity-report-${cell}-debug-local/parity-report.json"
	cp "$TMP_DIR/parity-debug.md" "$ARTIFACT_ROOT/parity-report-${cell}-debug-local/parity-report.md"
	./scripts/write_support_cell_manifest.sh parity "$cell" debug "$ARTIFACT_ROOT/parity-report-${cell}-debug-local/support-cell-manifest.json"
	rm -rf "$ARTIFACT_ROOT/parity-report-${cell}-debug-local/history"
	cp -R "$TMP_DIR/history-debug" "$ARTIFACT_ROOT/parity-report-${cell}-debug-local/history"

	cp "$TMP_DIR/parity-release.json" "$ARTIFACT_ROOT/parity-report-${cell}-release-local/parity-report.json"
	cp "$TMP_DIR/parity-release.md" "$ARTIFACT_ROOT/parity-report-${cell}-release-local/parity-report.md"
	./scripts/write_support_cell_manifest.sh parity "$cell" release "$ARTIFACT_ROOT/parity-report-${cell}-release-local/support-cell-manifest.json"
	rm -rf "$ARTIFACT_ROOT/parity-report-${cell}-release-local/history"
	cp -R "$TMP_DIR/history-release" "$ARTIFACT_ROOT/parity-report-${cell}-release-local/history"

	cp "$TMP_DIR/contract-dispatch-debug.log" "$ARTIFACT_ROOT/contract-parity-${cell}-debug-local/contract-dispatch.log"
	cp "$TMP_DIR/contract-descriptor-debug.log" "$ARTIFACT_ROOT/contract-parity-${cell}-debug-local/contract-descriptor.log"
	./scripts/write_support_cell_manifest.sh contract "$cell" debug "$ARTIFACT_ROOT/contract-parity-${cell}-debug-local/support-cell-manifest.json"

	cp "$TMP_DIR/contract-dispatch-release.log" "$ARTIFACT_ROOT/contract-parity-${cell}-release-local/contract-dispatch.log"
	cp "$TMP_DIR/contract-descriptor-release.log" "$ARTIFACT_ROOT/contract-parity-${cell}-release-local/contract-descriptor.log"
	./scripts/write_support_cell_manifest.sh contract "$cell" release "$ARTIFACT_ROOT/contract-parity-${cell}-release-local/support-cell-manifest.json"
done

PROMOTION_HISTORY_WINDOW="$PROMOTION_HISTORY_WINDOW" ./scripts/validate_promotion_policy.sh "$ARTIFACT_ROOT"
USE_MATRIX_CONTRACT=0 REQUIRED_CELLS="macos-14 macos-15" REQUIRED_PROFILES="debug release" ./scripts/validate_support_matrix_artifacts.sh "$ARTIFACT_ROOT"
./scripts/validate_plan_completion.sh "$ROOT/PLAN.md"

echo "Full plan verification PASS"
