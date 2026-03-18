#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
ARTIFACT_ROOT="${ARTIFACT_ROOT:-$ROOT/target/ci/parity-artifacts}"
OUT_MD="$OUT_DIR/ap1-support-matrix-wave.md"
STAMP="$(date -u +%Y-%m-%dT%H_%M_%SZ)"

mkdir -p "$OUT_DIR" "$ARTIFACT_ROOT"
cd "$ROOT"

cell_os="$(sw_vers -productVersion | cut -d. -f1 2>/dev/null || echo macos)"
cell_arch="$(uname -m)"
cell="macos-${cell_os}-${cell_arch}-local"

profiles=(debug release)
parity_debug="FAIL"
parity_release="FAIL"
contract_debug="FAIL"
contract_release="FAIL"

echo "# AP.1 Support Matrix Wave" > "$OUT_MD"
echo >> "$OUT_MD"
echo "- timestamp_utc: ${STAMP}" >> "$OUT_MD"
echo "- cell: ${cell}" >> "$OUT_MD"
echo "- profiles: ${profiles[*]}" >> "$OUT_MD"
echo >> "$OUT_MD"

for profile in "${profiles[@]}"; do
  parity_art_dir="$ARTIFACT_ROOT/parity-report-${cell}-${profile}-${STAMP}"
  contract_art_dir="$ARTIFACT_ROOT/contract-parity-${cell}-${profile}-${STAMP}"
  mkdir -p "$parity_art_dir" "$contract_art_dir"

  if PROFILE="$profile" ./scripts/run_parity_matrix.sh; then
    if [[ "$profile" == "debug" ]]; then
      parity_debug="PASS"
    else
      parity_release="PASS"
    fi
    cp "$OUT_DIR/parity-report.json" "$parity_art_dir/parity-report.json"
    cp "$OUT_DIR/parity-report.md" "$parity_art_dir/parity-report.md"
    ./scripts/write_support_cell_manifest.sh parity "$cell" "$profile" "$parity_art_dir/support-cell-manifest.json"
    rm -rf "$parity_art_dir/history"
    cp -R "$OUT_DIR/history" "$parity_art_dir/history"
  fi

  if PROFILE="$profile" ./scripts/run_contract_parity.sh; then
    if [[ "$profile" == "debug" ]]; then
      contract_debug="PASS"
    else
      contract_release="PASS"
    fi
    cp "$OUT_DIR/contract-dispatch.log" "$contract_art_dir/contract-dispatch.log"
    cp "$OUT_DIR/contract-descriptor.log" "$contract_art_dir/contract-descriptor.log"
    cp "$OUT_DIR/contract-parity.md" "$contract_art_dir/contract-parity.md"
    ./scripts/write_support_cell_manifest.sh contract "$cell" "$profile" "$contract_art_dir/support-cell-manifest.json"
  fi
done

echo "## Results" >> "$OUT_MD"
echo >> "$OUT_MD"
echo "| Profile | Parity Matrix | Contract Parity |" >> "$OUT_MD"
echo "|---|---|---|" >> "$OUT_MD"
echo "| debug | ${parity_debug} | ${contract_debug} |" >> "$OUT_MD"
echo "| release | ${parity_release} | ${contract_release} |" >> "$OUT_MD"

echo >> "$OUT_MD"
echo "## Artifact Root" >> "$OUT_MD"
echo >> "$OUT_MD"
echo "- ${ARTIFACT_ROOT}" >> "$OUT_MD"

echo "Wrote ${OUT_MD}"
cat "$OUT_MD"

if [[ "$parity_debug" != "PASS" || "$contract_debug" != "PASS" ]]; then
  exit 1
fi
if [[ "$parity_release" != "PASS" || "$contract_release" != "PASS" ]]; then
  exit 1
fi
