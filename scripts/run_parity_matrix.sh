#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/target/runtime-probe"
FIXTURE_DIR="$OUT_DIR/resilient-fixtures"
PROBE_LOG="$OUT_DIR/probe.log"
LLDB_LOG="$OUT_DIR/lldb_tmux.log"
REPORT_JSON="$OUT_DIR/parity-report.json"
REPORT_MD="$OUT_DIR/parity-report.md"

mkdir -p "$OUT_DIR"
mkdir -p "$FIXTURE_DIR"

cd "$ROOT"
./scripts/build_runtime_thunks.sh
swiftc -emit-library -emit-module -emit-module-path "$FIXTURE_DIR/ResilientFixtures.swiftmodule" -enable-library-evolution -g -module-name ResilientFixtures -o "$FIXTURE_DIR/libResilientFixtures.dylib" examples/ResilientFixtures.swift
swiftc -emit-library -g -I "$FIXTURE_DIR" -L "$FIXTURE_DIR" -lResilientFixtures -o libRustBridge.dylib examples/RustBridge.swift
cargo build --example runtime_raw_probe

RUNTIME_TRY_INCREMENT=1 DYLD_LIBRARY_PATH="$FIXTURE_DIR:." "target/debug/examples/runtime_raw_probe" > "$PROBE_LOG" 2>&1
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
string_storage_line=$(line_or_empty "string storage =>" "$PROBE_LOG")
array_storage_line=$(line_or_empty "array storage =>" "$PROBE_LOG")
closure_line=$(line_or_empty "closure adder =>" "$PROBE_LOG")
reflect_line=$(line_or_empty "reflection =>" "$PROBE_LOG")
error_line=$(line_or_empty "error boxing =>" "$PROBE_LOG")
error_roundtrip_line=$(line_or_empty "error roundtrip =>" "$PROBE_LOG")
objc_interop_line=$(line_or_empty "objc interop =>" "$PROBE_LOG")
weakref_line=$(line_or_empty "weak ref =>" "$PROBE_LOG")
conform_line=$(line_or_empty "conformance =>" "$PROBE_LOG")
async_line=$(line_or_empty "async task =>" "$PROBE_LOG")
actor_line=$(line_or_empty "actor =>" "$PROBE_LOG")
generic_meta_line=$(line_or_empty "generic metadata =>" "$PROBE_LOG")
synth_witness_line=$(line_or_empty "synth witness =>" "$PROBE_LOG")
value_existential_line=$(line_or_empty "value existential =>" "$PROBE_LOG")
resilient_layout_line=$(line_or_empty "resilient layout =>" "$PROBE_LOG")
cross_module_resilient_line=$(line_or_empty "cross-module resilient =>" "$PROBE_LOG")
arc_stress_line=$(line_or_empty "arc stress =>" "$PROBE_LOG")
fuzz_line=$(line_or_empty "fuzz parity =>" "$PROBE_LOG")
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
str_storage_tagged_diff=$(extract_number "tagged_diff" "$string_storage_line")
str_storage_short_ok=$(extract_number "short_utf8_ok" "$string_storage_line")
str_storage_long_ok=$(extract_number "long_utf8_ok" "$string_storage_line")
arr_storage_shared_before=$(extract_number "shared_before" "$array_storage_line")
arr_storage_split_after=$(extract_number "split_after" "$array_storage_line")
arr_storage_original_unchanged=$(extract_number "original_unchanged" "$array_storage_line")
closure_result=$(extract_number "result" "$closure_line")
reflect_fields=$(extract_number "point_fields" "$reflect_line")
reflect_first_x=$(extract_number "first_field_x" "$reflect_line")
error_nonnull=$(extract_number "nonnull" "$error_line")
error_rc=$(extract_number "rc" "$error_line")
error_semantic_ok=$(extract_number "semantic_ok" "$error_roundtrip_line")
objc_selector_ok=$(extract_number "selector_ok" "$objc_interop_line")
objc_string_bridge_ok=$(extract_number "string_bridge_ok" "$objc_interop_line")
objc_array_bridge_ok=$(extract_number "array_bridge_ok" "$objc_interop_line")
weak_loaded_eq=$(extract_number "loaded_eq_original" "$weakref_line")
conform_nonnull=$(extract_number "witness_nonnull" "$conform_line")
async_add=$(extract_number "add" "$async_line")
async_div_ok=$(extract_number "divide_ok" "$async_line")
async_div_throw=$(extract_number "divide_throw" "$async_line")
actor_create=$(extract_number "create" "$actor_line")
actor_inc=$(extract_number "inc" "$actor_line")
actor_cur=$(extract_number "cur" "$actor_line")
generic_meta_distinct=$(extract_number "distinct" "$generic_meta_line")
generic_constrained=$(extract_number "constrained" "$generic_meta_line")
synth_eq_true=$(extract_number "eq_true" "$synth_witness_line")
synth_eq_false=$(extract_number "eq_false" "$synth_witness_line")
synth_dedup_ok=$(extract_number "dedup_ok" "$synth_witness_line")
value_existential_current=$(extract_number "current" "$value_existential_line")
point_size=$(extract_number "point_size" "$resilient_layout_line")
point_stride=$(extract_number "point_stride" "$resilient_layout_line")
point_align=$(extract_number "point_align" "$resilient_layout_line")
resilient_size=$(extract_number "resilient_size" "$resilient_layout_line")
resilient_stride=$(extract_number "resilient_stride" "$resilient_layout_line")
resilient_align=$(extract_number "resilient_align" "$resilient_layout_line")
resilient_b_offset=$(extract_number "b_offset" "$resilient_layout_line")
cross_resilient_size=$(extract_number "size" "$cross_module_resilient_line")
cross_resilient_stride=$(extract_number "stride" "$cross_module_resilient_line")
cross_resilient_align=$(extract_number "align" "$cross_module_resilient_line")
cross_resilient_b_offset=$(extract_number "b_offset" "$cross_module_resilient_line")
cross_resilient_sample_ok=$(extract_number "sample_b_ok" "$cross_module_resilient_line")
arc_swift_edge=$(extract_number "swift_edge" "$arc_stress_line")
arc_runtime_balance=$(extract_number "runtime_balance" "$arc_stress_line")
fuzz_add_ok=$(extract_number "add_ok" "$fuzz_line")
fuzz_divide_ok=$(extract_number "divide_ok" "$fuzz_line")
fuzz_throw_ok=$(extract_number "throw_ok" "$fuzz_line")
fuzz_cases=$(extract_number "cases" "$fuzz_line")

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
synth_eq_lldb_hits=$(grep -c "LLDB_SYNTH_EQ_BREAK" "$LLDB_LOG" 2>/dev/null || true)
synth_hash_lldb_hits=$(grep -c "LLDB_SYNTH_HASH_BREAK" "$LLDB_LOG" 2>/dev/null || true)

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
pass_error_roundtrip=0
pass_objc_interop=0
pass_weak_ref=0
pass_conformance=0
pass_async_task=0
pass_actor_executor=0
pass_generic_metadata=0
pass_synth_witness=0
pass_synth_witness_lldb=0
pass_value_existential=0
pass_resilient_layout_metrics=0
pass_resilient_field_offset=0
pass_cross_module_resilient=0
pass_arc_edge_stress=0
pass_string_storage=0
pass_array_storage=0
pass_fuzz_parity=0

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
if [[ "$str_storage_tagged_diff" == "1" && "$str_storage_short_ok" == "1" && "$str_storage_long_ok" == "1" ]]; then pass_string_storage=1; fi
if [[ "$arr_storage_shared_before" == "1" && "$arr_storage_split_after" == "1" && "$arr_storage_original_unchanged" == "1" ]]; then pass_array_storage=1; fi
if [[ "$closure_result" == "12" ]]; then pass_closure=1; fi
if [[ "$reflect_fields" == "2" && "$reflect_first_x" == "1" ]]; then pass_reflection=1; fi
if [[ "$error_nonnull" == "1" && -n "$error_rc" ]]; then pass_error_boxing=1; fi
if [[ "$error_semantic_ok" == "1" ]]; then pass_error_roundtrip=1; fi
if [[ "$objc_selector_ok" == "1" && "$objc_string_bridge_ok" == "1" && "$objc_array_bridge_ok" == "1" ]]; then pass_objc_interop=1; fi
if [[ "$weak_loaded_eq" == "1" ]]; then pass_weak_ref=1; fi
if [[ "$conform_nonnull" == "1" ]]; then pass_conformance=1; fi
if [[ "$async_add" == "42" && "$async_div_ok" == "5" && "$async_div_throw" == "1" ]]; then pass_async_task=1; fi
if [[ "$actor_create" == "1" && "$actor_inc" == "15" && "$actor_cur" == "15" ]]; then pass_actor_executor=1; fi
if [[ "$generic_meta_distinct" == "1" && "$generic_constrained" == "77" ]]; then pass_generic_metadata=1; fi
if [[ "$synth_eq_true" == "1" && "$synth_eq_false" == "1" && "$synth_dedup_ok" == "1" ]]; then pass_synth_witness=1; fi
if [[ "$synth_eq_lldb_hits" -gt 0 && "$synth_hash_lldb_hits" -gt 0 ]]; then pass_synth_witness_lldb=1; fi
if [[ "$value_existential_current" == "88" ]]; then pass_value_existential=1; fi
if [[ "$point_size" == "8" && "$point_stride" == "8" && "$point_align" == "4" && "$resilient_size" == "16" && "$resilient_stride" == "16" && "$resilient_align" == "8" ]]; then pass_resilient_layout_metrics=1; fi
if [[ "$resilient_b_offset" == "8" ]]; then pass_resilient_field_offset=1; fi
if [[ "$cross_resilient_size" == "16" && "$cross_resilient_stride" == "16" && "$cross_resilient_align" == "8" && "$cross_resilient_b_offset" == "8" && "$cross_resilient_sample_ok" == "1" ]]; then pass_cross_module_resilient=1; fi
if [[ "$arc_swift_edge" == "1" && "$arc_runtime_balance" == "1" ]]; then pass_arc_edge_stress=1; fi
if [[ "$fuzz_add_ok" == "1" && "$fuzz_divide_ok" == "1" && "$fuzz_throw_ok" == "1" && -n "$fuzz_cases" && "$fuzz_cases" -ge 16 ]]; then pass_fuzz_parity=1; fi

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
    "string_storage_internals": $pass_string_storage,
    "array_storage_internals": $pass_array_storage,
    "closure": $pass_closure,
    "reflection": $pass_reflection,
    "error_boxing": $pass_error_boxing,
    "error_roundtrip": $pass_error_roundtrip,
    "objc_interop": $pass_objc_interop,
    "weak_reference": $pass_weak_ref,
    "conformance_check": $pass_conformance,
    "async_task_runtime": $pass_async_task,
    "actor_executor_behavior": $pass_actor_executor,
    "generic_metadata_instantiation": $pass_generic_metadata,
    "synthesized_witness_eq_hash": $pass_synth_witness,
    "synthesized_witness_lldb_hits": $pass_synth_witness_lldb,
    "value_existential_dispatch": $pass_value_existential,
    "resilient_layout_metrics": $pass_resilient_layout_metrics,
    "resilient_field_offset": $pass_resilient_field_offset,
    "cross_module_resilient_layout": $pass_cross_module_resilient,
    "arc_edge_stress": $pass_arc_edge_stress,
    "fuzz_parity": $pass_fuzz_parity
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
    "synth_eq_lldb_hits": ${synth_eq_lldb_hits},
    "synth_hash_lldb_hits": ${synth_hash_lldb_hits},
    "throws_ok_line": "${throws_ok_line}",
    "throws_err_line": "${throws_err_line}",
    "generic_line": "${generic_line}",
    "string_line": "${string_line}",
    "struct_method_line": "${struct_method_line}",
    "tuple_line": "${tuple_line}",
    "optional_line": "${optional_line}",
    "array_line": "${array_line}",
    "string_storage_line": "${string_storage_line}",
    "array_storage_line": "${array_storage_line}",
    "closure_line": "${closure_line}",
    "reflect_line": "${reflect_line}",
    "error_line": "${error_line}",
    "error_roundtrip_line": "${error_roundtrip_line}",
    "objc_interop_line": "${objc_interop_line}",
    "weakref_line": "${weakref_line}",
    "conform_line": "${conform_line}",
    "async_line": "${async_line}",
    "actor_line": "${actor_line}",
    "generic_meta_line": "${generic_meta_line}",
    "synth_witness_line": "${synth_witness_line}",
    "value_existential_line": "${value_existential_line}",
    "resilient_layout_line": "${resilient_layout_line}",
    "cross_module_resilient_line": "${cross_module_resilient_line}",
    "arc_stress_line": "${arc_stress_line}",
    "fuzz_line": "${fuzz_line}"
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

total_checks=44
pass_count=$(( pass_increment + pass_reset + pass_add_pair + pass_clear + pass_retain + pass_alloc_sizes + pass_lldb + pass_direct_field + pass_protocol_witness + pass_protocol_slot + pass_protocol_dispatch + pass_protocol_dispatch_semantic + pass_global_variable + pass_raw_metadata + pass_enum_simple + pass_enum_associated + pass_throws_success + pass_throws_error + pass_generic_type + pass_string + pass_struct_dispatch + pass_tuple_return + pass_optional_layout + pass_array + pass_string_storage + pass_array_storage + pass_closure + pass_reflection + pass_error_boxing + pass_error_roundtrip + pass_objc_interop + pass_weak_ref + pass_conformance + pass_async_task + pass_actor_executor + pass_generic_metadata + pass_synth_witness + pass_synth_witness_lldb + pass_value_existential + pass_resilient_layout_metrics + pass_resilient_field_offset + pass_cross_module_resilient + pass_arc_edge_stress + pass_fuzz_parity ))

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
    "string_storage_internals": $pass_string_storage,
    "array_storage_internals": $pass_array_storage,
    "closure": $pass_closure,
    "reflection": $pass_reflection,
    "error_boxing": $pass_error_boxing,
    "error_roundtrip": $pass_error_roundtrip,
    "objc_interop": $pass_objc_interop,
    "weak_reference": $pass_weak_ref,
    "conformance_check": $pass_conformance,
    "async_task_runtime": $pass_async_task,
    "actor_executor_behavior": $pass_actor_executor,
    "generic_metadata_instantiation": $pass_generic_metadata,
    "synthesized_witness_eq_hash": $pass_synth_witness,
    "synthesized_witness_lldb_hits": $pass_synth_witness_lldb,
    "value_existential_dispatch": $pass_value_existential,
    "resilient_layout_metrics": $pass_resilient_layout_metrics,
    "resilient_field_offset": $pass_resilient_field_offset,
    "cross_module_resilient_layout": $pass_cross_module_resilient,
    "arc_edge_stress": $pass_arc_edge_stress,
    "fuzz_parity": $pass_fuzz_parity
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
| string storage internals | $(status_symbol "$pass_string_storage") | tagged_diff=${str_storage_tagged_diff}, short_ok=${str_storage_short_ok}, long_ok=${str_storage_long_ok} |
| array storage internals | $(status_symbol "$pass_array_storage") | shared_before=${arr_storage_shared_before}, split_after=${arr_storage_split_after}, unchanged=${arr_storage_original_unchanged} |
| closure (thick fn ptr) | $(status_symbol "$pass_closure") | result=${closure_result} |
| reflection (Mirror) | $(status_symbol "$pass_reflection") | fields=${reflect_fields}, first_x=${reflect_first_x} |
| error boxing | $(status_symbol "$pass_error_boxing") | nonnull=${error_nonnull}, rc=${error_rc} |
| error round-trip semantic | $(status_symbol "$pass_error_roundtrip") | semantic_ok=${error_semantic_ok} |
| Objective-C interop | $(status_symbol "$pass_objc_interop") | selector_ok=${objc_selector_ok}, string_bridge_ok=${objc_string_bridge_ok}, array_bridge_ok=${objc_array_bridge_ok} |
| weak reference | $(status_symbol "$pass_weak_ref") | loaded_eq=${weak_loaded_eq} |
| swift_conformsToProtocol | $(status_symbol "$pass_conformance") | witness_nonnull=${conform_nonnull} |
| async/task runtime | $(status_symbol "$pass_async_task") | add=${async_add}, divide_ok=${async_div_ok}, divide_throw=${async_div_throw} |
| actor/executor behavior | $(status_symbol "$pass_actor_executor") | create=${actor_create}, inc=${actor_inc}, cur=${actor_cur} |
| generic metadata instantiation | $(status_symbol "$pass_generic_metadata") | distinct=${generic_meta_distinct}, constrained=${generic_constrained} |
| synthesized witness eq/hash | $(status_symbol "$pass_synth_witness") | eq_true=${synth_eq_true}, eq_false=${synth_eq_false}, dedup_ok=${synth_dedup_ok} |
| synthesized witness LLDB hits | $(status_symbol "$pass_synth_witness_lldb") | eq_hits=${synth_eq_lldb_hits}, hash_hits=${synth_hash_lldb_hits} |
| value existential dispatch | $(status_symbol "$pass_value_existential") | current=${value_existential_current} |
| resilient layout metrics | $(status_symbol "$pass_resilient_layout_metrics") | point(size=${point_size},stride=${point_stride},align=${point_align}) resilient(size=${resilient_size},stride=${resilient_stride},align=${resilient_align}) |
| resilient field offset | $(status_symbol "$pass_resilient_field_offset") | b_offset=${resilient_b_offset} |
| cross-module resilient layout | $(status_symbol "$pass_cross_module_resilient") | size=${cross_resilient_size}, stride=${cross_resilient_stride}, align=${cross_resilient_align}, b_offset=${cross_resilient_b_offset}, sample_ok=${cross_resilient_sample_ok} |
| ARC edge-case stress | $(status_symbol "$pass_arc_edge_stress") | swift_edge=${arc_swift_edge}, runtime_balance=${arc_runtime_balance} |
| fuzz parity (seeded random) | $(status_symbol "$pass_fuzz_parity") | add_ok=${fuzz_add_ok}, divide_ok=${fuzz_divide_ok}, throw_ok=${fuzz_throw_ok}, cases=${fuzz_cases} |

## Artifacts

- target/runtime-probe/probe.log
- target/runtime-probe/lldb_tmux.log
- target/runtime-probe/parity-report.json
MD

echo "Wrote $REPORT_JSON"
echo "Wrote $REPORT_MD"
