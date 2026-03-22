/// Phase A.2 Array<T> Bridging Probe
/// Tests Array<Int32> and Array<OpaqueRef> construction, access, mutation, and bounds safety.
use std::ffi::CString;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

type ArrayInt32Ptr = *mut std::ffi::c_void;
type ArrayOpaqueRefPtr = *mut std::ffi::c_void;

struct TestResult {
    name: &'static str,
    passed: bool,
    details: Option<String>,
}

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .unwrap_or_else(|e| panic!("factory init failed: {e:?}"));

    let mut results = Vec::new();
    let mut passed = 0;
    let mut total = 0;

    // Array<Int32> Tests

    results.push(run_test_make_empty_int32(&factory, &mut total, &mut passed));
    results.push(run_test_make_with_capacity(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_make_negative_capacity(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_int32_len(&factory, &mut total, &mut passed));
    results.push(run_test_int32_get_bounds(&factory, &mut total, &mut passed));
    results.push(run_test_int32_set_element(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_int32_set_bounds(&factory, &mut total, &mut passed));
    results.push(run_test_int32_append(&factory, &mut total, &mut passed));
    results.push(run_test_int32_append_growth(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_int32_data_ptr(&factory, &mut total, &mut passed));
    results.push(run_test_int32_round_trip(&factory, &mut total, &mut passed));
    results.push(run_test_int32_release(&factory, &mut total, &mut passed));

    // Array<OpaqueRef> Tests

    results.push(run_test_make_opaque_ref(&factory, &mut total, &mut passed));
    results.push(run_test_opaque_ref_len(&factory, &mut total, &mut passed));
    results.push(run_test_opaque_ref_get_set(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_opaque_ref_append(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_opaque_ref_nil_handling(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_opaque_ref_bounds(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_opaque_ref_release(
        &factory,
        &mut total,
        &mut passed,
    ));
    results.push(run_test_mixed_operations(&factory, &mut total, &mut passed));

    println!("\n=== Array Bridging Probe Results ===");
    for result in &results {
        let status = if result.passed { "PASS" } else { "FAIL" };
        println!("{}: {}", result.name, status);
        if let Some(detail) = &result.details {
            println!("  {}", detail);
        }
    }

    println!("\n=== Summary ===");
    println!("Passed: {}/{}", passed, total);

    if passed == total {
        println!("semantic=PASS");
        std::process::exit(0);
    } else {
        println!("semantic=FAIL");
        std::process::exit(1);
    }
}

fn run_test_make_empty_int32(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(0)
    };

    if !ptr.is_null() {
        *passed += 1;
        TestResult {
            name: "array_int32_make_empty",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_make_empty",
            passed: false,
            details: Some("Failed to create empty array".to_string()),
        }
    }
}

fn run_test_make_with_capacity(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(10)
    };

    if !ptr.is_null() {
        let len = unsafe {
            let len_fn: extern "C" fn(*mut std::ffi::c_void) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_len")
                    .unwrap(),
            );
            len_fn(ptr)
        };

        unsafe {
            let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_release")
                    .unwrap(),
            );
            release_fn(ptr);
        }

        if len == 10 {
            *passed += 1;
            TestResult {
                name: "array_int32_make_with_capacity",
                passed: true,
                details: None,
            }
        } else {
            TestResult {
                name: "array_int32_make_with_capacity",
                passed: false,
                details: Some(format!("Expected len=10, got {}", len)),
            }
        }
    } else {
        TestResult {
            name: "array_int32_make_with_capacity",
            passed: false,
            details: Some("Failed to create array with capacity".to_string()),
        }
    }
}

fn run_test_make_negative_capacity(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(-1)
    };

    if ptr.is_null() {
        *passed += 1;
        TestResult {
            name: "array_int32_make_negative_capacity",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_make_negative_capacity",
            passed: false,
            details: Some("Negative capacity should return nil".to_string()),
        }
    }
}

fn run_test_int32_len(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(5)
    };

    let len = unsafe {
        let len_fn: extern "C" fn(*mut std::ffi::c_void) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_len")
                .unwrap(),
        );
        len_fn(ptr)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if len == 5 {
        *passed += 1;
        TestResult {
            name: "array_int32_len",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_len",
            passed: false,
            details: Some(format!("Expected len=5, got {}", len)),
        }
    }
}

fn run_test_int32_get_bounds(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(3)
    };

    let val_neg = unsafe {
        let get_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_get")
                .unwrap(),
        );
        get_fn(ptr, -1)
    };

    let val_oob = unsafe {
        let get_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_get")
                .unwrap(),
        );
        get_fn(ptr, 10)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if val_neg == 0 && val_oob == 0 {
        *passed += 1;
        TestResult {
            name: "array_int32_get_bounds",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_get_bounds",
            passed: false,
            details: Some(format!(
                "Out-of-bounds get should return 0, got {} and {}",
                val_neg, val_oob
            )),
        }
    }
}

fn run_test_int32_set_element(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(3)
    };

    let result = unsafe {
        let set_fn: extern "C" fn(*mut std::ffi::c_void, i32, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_set")
                .unwrap(),
        );
        set_fn(ptr, 1, 42)
    };

    let val = unsafe {
        let get_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_get")
                .unwrap(),
        );
        get_fn(ptr, 1)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if result == 0 && val == 42 {
        *passed += 1;
        TestResult {
            name: "array_int32_set_element",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_set_element",
            passed: false,
            details: Some(format!(
                "Set failed or value mismatch: result={}, val={}",
                result, val
            )),
        }
    }
}

fn run_test_int32_set_bounds(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(3)
    };

    let result_neg = unsafe {
        let set_fn: extern "C" fn(*mut std::ffi::c_void, i32, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_set")
                .unwrap(),
        );
        set_fn(ptr, -1, 99)
    };

    let result_oob = unsafe {
        let set_fn: extern "C" fn(*mut std::ffi::c_void, i32, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_set")
                .unwrap(),
        );
        set_fn(ptr, 10, 99)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if result_neg == -1 && result_oob == -1 {
        *passed += 1;
        TestResult {
            name: "array_int32_set_bounds",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_set_bounds",
            passed: false,
            details: Some(format!(
                "Out-of-bounds set should return -1, got {} and {}",
                result_neg, result_oob
            )),
        }
    }
}

fn run_test_int32_append(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(0)
    };

    let len1 = unsafe {
        let append_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_append")
                .unwrap(),
        );
        append_fn(ptr, 10)
    };

    let len2 = unsafe {
        let append_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_append")
                .unwrap(),
        );
        append_fn(ptr, 20)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if len1 == 1 && len2 == 2 {
        *passed += 1;
        TestResult {
            name: "array_int32_append",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_append",
            passed: false,
            details: Some(format!("Append failed: len1={}, len2={}", len1, len2)),
        }
    }
}

fn run_test_int32_append_growth(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(0)
    };

    let mut last_len = 0i32;
    for i in 0..20 {
        last_len = unsafe {
            let append_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_append")
                    .unwrap(),
            );
            append_fn(ptr, i)
        };
    }

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if last_len == 20 {
        *passed += 1;
        TestResult {
            name: "array_int32_append_growth",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_append_growth",
            passed: false,
            details: Some(format!(
                "Expected len=20 after 20 appends, got {}",
                last_len
            )),
        }
    }
}

fn run_test_int32_data_ptr(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(3)
    };

    // Set a few values
    unsafe {
        let set_fn: extern "C" fn(*mut std::ffi::c_void, i32, i32) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_set")
                .unwrap(),
        );
        set_fn(ptr, 0, 100);
        set_fn(ptr, 1, 200);
        set_fn(ptr, 2, 300);
    }

    let data_ptr = unsafe {
        let data_fn: extern "C" fn(*mut std::ffi::c_void) -> *const i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_data")
                .unwrap(),
        );
        data_fn(ptr)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if !data_ptr.is_null() {
        *passed += 1;
        TestResult {
            name: "array_int32_data_ptr",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_data_ptr",
            passed: false,
            details: Some("data_ptr returned null for non-empty array".to_string()),
        }
    }
}

fn run_test_int32_round_trip(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(0)
    };

    let values = vec![10, 20, 30, 40, 50];
    for val in &values {
        unsafe {
            let append_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_append")
                    .unwrap(),
            );
            append_fn(ptr, *val);
        }
    }

    let mut match_all = true;
    for (i, expected) in values.iter().enumerate() {
        let val = unsafe {
            let get_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> i32 = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_get")
                    .unwrap(),
            );
            get_fn(ptr, i as i32)
        };
        if val != *expected {
            match_all = false;
            break;
        }
    }

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if match_all {
        *passed += 1;
        TestResult {
            name: "array_int32_round_trip",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_int32_round_trip",
            passed: false,
            details: Some("Round-trip values do not match".to_string()),
        }
    }
}

fn run_test_int32_release(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_make")
                .unwrap(),
        );
        make_fn(1)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_int32_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    *passed += 1;
    TestResult {
        name: "array_int32_release",
        passed: true,
        details: None,
    }
}

fn run_test_make_opaque_ref(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(5)
    };

    if !ptr.is_null() {
        unsafe {
            let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_release")
                    .unwrap(),
            );
            release_fn(ptr);
        }
        *passed += 1;
        TestResult {
            name: "array_opaque_ref_make",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_opaque_ref_make",
            passed: false,
            details: Some("Failed to create Array<OpaqueRef>".to_string()),
        }
    }
}

fn run_test_opaque_ref_len(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(3)
    };

    let len = unsafe {
        let len_fn: extern "C" fn(*mut std::ffi::c_void) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_len")
                .unwrap(),
        );
        len_fn(ptr)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if len == 3 {
        *passed += 1;
        TestResult {
            name: "array_opaque_ref_len",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_opaque_ref_len",
            passed: false,
            details: Some(format!("Expected len=3, got {}", len)),
        }
    }
}

fn run_test_opaque_ref_get_set(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(3)
    };

    let test_ptr = 0xdeadbeefusize as *mut std::ffi::c_void;

    let result = unsafe {
        let set_fn: extern "C" fn(*mut std::ffi::c_void, i32, *mut std::ffi::c_void) -> i32 =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_set")
                    .unwrap(),
            );
        set_fn(ptr, 1, test_ptr)
    };

    let val = unsafe {
        let get_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> *mut std::ffi::c_void =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_get")
                    .unwrap(),
            );
        get_fn(ptr, 1)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if result == 0 && val == test_ptr {
        *passed += 1;
        TestResult {
            name: "array_opaque_ref_get_set",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_opaque_ref_get_set",
            passed: false,
            details: Some(format!(
                "Set/get mismatch: result={}, match={}",
                result,
                val == test_ptr
            )),
        }
    }
}

fn run_test_opaque_ref_append(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(0)
    };

    let len1 = unsafe {
        let append_fn: extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32 =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_append")
                    .unwrap(),
            );
        append_fn(ptr, (0x1000usize as *mut std::ffi::c_void))
    };

    let len2 = unsafe {
        let append_fn: extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32 =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_append")
                    .unwrap(),
            );
        append_fn(ptr, (0x2000usize as *mut std::ffi::c_void))
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if len1 == 1 && len2 == 2 {
        *passed += 1;
        TestResult {
            name: "array_opaque_ref_append",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_opaque_ref_append",
            passed: false,
            details: Some(format!("Append failed: len1={}, len2={}", len1, len2)),
        }
    }
}

fn run_test_opaque_ref_nil_handling(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(0)
    };

    let len1 = unsafe {
        let append_fn: extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32 =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_append")
                    .unwrap(),
            );
        append_fn(ptr, std::ptr::null_mut())
    };

    let val = unsafe {
        let get_fn: extern "C" fn(*mut std::ffi::c_void, i32) -> *mut std::ffi::c_void =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_get")
                    .unwrap(),
            );
        get_fn(ptr, 0)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if len1 == 1 && val.is_null() {
        *passed += 1;
        TestResult {
            name: "array_opaque_ref_nil_handling",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_opaque_ref_nil_handling",
            passed: false,
            details: Some(format!(
                "Nil handling failed: len={}, is_null={}",
                len1,
                val.is_null()
            )),
        }
    }
}

fn run_test_opaque_ref_bounds(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(2)
    };

    let result = unsafe {
        let set_fn: extern "C" fn(*mut std::ffi::c_void, i32, *mut std::ffi::c_void) -> i32 =
            std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_opaque_ref_set")
                    .unwrap(),
            );
        set_fn(ptr, 10, std::ptr::null_mut())
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    if result == -1 {
        *passed += 1;
        TestResult {
            name: "array_opaque_ref_bounds",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_opaque_ref_bounds",
            passed: false,
            details: Some(format!("Expected -1 for out-of-bounds, got {}", result)),
        }
    }
}

fn run_test_opaque_ref_release(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;
    let ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(1)
    };

    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(ptr);
    }

    *passed += 1;
    TestResult {
        name: "array_opaque_ref_release",
        passed: true,
        details: None,
    }
}

fn run_test_mixed_operations(
    factory: &RuntimeFactory,
    total: &mut usize,
    passed: &mut usize,
) -> TestResult {
    *total += 1;

    // Create an array of opaque refs
    let arr_ptr = unsafe {
        let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_make")
                .unwrap(),
        );
        make_fn(0)
    };

    // Create a few int32 arrays to store in it
    let int_ptrs: Vec<*mut std::ffi::c_void> = (0..3)
        .map(|_| unsafe {
            let make_fn: extern "C" fn(i32) -> *mut std::ffi::c_void = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_make")
                    .unwrap(),
            );
            make_fn(5)
        })
        .collect();

    // Append int array pointers to the opaque ref array
    for int_ptr in &int_ptrs {
        unsafe {
            let append_fn: extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32 =
                std::mem::transmute(
                    factory
                        .symbol_address("swift_contract_array_opaque_ref_append")
                        .unwrap(),
                );
            append_fn(arr_ptr, *int_ptr);
        }
    }

    // Verify length
    let len = unsafe {
        let len_fn: extern "C" fn(*mut std::ffi::c_void) -> i32 = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_len")
                .unwrap(),
        );
        len_fn(arr_ptr)
    };

    // Cleanup
    unsafe {
        let release_fn: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
            factory
                .symbol_address("swift_contract_array_opaque_ref_release")
                .unwrap(),
        );
        release_fn(arr_ptr);
        for int_ptr in int_ptrs {
            let int_release: extern "C" fn(*mut std::ffi::c_void) -> () = std::mem::transmute(
                factory
                    .symbol_address("swift_contract_array_int32_release")
                    .unwrap(),
            );
            int_release(int_ptr);
        }
    }

    if len == 3 {
        *passed += 1;
        TestResult {
            name: "array_mixed_operations",
            passed: true,
            details: None,
        }
    } else {
        TestResult {
            name: "array_mixed_operations",
            passed: false,
            details: Some(format!(
                "Mixed operations failed: expected len=3, got {}",
                len
            )),
        }
    }
}
