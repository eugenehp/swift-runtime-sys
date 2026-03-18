/// Task-local values probe for Track G.4.
/// Tests: default read, scoped insertion, inheritance, detached isolation, and repeatability.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Task-Local Values (Track G.4) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        ("Task-local default value", test_task_local_default),
        ("Task-local scoped insertion", test_task_local_scoped_insertion),
        ("Task-local child inheritance", test_task_local_child_inheritance),
        ("Task-local detached isolation", test_task_local_detached_isolation),
        ("Task-local repeated runs", test_task_local_repeated_runs),
        ("Task-local negative values", test_task_local_negative_values),
        ("Task-local large value", test_task_local_large_value),
        ("Task-local scope does not leak", test_task_local_scope_no_leak),
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

    println!("\n=== Track G.4 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track G.4 tests PASSED");
    } else {
        panic!("✗ Track G.4 tests FAILED");
    }
}

fn test_task_local_default(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_local_get_default()? == -1)
}

fn test_task_local_scoped_insertion(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_local_run_with(9, 1)? == 10)
}

fn test_task_local_child_inheritance(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_local_run_with(42, 0)? == 42)
}

fn test_task_local_detached_isolation(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.task_local_isolation_check(17)
}

fn test_task_local_repeated_runs(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let mut ok = true;
    for i in 0..5 {
        ok &= contract.task_local_run_with(i, 2)? == i + 2;
    }
    Ok(ok)
}

fn test_task_local_negative_values(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_local_run_with(-7, 3)? == -4)
}

fn test_task_local_large_value(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_local_run_with(1_000_000, 5)? == 1_000_005)
}

fn test_task_local_scope_no_leak(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let _ = contract.task_local_run_with(123, 0)?;
    Ok(contract.task_local_get_default()? == -1)
}
