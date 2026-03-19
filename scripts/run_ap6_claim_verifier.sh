#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

cd "$ROOT"
./scripts/run_full_plan_verification.sh
./scripts/build_claim_evidence_bundle.sh

echo "AP.6 claim verifier PASS"