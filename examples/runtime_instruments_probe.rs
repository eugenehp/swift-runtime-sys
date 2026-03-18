/// Instruments integration probe for Track M.1.
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

    println!("\n=== Instruments Integration (Track M.1) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        ("Reset clears event count", test_reset_clears),
        ("os_log event increments count", test_log_increments),
        ("POI begin returns success", test_poi_begin),
        ("POI end returns success", test_poi_end),
        ("POI duration captured", test_poi_duration),
        ("Profile iterations non-zero", test_profile_nonzero),
        ("Multiple events accumulate", test_multi_event_count),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track M.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 { println!("✓ All Track M.1 tests PASSED"); }
    else { panic!("✗ Track M.1 tests FAILED"); }
}

fn test_reset_clears(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m1_reset()?;
    Ok(contract.m1_event_count()? == 0)
}

fn test_log_increments(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m1_reset()?;
    let ok = contract.m1_os_log_event("rust.call.begin")?;
    Ok(ok && contract.m1_event_count()? == 1)
}

fn test_poi_begin(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m1_reset()?;
    contract.m1_poi_begin(123)
}

fn test_poi_end(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m1_reset()?;
    let b = contract.m1_poi_begin(42)?;
    let e = contract.m1_poi_end(42)?;
    Ok(b && e)
}

fn test_poi_duration(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m1_reset()?;
    contract.m1_poi_begin(77)?;
    let _ = contract.m1_profile_iterations(2_000)?;
    contract.m1_poi_end(77)?;
    Ok(contract.m1_last_duration_nanos()? > 0)
}

fn test_profile_nonzero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.m1_profile_iterations(10_000)? > 0)
}

fn test_multi_event_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m1_reset()?;
    contract.m1_os_log_event("a")?;
    contract.m1_os_log_event("b")?;
    contract.m1_os_log_event("c")?;
    Ok(contract.m1_event_count()? == 3)
}
