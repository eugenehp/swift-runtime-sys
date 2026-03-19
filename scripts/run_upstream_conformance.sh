#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe/upstream-conformance"
CURRENT_DIR="$OUT_DIR/current"
HISTORY_DIR="$OUT_DIR/history"
TARGETS_JSON="${TARGETS_JSON:-$ROOT/scripts/upstream_conformance_targets.json}"
REF="${1:-}"
AP7_FORCE_VERIFY="${AP7_FORCE_VERIFY:-0}"

if [[ -z "$REF" ]]; then
  echo "usage: $0 <upstream-ref>" >&2
  exit 2
fi

mkdir -p "$CURRENT_DIR" "$HISTORY_DIR"
cd "$ROOT"

if [[ ! -f "$TARGETS_JSON" ]]; then
  echo "targets file missing: $TARGETS_JSON" >&2
  exit 1
fi

if ! python3 - "$TARGETS_JSON" "$REF" <<'PY'
import json
import sys

targets = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
ref = sys.argv[2]
if ref not in targets.get('tracked_refs', []):
    raise SystemExit(1)
PY
then
  echo "ref is not tracked by targets config: $REF" >&2
  exit 1
fi

verify_mode="reuse-existing"
if [[ "$AP7_FORCE_VERIFY" == "1" ]]; then
  verify_mode="full"
  ./scripts/run_ap6_claim_verifier.sh
elif [[ ! -f "$ROOT/target/runtime-probe/parity-claim-signoff.md" ]] || ! grep -q "result: PASS" "$ROOT/target/runtime-probe/parity-claim-signoff.md"; then
  verify_mode="full"
  ./scripts/run_ap6_claim_verifier.sh
fi

remote="https://github.com/swiftlang/swift.git"
upstream_sha="$(
  {
    git ls-remote --heads "$remote" "refs/heads/$REF" || true
    git ls-remote --tags "$remote" "refs/tags/$REF" || true
    git ls-remote "$remote" "$REF" || true
  } | awk '{print $1}' | head -n 1
)"
if [[ -z "$upstream_sha" ]]; then
  echo "unable to resolve upstream SHA for ref: $REF" >&2
  exit 1
fi

claim_result="UNKNOWN"
if [[ -f "$ROOT/target/runtime-probe/parity-claim-signoff.md" ]] && grep -q "result: PASS" "$ROOT/target/runtime-probe/parity-claim-signoff.md"; then
  claim_result="PASS"
else
  claim_result="FAIL"
fi

timestamp_utc="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
stamp_file="$(date -u +%Y-%m-%dT%H_%M_%SZ)"
git_commit="$(git rev-parse HEAD 2>/dev/null || true)"
workflow_run_url=""
if [[ -n "${GITHUB_SERVER_URL:-}" && -n "${GITHUB_REPOSITORY:-}" && -n "${GITHUB_RUN_ID:-}" ]]; then
  workflow_run_url="${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}"
fi

out_json="$CURRENT_DIR/${REF}.json"
hist_json="$HISTORY_DIR/${REF}-${stamp_file}.json"

python3 - "$out_json" "$hist_json" "$REF" "$upstream_sha" "$claim_result" "$timestamp_utc" "$git_commit" "$verify_mode" "$workflow_run_url" <<'PY'
import json
import pathlib
import sys

out_path = pathlib.Path(sys.argv[1])
hist_path = pathlib.Path(sys.argv[2])
payload = {
    "version": 1,
    "upstream_ref": sys.argv[3],
    "upstream_sha": sys.argv[4],
    "result": sys.argv[5],
    "timestamp_utc": sys.argv[6],
    "repo_commit": sys.argv[7],
    "verification_mode": sys.argv[8],
    "workflow_run_url": sys.argv[9],
    "required_gate": "ap6_claim_verifier"
}

out_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
hist_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
print(f"Wrote {out_path}")
print(f"Wrote {hist_path}")
PY

out_md="$CURRENT_DIR/${REF}.md"
cat > "$out_md" <<MD
# Upstream Conformance

- upstream_ref: ${REF}
- upstream_sha: ${upstream_sha}
- result: ${claim_result}
- verification_mode: ${verify_mode}
- workflow_run_url: ${workflow_run_url:-n/a}
MD

echo "Wrote $out_md"

if [[ "$claim_result" != "PASS" ]]; then
  exit 1
fi
