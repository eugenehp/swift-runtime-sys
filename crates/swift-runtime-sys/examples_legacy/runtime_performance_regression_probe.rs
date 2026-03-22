/// Performance regression testing probe for Track M.4.
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

    println!("\n=== Performance Regression Testing (Track M.4) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        ("Construct benchmark returns non-zero", test_construct_nonzero),
        ("Invoke benchmark returns non-zero", test_invoke_nonzero),
        ("Release benchmark returns non-zero", test_release_nonzero),
        ("Set/get baseline roundtrip", test_baseline_roundtrip),
        ("No regression below threshold", test_no_regression),
        ("Regression triggers above threshold", test_regression_alarm),
        ("Unknown baseline defaults zero", test_unknown_baseline),
        ("CI trend scenario stable", test_ci_trend_stable),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track M.4 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 { println!("✓ All Track M.4 tests PASSED"); }
    else { panic!("✗ Track M.4 tests FAILED"); }
}

fn test_construct_nonzero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.m4_run_benchmark("construct", 1000)? > 0)
}

fn test_invoke_nonzero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.m4_run_benchmark("invoke", 20_000)? > 0)
}

fn test_release_nonzero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.m4_run_benchmark("release", 1000)? > 0)
}

fn test_baseline_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let base = contract.m4_run_benchmark("invoke", 5_000)?;
    let set = contract.m4_set_baseline("invoke", base)?;
    let got = contract.m4_baseline_get("invoke")?;
    Ok(set && got == base)
}

fn test_no_regression(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m4_set_baseline("construct", 1_000)?;
    let alarm = contract.m4_regression_alarm("construct", 1_050, 10)?;
    Ok(!alarm)
}

fn test_regression_alarm(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.m4_set_baseline("release", 1_000)?;
    contract.m4_regression_alarm("release", 1_500, 10)
}

fn test_unknown_baseline(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.m4_baseline_get("missing")? == 0)
}

fn test_ci_trend_stable(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let baseline = contract.m4_run_benchmark("invoke", 10_000)?;
    contract.m4_set_baseline("invoke_ci", baseline)?;
    let current = baseline + (baseline / 20); // +5%
    let alarm = contract.m4_regression_alarm("invoke_ci", current, 10)?;
    Ok(!alarm)
}
