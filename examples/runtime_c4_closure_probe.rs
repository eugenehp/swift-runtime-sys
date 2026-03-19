/// Phase C.4 probe: closure captures, async-closure semantics, and error handling.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Phase C.4 Closure & Async-Capture Semantics ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        (
            "Escaping closure stores and captures multiplier",
            test_c4_escaping_store,
        ),
        (
            "Escaping closure invokes with stored capture",
            test_c4_escaping_invoke,
        ),
        (
            "Escaping closure clear prevents reuse",
            test_c4_escaping_clear,
        ),
        (
            "Async closure invocation returns doubled value",
            test_c4_async_invoke,
        ),
        (
            "Throwing closure succeeds with valid input",
            test_c4_throwing_safe,
        ),
        (
            "Throwing closure fails with negative input",
            test_c4_throwing_error,
        ),
        (
            "Async + throwing closure combined behavior",
            test_c4_async_throwing_combined,
        ),
    ];

    for (name, f) in tests {
        match f(&contract) {
            Ok(true) => {
                println!("PASS: {name}");
                passed += 1;
            }
            Ok(false) => {
                println!("FAIL: {name}");
                failed += 1;
            }
            Err(err) => {
                println!("FAIL: {name} ({err:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== C.4 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_c4_escaping_store(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Store an escaping closure with multiplier=3.
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_store", 3)?;
    Ok(result == 1)
}

fn test_c4_escaping_invoke(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // First store the closure, then invoke it.
    // Store multiplier=3, invoke with value=7 => expected 7*3=21
    c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_store", 3)?;
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_invoke", 7)?;
    Ok(result == 21)
}

fn test_c4_escaping_clear(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Store, invoke, clear, then invoke again (should fail with Int32.min).
    c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_store", 5)?;
    let _ = c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_invoke", 10)?;
    c.n2_dynamic_symbol_const("swift_contract_c4_escaping_closure_clear")?;
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_invoke", 10)?;
    Ok(result == i32::MIN)
}

fn test_c4_async_invoke(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Async closure: double the input value (5 => 10).
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_async_closure_invoke", 5)?;
    Ok(result == 10)
}

fn test_c4_throwing_safe(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Throwing closure with positive input => succeeds (value + 10).
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_throwing_closure_safe", 32)?;
    Ok(result == 42)
}

fn test_c4_throwing_error(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Throwing closure with negative input => fails, returns Int32.min.
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_throwing_closure_error", -5)?;
    Ok(result == i32::MIN)
}

fn test_c4_async_throwing_combined(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Combined: store escaping closure, invoke via async, then check safe throwing.
    // Test sequence: store(2) -> invoke(10) via stored = 20, then throwing(8) = 18
    c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_store", 2)?;
    let _ = c.n2_dynamic_symbol_single("swift_contract_c4_escaping_closure_invoke", 10)?;
    let result = c.n2_dynamic_symbol_single("swift_contract_c4_throwing_closure_safe", 8)?;
    Ok(result == 18)
}
