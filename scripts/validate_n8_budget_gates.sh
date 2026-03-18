#!/usr/bin/env bash
set -euo pipefail

ARTIFACT="target/runtime-probe/n8-budget-gates.json"
if [[ ! -f "$ARTIFACT" ]]; then
  echo "missing artifact: $ARTIFACT"
  exit 2
fi

if command -v jq >/dev/null 2>&1; then
  FAILED_COUNT=$(jq '[.gates[] | select(.passed == false)] | length' "$ARTIFACT")
  if [[ "$FAILED_COUNT" -gt 0 ]]; then
    echo "N.8 budget gate failure count: $FAILED_COUNT"
    jq -r '.gates[] | select(.passed == false) | "- " + .operation + ": " + (.reasons | join("; "))' "$ARTIFACT"
    exit 1
  fi
  echo "N.8 budget gates: all passed"
  exit 0
fi

if grep -q '"passed": false' "$ARTIFACT"; then
  echo "N.8 budget gates: failures detected"
  exit 1
fi

echo "N.8 budget gates: all passed"
