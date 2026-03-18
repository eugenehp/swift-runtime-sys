/// Memory profiling and malloc-tagging probe for Track M.3.
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

    println!("\n=== Memory Profiling & Malloc Tagging (Track M.3) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        ("Reset starts at zero usage", test_reset_zero),
        ("Tagged alloc increases usage", test_alloc_increase),
        ("Release reduces usage", test_release_reduce),
        ("Multiple subsystem attribution", test_multi_subsystem),
        ("Health report has live_tokens", test_health_report_tokens),
        ("Health report has total_bytes", test_health_report_bytes),
        ("Unknown subsystem usage is zero", test_unknown_subsystem),
        ("Full cleanup returns zero", test_full_cleanup),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track M.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 { println!("✓ All Track M.3 tests PASSED"); }
    else { panic!("✗ Track M.3 tests FAILED"); }
}

fn test_reset_zero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    Ok(contract.m3_usage_for_subsystem("runtime")? == 0)
}

fn test_alloc_increase(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    let _t = contract.m3_tag_alloc("runtime", 128)?;
    Ok(contract.m3_usage_for_subsystem("runtime")? == 128)
}

fn test_release_reduce(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    let t = contract.m3_tag_alloc("runtime", 128)?;
    let ok = contract.m3_release_alloc(t)?;
    Ok(ok && contract.m3_usage_for_subsystem("runtime")? == 0)
}

fn test_multi_subsystem(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    let _a = contract.m3_tag_alloc("ffi", 64)?;
    let _b = contract.m3_tag_alloc("bridge", 96)?;
    Ok(contract.m3_usage_for_subsystem("ffi")? == 64 && contract.m3_usage_for_subsystem("bridge")? == 96)
}

fn test_health_report_tokens(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    let _a = contract.m3_tag_alloc("ffi", 10)?;
    Ok(contract.m3_health_report()?.contains("\"live_tokens\":1"))
}

fn test_health_report_bytes(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    let _a = contract.m3_tag_alloc("ffi", 10)?;
    let _b = contract.m3_tag_alloc("ffi", 22)?;
    Ok(contract.m3_health_report()?.contains("\"total_bytes\":32"))
}

fn test_unknown_subsystem(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    Ok(contract.m3_usage_for_subsystem("missing")? == 0)
}

fn test_full_cleanup(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m3_reset()?;
    let a = contract.m3_tag_alloc("runtime", 50)?;
    let b = contract.m3_tag_alloc("runtime", 70)?;
    contract.m3_release_alloc(a)?;
    contract.m3_release_alloc(b)?;
    Ok(contract.m3_usage_for_subsystem("runtime")? == 0)
}
