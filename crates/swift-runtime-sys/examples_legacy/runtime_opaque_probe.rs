/// Opaque type bridging probe for Track J.3.
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

    println!("\n=== Opaque Type Bridging (Track J.3) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 6] = [
        ("Opaque even tag name is 'even'", test_even_name),
        ("Opaque odd tag name is 'odd'", test_odd_name),
        ("Opaque even name length", test_even_len),
        ("Opaque odd name length", test_odd_len),
        ("Opaque negative odd tag", test_negative_odd),
        ("Opaque even/odd differ", test_even_odd_differ),
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

    println!("\n=== Track J.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track J.3 tests PASSED");
    } else {
        panic!("✗ Track J.3 tests FAILED");
    }
}

fn test_even_name(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.opaque_named_get_name(2)? == "even")
}

fn test_odd_name(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.opaque_named_get_name(3)? == "odd")
}

fn test_even_len(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.opaque_named_name_len(10)? == 4)
}

fn test_odd_len(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.opaque_named_name_len(11)? == 3)
}

fn test_negative_odd(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.opaque_named_get_name(-1)? == "odd")
}

fn test_even_odd_differ(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let even = contract.opaque_named_get_name(100)?;
    let odd = contract.opaque_named_get_name(101)?;
    Ok(even != odd)
}
