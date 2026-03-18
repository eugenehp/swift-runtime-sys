/// Property wrapper metadata probe for Track J.2.
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

    println!("\n=== Property Wrapper Metadata (Track J.2) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 6] = [
        ("Init clamp below range", test_init_clamp_low),
        ("Init clamp above range", test_init_clamp_high),
        ("Set clamp below range", test_set_clamp_low),
        ("Set clamp above range", test_set_clamp_high),
        ("Set in range unchanged", test_set_in_range),
        ("Projected value tracks wrapped", test_projected),
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

    println!("\n=== Track J.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track J.2 tests PASSED");
    } else {
        panic!("✗ Track J.2 tests FAILED");
    }
}

fn test_init_clamp_low(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.wrapper_init_clamped(-10)? == 0)
}

fn test_init_clamp_high(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.wrapper_init_clamped(150)? == 100)
}

fn test_set_clamp_low(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.wrapper_set_clamped(50, -1)? == 0)
}

fn test_set_clamp_high(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.wrapper_set_clamped(50, 101)? == 100)
}

fn test_set_in_range(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.wrapper_set_clamped(0, 77)? == 77)
}

fn test_projected(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.wrapper_projected_value(88)? == 88)
}
