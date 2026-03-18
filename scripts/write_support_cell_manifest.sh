#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 4 ]]; then
  echo "usage: $0 <gate> <cell> <profile> <out-json>" >&2
  exit 1
fi

gate="$1"
cell="$2"
profile="$3"
out_json="$4"

if [[ "$gate" != "parity" && "$gate" != "contract" ]]; then
  echo "gate must be one of: parity, contract" >&2
  exit 1
fi

swift_version="$(swift --version 2>/dev/null | head -n 1 || true)"
swift_triple="$(swift --version 2>/dev/null | tail -n 1 || true)"
arch="$(uname -m)"
os_version="$(sw_vers -productVersion 2>/dev/null || echo unknown)"
timestamp_utc="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

mkdir -p "$(dirname "$out_json")"

python3 - "$gate" "$cell" "$profile" "$swift_version" "$swift_triple" "$arch" "$os_version" "$timestamp_utc" "$out_json" <<'PY'
import json
import pathlib
import sys

gate = sys.argv[1]
cell = sys.argv[2]
profile = sys.argv[3]
swift_version = sys.argv[4]
swift_triple = sys.argv[5]
arch = sys.argv[6]
os_version = sys.argv[7]
timestamp_utc = sys.argv[8]
out_path = pathlib.Path(sys.argv[9])

payload = {
    "version": 1,
    "gate": gate,
    "cell": cell,
    "profile": profile,
    "arch": arch,
    "os_version": os_version,
    "swift_version": swift_version,
    "swift_triple": swift_triple,
    "timestamp_utc": timestamp_utc,
}

out_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
print(f"Wrote {out_path}")
PY