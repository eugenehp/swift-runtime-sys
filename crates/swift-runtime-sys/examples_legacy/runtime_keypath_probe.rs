/// KeyPath runtime probe for Track J.1.
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

    println!("\n=== KeyPath Runtime Support (Track J.1) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 5] = [
        ("Typed key path age reads value", test_keypath_age),
        ("Typed key path age negative value", test_keypath_age_negative),
        ("Composed key path nested score", test_keypath_nested_score),
        ("Composed key path nested score zero", test_keypath_nested_score_zero),
        ("AnyKeyPath match check", test_keypath_any_matches),
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

    println!("\n=== Track J.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track J.1 tests PASSED");
    } else {
        panic!("✗ Track J.1 tests FAILED");
    }
}

fn test_keypath_age(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.keypath_get_age(42)? == 42)
}

fn test_keypath_age_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.keypath_get_age(-7)? == -7)
}

fn test_keypath_nested_score(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.keypath_get_nested_score(99)? == 99)
}

fn test_keypath_nested_score_zero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.keypath_get_nested_score(0)? == 0)
}

fn test_keypath_any_matches(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.keypath_any_matches()
}
