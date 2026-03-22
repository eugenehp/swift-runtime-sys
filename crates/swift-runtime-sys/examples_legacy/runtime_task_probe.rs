/// Task creation and continuation safety probe for Track G.1.
/// Tests: deterministic task spawn and checked-continuation resume-once behavior.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Task Creation & Continuation (Track G.1) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        ("Task sum basic", test_task_sum_basic),
        ("Task sum negative", test_task_sum_negative),
        ("Task chain deterministic", test_task_chain_deterministic),
        ("Task chain zero steps", test_task_chain_zero_steps),
        ("Task spawn sequence", test_task_spawn_sequence),
        ("Continuation reset count", test_continuation_reset_count),
        (
            "Continuation roundtrip value",
            test_continuation_roundtrip_value,
        ),
        (
            "Continuation count increments once",
            test_continuation_count_increment,
        ),
        (
            "Continuation resume-once validation",
            test_continuation_validate_once,
        ),
        (
            "Continuation reset after use",
            test_continuation_reset_after_use,
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

    println!("\n=== Track G.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track G.1 tests PASSED");
    } else {
        panic!("✗ Track G.1 tests FAILED");
    }
}

fn test_task_sum_basic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_spawn_sum(2, 5)? == 7)
}

fn test_task_sum_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_spawn_sum(-8, 3)? == -5)
}

fn test_task_chain_deterministic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // base + sum(0..5) = 10 + 10 = 20
    Ok(contract.task_spawn_chain(10, 5)? == 20)
}

fn test_task_chain_zero_steps(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.task_spawn_chain(42, 0)? == 42)
}

fn test_task_spawn_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let mut acc = 0;
    for i in 0..4 {
        acc += contract.task_spawn_sum(i, i + 1)?;
    }
    // (0+1) + (1+2) + (2+3) + (3+4) = 16
    Ok(acc == 16)
}

fn test_continuation_reset_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.continuation_reset()?;
    Ok(contract.continuation_resume_count()? == 0)
}

fn test_continuation_roundtrip_value(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.continuation_reset()?;
    Ok(contract.continuation_roundtrip(33)? == 33)
}

fn test_continuation_count_increment(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.continuation_reset()?;
    let before = contract.continuation_resume_count()?;
    let _ = contract.continuation_roundtrip(77)?;
    let after = contract.continuation_resume_count()?;
    Ok((after - before) == 1)
}

fn test_continuation_validate_once(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.continuation_reset()?;
    contract.continuation_validate_resume_once()
}

fn test_continuation_reset_after_use(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let _ = contract.continuation_roundtrip(9)?;
    contract.continuation_reset()?;
    Ok(contract.continuation_resume_count()? == 0)
}
