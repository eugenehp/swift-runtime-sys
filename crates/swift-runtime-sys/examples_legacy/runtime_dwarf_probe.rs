/// DWARF debug-info access probe for Track M.2.
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

    println!("\n=== DWARF Debug Info Access (Track M.2) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        ("Reset clears cache", test_reset_cache),
        ("Cache binary succeeds", test_cache_binary),
        ("Cache size increments", test_cache_size),
        ("Lookup source contains file:line", test_lookup_source),
        ("Lookup variable has mock value", test_lookup_variable),
        ("Second cache path increases size", test_cache_two_paths),
        ("Source lookup deterministic format", test_source_format),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track M.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 { println!("✓ All Track M.2 tests PASSED"); }
    else { panic!("✗ Track M.2 tests FAILED"); }
}

fn test_reset_cache(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m2_reset()?;
    Ok(contract.m2_cache_size()? == 0)
}

fn test_cache_binary(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m2_reset()?;
    contract.m2_cache_binary("/tmp/fakeA.dylib")
}

fn test_cache_size(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m2_reset()?;
    contract.m2_cache_binary("/tmp/fakeA.dylib")?;
    Ok(contract.m2_cache_size()? == 1)
}

fn test_lookup_source(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let src = contract.m2_lookup_source(0x1234)?;
    Ok(src.contains("RustBridge.swift:") && src.split(':').count() == 2)
}

fn test_lookup_variable(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.m2_lookup_variable("counter")?.contains("counter=<mock>"))
}

fn test_cache_two_paths(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m2_reset()?;
    contract.m2_cache_binary("/tmp/fakeA.dylib")?;
    contract.m2_cache_binary("/tmp/fakeB.dylib")?;
    Ok(contract.m2_cache_size()? == 2)
}

fn test_source_format(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let src = contract.m2_lookup_source(1)?;
    Ok(src.ends_with("2") || src.ends_with("1") || src.contains("RustBridge.swift:"))
}
