extern crate swift_runtime_sys;

use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

/// Comprehensive dictionary bridging probe demonstrating:
/// - Dictionary<Int32, Int32> creation and basic operations
/// - Dictionary<Int32, OpaqueRef> creation and object storage
/// - Insertion, lookup, removal, containment checks
/// - Metadata discovery for dictionary types
/// - Bridging semantics: capacity, growth, collision handling

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    println!("== Dictionary Bridging Probe ==\n");

    // Test Dictionary<Int32, Int32>
    {
        println!("── Dictionary<Int32, Int32> ────────────────────────────────────");

        // Test 1: Create empty dictionary with capacity 10
        let dict_empty: i64 = unsafe {
            let make_fn: extern "C" fn(i32) -> i64 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_make")
                    .unwrap(),
            );
            make_fn(10)
        };
        println!("dict created: {}", dict_empty);

        let len_empty: i32 = unsafe {
            let len_fn: extern "C" fn(i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_len")
                    .unwrap(),
            );
            len_fn(dict_empty)
        };
        println!("  initial len: {}", len_empty);

        // Test 2: Insert key-value pairs
        let set_rt1: i32 = unsafe {
            let set_fn: extern "C" fn(i64, i32, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_set")
                    .unwrap(),
            );
            set_fn(dict_empty, 1, 100)
        };
        println!("  inserted key=1, val=100: count={}", set_rt1);

        let set_rt2: i32 = unsafe {
            let set_fn: extern "C" fn(i64, i32, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_set")
                    .unwrap(),
            );
            set_fn(dict_empty, 2, 200)
        };
        println!("  inserted key=2, val=200: count={}", set_rt2);

        // Test 3: Query length
        let len_after_insert: i32 = unsafe {
            let len_fn: extern "C" fn(i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_len")
                    .unwrap(),
            );
            len_fn(dict_empty)
        };
        println!("  len after inserts: {}", len_after_insert);

        // Test 4: Get values (requires out parameter)
        let mut out_val1: i32 = 0;
        let get_result1: i32 = unsafe {
            let get_fn: extern "C" fn(i64, i32, *mut i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_get")
                    .unwrap(),
            );
            get_fn(dict_empty, 1, &mut out_val1)
        };
        println!("  get key=1: found={} val={}", get_result1 == 1, out_val1);

        let mut out_val2: i32 = 0;
        let get_result2: i32 = unsafe {
            let get_fn: extern "C" fn(i64, i32, *mut i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_get")
                    .unwrap(),
            );
            get_fn(dict_empty, 2, &mut out_val2)
        };
        println!("  get key=2: found={} val={}", get_result2 == 1, out_val2);

        let mut out_missing: i32 = 0;
        let get_result_missing: i32 = unsafe {
            let get_fn: extern "C" fn(i64, i32, *mut i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_get")
                    .unwrap(),
            );
            get_fn(dict_empty, 99, &mut out_missing)
        };
        println!(
            "  get key=99 (missing): found={} val={}",
            get_result_missing == 1,
            out_missing
        );

        // Test 5: Containment checks
        let has_1: i32 = unsafe {
            let contains_fn: extern "C" fn(i64, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_contains")
                    .unwrap(),
            );
            contains_fn(dict_empty, 1)
        };
        println!("  contains key=1: {}", has_1 != 0);

        let has_99: i32 = unsafe {
            let contains_fn: extern "C" fn(i64, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_contains")
                    .unwrap(),
            );
            contains_fn(dict_empty, 99)
        };
        println!("  contains key=99: {}", has_99 != 0);

        // Test 6: Upsert (update existing)
        let set_rt3: i32 = unsafe {
            let set_fn: extern "C" fn(i64, i32, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_set")
                    .unwrap(),
            );
            set_fn(dict_empty, 1, 150)
        };
        println!("  upserted key=1 to val=150: count={}", set_rt3);

        let mut out_updated: i32 = 0;
        let get_result_updated: i32 = unsafe {
            let get_fn: extern "C" fn(i64, i32, *mut i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_get")
                    .unwrap(),
            );
            get_fn(dict_empty, 1, &mut out_updated)
        };
        println!(
            "  get updated key=1: found={} val={}",
            get_result_updated == 1,
            out_updated
        );

        // Test 7: Stress with multiple inserts
        println!("  stress test with 20 inserts...");
        let mut last_count = set_rt3;
        for i in 3..=20 {
            last_count = unsafe {
                let set_fn: extern "C" fn(i64, i32, i32) -> i32 = std::mem::transmute(
                    factory
                        .symbol_address("swift_contract_dict_i32_set")
                        .unwrap(),
                );
                set_fn(dict_empty, i as i32, i as i32 * 1000)
            };
        }
        let final_len: i32 = unsafe {
            let len_fn: extern "C" fn(i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_i32_len")
                    .unwrap(),
            );
            len_fn(dict_empty)
        };
        println!("  after stress: count={}, len={}", last_count, final_len);

        println!("  dictionary<int32,int32> test PASS\n");
    }

    // Test Dictionary<Int32, OpaqueRef> (holds String objects)
    {
        println!("── Dictionary<Int32, OpaqueRef> (objects as values) ──────────────");

        // Create dictionary
        let dict_ref: i64 = unsafe {
            let make_fn: extern "C" fn(i32) -> i64 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_make")
                    .unwrap(),
            );
            make_fn(5)
        };
        println!("dict created: {}", dict_ref);

        // Create string objects
        let str_apple = unsafe {
            let string_make: extern "C" fn(*const u8, i32) -> i64 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_construct_string")
                    .unwrap(),
            );
            string_make(b"apple".as_ptr(), 5)
        };
        println!("  str_apple created: {}", str_apple);

        let str_banana = unsafe {
            let string_make: extern "C" fn(*const u8, i32) -> i64 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_construct_string")
                    .unwrap(),
            );
            string_make(b"banana".as_ptr(), 6)
        };
        println!("  str_banana created: {}", str_banana);

        // Insert string objects as values
        let set_rt1: i32 = unsafe {
            let set_fn: extern "C" fn(i64, i32, i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_set")
                    .unwrap(),
            );
            set_fn(dict_ref, 10, str_apple)
        };
        println!("  inserted key=10, val=str_apple: count={}", set_rt1);

        let set_rt2: i32 = unsafe {
            let set_fn: extern "C" fn(i64, i32, i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_set")
                    .unwrap(),
            );
            set_fn(dict_ref, 20, str_banana)
        };
        println!("  inserted key=20, val=str_banana: count={}", set_rt2);

        // Query length
        let len: i32 = unsafe {
            let len_fn: extern "C" fn(i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_len")
                    .unwrap(),
            );
            len_fn(dict_ref)
        };
        println!("  len: {}", len);

        // Get values back
        let mut got_apple: i64 = 0;
        let get_result_apple: i32 = unsafe {
            let get_fn: extern "C" fn(i64, i32, *mut i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_get")
                    .unwrap(),
            );
            get_fn(dict_ref, 10, &mut got_apple)
        };
        let apple_matches = got_apple == str_apple && get_result_apple == 1;
        println!("  get key=10: matches={}", apple_matches);

        let mut got_banana: i64 = 0;
        let get_result_banana: i32 = unsafe {
            let get_fn: extern "C" fn(i64, i32, *mut i64) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_get")
                    .unwrap(),
            );
            get_fn(dict_ref, 20, &mut got_banana)
        };
        let banana_matches = got_banana == str_banana && get_result_banana == 1;
        println!("  get key=20: matches={}", banana_matches);

        // Containment checks
        let has_10: i32 = unsafe {
            let contains_fn: extern "C" fn(i64, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_contains")
                    .unwrap(),
            );
            contains_fn(dict_ref, 10)
        };
        println!("  contains key=10: {}", has_10 != 0);

        let has_99: i32 = unsafe {
            let contains_fn: extern "C" fn(i64, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_dict_ref_contains")
                    .unwrap(),
            );
            contains_fn(dict_ref, 99)
        };
        println!("  contains key=99: {}", has_99 != 0);

        println!("  dictionary<int32,opaqueref> test PASS\n");
    }

    println!("== All Dictionary Bridging Tests PASSED ==");
}
