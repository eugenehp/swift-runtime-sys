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
enum_payload_line=$(line_or_empty "enum payload =>" "$PROBE_LOG")
codable_line=$(line_or_empty "codable =>" "$PROBE_LOG")
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
enum_multi_semantic_ok=$(extract_number "multi_semantic_ok" "$enum_payload_line")
enum_multi_distinct=$(extract_number "multi_distinct" "$enum_payload_line")
enum_multi_layout_sane=$(extract_number "multi_layout_sane" "$enum_payload_line")
enum_spare_semantic_ok=$(extract_number "spare_semantic_ok" "$enum_payload_line")
enum_spare_nil_zero=$(extract_number "spare_nil_zero" "$enum_payload_line")
enum_spare_some_nonzero=$(extract_number "spare_some_nonzero" "$enum_payload_line")
enum_spare_size_eight=$(extract_number "spare_size_eight" "$enum_payload_line")
codable_encode_ok=$(extract_number "encode_ok" "$codable_line")
codable_roundtrip_ok=$(extract_number "roundtrip_ok" "$codable_line")
codable_known_decode_ok=$(extract_number "known_decode_ok" "$codable_line")
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
generic_specialization_line=$(line_or_empty "generic specialization =>" "$PROBE_LOG")
synth_witness_line=$(line_or_empty "synth witness =>" "$PROBE_LOG")
keypath_synth_line=$(line_or_empty "keypath synth =>" "$PROBE_LOG")
property_wrapper_synth_line=$(line_or_empty "property wrapper synth =>" "$PROBE_LOG")
result_builder_synth_line=$(line_or_empty "result builder synth =>" "$PROBE_LOG")
opaque_return_line=$(line_or_empty "opaque return =>" "$PROBE_LOG")
task_local_line=$(line_or_empty "task local =>" "$PROBE_LOG")
dynamic_replacement_line=$(line_or_empty "dynamic replacement =>" "$PROBE_LOG")
sendable_line=$(line_or_empty "sendable =>" "$PROBE_LOG")
continuation_line=$(line_or_empty "continuation =>" "$PROBE_LOG")
task_group_line=$(line_or_empty "task group =>" "$PROBE_LOG")
async_stream_line=$(line_or_empty "async stream =>" "$PROBE_LOG")
unsafe_memory_line=$(line_or_empty "unsafe memory =>" "$PROBE_LOG")
proto_composition_line=$(line_or_empty "protocol composition =>" "$PROBE_LOG")
enum_raw_value_line=$(line_or_empty "enum raw value =>" "$PROBE_LOG")
option_set_line=$(line_or_empty "option set =>" "$PROBE_LOG")
case_iterable_line=$(line_or_empty "case iterable =>" "$PROBE_LOG")
set_algebra_line=$(line_or_empty "set algebra =>" "$PROBE_LOG")
dictionary_line=$(line_or_empty "dictionary =>" "$PROBE_LOG")
comparable_line=$(line_or_empty "comparable =>" "$PROBE_LOG")
result_line=$(line_or_empty "result =>" "$PROBE_LOG")
data_line=$(line_or_empty "^data =>" "$PROBE_LOG")
uuid_line=$(line_or_empty "uuid =>" "$PROBE_LOG")
character_set_line=$(line_or_empty "character set =>" "$PROBE_LOG")
url_components_line=$(line_or_empty "url components =>" "$PROBE_LOG")
calendar_line=$(line_or_empty "calendar =>" "$PROBE_LOG")
index_set_line=$(line_or_empty "index set =>" "$PROBE_LOG")
time_zone_line=$(line_or_empty "time zone =>" "$PROBE_LOG")
measurement_line=$(line_or_empty "measurement =>" "$PROBE_LOG")
date_formatter_line=$(line_or_empty "date formatter =>" "$PROBE_LOG")
scanner_line=$(line_or_empty "scanner =>" "$PROBE_LOG")
locale_line=$(line_or_empty "locale =>" "$PROBE_LOG")
number_formatter_line=$(line_or_empty "number formatter =>" "$PROBE_LOG")
url_line=$(line_or_empty "url =>" "$PROBE_LOG")
decimal_line=$(line_or_empty "decimal =>" "$PROBE_LOG")
url_request_line=$(line_or_empty "url request =>" "$PROBE_LOG")
data_base64_line=$(line_or_empty "data base64 =>" "$PROBE_LOG")
http_response_line=$(line_or_empty "http url response =>" "$PROBE_LOG")
json_encoder_line=$(line_or_empty "json encoder =>" "$PROBE_LOG")
plist_encoder_line=$(line_or_empty "plist encoder =>" "$PROBE_LOG")
range_line=$(line_or_empty "range =>" "$PROBE_LOG")
value_existential_line=$(line_or_empty "value existential =>" "$PROBE_LOG")
resilient_layout_line=$(line_or_empty "resilient layout =>" "$PROBE_LOG")
cross_module_resilient_line=$(line_or_empty "cross-module resilient =>" "$PROBE_LOG")
cross_module_existential_line=$(line_or_empty "cross-module existential =>" "$PROBE_LOG")
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
generic_specialization_int=$(extract_number "int_ok" "$generic_specialization_line")
generic_specialization_string=$(extract_number "string_ok" "$generic_specialization_line")
generic_reabstract_ok=$(extract_number "reabstract_ok" "$generic_specialization_line")
synth_eq_true=$(extract_number "eq_true" "$synth_witness_line")
synth_eq_false=$(extract_number "eq_false" "$synth_witness_line")
synth_dedup_ok=$(extract_number "dedup_ok" "$synth_witness_line")
keypath_read_ok=$(extract_number "read_ok" "$keypath_synth_line")
keypath_write_ok=$(extract_number "write_ok" "$keypath_synth_line")
keypath_append_ok=$(extract_number "append_ok" "$keypath_synth_line")
property_wrapper_default_ok=$(extract_number "default_ok" "$property_wrapper_synth_line")
property_wrapper_clamp_ok=$(extract_number "clamp_ok" "$property_wrapper_synth_line")
property_wrapper_projected_ok=$(extract_number "projected_ok" "$property_wrapper_synth_line")
property_wrapper_memberwise_ok=$(extract_number "memberwise_ok" "$property_wrapper_synth_line")
result_builder_branch_ok=$(extract_number "branch_ok" "$result_builder_synth_line")
result_builder_optional_ok=$(extract_number "optional_ok" "$result_builder_synth_line")
result_builder_loop_ok=$(extract_number "loop_ok" "$result_builder_synth_line")
opaque_return_value_ok=$(extract_number "value_ok" "$opaque_return_line")
opaque_return_generic_ok=$(extract_number "generic_ok" "$opaque_return_line")
opaque_return_type_ok=$(extract_number "type_ok" "$opaque_return_line")
task_local_outside_ok=$(extract_number "outside_ok" "$task_local_line")
task_local_inside_ok=$(extract_number "inside_ok" "$task_local_line")
task_local_nested_ok=$(extract_number "nested_ok" "$task_local_line")
task_local_restored_ok=$(extract_number "restored_ok" "$task_local_line")
dynamic_replacement_direct_ok=$(extract_number "direct_ok" "$dynamic_replacement_line")
dynamic_replacement_indirect_ok=$(extract_number "indirect_ok" "$dynamic_replacement_line")
sendable_payload_ok=$(extract_number "payload_ok" "$sendable_line")
sendable_detached_ok=$(extract_number "detached_ok" "$sendable_line")
sendable_child_ok=$(extract_number "child_ok" "$sendable_line")
continuation_async_ok=$(extract_number "async_ok" "$continuation_line")
continuation_sync_ok=$(extract_number "sync_ok" "$continuation_line")
continuation_throwing_ok=$(extract_number "throwing_ok" "$continuation_line")
task_group_sum_ok=$(extract_number "sum_ok" "$task_group_line")
task_group_throw_sum_ok=$(extract_number "throw_sum_ok" "$task_group_line")
task_group_max_ok=$(extract_number "max_ok" "$task_group_line")
async_stream_count_ok=$(extract_number "count_ok" "$async_stream_line")
async_stream_sum_ok=$(extract_number "sum_ok" "$async_stream_line")
async_stream_term_ok=$(extract_number "term_ok" "$async_stream_line")
unsafe_field_x_ok=$(extract_number "field_x_ok" "$unsafe_memory_line")
unsafe_field_y_ok=$(extract_number "field_y_ok" "$unsafe_memory_line")
unsafe_ptr_rw_ok=$(extract_number "ptr_rw_ok" "$unsafe_memory_line")
proto_comp_scale_ok=$(extract_number "scale_ok" "$proto_composition_line")
proto_comp_label_ok=$(extract_number "label_ok" "$proto_composition_line")
proto_comp_cast_ok=$(extract_number "cast_ok" "$proto_composition_line")
enum_raw_roundtrip_ok=$(extract_number "roundtrip_ok" "$enum_raw_value_line")
enum_raw_init_ok=$(extract_number "init_ok" "$enum_raw_value_line")
enum_raw_nil_ok=$(extract_number "nil_ok" "$enum_raw_value_line")
enum_raw_auto_inc_ok=$(extract_number "auto_inc_ok" "$enum_raw_value_line")
option_set_contains_ok=$(extract_number "contains_ok" "$option_set_line")
option_set_union_ok=$(extract_number "union_ok" "$option_set_line")
option_set_intersection_ok=$(extract_number "intersection_ok" "$option_set_line")
option_set_raw_ok=$(extract_number "raw_ok" "$option_set_line")
case_iterable_count_ok=$(extract_number "count_ok" "$case_iterable_line")
case_iterable_endpoints_ok=$(extract_number "endpoints_ok" "$case_iterable_line")
case_iterable_sum_ok=$(extract_number "sum_ok" "$case_iterable_line")
case_iterable_order_ok=$(extract_number "order_ok" "$case_iterable_line")
set_union_ok=$(extract_number "union_ok" "$set_algebra_line")
set_intersection_ok=$(extract_number "intersection_ok" "$set_algebra_line")
set_subtract_ok=$(extract_number "subtract_ok" "$set_algebra_line")
set_symdiff_ok=$(extract_number "symdiff_ok" "$set_algebra_line")
dict_lookup_ok=$(extract_number "lookup_ok" "$dictionary_line")
dict_default_ok=$(extract_number "default_ok" "$dictionary_line")
dict_update_ok=$(extract_number "update_ok" "$dictionary_line")
dict_remove_ok=$(extract_number "remove_ok" "$dictionary_line")
comparable_sorted_ok=$(extract_number "sorted_ok" "$comparable_line")
comparable_lt_ok=$(extract_number "lt_ok" "$comparable_line")
comparable_gt_ok=$(extract_number "gt_ok" "$comparable_line")
comparable_eq_ok=$(extract_number "eq_ok" "$comparable_line")
result_get_ok=$(extract_number "get_ok" "$result_line")
result_get_err_ok=$(extract_number "get_err_ok" "$result_line")
result_map_ok=$(extract_number "map_ok" "$result_line")
result_map_err_ok=$(extract_number "map_err_ok" "$result_line")
data_count_ok=$(extract_number "count_ok" "$data_line")
data_sum_ok=$(extract_number "sum_ok" "$data_line")
data_append_ok=$(extract_number "append_ok" "$data_line")
data_bytes_ok=$(extract_number "bytes_ok" "$data_line")
uuid_parse_ok=$(extract_number "parse_ok" "$uuid_line")
uuid_normalized_ok=$(extract_number "normalized_ok" "$uuid_line")
uuid_bytes_ok=$(extract_number "bytes_ok" "$uuid_line")
uuid_invalid_ok=$(extract_number "invalid_ok" "$uuid_line")
charset_digit_ok=$(extract_number "digit_ok" "$character_set_line")
charset_nondigit_ok=$(extract_number "nondigit_ok" "$character_set_line")
charset_vowel_ok=$(extract_number "vowel_ok" "$character_set_line")
charset_nonvowel_ok=$(extract_number "nonvowel_ok" "$character_set_line")
url_scheme_host_ok=$(extract_number "scheme_host_ok" "$url_components_line")
url_port_path_ok=$(extract_number "port_path_ok" "$url_components_line")
url_query_ok=$(extract_number "query_ok" "$url_components_line")
url_fragment_ok=$(extract_number "fragment_ok" "$url_components_line")
calendar_construct_ok=$(extract_number "construct_ok" "$calendar_line")
calendar_roundtrip_ok=$(extract_number "roundtrip_ok" "$calendar_line")
calendar_weekday_ok=$(extract_number "weekday_ok" "$calendar_line")
calendar_leap_ok=$(extract_number "leap_ok" "$calendar_line")
index_set_membership_ok=$(extract_number "membership_ok" "$index_set_line")
index_set_insert_ok=$(extract_number "insert_ok" "$index_set_line")
index_set_remove_ok=$(extract_number "remove_ok" "$index_set_line")
index_set_bounds_ok=$(extract_number "bounds_ok" "$index_set_line")
tz_gmt_offset_ok=$(extract_number "gmt_offset_ok" "$time_zone_line")
tz_gmt_id_ok=$(extract_number "gmt_id_ok" "$time_zone_line")
tz_kolkata_offset_ok=$(extract_number "kolkata_offset_ok" "$time_zone_line")
tz_kolkata_id_ok=$(extract_number "kolkata_id_ok" "$time_zone_line")
measure_length_ok=$(extract_number "length_ok" "$measurement_line")
measure_temp_ok=$(extract_number "temp_ok" "$measurement_line")
measure_mass_ok=$(extract_number "mass_ok" "$measurement_line")
measure_speed_ok=$(extract_number "speed_ok" "$measurement_line")
date_string_ok=$(extract_number "string_ok" "$date_formatter_line")
date_roundtrip_ok=$(extract_number "roundtrip_ok" "$date_formatter_line")
date_iso_string_ok=$(extract_number "iso_string_ok" "$date_formatter_line")
date_iso_roundtrip_ok=$(extract_number "iso_roundtrip_ok" "$date_formatter_line")
scanner_int_ok=$(extract_number "int_ok" "$scanner_line")
scanner_double_ok=$(extract_number "double_ok" "$scanner_line")
scanner_token_ok=$(extract_number "token_ok" "$scanner_line")
scanner_end_ok=$(extract_number "end_ok" "$scanner_line")
locale_identifier_ok=$(extract_number "identifier_ok" "$locale_line")
locale_canonical_ok=$(extract_number "canonical_ok" "$locale_line")
locale_decimal_ok=$(extract_number "decimal_ok" "$locale_line")
locale_components_ok=$(extract_number "components_ok" "$locale_line")
number_format_ok=$(extract_number "format_ok" "$number_formatter_line")
number_parse_ok=$(extract_number "parse_ok" "$number_formatter_line")
number_round_ok=$(extract_number "round_ok" "$number_formatter_line")
number_invalid_ok=$(extract_number "invalid_ok" "$number_formatter_line")
url_scheme_host_path_ok=$(extract_number "scheme_host_path_ok" "$url_line")
url_query_fragment_ok=$(extract_number "query_fragment_ok" "$url_line")
url_absolute_ok=$(extract_number "absolute_ok" "$url_line")
url_relative_ok=$(extract_number "relative_ok" "$url_line")
decimal_add_ok=$(extract_number "add_ok" "$decimal_line")
decimal_mul_ok=$(extract_number "mul_ok" "$decimal_line")
decimal_round_ok=$(extract_number "round_ok" "$decimal_line")
decimal_invalid_ok=$(extract_number "invalid_ok" "$decimal_line")
request_url_method_ok=$(extract_number "url_method_ok" "$url_request_line")
request_header_ok=$(extract_number "header_ok" "$url_request_line")
request_timeout_ok=$(extract_number "timeout_ok" "$url_request_line")
request_body_ok=$(extract_number "body_ok" "$url_request_line")
base64_encode_ok=$(extract_number "encode_ok" "$data_base64_line")
base64_decode_ok=$(extract_number "decode_ok" "$data_base64_line")
base64_ignore_ok=$(extract_number "ignore_ok" "$data_base64_line")
base64_invalid_ok=$(extract_number "invalid_ok" "$data_base64_line")
response_status_code_ok=$(extract_number "status_code_ok" "$http_response_line")
response_header_ok=$(extract_number "header_ok" "$http_response_line")
response_url_ok=$(extract_number "url_ok" "$http_response_line")
response_content_type_ok=$(extract_number "content_type_ok" "$http_response_line")
json_encode_ok=$(extract_number "encode_ok" "$json_encoder_line")
json_decode_ok=$(extract_number "decode_ok" "$json_encoder_line")
json_nested_ok=$(extract_number "nested_ok" "$json_encoder_line")
json_null_ok=$(extract_number "null_ok" "$json_encoder_line")
plist_encode_ok=$(extract_number "encode_ok" "$plist_encoder_line")
plist_decode_ok=$(extract_number "decode_ok" "$plist_encoder_line")
plist_binary_ok=$(extract_number "binary_ok" "$plist_encoder_line")
plist_binary_decode_ok=$(extract_number "binary_decode_ok" "$plist_encoder_line")
range_contains_ok=$(extract_number "contains_ok" "$range_line")
range_exclude_ok=$(extract_number "exclude_ok" "$range_line")
range_not_empty_ok=$(extract_number "not_empty_ok" "$range_line")
range_count_ok=$(extract_number "count_ok" "$range_line")
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
cross_existential_value=$(extract_number "value_current" "$cross_module_existential_line")
cross_existential_ref=$(extract_number "ref_current" "$cross_module_existential_line")
cross_existential_class=$(extract_number "class_current" "$cross_module_existential_line")
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
pass_enum_payload=0
pass_codable=0
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
pass_generic_specialization=0
pass_synth_witness=0
pass_synth_witness_lldb=0
pass_keypath_synth=0
pass_property_wrapper_synth=0
pass_result_builder_synth=0
pass_opaque_return=0
pass_task_local=0
pass_dynamic_replacement=0
pass_sendable=0
pass_continuation=0
pass_task_group=0
pass_async_stream=0
pass_unsafe_memory=0
pass_proto_composition=0
pass_enum_raw_value=0
pass_option_set=0
pass_case_iterable=0
pass_set_algebra=0
pass_dictionary=0
pass_comparable=0
pass_result=0
pass_data=0
pass_uuid=0
pass_character_set=0
pass_url_components=0
pass_calendar=0
pass_index_set=0
pass_time_zone=0
pass_measurement=0
pass_date_formatter=0
pass_scanner=0
pass_locale=0
pass_number_formatter=0
pass_url=0
pass_decimal=0
pass_url_request=0
pass_data_base64=0
pass_http_response=0
pass_json_encoder=0
pass_plist_encoder=0
pass_range=0
pass_value_existential=0
pass_resilient_layout_metrics=0
pass_resilient_field_offset=0
pass_cross_module_resilient=0
pass_cross_module_existential=0
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
if [[ "$enum_multi_semantic_ok" == "1" && "$enum_multi_distinct" == "1" && "$enum_multi_layout_sane" == "1" && "$enum_spare_semantic_ok" == "1" && "$enum_spare_nil_zero" == "1" && "$enum_spare_some_nonzero" == "1" && "$enum_spare_size_eight" == "1" ]]; then pass_enum_payload=1; fi
if [[ "$codable_encode_ok" == "1" && "$codable_roundtrip_ok" == "1" && "$codable_known_decode_ok" == "1" ]]; then pass_codable=1; fi
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
if [[ "$generic_specialization_int" == "1" && "$generic_specialization_string" == "1" && "$generic_reabstract_ok" == "1" ]]; then pass_generic_specialization=1; fi
if [[ "$synth_eq_true" == "1" && "$synth_eq_false" == "1" && "$synth_dedup_ok" == "1" ]]; then pass_synth_witness=1; fi
if [[ "$synth_eq_lldb_hits" -gt 0 && "$synth_hash_lldb_hits" -gt 0 ]]; then pass_synth_witness_lldb=1; fi
if [[ "$keypath_read_ok" == "1" && "$keypath_write_ok" == "1" && "$keypath_append_ok" == "1" ]]; then pass_keypath_synth=1; fi
if [[ "$property_wrapper_default_ok" == "1" && "$property_wrapper_clamp_ok" == "1" && "$property_wrapper_projected_ok" == "1" && "$property_wrapper_memberwise_ok" == "1" ]]; then pass_property_wrapper_synth=1; fi
if [[ "$result_builder_branch_ok" == "1" && "$result_builder_optional_ok" == "1" && "$result_builder_loop_ok" == "1" ]]; then pass_result_builder_synth=1; fi
if [[ "$opaque_return_value_ok" == "1" && "$opaque_return_generic_ok" == "1" && "$opaque_return_type_ok" == "1" ]]; then pass_opaque_return=1; fi
if [[ "$task_local_outside_ok" == "1" && "$task_local_inside_ok" == "1" && "$task_local_nested_ok" == "1" && "$task_local_restored_ok" == "1" ]]; then pass_task_local=1; fi
if [[ "$dynamic_replacement_direct_ok" == "1" && "$dynamic_replacement_indirect_ok" == "1" ]]; then pass_dynamic_replacement=1; fi
if [[ "$sendable_payload_ok" == "1" && "$sendable_detached_ok" == "1" && "$sendable_child_ok" == "1" ]]; then pass_sendable=1; fi
if [[ "$continuation_async_ok" == "1" && "$continuation_sync_ok" == "1" && "$continuation_throwing_ok" == "1" ]]; then pass_continuation=1; fi
if [[ "$task_group_sum_ok" == "1" && "$task_group_throw_sum_ok" == "1" && "$task_group_max_ok" == "1" ]]; then pass_task_group=1; fi
if [[ "$async_stream_count_ok" == "1" && "$async_stream_sum_ok" == "1" && "$async_stream_term_ok" == "1" ]]; then pass_async_stream=1; fi
if [[ "$unsafe_field_x_ok" == "1" && "$unsafe_field_y_ok" == "1" && "$unsafe_ptr_rw_ok" == "1" ]]; then pass_unsafe_memory=1; fi
if [[ "$proto_comp_scale_ok" == "1" && "$proto_comp_label_ok" == "1" && "$proto_comp_cast_ok" == "1" ]]; then pass_proto_composition=1; fi
if [[ "$enum_raw_roundtrip_ok" == "1" && "$enum_raw_init_ok" == "1" && "$enum_raw_nil_ok" == "1" && "$enum_raw_auto_inc_ok" == "1" ]]; then pass_enum_raw_value=1; fi
if [[ "$option_set_contains_ok" == "1" && "$option_set_union_ok" == "1" && "$option_set_intersection_ok" == "1" && "$option_set_raw_ok" == "1" ]]; then pass_option_set=1; fi
if [[ "$case_iterable_count_ok" == "1" && "$case_iterable_endpoints_ok" == "1" && "$case_iterable_sum_ok" == "1" && "$case_iterable_order_ok" == "1" ]]; then pass_case_iterable=1; fi
if [[ "$set_union_ok" == "1" && "$set_intersection_ok" == "1" && "$set_subtract_ok" == "1" && "$set_symdiff_ok" == "1" ]]; then pass_set_algebra=1; fi
if [[ "$dict_lookup_ok" == "1" && "$dict_default_ok" == "1" && "$dict_update_ok" == "1" && "$dict_remove_ok" == "1" ]]; then pass_dictionary=1; fi
if [[ "$comparable_sorted_ok" == "1" && "$comparable_lt_ok" == "1" && "$comparable_gt_ok" == "1" && "$comparable_eq_ok" == "1" ]]; then pass_comparable=1; fi
if [[ "$result_get_ok" == "1" && "$result_get_err_ok" == "1" && "$result_map_ok" == "1" && "$result_map_err_ok" == "1" ]]; then pass_result=1; fi
if [[ "$data_count_ok" == "1" && "$data_sum_ok" == "1" && "$data_append_ok" == "1" && "$data_bytes_ok" == "1" ]]; then pass_data=1; fi
if [[ "$uuid_parse_ok" == "1" && "$uuid_normalized_ok" == "1" && "$uuid_bytes_ok" == "1" && "$uuid_invalid_ok" == "1" ]]; then pass_uuid=1; fi
if [[ "$charset_digit_ok" == "1" && "$charset_nondigit_ok" == "1" && "$charset_vowel_ok" == "1" && "$charset_nonvowel_ok" == "1" ]]; then pass_character_set=1; fi
if [[ "$url_scheme_host_ok" == "1" && "$url_port_path_ok" == "1" && "$url_query_ok" == "1" && "$url_fragment_ok" == "1" ]]; then pass_url_components=1; fi
if [[ "$calendar_construct_ok" == "1" && "$calendar_roundtrip_ok" == "1" && "$calendar_weekday_ok" == "1" && "$calendar_leap_ok" == "1" ]]; then pass_calendar=1; fi
if [[ "$index_set_membership_ok" == "1" && "$index_set_insert_ok" == "1" && "$index_set_remove_ok" == "1" && "$index_set_bounds_ok" == "1" ]]; then pass_index_set=1; fi
if [[ "$tz_gmt_offset_ok" == "1" && "$tz_gmt_id_ok" == "1" && "$tz_kolkata_offset_ok" == "1" && "$tz_kolkata_id_ok" == "1" ]]; then pass_time_zone=1; fi
if [[ "$measure_length_ok" == "1" && "$measure_temp_ok" == "1" && "$measure_mass_ok" == "1" && "$measure_speed_ok" == "1" ]]; then pass_measurement=1; fi
if [[ "$date_string_ok" == "1" && "$date_roundtrip_ok" == "1" && "$date_iso_string_ok" == "1" && "$date_iso_roundtrip_ok" == "1" ]]; then pass_date_formatter=1; fi
if [[ "$scanner_int_ok" == "1" && "$scanner_double_ok" == "1" && "$scanner_token_ok" == "1" && "$scanner_end_ok" == "1" ]]; then pass_scanner=1; fi
if [[ "$locale_identifier_ok" == "1" && "$locale_canonical_ok" == "1" && "$locale_decimal_ok" == "1" && "$locale_components_ok" == "1" ]]; then pass_locale=1; fi
if [[ "$number_format_ok" == "1" && "$number_parse_ok" == "1" && "$number_round_ok" == "1" && "$number_invalid_ok" == "1" ]]; then pass_number_formatter=1; fi
if [[ "$url_scheme_host_path_ok" == "1" && "$url_query_fragment_ok" == "1" && "$url_absolute_ok" == "1" && "$url_relative_ok" == "1" ]]; then pass_url=1; fi
if [[ "$decimal_add_ok" == "1" && "$decimal_mul_ok" == "1" && "$decimal_round_ok" == "1" && "$decimal_invalid_ok" == "1" ]]; then pass_decimal=1; fi
if [[ "$request_url_method_ok" == "1" && "$request_header_ok" == "1" && "$request_timeout_ok" == "1" && "$request_body_ok" == "1" ]]; then pass_url_request=1; fi
if [[ "$base64_encode_ok" == "1" && "$base64_decode_ok" == "1" && "$base64_ignore_ok" == "1" && "$base64_invalid_ok" == "1" ]]; then pass_data_base64=1; fi
if [[ "$response_status_code_ok" == "1" && "$response_header_ok" == "1" && "$response_url_ok" == "1" && "$response_content_type_ok" == "1" ]]; then pass_http_response=1; fi
if [[ "$json_encode_ok" == "1" && "$json_decode_ok" == "1" && "$json_nested_ok" == "1" && "$json_null_ok" == "1" ]]; then pass_json_encoder=1; fi
if [[ "$plist_encode_ok" == "1" && "$plist_decode_ok" == "1" && "$plist_binary_ok" == "1" && "$plist_binary_decode_ok" == "1" ]]; then pass_plist_encoder=1; fi
if [[ "$range_contains_ok" == "1" && "$range_exclude_ok" == "1" && "$range_not_empty_ok" == "1" && "$range_count_ok" == "1" ]]; then pass_range=1; fi
if [[ "$value_existential_current" == "88" ]]; then pass_value_existential=1; fi
if [[ "$point_size" == "8" && "$point_stride" == "8" && "$point_align" == "4" && "$resilient_size" == "16" && "$resilient_stride" == "16" && "$resilient_align" == "8" ]]; then pass_resilient_layout_metrics=1; fi
if [[ "$resilient_b_offset" == "8" ]]; then pass_resilient_field_offset=1; fi
if [[ "$cross_resilient_size" == "16" && "$cross_resilient_stride" == "16" && "$cross_resilient_align" == "8" && "$cross_resilient_b_offset" == "8" && "$cross_resilient_sample_ok" == "1" ]]; then pass_cross_module_resilient=1; fi
if [[ "$cross_existential_value" == "91" && "$cross_existential_ref" == "73" && "$cross_existential_class" == "64" ]]; then pass_cross_module_existential=1; fi
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
    "enum_payload_encoding": $pass_enum_payload,
    "codable_synthesized": $pass_codable,
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
    "generic_specialization_reabstraction": $pass_generic_specialization,
    "synthesized_witness_eq_hash": $pass_synth_witness,
    "synthesized_witness_lldb_hits": $pass_synth_witness_lldb,
    "keypath_synthesis": $pass_keypath_synth,
    "property_wrapper_synthesis": $pass_property_wrapper_synth,
    "result_builder_synthesis": $pass_result_builder_synth,
    "opaque_return_types": $pass_opaque_return,
    "task_local_runtime": $pass_task_local,
    "dynamic_replacement": $pass_dynamic_replacement,
    "sendable_concurrency": $pass_sendable,
    "checked_continuation": $pass_continuation,
    "task_group_concurrency": $pass_task_group,
    "async_stream": $pass_async_stream,
    "unsafe_memory_layout": $pass_unsafe_memory,
    "protocol_composition_existential": $pass_proto_composition,
    "enum_raw_value_synthesis": $pass_enum_raw_value,
    "option_set_synthesis": $pass_option_set,
    "case_iterable_synthesis": $pass_case_iterable,
    "set_algebra": $pass_set_algebra,
    "dictionary_semantics": $pass_dictionary,
    "comparable_synthesis": $pass_comparable,
    "result_semantics": $pass_result,
    "data_semantics": $pass_data,
    "uuid_semantics": $pass_uuid,
    "character_set_semantics": $pass_character_set,
    "url_components_semantics": $pass_url_components,
    "calendar_semantics": $pass_calendar,
    "index_set_semantics": $pass_index_set,
    "time_zone_semantics": $pass_time_zone,
    "measurement_semantics": $pass_measurement,
    "date_formatter_semantics": $pass_date_formatter,
    "scanner_semantics": $pass_scanner,
    "locale_semantics": $pass_locale,
    "number_formatter_semantics": $pass_number_formatter,
    "url_semantics": $pass_url,
    "decimal_semantics": $pass_decimal,
    "url_request_semantics": $pass_url_request,
    "data_base64_semantics": $pass_data_base64,
    "http_url_response_semantics": $pass_http_response,
    "json_encoder_semantics": $pass_json_encoder,
    "plist_encoder_semantics": $pass_plist_encoder,
    "range_semantics": $pass_range,
    "plist_encoder_semantics": $pass_plist_encoder,
    "range_semantics": $pass_range,
    "value_existential_dispatch": $pass_value_existential,
    "resilient_layout_metrics": $pass_resilient_layout_metrics,
    "resilient_field_offset": $pass_resilient_field_offset,
    "cross_module_resilient_layout": $pass_cross_module_resilient,
    "cross_module_existential_dispatch": $pass_cross_module_existential,
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
    "enum_payload_line": "${enum_payload_line}",
    "codable_line": "${codable_line}",
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
    "generic_specialization_line": "${generic_specialization_line}",
    "synth_witness_line": "${synth_witness_line}",
    "keypath_synth_line": "${keypath_synth_line}",
    "property_wrapper_synth_line": "${property_wrapper_synth_line}",
    "result_builder_synth_line": "${result_builder_synth_line}",
    "opaque_return_line": "${opaque_return_line}",
    "task_local_line": "${task_local_line}",
    "dynamic_replacement_line": "${dynamic_replacement_line}",
    "sendable_line": "${sendable_line}",
    "value_existential_line": "${value_existential_line}",
    "resilient_layout_line": "${resilient_layout_line}",
    "cross_module_resilient_line": "${cross_module_resilient_line}",
    "cross_module_existential_line": "${cross_module_existential_line}",
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

total_checks=87
pass_count=$(( pass_increment + pass_reset + pass_add_pair + pass_clear + pass_retain + pass_alloc_sizes + pass_lldb + pass_direct_field + pass_protocol_witness + pass_protocol_slot + pass_protocol_dispatch + pass_protocol_dispatch_semantic + pass_global_variable + pass_raw_metadata + pass_enum_simple + pass_enum_associated + pass_enum_payload + pass_codable + pass_throws_success + pass_throws_error + pass_generic_type + pass_string + pass_struct_dispatch + pass_tuple_return + pass_optional_layout + pass_array + pass_string_storage + pass_array_storage + pass_closure + pass_reflection + pass_error_boxing + pass_error_roundtrip + pass_objc_interop + pass_weak_ref + pass_conformance + pass_async_task + pass_actor_executor + pass_generic_metadata + pass_generic_specialization + pass_synth_witness + pass_synth_witness_lldb + pass_keypath_synth + pass_property_wrapper_synth + pass_result_builder_synth + pass_opaque_return + pass_task_local + pass_dynamic_replacement + pass_sendable + pass_continuation + pass_task_group + pass_async_stream + pass_unsafe_memory + pass_proto_composition + pass_enum_raw_value + pass_option_set + pass_case_iterable + pass_set_algebra + pass_dictionary + pass_comparable + pass_result + pass_data + pass_uuid + pass_character_set + pass_url_components + pass_calendar + pass_index_set + pass_time_zone + pass_measurement + pass_date_formatter + pass_scanner + pass_locale + pass_number_formatter + pass_url + pass_decimal + pass_url_request + pass_data_base64 + pass_http_response + pass_json_encoder + pass_plist_encoder + pass_range + pass_value_existential + pass_resilient_layout_metrics + pass_resilient_field_offset + pass_cross_module_resilient + pass_cross_module_existential + pass_arc_edge_stress + pass_fuzz_parity ))

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
    "enum_payload_encoding": $pass_enum_payload,
    "codable_synthesized": $pass_codable,
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
    "generic_specialization_reabstraction": $pass_generic_specialization,
    "synthesized_witness_eq_hash": $pass_synth_witness,
    "synthesized_witness_lldb_hits": $pass_synth_witness_lldb,
    "keypath_synthesis": $pass_keypath_synth,
    "property_wrapper_synthesis": $pass_property_wrapper_synth,
    "result_builder_synthesis": $pass_result_builder_synth,
    "opaque_return_types": $pass_opaque_return,
    "task_local_runtime": $pass_task_local,
    "dynamic_replacement": $pass_dynamic_replacement,
    "sendable_concurrency": $pass_sendable,
    "checked_continuation": $pass_continuation,
    "task_group_concurrency": $pass_task_group,
    "async_stream": $pass_async_stream,
    "unsafe_memory_layout": $pass_unsafe_memory,
    "protocol_composition_existential": $pass_proto_composition,
    "enum_raw_value_synthesis": $pass_enum_raw_value,
    "option_set_synthesis": $pass_option_set,
    "case_iterable_synthesis": $pass_case_iterable,
    "set_algebra": $pass_set_algebra,
    "dictionary_semantics": $pass_dictionary,
    "comparable_synthesis": $pass_comparable,
    "result_semantics": $pass_result,
    "data_semantics": $pass_data,
    "uuid_semantics": $pass_uuid,
    "character_set_semantics": $pass_character_set,
    "url_components_semantics": $pass_url_components,
    "calendar_semantics": $pass_calendar,
    "index_set_semantics": $pass_index_set,
    "time_zone_semantics": $pass_time_zone,
    "measurement_semantics": $pass_measurement,
    "date_formatter_semantics": $pass_date_formatter,
    "scanner_semantics": $pass_scanner,
    "locale_semantics": $pass_locale,
    "number_formatter_semantics": $pass_number_formatter,
    "url_semantics": $pass_url,
    "decimal_semantics": $pass_decimal,
    "url_request_semantics": $pass_url_request,
    "data_base64_semantics": $pass_data_base64,
    "value_existential_dispatch": $pass_value_existential,
    "resilient_layout_metrics": $pass_resilient_layout_metrics,
    "resilient_field_offset": $pass_resilient_field_offset,
    "cross_module_resilient_layout": $pass_cross_module_resilient,
    "cross_module_existential_dispatch": $pass_cross_module_existential,
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
| enum payload encoding | $(status_symbol "$pass_enum_payload") | multi_semantic_ok=${enum_multi_semantic_ok}, multi_distinct=${enum_multi_distinct}, multi_layout_sane=${enum_multi_layout_sane}, spare_semantic_ok=${enum_spare_semantic_ok}, spare_nil_zero=${enum_spare_nil_zero}, spare_some_nonzero=${enum_spare_some_nonzero}, spare_size_eight=${enum_spare_size_eight} |
| Codable synthesized round-trip | $(status_symbol "$pass_codable") | encode_ok=${codable_encode_ok}, roundtrip_ok=${codable_roundtrip_ok}, known_decode_ok=${codable_known_decode_ok} |
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
| generic specialization/reabstraction | $(status_symbol "$pass_generic_specialization") | int_ok=${generic_specialization_int}, string_ok=${generic_specialization_string}, reabstract_ok=${generic_reabstract_ok} |
| synthesized witness eq/hash | $(status_symbol "$pass_synth_witness") | eq_true=${synth_eq_true}, eq_false=${synth_eq_false}, dedup_ok=${synth_dedup_ok} |
| synthesized witness LLDB hits | $(status_symbol "$pass_synth_witness_lldb") | eq_hits=${synth_eq_lldb_hits}, hash_hits=${synth_hash_lldb_hits} |
| keypath synthesis | $(status_symbol "$pass_keypath_synth") | read_ok=${keypath_read_ok}, write_ok=${keypath_write_ok}, append_ok=${keypath_append_ok} |
| property-wrapper synthesis | $(status_symbol "$pass_property_wrapper_synth") | default_ok=${property_wrapper_default_ok}, clamp_ok=${property_wrapper_clamp_ok}, projected_ok=${property_wrapper_projected_ok}, memberwise_ok=${property_wrapper_memberwise_ok} |
| result-builder synthesis | $(status_symbol "$pass_result_builder_synth") | branch_ok=${result_builder_branch_ok}, optional_ok=${result_builder_optional_ok}, loop_ok=${result_builder_loop_ok} |
| opaque return types | $(status_symbol "$pass_opaque_return") | value_ok=${opaque_return_value_ok}, generic_ok=${opaque_return_generic_ok}, type_ok=${opaque_return_type_ok} |
| task-local runtime | $(status_symbol "$pass_task_local") | outside_ok=${task_local_outside_ok}, inside_ok=${task_local_inside_ok}, nested_ok=${task_local_nested_ok}, restored_ok=${task_local_restored_ok} |
| dynamic replacement | $(status_symbol "$pass_dynamic_replacement") | direct_ok=${dynamic_replacement_direct_ok}, indirect_ok=${dynamic_replacement_indirect_ok} |
| sendable concurrency | $(status_symbol "$pass_sendable") | payload_ok=${sendable_payload_ok}, detached_ok=${sendable_detached_ok}, child_ok=${sendable_child_ok} |
| checked continuation | $(status_symbol "$pass_continuation") | async_ok=${continuation_async_ok}, sync_ok=${continuation_sync_ok}, throwing_ok=${continuation_throwing_ok} |
| task group concurrency | $(status_symbol "$pass_task_group") | sum_ok=${task_group_sum_ok}, throw_sum_ok=${task_group_throw_sum_ok}, max_ok=${task_group_max_ok} |
| AsyncStream | $(status_symbol "$pass_async_stream") | count_ok=${async_stream_count_ok}, sum_ok=${async_stream_sum_ok}, term_ok=${async_stream_term_ok} |
| unsafe memory layout | $(status_symbol "$pass_unsafe_memory") | field_x_ok=${unsafe_field_x_ok}, field_y_ok=${unsafe_field_y_ok}, ptr_rw_ok=${unsafe_ptr_rw_ok} |
| protocol composition existential | $(status_symbol "$pass_proto_composition") | scale_ok=${proto_comp_scale_ok}, label_ok=${proto_comp_label_ok}, cast_ok=${proto_comp_cast_ok} |
| enum raw-value synthesis | $(status_symbol "$pass_enum_raw_value") | roundtrip_ok=${enum_raw_roundtrip_ok}, init_ok=${enum_raw_init_ok}, nil_ok=${enum_raw_nil_ok}, auto_inc_ok=${enum_raw_auto_inc_ok} |
| OptionSet synthesis | $(status_symbol "$pass_option_set") | contains_ok=${option_set_contains_ok}, union_ok=${option_set_union_ok}, intersection_ok=${option_set_intersection_ok}, raw_ok=${option_set_raw_ok} |
| CaseIterable synthesis | $(status_symbol "$pass_case_iterable") | count_ok=${case_iterable_count_ok}, endpoints_ok=${case_iterable_endpoints_ok}, sum_ok=${case_iterable_sum_ok}, order_ok=${case_iterable_order_ok} |
| set algebra | $(status_symbol "$pass_set_algebra") | union_ok=${set_union_ok}, intersection_ok=${set_intersection_ok}, subtract_ok=${set_subtract_ok}, symdiff_ok=${set_symdiff_ok} |
| dictionary semantics | $(status_symbol "$pass_dictionary") | lookup_ok=${dict_lookup_ok}, default_ok=${dict_default_ok}, update_ok=${dict_update_ok}, remove_ok=${dict_remove_ok} |
| comparable synthesis | $(status_symbol "$pass_comparable") | sorted_ok=${comparable_sorted_ok}, lt_ok=${comparable_lt_ok}, gt_ok=${comparable_gt_ok}, eq_ok=${comparable_eq_ok} |
| result semantics | $(status_symbol "$pass_result") | get_ok=${result_get_ok}, get_err_ok=${result_get_err_ok}, map_ok=${result_map_ok}, map_err_ok=${result_map_err_ok} |
| data semantics | $(status_symbol "$pass_data") | count_ok=${data_count_ok}, sum_ok=${data_sum_ok}, append_ok=${data_append_ok}, bytes_ok=${data_bytes_ok} |
| uuid semantics | $(status_symbol "$pass_uuid") | parse_ok=${uuid_parse_ok}, normalized_ok=${uuid_normalized_ok}, bytes_ok=${uuid_bytes_ok}, invalid_ok=${uuid_invalid_ok} |
| character set semantics | $(status_symbol "$pass_character_set") | digit_ok=${charset_digit_ok}, nondigit_ok=${charset_nondigit_ok}, vowel_ok=${charset_vowel_ok}, nonvowel_ok=${charset_nonvowel_ok} |
| URLComponents semantics | $(status_symbol "$pass_url_components") | scheme_host_ok=${url_scheme_host_ok}, port_path_ok=${url_port_path_ok}, query_ok=${url_query_ok}, fragment_ok=${url_fragment_ok} |
| calendar semantics | $(status_symbol "$pass_calendar") | construct_ok=${calendar_construct_ok}, roundtrip_ok=${calendar_roundtrip_ok}, weekday_ok=${calendar_weekday_ok}, leap_ok=${calendar_leap_ok} |
| IndexSet semantics | $(status_symbol "$pass_index_set") | membership_ok=${index_set_membership_ok}, insert_ok=${index_set_insert_ok}, remove_ok=${index_set_remove_ok}, bounds_ok=${index_set_bounds_ok} |
| time zone semantics | $(status_symbol "$pass_time_zone") | gmt_offset_ok=${tz_gmt_offset_ok}, gmt_id_ok=${tz_gmt_id_ok}, kolkata_offset_ok=${tz_kolkata_offset_ok}, kolkata_id_ok=${tz_kolkata_id_ok} |
| measurement semantics | $(status_symbol "$pass_measurement") | length_ok=${measure_length_ok}, temp_ok=${measure_temp_ok}, mass_ok=${measure_mass_ok}, speed_ok=${measure_speed_ok} |
| date formatter semantics | $(status_symbol "$pass_date_formatter") | string_ok=${date_string_ok}, roundtrip_ok=${date_roundtrip_ok}, iso_string_ok=${date_iso_string_ok}, iso_roundtrip_ok=${date_iso_roundtrip_ok} |
| scanner semantics | $(status_symbol "$pass_scanner") | int_ok=${scanner_int_ok}, double_ok=${scanner_double_ok}, token_ok=${scanner_token_ok}, end_ok=${scanner_end_ok} |
| locale semantics | $(status_symbol "$pass_locale") | identifier_ok=${locale_identifier_ok}, canonical_ok=${locale_canonical_ok}, decimal_ok=${locale_decimal_ok}, components_ok=${locale_components_ok} |
| number formatter semantics | $(status_symbol "$pass_number_formatter") | format_ok=${number_format_ok}, parse_ok=${number_parse_ok}, round_ok=${number_round_ok}, invalid_ok=${number_invalid_ok} |
| URL semantics | $(status_symbol "$pass_url") | scheme_host_path_ok=${url_scheme_host_path_ok}, query_fragment_ok=${url_query_fragment_ok}, absolute_ok=${url_absolute_ok}, relative_ok=${url_relative_ok} |
| decimal semantics | $(status_symbol "$pass_decimal") | add_ok=${decimal_add_ok}, mul_ok=${decimal_mul_ok}, round_ok=${decimal_round_ok}, invalid_ok=${decimal_invalid_ok} |
| URLRequest semantics | $(status_symbol "$pass_url_request") | url_method_ok=${request_url_method_ok}, header_ok=${request_header_ok}, timeout_ok=${request_timeout_ok}, body_ok=${request_body_ok} |
| data base64 semantics | $(status_symbol "$pass_data_base64") | encode_ok=${base64_encode_ok}, decode_ok=${base64_decode_ok}, ignore_ok=${base64_ignore_ok}, invalid_ok=${base64_invalid_ok} |
| http url response semantics | $(status_symbol "$pass_http_response") | status_code_ok=${response_status_code_ok}, header_ok=${response_header_ok}, url_ok=${response_url_ok}, content_type_ok=${response_content_type_ok} |
| json encoder semantics | $(status_symbol "$pass_json_encoder") | encode_ok=${json_encode_ok}, decode_ok=${json_decode_ok}, nested_ok=${json_nested_ok}, null_ok=${json_null_ok} |
| plist encoder semantics | $(status_symbol "$pass_plist_encoder") | encode_ok=${plist_encode_ok}, decode_ok=${plist_decode_ok}, binary_ok=${plist_binary_ok}, binary_decode_ok=${plist_binary_decode_ok} |
| range semantics | $(status_symbol "$pass_range") | contains_ok=${range_contains_ok}, exclude_ok=${range_exclude_ok}, not_empty_ok=${range_not_empty_ok}, count_ok=${range_count_ok} |
| value existential dispatch | $(status_symbol "$pass_value_existential") | current=${value_existential_current} |
| resilient layout metrics | $(status_symbol "$pass_resilient_layout_metrics") | point(size=${point_size},stride=${point_stride},align=${point_align}) resilient(size=${resilient_size},stride=${resilient_stride},align=${resilient_align}) |
| resilient field offset | $(status_symbol "$pass_resilient_field_offset") | b_offset=${resilient_b_offset} |
| cross-module resilient layout | $(status_symbol "$pass_cross_module_resilient") | size=${cross_resilient_size}, stride=${cross_resilient_stride}, align=${cross_resilient_align}, b_offset=${cross_resilient_b_offset}, sample_ok=${cross_resilient_sample_ok} |
| cross-module existential dispatch | $(status_symbol "$pass_cross_module_existential") | value_current=${cross_existential_value}, ref_current=${cross_existential_ref}, class_current=${cross_existential_class} |
| ARC edge-case stress | $(status_symbol "$pass_arc_edge_stress") | swift_edge=${arc_swift_edge}, runtime_balance=${arc_runtime_balance} |
| fuzz parity (seeded random) | $(status_symbol "$pass_fuzz_parity") | add_ok=${fuzz_add_ok}, divide_ok=${fuzz_divide_ok}, throw_ok=${fuzz_throw_ok}, cases=${fuzz_cases} |

## Artifacts

- target/runtime-probe/probe.log
- target/runtime-probe/lldb_tmux.log
- target/runtime-probe/parity-report.json
MD

echo "Wrote $REPORT_JSON"
echo "Wrote $REPORT_MD"
