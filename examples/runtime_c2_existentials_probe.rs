/// Phase C.2 probe: advanced existentials, protocol composition, and generic constraints.
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

    println!("\n=== Phase C.2 Advanced Existentials & Generic Constraints ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 6] = [
        (
            "Protocol composition sum dispatches correctly",
            test_c2_composition_sum,
        ),
        (
            "Validatable protocol checks positive values",
            test_c2_validation_positive,
        ),
        (
            "Validatable protocol rejects negative values",
            test_c2_validation_negative,
        ),
        (
            "Conditional Array conformance accepts all-positive",
            test_c2_conditional_all_positive,
        ),
        (
            "Conditional Array conformance rejects any-negative",
            test_c2_conditional_with_negative,
        ),
        (
            "Generic constraint multi-bound (Comparable & Hashable)",
            test_c2_multi_bound,
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

    println!("\n=== C.2 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_c2_composition_sum(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Protocol composition: sum via composed Summable & Validatable protocol.
    let result = c.n2_dynamic_symbol_i32(
        "swift_contract_c2_protocol_composition_sum",
        10,
        20,
    )?;
    Ok(result == 30)
}

fn test_c2_validation_positive(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Validatable: positive values pass validation check.
    let result = c.n2_dynamic_symbol_single("swift_contract_c2_validation_check", 42)?;
    Ok(result == 1)
}

fn test_c2_validation_negative(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Validatable: negative values fail validation check.
    let result = c.n2_dynamic_symbol_single("swift_contract_c2_validation_check", -5)?;
    Ok(result == 0)
}

fn test_c2_conditional_all_positive(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Conditional Array conformance: [val1, val2, 1] all positive elements pass.
    let result = c.n2_dynamic_symbol_i32(
        "swift_contract_c2_conditional_collection",
        5,
        10,
    )?;
    Ok(result == 1)
}

fn test_c2_conditional_with_negative(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Conditional Array conformance: [val1, val2, 1] fails if val2 is negative.
    let result = c.n2_dynamic_symbol_i32(
        "swift_contract_c2_conditional_collection",
        5,
        -3,
    )?;
    Ok(result == 0)
}

fn test_c2_multi_bound(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Multi-constraint: Comparable & Hashable.
    let result = c.n2_dynamic_symbol_i32(
        "swift_contract_constrained_multi_min",
        50,
        30,
    )?;
    Ok(result == 30)
}
