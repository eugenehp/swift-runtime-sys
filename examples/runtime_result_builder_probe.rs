/// Result builder DSL probe for Track J.4.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Result Builder DSL Support (Track J.4) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 6] = [
        ("Builder sum2(3,4)=7", test_sum2_basic),
        ("Builder sum2 negative", test_sum2_negative),
        ("Builder conditional true=10", test_conditional_true),
        ("Builder conditional false=20", test_conditional_false),
        ("Builder loop sum n=5 => 15", test_loop_sum_5),
        ("Builder loop sum n=0 => 0", test_loop_sum_0),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(err) => {
                println!("✗ {name} FAIL ({err:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== Track J.4 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track J.4 tests PASSED");
    } else {
        panic!("✗ Track J.4 tests FAILED");
    }
}

fn test_sum2_basic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.builder_sum2(3, 4)? == 7)
}

fn test_sum2_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.builder_sum2(-2, 5)? == 3)
}

fn test_conditional_true(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.builder_conditional(1)? == 10)
}

fn test_conditional_false(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.builder_conditional(0)? == 20)
}

fn test_loop_sum_5(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 1 + 2 + 3 + 4 + 5 = 15
    Ok(contract.builder_loop_sum(5)? == 15)
}

fn test_loop_sum_0(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.builder_loop_sum(0)? == 0)
}
