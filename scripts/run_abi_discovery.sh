#!/usr/bin/env bash
# run_abi_discovery.sh — ABI discovery pipeline
#
# Usage:
#   ./scripts/run_abi_discovery.sh examples/abi_targets/example.swift [--module MODULE]
#
# Pipeline:
#   Phase 1 – compile fixture as executable (static JSON probe)
#   Phase 2 – compile fixture as dylib with -g (for lldb type query)
#   Phase 3 – run the probe executable → JSON Lines (MemoryLayout + ivar offsets)
#   Phase 4 – lldb Python: query any remaining type layout from DWARF
#   Phase 5 – nm: extract exported Swift symbols → mangled name list
#   Phase 6 – merge phases 3+4+5 → abi-report.json
#   Phase 7 – abi_to_rust.py → Rust struct template
#
# Output (all in target/abi-discovery/<fixture-stem>/):
#   probe.jsonl        raw JSON Lines from phase 3
#   lldb-types.json    DWARF type records from phase 4
#   symbols.txt        exported mangled symbols from phase 5
#   abi-report.json    merged report
#   rust_template.rs   Rust struct + bridge templates

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# ── Parse arguments ───────────────────────────────────────────────────────────
FIXTURE=""
MODULE="RustBridge"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --module)
            if [[ $# -lt 2 ]]; then
                echo "error: --module requires a value"
                exit 1
            fi
            MODULE="$2"
            shift 2
            ;;
        --module=*)
            MODULE="${1#--module=}"
            shift
            ;;
        *)
            if [[ -z "$FIXTURE" ]]; then
                FIXTURE="$1"
            else
                echo "error: unexpected argument: $1"
                exit 1
            fi
            shift
            ;;
    esac
done

if [[ -z "$FIXTURE" ]]; then
    echo "usage: $0 <fixture.swift> [--module MODULE]"
    echo "example: $0 examples/abi_targets/example.swift"
    exit 1
fi

if [[ ! -f "$FIXTURE" ]]; then
    echo "error: fixture not found: $FIXTURE"
    exit 1
fi

STEM="$(basename "$FIXTURE" .swift)"
OUT_DIR="$ROOT/target/abi-discovery/$STEM"
mkdir -p "$OUT_DIR"

PROBE_BIN="$OUT_DIR/probe_bin"
PROBE_DBG_BIN="$OUT_DIR/probe_dbg"
PROBE_DYLIB="$OUT_DIR/probe.dylib"
PROBE_JSONL="$OUT_DIR/probe.jsonl"
LLDB_JSON="$OUT_DIR/lldb-types.json"
SYMBOLS_TXT="$OUT_DIR/symbols.txt"
MERGED_JSON="$OUT_DIR/abi-report.json"
RUST_TEMPLATE="$OUT_DIR/rust_template.rs"

LLDB_PYTHON="$(xcrun lldb --python-path 2>/dev/null || echo '')"

echo "==> ABI discovery: $FIXTURE (module=$MODULE)"
echo "    Output dir: $OUT_DIR"
echo ""

# ── Phase 1+2: Compile ────────────────────────────────────────────────────────
echo "[1/7] Compiling probe executable..."
swiftc -O -o "$PROBE_BIN" "$FIXTURE"
echo "      $PROBE_BIN"

echo "[2/7] Compiling debug artifact for lldb + symbols..."
LLDB_TARGET=""
if swiftc -g -emit-library -module-name "$MODULE" -o "$PROBE_DYLIB" "$FIXTURE" 2>/dev/null || \
   swiftc -g -emit-library -o "$PROBE_DYLIB" "$FIXTURE" 2>/dev/null; then
    LLDB_TARGET="$PROBE_DYLIB"
    echo "      dylib: $PROBE_DYLIB"
else
    # Top-level probe fixtures cannot be compiled as libraries. Fall back to a
    # debug executable so lldb inspection and symbol extraction still work.
    swiftc -g -o "$PROBE_DBG_BIN" "$FIXTURE"
    LLDB_TARGET="$PROBE_DBG_BIN"
    echo "      debug executable fallback: $PROBE_DBG_BIN"
fi

# ── Phase 3: Run probe ────────────────────────────────────────────────────────
echo "[3/7] Running static layout probe..."
"$PROBE_BIN" > "$PROBE_JSONL"
COUNT=$(wc -l < "$PROBE_JSONL" | tr -d ' ')
echo "      $PROBE_JSONL ($COUNT lines)"

# Collect known type names from probe output to help lldb lookup in fallback
# executable mode where type auto-discovery can be sparse.
TYPE_LIST=()
while IFS= read -r tname; do
    [[ -n "$tname" ]] && TYPE_LIST+=("$tname")
done < <(python3 - "$PROBE_JSONL" << 'PY'
import json, sys
names = set()
with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        try:
            r = json.loads(line)
        except Exception:
            continue
        if r.get("kind") in ("struct", "class", "enum"):
            n = r.get("name", "")
            if n:
                names.add(n)
for n in sorted(names):
    print(n)
PY
)

# ── Phase 4: lldb DWARF type query ───────────────────────────────────────────
echo "[4/7] Querying DWARF types via lldb Python..."
if [[ -n "$LLDB_PYTHON" ]]; then
    PYTHONPATH="$LLDB_PYTHON" python3 "$ROOT/scripts/abi_lldb_inspect.py" \
        "$LLDB_TARGET" "${TYPE_LIST[@]}" > "$LLDB_JSON" 2>/dev/null || echo "[]" > "$LLDB_JSON"
    NLLDB=$(python3 -c "import json; d=json.load(open('$LLDB_JSON')); print(len(d))" 2>/dev/null || echo 0)
    echo "      $LLDB_JSON ($NLLDB type records)"
else
    echo "[]" > "$LLDB_JSON"
    echo "      skipped (lldb --python-path not available)"
fi

# ── Phase 5: Symbol extraction ────────────────────────────────────────────────
echo "[5/7] Extracting exported symbols..."
nm -gU "$LLDB_TARGET" 2>/dev/null | grep '^\S' | awk '{print $NF}' | \
    sed -E 's/^_//' | grep '^\$s' | sort > "$SYMBOLS_TXT" || true
NSYM=$(wc -l < "$SYMBOLS_TXT" | tr -d ' ')
if [[ "$NSYM" == "0" ]]; then
    # Fallback for executables and non-exported Swift symbols.
    nm -j "$LLDB_TARGET" 2>/dev/null | sed -E 's/^_//' | grep '^\$s' | sort > "$SYMBOLS_TXT" || true
    NSYM=$(wc -l < "$SYMBOLS_TXT" | tr -d ' ')
fi
echo "      $SYMBOLS_TXT ($NSYM Swift symbols)"

# ── Phase 6: Merge into abi-report.json ──────────────────────────────────────
echo "[6/7] Merging reports..."
python3 - "$PROBE_JSONL" "$LLDB_JSON" "$SYMBOLS_TXT" "$MODULE" > "$MERGED_JSON" << 'PY'
import json, sys

probe_file, lldb_file, sym_file, module = sys.argv[1:]

probe_records = []
with open(probe_file) as f:
    for line in f:
        line = line.strip()
        if line:
            try:
                probe_records.append(json.loads(line))
            except Exception:
                pass

lldb_records = []
try:
    with open(lldb_file) as f:
        lldb_records = json.load(f)
except Exception:
    pass

symbols = []
try:
    with open(sym_file) as f:
        symbols = [l.strip() for l in f if l.strip()]
except Exception:
    pass

report = {
    "module": module,
    "probe_records": probe_records,
    "lldb_types": lldb_records,
    "exported_symbols": symbols,
}
print(json.dumps(report, indent=2))
PY
echo "      $MERGED_JSON"

# ── Phase 7: Rust code generation ────────────────────────────────────────────
echo "[7/7] Generating Rust template..."
python3 "$ROOT/scripts/abi_to_rust.py" "$PROBE_JSONL" --module "$MODULE" > "$RUST_TEMPLATE"
echo "      $RUST_TEMPLATE"

# ── Summary ──────────────────────────────────────────────────────────────────
echo ""
echo "==> Layout summary:"
python3 - "$PROBE_JSONL" << 'PY'
import json, sys
from collections import defaultdict

records = []
with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        if line:
            try:
                records.append(json.loads(line))
            except Exception:
                pass

types = {}
fields = defaultdict(list)
for r in records:
    k = r.get("kind","")
    if k in ("struct","class","enum"):
        types[r["name"]] = r
    elif k == "field":
        fields[r["parent"]].append(r)

for name, t in sorted(types.items()):
    kind = t["kind"]
    if kind == "struct":
        print(f"\n  struct {name}  size={t['size']} stride={t['stride']} align={t['alignment']}")
    elif kind == "class":
        print(f"\n  class  {name}  instance_size={t.get('instance_size','?')} ref_size=8")
    elif kind == "enum":
        print(f"\n  enum   {name}  size={t['size']} stride={t['stride']} align={t['alignment']}")
    for f in sorted(fields.get(name,[]), key=lambda x: x["offset"]):
        enc = f.get("encoding","")
        st  = f.get("swift_type","")
        tname = st if st else enc if enc else "?"
        print(f"    +{f['offset']:3d}  {f['name']}: {tname}  (size={f['size']})")
PY

echo ""
echo "==> Mangled symbols (first 20):"
head -20 "$SYMBOLS_TXT" | while read -r sym; do
    demangled="$(swift-demangle "$sym" 2>/dev/null || echo "$sym")"
    printf "    %-60s  %s\n" "$sym" "$demangled"
done

echo ""
echo "==> Done."
echo "    Rust template : $RUST_TEMPLATE"
echo "    ABI report    : $MERGED_JSON"
echo "    Symbol list   : $SYMBOLS_TXT"
