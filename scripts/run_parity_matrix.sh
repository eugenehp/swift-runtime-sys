#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
PROBE_LOG="$OUT_DIR/probe.log"
LLDB_LOG="$OUT_DIR/lldb_tmux.log"
REPORT_JSON="$OUT_DIR/parity-report.json"
REPORT_MD="$OUT_DIR/parity-report.md"

mkdir -p "$OUT_DIR"

cd "$ROOT"
./scripts/build_runtime_thunks.sh
swiftc -emit-library -g -o libRustBridge.dylib examples/RustBridge.swift
cargo build --example runtime_raw_probe

RUNTIME_TRY_INCREMENT=1 DYLD_LIBRARY_PATH=. "target/debug/examples/runtime_raw_probe" > "$PROBE_LOG" 2>&1
RUNTIME_TRY_INCREMENT=1 ./scripts/run_tmux_lldb.sh > /dev/null

line_or_empty() {
  local pattern="$1"
  local file="$2"
  grep -E "$pattern" "$file" | head -n 1 || true
}

extract_number() {
  local key="$1"
  local line="$2"
  echo "$line" | sed -nE "s/.*${key}=(-?[0-9]+).*/\\1/p"
}

probe_line=$(line_or_empty "counter increments via x20 thunk" "$PROBE_LOG")
direct_line=$(line_or_empty "direct field write" "$PROBE_LOG")
protocol_line=$(line_or_empty "protocol witness" "$PROBE_LOG")
protocol_slot_line=$(line_or_empty "protocol witness slot0" "$PROBE_LOG")
protocol_slot1_line=$(line_or_empty "protocol witness slot1" "$PROBE_LOG")
protocol_slot2_line=$(line_or_empty "protocol witness slot2" "$PROBE_LOG")
protocol_dispatch_line=$(line_or_empty "protocol witness dispatch" "$PROBE_LOG")
global_line=$(line_or_empty "global variable =>" "$PROBE_LOG")
raw_meta_line=$(line_or_empty "raw metadata parity =>" "$PROBE_LOG")
enum_dir_line=$(line_or_empty "enum Direction tag =>" "$PROBE_LOG")
enum_shape_line=$(line_or_empty "enum Shape area =>" "$PROBE_LOG")
inc1=$(echo "$probe_line" | sed -nE 's/.*=> ([0-9-]+), ([0-9-]+).*/\1/p')
inc2=$(echo "$probe_line" | sed -nE 's/.*=> ([0-9-]+), ([0-9-]+).*/\2/p')
current=$(extract_number "current" "$probe_line")
after_reset=$(extract_number "after_reset" "$probe_line")
add_pair=$(extract_number "add_pair" "$probe_line")
after_clear=$(extract_number "after_clear" "$probe_line")
direct_field=$(extract_number "direct" "$direct_line")
after_direct=$(extract_number "current" "$direct_line")
protocol_nonnull=$(extract_number "nonnull" "$protocol_line")
protocol_dispatch_existential=$(extract_number "existential" "$protocol_dispatch_line")
protocol_dispatch_x20=$(extract_number "x20" "$protocol_dispatch_line")
protocol_dispatch_x0=$(extract_number "x0" "$protocol_dispatch_line")
global_initial=$(extract_number "initial" "$global_line")
global_after=$(extract_number "after_write" "$global_line")
raw_meta_match=$(extract_number "match" "$raw_meta_line")
enum_dir_initial=$(extract_number "initial" "$enum_dir_line")
enum_dir_after=$(extract_number "after_write" "$enum_dir_line")
enum_shape_circle=$(echo "$enum_shape_line" | sed -nE 's/.*circle=([0-9.]+).*/\1/p')
enum_shape_rect=$(echo "$enum_shape_line" | sed -nE 's/.*rect=([0-9.]+).*/\1/p')
throws_ok_line=$(line_or_empty "throws safeDivide =>" "$PROBE_LOG")
throws_err_line=$(line_or_empty "throws safeDivide error =>" "$PROBE_LOG")
generic_line=$(line_or_empty "generic TypedBox =>" "$PROBE_LOG")
string_line=$(line_or_empty "string =>" "$PROBE_LOG")
struct_method_line=$(line_or_empty "struct method =>" "$PROBE_LOG")
tuple_line=$(line_or_empty "tuple =>" "$PROBE_LOG")
optional_line=$(line_or_empty "optional =>" "$PROBE_LOG")
array_line=$(line_or_empty "array =>" "$PROBE_LOG")
closure_line=$(line_or_empty "closure adder =>" "$PROBE_LOG")
reflect_line=$(line_or_empty "reflection =>" "$PROBE_LOG")
error_line=$(line_or_empty "error boxing =>" "$PROBE_LOG")
weakref_line=$(line_or_empty "weak ref =>" "$PROBE_LOG")
conform_line=$(line_or_empty "conformance =>" "$PROBE_LOG")
throws_ok_result=$(extract_number "ok_result" "$throws_ok_line")
throws_ok_err_null=$(extract_number "err_null" "$throws_ok_line")
throws_err_nonnull=$(extract_number "throws_nonnull" "$throws_err_line")
generic_get1=$(extract_number "get1" "$generic_line")
generic_get2=$(extract_number "get2" "$generic_line")
str_char_len=$(extract_number "char_len" "$string_line")
str_utf8_len=$(extract_number "utf8_len" "$string_line")
struct_sum=$(extract_number "point_sum" "$struct_method_line")
struct_product=$(extract_number "point_product" "$struct_method_line")
tuple_first=$(extract_number "first" "$tuple_line")
tuple_second=$(extract_number "second" "$tuple_line")
opt_none=$(extract_number "none_get" "$optional_line")
opt_some=$(extract_number "some_get" "$optional_line")
opt_layout=$(extract_number "some_layout_ok" "$optional_line")
arr_count=$(extract_number "count" "$array_line")
arr_elem2=$(extract_number "elem2" "$array_line")
arr_count_after=$(extract_number "count_after_append" "$array_line")
closure_result=$(extract_number "result" "$closure_line")
reflect_fields=$(extract_number "point_fields" "$reflect_line")
reflect_first_x=$(extract_number "first_field_x" "$reflect_line")
error_nonnull=$(extract_number "nonnull" "$error_line")
error_rc=$(extract_number "rc" "$error_line")
weak_loaded_eq=$(extract_number "loaded_eq_original" "$weakref_line")
conform_nonnull=$(extract_number "witness_nonnull" "$conform_line")

retain_init_line=$(line_or_empty "retain count after init=" "$PROBE_LOG")
retain_after_release_line=$(line_or_empty "retain count after one release=" "$PROBE_LOG")
retain_init=$(extract_number "init" "$retain_init_line")
retain_after_release=$(extract_number "release" "$retain_after_release_line")

malloc_counter_line=$(line_or_empty "malloc_size\(counter\)=" "$PROBE_LOG")
malloc_raw_line=$(line_or_empty "malloc_size\(raw_counter\)=" "$PROBE_LOG")
malloc_counter=$(echo "$malloc_counter_line" | sed -nE 's/.*=([0-9]+).*/\1/p')
malloc_raw=$(echo "$malloc_raw_line" | sed -nE 's/.*=([0-9]+).*/\1/p')

person_bits_line=$(line_or_empty "person raw bits=" "$PROBE_LOG")

has_person_init=0
has_counter_alloc=0
if grep -q "person raw bits=" "$PROBE_LOG"; then has_person_init=1; fi
if grep -q "counter object=" "$PROBE_LOG"; then has_counter_alloc=1; fi
if grep -q "EXC_BAD_ACCESS" "$LLDB_LOG"; then has_bad_access=1; else has_bad_access=0; fi
if grep -q "exited with status" "$LLDB_LOG"; then has_lldb_exit=1; else has_lldb_exit=0; fi

pass_increment=0
pass_reset=0
pass_add_pair=0
pass_clear=0
pass_retain=0
pass_alloc_sizes=0
pass_lldb=0
pass_direct_field=0
pass_protocol_witness=0
pass_protocol_slot=0
pass_global_variable=0
pass_raw_metadata=0
pass_protocol_dispatch=0
pass_protocol_dispatch_semantic=0
pass_enum_simple=0
pass_enum_associated=0
pass_throws_success=0
pass_throws_error=0
pass_generic_type=0
pass_string=0
pass_struct_dispatch=0
pass_tuple_return=0
pass_optional_layout=0
pass_array=0
pass_closure=0
pass_reflection=0
pass_error_boxing=0
pass_weak_ref=0
pass_conformance=0

if [[ "$inc1" == "15" && "$inc2" == "18" && "$current" == "18" ]]; then pass_increment=1; fi
if [[ "$after_reset" == "4" ]]; then pass_reset=1; fi
if [[ "$add_pair" == "17" ]]; then pass_add_pair=1; fi
if [[ "$after_clear" == "0" ]]; then pass_clear=1; fi
if [[ "$retain_init" == "1" && "$retain_after_release" == "1" ]]; then pass_retain=1; fi
if [[ "$malloc_counter" == "32" && "$malloc_raw" == "32" ]]; then pass_alloc_sizes=1; fi
if [[ "$has_person_init" == "1" && "$has_counter_alloc" == "1" && "$has_bad_access" == "0" ]]; then pass_lldb=1; fi
if [[ "$direct_field" == "99" && "$after_direct" == "99" ]]; then pass_direct_field=1; fi
if [[ "$protocol_nonnull" == "1" ]]; then pass_protocol_witness=1; fi
if echo "$protocol_slot_line $protocol_slot1_line $protocol_slot2_line" | grep -Eq "CounterC7current|A2aDP7current"; then pass_protocol_slot=1; fi
if [[ "$protocol_dispatch_existential" != "-2147483648" && "$protocol_dispatch_existential" != "" ]]; then pass_protocol_dispatch=1; fi
if [[ "$protocol_dispatch_existential" == "99" ]]; then pass_protocol_dispatch_semantic=1; fi
if [[ "$global_after" == "321" ]]; then pass_global_variable=1; fi
if [[ "$raw_meta_match" == "1" ]]; then pass_raw_metadata=1; fi
# Direction.north=0 initially; after writing east=2, bridge must return 2
if [[ "$enum_dir_initial" == "0" && "$enum_dir_after" == "2" ]]; then pass_enum_simple=1; fi
# circle area ≈78.54, rect area == 12.0; check integer part only via prefix match
if echo "$enum_shape_circle" | grep -q "^78" && [[ "$enum_shape_rect" == "12.0000"* ]]; then pass_enum_associated=1; fi
if [[ "$throws_ok_result" == "5" && "$throws_ok_err_null" == "1" ]]; then pass_throws_success=1; fi
if [[ "$throws_err_nonnull" == "1" ]]; then pass_throws_error=1; fi
if [[ "$generic_get1" == "42" && "$generic_get2" == "99" ]]; then pass_generic_type=1; fi
if [[ "$str_char_len" == "5" && "$str_utf8_len" == "5" ]]; then pass_string=1; fi
if [[ "$struct_sum" == "7" && "$struct_product" == "12" ]]; then pass_struct_dispatch=1; fi
if [[ "$tuple_first" == "13" && "$tuple_second" == "7" ]]; then pass_tuple_return=1; fi
if [[ "$opt_none" == "-999" && "$opt_some" == "42" && "$opt_layout" == "1" ]]; then pass_optional_layout=1; fi
if [[ "$arr_count" == "5" && "$arr_elem2" == "30" && "$arr_count_after" == "6" ]]; then pass_array=1; fi
if [[ "$closure_result" == "12" ]]; then pass_closure=1; fi
if [[ "$reflect_fields" == "2" && "$reflect_first_x" == "1" ]]; then pass_reflection=1; fi
if [[ "$error_nonnull" == "1" && -n "$error_rc" ]]; then pass_error_boxing=1; fi
if [[ "$weak_loaded_eq" == "1" ]]; then pass_weak_ref=1; fi
if [[ "$conform_nonnull" == "1" ]]; then pass_conformance=1; fi

cat > "$REPORT_JSON" <<JSON
{
  "status": {
    "increment": $pass_increment,
    "reset": $pass_reset,
    "add_pair": $pass_add_pair,
    "clear": $pass_clear,
    "retain_counts": $pass_retain,
    "allocation_sizes": $pass_alloc_sizes,
    "lldb_breakpoints": $pass_lldb,
    "direct_field_write": $pass_direct_field,
    "protocol_witness": $pass_protocol_witness,
    "protocol_witness_slot": $pass_protocol_slot,
    "protocol_witness_dispatch": $pass_protocol_dispatch,
    "protocol_witness_dispatch_semantic": $pass_protocol_dispatch_semantic,
    "global_variable": $pass_global_variable,
    "raw_metadata_header": $pass_raw_metadata,
    "enum_simple_tag": $pass_enum_simple,
    "enum_associated_value": $pass_enum_associated,
    "throws_success": $pass_throws_success,
    "throws_error": $pass_throws_error,
    "generic_type": $pass_generic_type,
    "string": $pass_string,
    "struct_method_dispatch": $pass_struct_dispatch,
    "tuple_return": $pass_tuple_return,
    "optional_layout": $pass_optional_layout,
    "array": $pass_array,
    "closure": $pass_closure,
    "reflection": $pass_reflection,
    "error_boxing": $pass_error_boxing,
    "weak_reference": $pass_weak_ref,
    "conformance_check": $pass_conformance
  },
  "observed": {
    "probe_line": "${probe_line}",
    "direct_line": "${direct_line}",
    "protocol_line": "${protocol_line}",
    "protocol_slot_line": "${protocol_slot_line}",
    "protocol_slot1_line": "${protocol_slot1_line}",
    "protocol_slot2_line": "${protocol_slot2_line}",
    "protocol_dispatch_line": "${protocol_dispatch_line}",
    "global_line": "${global_line}",
    "raw_meta_line": "${raw_meta_line}",
    "person_bits_line": "${person_bits_line}",
    "retain_init": "${retain_init}",
    "retain_after_release": "${retain_after_release}",
    "malloc_counter": "${malloc_counter}",
    "malloc_raw_counter": "${malloc_raw}",
    "lldb_person_init": $has_person_init,
    "lldb_counter_alloc_init": $has_counter_alloc,
    "lldb_bad_access": $has_bad_access,
    "throws_ok_line": "${throws_ok_line}",
    "throws_err_line": "${throws_err_line}",
    "generic_line": "${generic_line}",
    "string_line": "${string_line}",
    "struct_method_line": "${struct_method_line}",
    "tuple_line": "${tuple_line}",
    "optional_line": "${optional_line}",
    "array_line": "${array_line}",
    "closure_line": "${closure_line}",
    "reflect_line": "${reflect_line}",
    "error_line": "${error_line}",
    "weakref_line": "${weakref_line}",
    "conform_line": "${conform_line}"
  },
  "artifacts": {
    "probe_log": "target/runtime-probe/probe.log",
    "lldb_log": "target/runtime-probe/lldb_tmux.log",
    "report_markdown": "target/runtime-probe/parity-report.md"
  }
}
JSON

# ── History record ─────────────────────────────────────────────────────────────
HISTORY_DIR="$OUT_DIR/history"
mkdir -p "$HISTORY_DIR"
RUN_TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
GIT_HASH=$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown")
HISTORY_FILE="$HISTORY_DIR/${RUN_TS//[: ]/_}_${GIT_HASH}.json"

total_checks=29
pass_count=$(( pass_increment + pass_reset + pass_add_pair + pass_clear + pass_retain + pass_alloc_sizes + pass_lldb + pass_direct_field + pass_protocol_witness + pass_protocol_slot + pass_protocol_dispatch + pass_protocol_dispatch_semantic + pass_global_variable + pass_raw_metadata + pass_enum_simple + pass_enum_associated + pass_throws_success + pass_throws_error + pass_generic_type + pass_string + pass_struct_dispatch + pass_tuple_return + pass_optional_layout + pass_array + pass_closure + pass_reflection + pass_error_boxing + pass_weak_ref + pass_conformance ))

cat > "$HISTORY_FILE" <<HIST
{
  "timestamp": "${RUN_TS}",
  "git_hash": "${GIT_HASH}",
  "total": ${total_checks},
  "passed": ${pass_count},
  "failed": $(( total_checks - pass_count )),
  "checks": {
    "increment": $pass_increment,
    "reset": $pass_reset,
    "add_pair": $pass_add_pair,
    "clear": $pass_clear,
    "retain_counts": $pass_retain,
    "allocation_sizes": $pass_alloc_sizes,
    "lldb_breakpoints": $pass_lldb,
    "direct_field_write": $pass_direct_field,
    "protocol_witness": $pass_protocol_witness,
    "protocol_witness_slot": $pass_protocol_slot,
    "protocol_witness_dispatch": $pass_protocol_dispatch,
    "protocol_witness_dispatch_semantic": $pass_protocol_dispatch_semantic,
    "global_variable": $pass_global_variable,
    "raw_metadata_header": $pass_raw_metadata,
    "enum_simple_tag": $pass_enum_simple,
    "enum_associated_value": $pass_enum_associated,
    "throws_success": $pass_throws_success,
    "throws_error": $pass_throws_error,
    "generic_type": $pass_generic_type,
    "string": $pass_string,
    "struct_method_dispatch": $pass_struct_dispatch,
    "tuple_return": $pass_tuple_return,
    "optional_layout": $pass_optional_layout,
    "array": $pass_array,
    "closure": $pass_closure,
    "reflection": $pass_reflection,
    "error_boxing": $pass_error_boxing,
    "weak_reference": $pass_weak_ref,
    "conformance_check": $pass_conformance
  }
}
HIST

echo "History record: $HISTORY_FILE  (${pass_count}/${total_checks} PASS)"

status_symbol() {
  if [[ "$1" == "1" ]]; then echo "PASS"; else echo "FAIL"; fi
}

cat > "$REPORT_MD" <<MD
# Runtime Parity Report

| Check | Result | Evidence |
|---|---|---|
| increment (self_i32_to_i32) | $(status_symbol "$pass_increment") | ${probe_line} |
| reset (self_i32_to_void) | $(status_symbol "$pass_reset") | after_reset=${after_reset} |
| addPair (self_i32_i32_to_i32) | $(status_symbol "$pass_add_pair") | add_pair=${add_pair} |
| clear (self_to_void) | $(status_symbol "$pass_clear") | after_clear=${after_clear} |
| retain counts | $(status_symbol "$pass_retain") | init=${retain_init}, after_release=${retain_after_release} |
| allocation sizes | $(status_symbol "$pass_alloc_sizes") | counter=${malloc_counter}, raw=${malloc_raw} |
| lldb constructor path | $(status_symbol "$pass_lldb") | probe_person=${has_person_init}, probe_counter=${has_counter_alloc}, lldb_exit=${has_lldb_exit}, bad_access=${has_bad_access} |
| direct field write | $(status_symbol "$pass_direct_field") | direct=${direct_field}, current=${after_direct} |
| protocol witness | $(status_symbol "$pass_protocol_witness") | nonnull=${protocol_nonnull} |
| protocol witness slot | $(status_symbol "$pass_protocol_slot") | ${protocol_slot_line} / ${protocol_slot1_line} / ${protocol_slot2_line} |
| protocol witness dispatch | $(status_symbol "$pass_protocol_dispatch") | ${protocol_dispatch_line} |
| protocol witness semantic | $(status_symbol "$pass_protocol_dispatch_semantic") | target=99; ${protocol_dispatch_line} |
| global variable storage | $(status_symbol "$pass_global_variable") | initial=${global_initial}, after_write=${global_after} |
| raw metadata header | $(status_symbol "$pass_raw_metadata") | ${raw_meta_line} |
| enum simple tag (Direction) | $(status_symbol "$pass_enum_simple") | initial=${enum_dir_initial}, after_write=${enum_dir_after} |
| enum associated value (Shape) | $(status_symbol "$pass_enum_associated") | circle=${enum_shape_circle}, rect=${enum_shape_rect} |
| throws — success path | $(status_symbol "$pass_throws_success") | ok_result=${throws_ok_result}, err_null=${throws_ok_err_null} |
| throws — error path | $(status_symbol "$pass_throws_error") | throws_nonnull=${throws_err_nonnull} |
| generic type TypedBox\<Int32\> | $(status_symbol "$pass_generic_type") | get1=${generic_get1}, get2=${generic_get2} |
| string (heap) | $(status_symbol "$pass_string") | char_len=${str_char_len}, utf8_len=${str_utf8_len} |
| struct method dispatch | $(status_symbol "$pass_struct_dispatch") | sum=${struct_sum}, product=${struct_product} |
| tuple return | $(status_symbol "$pass_tuple_return") | first=${tuple_first}, second=${tuple_second} |
| Optional\<T\> layout | $(status_symbol "$pass_optional_layout") | none=${opt_none}, some=${opt_some}, layout_ok=${opt_layout} |
| Array\<T\> | $(status_symbol "$pass_array") | count=${arr_count}, elem2=${arr_elem2}, after_append=${arr_count_after} |
| closure (thick fn ptr) | $(status_symbol "$pass_closure") | result=${closure_result} |
| reflection (Mirror) | $(status_symbol "$pass_reflection") | fields=${reflect_fields}, first_x=${reflect_first_x} |
| error boxing | $(status_symbol "$pass_error_boxing") | nonnull=${error_nonnull}, rc=${error_rc} |
| weak reference | $(status_symbol "$pass_weak_ref") | loaded_eq=${weak_loaded_eq} |
| swift_conformsToProtocol | $(status_symbol "$pass_conformance") | witness_nonnull=${conform_nonnull} |

## Artifacts

- target/runtime-probe/probe.log
- target/runtime-probe/lldb_tmux.log
- target/runtime-probe/parity-report.json
MD

echo "Wrote $REPORT_JSON"
echo "Wrote $REPORT_MD"
