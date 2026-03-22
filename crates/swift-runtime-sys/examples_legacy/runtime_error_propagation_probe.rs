/// Phase A.4 Error/Throws Bridging Probe
/// Tests error creation, propagation, catching, and safe unwinding across the bridge.
/// Covers: ValidationError, IOError, OutOfRangeError variants with throw/catch cycles.

use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    println!("== Error Propagation & Bridging Probe ==\n");
    
    let mut passed = 0;
    let mut total = 0;

    // ─── Test 1: ValidationError Creation ───────────────────────────────────
    {
        println!("── Test 1: ValidationError Creation ────────────────────────");
        total += 1;
        
        let make_validation: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_validation").unwrap())
        };
        
        let code: i32 = 42;
        let result = make_validation(code);
        
        println!("  Created ValidationError with code={}: result={}", code, result);
        if result == 1 {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: Expected success (1), got {}", result);
        }
    }

    // ─── Test 2: Get Error Description ─────────────────────────────────────
    {
        println!("\n── Test 2: Error Description Extraction ────────────────────");
        total += 1;
        
        let make_validation: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_validation").unwrap())
        };
        let get_description: extern "C" fn() -> *const i8 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_get_description").unwrap())
        };
        
        let _ = make_validation(99);
        let desc_ptr = get_description();
        
        let desc = if !desc_ptr.is_null() {
            unsafe { std::ffi::CStr::from_ptr(desc_ptr).to_string_lossy().to_string() }
        } else {
            "null".to_string()
        };
        
        println!("  Error description: {:?}", desc);
        if !desc_ptr.is_null() && desc.len() > 0 {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: Got empty/null description");
        }
    }

    // ─── Test 3: Get Error Code ────────────────────────────────────────────
    {
        println!("\n── Test 3: Error Code Extraction ──────────────────────────");
        total += 1;
        
        let make_validation: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_validation").unwrap())
        };
        let get_code: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_get_code").unwrap())
        };
        
        let code_in = 123;
        let _ = make_validation(code_in);
        let code_out = get_code();
        
        println!("  Input code: {}, retrieved code: {}", code_in, code_out);
        if code_out == code_in {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: Code mismatch");
        }
    }

    // ─── Test 4: Error Type Identity (ValidationError) ─────────────────────
    {
        println!("\n── Test 4: Error Type Identity (ValidationError) ──────────");
        total += 1;
        
        let make_validation: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_validation").unwrap())
        };
        let is_validation: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_is_validation").unwrap())
        };
        let is_io: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_is_io").unwrap())
        };
        
        let _ = make_validation(50);
        let is_val = is_validation() != 0;
        let is_io_err = is_io() != 0;
        
        println!("  Is ValidationError: {}, Is IOError: {}", is_val, is_io_err);
        if is_val && !is_io_err {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: Type identity mismatch");
        }
    }

    // ─── Test 5: IOError Creation and Identity ──────────────────────────────
    {
        println!("\n── Test 5: IOError Creation and Identity ────────────────────");
        total += 1;
        
        let make_io: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_io").unwrap())
        };
        let is_io: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_is_io").unwrap())
        };
        let is_validation: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_is_validation").unwrap())
        };
        
        let _ = make_io(77);
        let is_io_err = is_io() != 0;
        let is_val = is_validation() != 0;
        
        println!("  Is IOError: {}, Is ValidationError: {}", is_io_err, is_val);
        if is_io_err && !is_val {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL");
        }
    }

    // ─── Test 6: OutOfRangeError ────────────────────────────────────────────
    {
        println!("\n── Test 6: OutOfRangeError Creation ────────────────────────");
        total += 1;
        
        let make_oor: extern "C" fn(i32, i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_out_of_range").unwrap())
        };
        let get_code: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_get_code").unwrap())
        };
        
        let code = 55;
        let limit = 100;
        let _ = make_oor(code, limit);
        let retrieved = get_code();
        
        println!("  OutOfRangeError(code={}, limit={}): code_out={}", code, limit, retrieved);
        if retrieved == code {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL");
        }
    }

    // ─── Test 7: Error Clearing ────────────────────────────────────────────
    {
        println!("\n── Test 7: Error Clearing ─────────────────────────────────");
        total += 1;
        
        let make_validation: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_validation").unwrap())
        };
        let clear_error: extern "C" fn() = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_clear").unwrap())
        };
        let is_validation: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_is_validation").unwrap())
        };
        
        let _ = make_validation(66);
        println!("  Before clear - is_validation: {}", is_validation() != 0);
        
        clear_error();
        let after_clear = is_validation();
        println!("  After clear - is_validation: {}", after_clear != 0);
        
        if after_clear == 0 {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: Error not cleared");
        }
    }

    // ─── Test 8: Error Context (Validation) ────────────────────────────────
    {
        println!("\n── Test 8: Error Context with Cause ────────────────────────");
        total += 1;
        
        let make_context_val: extern "C" fn(i32, i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_context_make_validation").unwrap())
        };
        let get_code: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_get_code").unwrap())
        };
        
        let code = 88;
        let cause_code = 44;
        let _ = make_context_val(code, cause_code);
        let retrieved = get_code();
        
        println!("  ContextError(code={}, cause={}): code_out={}", code, cause_code, retrieved);
        if retrieved == code {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL");
        }
    }

    // ─── Test 9: Error Context (IO) ────────────────────────────────────────
    {
        println!("\n── Test 9: Error Context IO ───────────────────────────────");
        total += 1;
        
        let make_context_io: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_context_make_io").unwrap())
        };
        let is_io: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_is_io").unwrap())
        };
        
        let code = 11;
        let _ = make_context_io(code);
        let is_io_err = is_io() != 0;
        
        println!("  IOContext(code={}): is_io={}", code, is_io_err);
        if is_io_err {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL");
        }
    }

    // ─── Test 10: Sequential Error Creation (Round-trip) ──────────────────
    {
        println!("\n── Test 10: Sequential Error Creation & Clearing ──────────");
        total += 1;
        
        let make_validation: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_validation").unwrap())
        };
        let make_io: extern "C" fn(i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_make_io").unwrap())
        };
        let clear_error: extern "C" fn() = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_clear").unwrap())
        };
        let get_code: extern "C" fn() -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_get_code").unwrap())
        };
        
        // Create validation, check, clear
        let _ = make_validation(111);
        let code1 = get_code();
        clear_error();
        
        // Create IO, check, clear
        let _ = make_io(222);
        let code2 = get_code();
        clear_error();
        
        // Create validation again
        let _ = make_validation(333);
        let code3 = get_code();
        
        println!("  Sequence: val(111)={}, io(222)={}, val(333)={}", code1, code2, code3);
        if code1 == 111 && code2 == 222 && code3 == 333 {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: Codes don't match sequence");
        }
    }

    // ─── Test 11: Error Description JSON (Context) ────────────────────────
    {
        println!("\n── Test 11: Error Context JSON Description ────────────────");
        total += 1;
        
        let make_context_val: extern "C" fn(i32, i32) -> i32 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_context_make_validation").unwrap())
        };
        let get_json: extern "C" fn() -> *const i8 = unsafe {
            std::mem::transmute(factory.symbol_address("swift_contract_error_context_get_json").unwrap())
        };
        
        let _ = make_context_val(77, 88);
        let json_ptr = get_json();
        
        let json_str = if !json_ptr.is_null() {
            unsafe { std::ffi::CStr::from_ptr(json_ptr).to_string_lossy().to_string() }
        } else {
            "null".to_string()
        };
        
        println!("  Context JSON: {}", json_str);
        if !json_ptr.is_null() && json_str.len() > 0 && json_str.contains("77") {
            println!("  ✓ PASS");
            passed += 1;
        } else {
            println!("  ✗ FAIL: No valid JSON with code");
        }
    }

    // ─── Summary ────────────────────────────────────────────────────────────
    println!("\n=== Error Propagation Probe Results ===");
    println!("Passed: {}/{}", passed, total);
    
    if passed == total {
        println!("\n✓ All error propagation tests PASSED\n");
        std::process::exit(0);
    } else {
        println!("\n✗ Some tests FAILED\n");
        std::process::exit(1);
    }
}
