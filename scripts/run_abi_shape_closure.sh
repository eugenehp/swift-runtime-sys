#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"
CATALOG_JSON="${CATALOG_JSON:-$ROOT/scripts/abi_shape_catalog.json}"
OUT_JSON="$OUT_DIR/abi-shape-closure.json"
OUT_MD="$OUT_DIR/abi-shape-closure.md"
DETAIL_DIR="$OUT_DIR/abi-shape-results"
PROFILE="${PROFILE:-debug}"

if [[ "$PROFILE" == "release" ]]; then
  BIN_PREFIX="target/release/examples"
else
  BIN_PREFIX="target/debug/examples"
fi

mkdir -p "$OUT_DIR" "$FIXTURE_DIR" "$DETAIL_DIR"
cd "$ROOT"

./scripts/build_runtime_thunks.sh
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift

if [[ "$PROFILE" == "release" ]]; then
  cargo build --release --example runtime_abi_shape_probe --example runtime_protocol_probe --example runtime_generic_protocol_probe >/dev/null
else
  cargo build --example runtime_abi_shape_probe --example runtime_protocol_probe --example runtime_generic_protocol_probe >/dev/null
fi

python3 - "$CATALOG_JSON" "$OUT_JSON" "$OUT_MD" "$DETAIL_DIR" "$PROFILE" "$BIN_PREFIX" "$FIXTURE_DIR" <<'PY'
import json
import os
import pathlib
import subprocess
import sys

catalog_path = pathlib.Path(sys.argv[1])
out_json = pathlib.Path(sys.argv[2])
out_md = pathlib.Path(sys.argv[3])
detail_dir = pathlib.Path(sys.argv[4])
profile = sys.argv[5]
bin_prefix = pathlib.Path(sys.argv[6])
fixture_dir = sys.argv[7]

catalog = json.loads(catalog_path.read_text(encoding="utf-8"))
required = catalog.get("required_shapes", [])

def run_shape(shape_id: str):
    env = os.environ.copy()
    env["DYLD_LIBRARY_PATH"] = f"{fixture_dir}:."
    if shape_id in {
        "direct.value.i32_i32_to_i32",
        "inout.mutating.i32ptr_i32_to_i32",
        "indirect_ret.pair.i32_i32_to_pair",
        "throwing.success.i32_to_i32",
        "throwing.error.i32_to_i32",
        "async.value.i32_to_i32",
        "resilient.counter_addpair.i32_i32_to_i32",
    }:
        env["ABI_SHAPE_ID"] = shape_id
        cmd = [str(bin_prefix / "runtime_abi_shape_probe")]
    elif shape_id == "existential.protocol.dispatch":
        env["RUNTIME_PROTOCOL_VARIANT"] = "existential"
        cmd = [str(bin_prefix / "runtime_protocol_probe")]
    elif shape_id == "generic.protocol.dispatch":
        cmd = [str(bin_prefix / "runtime_generic_protocol_probe")]
    else:
        return {
            "id": shape_id,
            "exit_code": 127,
            "pass": False,
            "reason": "unknown shape id",
            "log": "",
        }

    proc = subprocess.run(cmd, env=env, capture_output=True, text=True)
    log = (proc.stdout or "") + ("\n" + proc.stderr if proc.stderr else "")
    passed = proc.returncode == 0
    if shape_id == "existential.protocol.dispatch":
        passed = passed and ("semantic=PASS" in log)

    return {
        "id": shape_id,
        "exit_code": proc.returncode,
        "pass": passed,
        "log": log,
    }

results = []
for shape in required:
    shape_id = shape["id"]
    result = run_shape(shape_id)
    result["class"] = shape.get("class", "")
    result["risk"] = shape.get("risk", "")
    result["description"] = shape.get("description", "")
    results.append(result)

    slug = shape_id.replace("/", "_").replace(".", "_")
    (detail_dir / f"{slug}.log").write_text(result["log"], encoding="utf-8")

passed = sum(1 for r in results if r["pass"])
total = len(results)
status = "PASS" if passed == total and total > 0 else "FAIL"

payload = {
    "version": 1,
    "profile": profile,
    "catalog": str(catalog_path),
    "passed": passed,
    "total": total,
    "status": status,
    "results": [
        {
            "id": r["id"],
            "class": r["class"],
            "risk": r["risk"],
            "description": r["description"],
            "exit_code": r["exit_code"],
            "pass": r["pass"],
        }
        for r in results
    ],
}
out_json.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")

lines = [
    "# ABI Shape Closure",
    "",
    f"- profile: {profile}",
    f"- catalog: {catalog_path}",
    f"- result: {status} ({passed}/{total})",
    "",
    "| Shape | Class | Risk | Exit | Result |",
    "|---|---|---|---:|---|",
]
for r in results:
    lines.append(
        f"| {r['id']} | {r['class']} | {r['risk']} | {r['exit_code']} | {'PASS' if r['pass'] else 'FAIL'} |"
    )
out_md.write_text("\n".join(lines) + "\n", encoding="utf-8")

print(f"Wrote {out_json}")
print(f"Wrote {out_md}")
print(f"ABI shape closure {status}: {passed}/{total}")

if status != "PASS":
    sys.exit(1)
PY