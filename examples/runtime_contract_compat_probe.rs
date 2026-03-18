/// Cross-version compatibility probe for Track L.2.
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

    println!("\n=== Cross-Version Binary Compatibility (Track L.2) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        ("Diff count no breaking when new >= old", test_diff_no_breaking),
        ("Diff count reports removals", test_diff_breaking_count),
        ("Binary compat same major forward minor", test_binary_compat_forward_minor),
        ("Binary compat same major backward minor fails", test_binary_compat_backward_minor),
        ("Binary compat higher runtime major", test_binary_compat_higher_major),
        ("Marker resilient_layout bit", test_marker_resilient_layout),
        ("Marker private_fields bit", test_marker_private_fields),
        ("Unknown marker returns 0", test_marker_unknown),
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

    println!("\n=== Track L.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track L.2 tests PASSED");
    } else {
        panic!("✗ Track L.2 tests FAILED");
    }
}

fn test_diff_no_breaking(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.l2_contract_diff_breaking_count(3, 5)? == 0)
}

fn test_diff_breaking_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.l2_contract_diff_breaking_count(7, 4)? == 3)
}

fn test_binary_compat_forward_minor(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l2_binary_version_compatible(2005, 2001)
}

fn test_binary_compat_backward_minor(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let compat = contract.l2_binary_version_compatible(2000, 2001)?;
    Ok(!compat)
}

fn test_binary_compat_higher_major(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l2_binary_version_compatible(3000, 2001)
}

fn test_marker_resilient_layout(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.l2_resilience_marker("resilient_layout")? == 1)
}

fn test_marker_private_fields(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.l2_resilience_marker("private_fields")? == 2)
}

fn test_marker_unknown(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.l2_resilience_marker("does_not_exist")? == 0)
}
