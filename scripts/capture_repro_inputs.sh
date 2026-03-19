#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
OUT_JSON="$OUT_DIR/repro-inputs.json"

mkdir -p "$OUT_DIR"
cd "$ROOT"

if [[ ! -f Cargo.lock ]]; then
  echo "Cargo.lock missing" >&2
  exit 1
fi

cargo_lock_sha256="$(shasum -a 256 Cargo.lock | awk '{print $1}')"
plan_sha256="$(shasum -a 256 PLAN.md | awk '{print $1}')"
contract_sha256="$(shasum -a 256 scripts/parity_claim_contract.json | awk '{print $1}')"
swift_line1="$(swift --version 2>/dev/null | head -n 1 || true)"
swift_line2="$(swift --version 2>/dev/null | sed -n '2p' || true)"
rustc_version="$(rustc --version 2>/dev/null || true)"
cargo_version="$(cargo --version 2>/dev/null || true)"
git_commit="$(git rev-parse HEAD 2>/dev/null || true)"
timestamp_utc="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

python3 - "$OUT_JSON" "$cargo_lock_sha256" "$plan_sha256" "$contract_sha256" "$swift_line1" "$swift_line2" "$rustc_version" "$cargo_version" "$git_commit" "$timestamp_utc" <<'PY'
import json
import pathlib
import sys

out_path = pathlib.Path(sys.argv[1])
payload = {
    "version": 1,
    "timestamp_utc": sys.argv[10],
    "git_commit": sys.argv[9],
    "toolchains": {
        "rustc": sys.argv[7],
        "cargo": sys.argv[8],
        "swift_line1": sys.argv[5],
        "swift_line2": sys.argv[6],
    },
    "pins": {
        "cargo_lock_sha256": sys.argv[2],
        "plan_sha256": sys.argv[3],
        "claim_contract_sha256": sys.argv[4],
    },
}

out_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
print(f"Wrote {out_path}")
PY