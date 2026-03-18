/// Reference cycle and weak/unowned tracking probe for Track K.1.
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

    println!("\n=== Weak/Unowned & Cycle Tracking (Track K.1) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 5] = [
        ("Weak lifecycle clears after drop", test_weak_lifecycle),
        ("Unowned dangling is detected", test_unowned_detected),
        ("Strong pair cycle is detected", test_cycle_strong_pair),
        ("Acyclic pair deallocates", test_cycle_acyclic_pair),
        (
            "Cycle and acyclic behavior differ",
            test_cycle_differentiation,
        ),
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

    println!("\n=== Track K.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track K.1 tests PASSED");
    } else {
        panic!("✗ Track K.1 tests FAILED");
    }
}

fn test_weak_lifecycle(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k1_weak_lifecycle()
}

fn test_unowned_detected(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k1_unowned_dangling_detected()
}

fn test_cycle_strong_pair(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k1_cycle_detect_strong_pair()
}

fn test_cycle_acyclic_pair(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k1_cycle_detect_acyclic_pair()
}

fn test_cycle_differentiation(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let strong = contract.k1_cycle_detect_strong_pair()?;
    let acyclic = contract.k1_cycle_detect_acyclic_pair()?;
    Ok(strong && acyclic)
}
