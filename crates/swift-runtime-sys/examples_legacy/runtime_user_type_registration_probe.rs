/// User-defined type registration probe for Track L.1.
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

    println!("\n=== User-Defined Type Registration (Track L.1) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        ("Registry reset clears previous state", test_reset),
        ("Register type returns non-negative ID", test_register_returns_id),
        ("Register same type returns stable ID", test_register_stable_id),
        ("Lookup returns registered ID", test_lookup_registered),
        ("Bump version increments to 2", test_bump_version),
        ("Update compat allows minor forward", test_update_compat_forward),
        ("Update compat rejects major mismatch", test_update_compat_major_mismatch),
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

    println!("\n=== Track L.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track L.1 tests PASSED");
    } else {
        panic!("✗ Track L.1 tests FAILED");
    }
}

fn test_reset(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l1_registry_reset()?;
    Ok(contract.l1_lookup_type_id("MyType").is_err())
}

fn test_register_returns_id(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l1_registry_reset()?;
    Ok(contract.l1_register_type("MyType")? >= 10000)
}

fn test_register_stable_id(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l1_registry_reset()?;
    let a = contract.l1_register_type("MyType")?;
    let b = contract.l1_register_type("MyType")?;
    Ok(a == b)
}

fn test_lookup_registered(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l1_registry_reset()?;
    let id = contract.l1_register_type("Thing")?;
    Ok(contract.l1_lookup_type_id("Thing")? == id)
}

fn test_bump_version(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l1_registry_reset()?;
    let id = contract.l1_register_type("Versioned")?;
    Ok(contract.l1_bump_type_version(id)? == 2)
}

fn test_update_compat_forward(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.l1_update_compat(1001, 1002)
}

fn test_update_compat_major_mismatch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let compat = contract.l1_update_compat(1001, 2000)?;
    Ok(!compat)
}
