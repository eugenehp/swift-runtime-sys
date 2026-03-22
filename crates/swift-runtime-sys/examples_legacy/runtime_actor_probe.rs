/// Actor isolation probe for Track G.2.
/// Tests: actor construction, isolated invocation, and concurrent isolation validation.
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

    println!("\n=== Actor Isolation & Isolation Domains (Track G.2) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("Actor construct", test_actor_construct),
        ("Actor current initial", test_actor_current_initial),
        ("Actor add updates", test_actor_add_updates),
        ("Actor add sequence", test_actor_add_sequence),
        (
            "Actor isolation validation",
            test_actor_isolation_validation,
        ),
        (
            "Actor concurrent final state",
            test_actor_concurrent_final_state,
        ),
        ("Actor separate instances", test_actor_separate_instances),
        ("Actor release path", test_actor_release_path),
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

    println!("\n=== Track G.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track G.2 tests PASSED");
    } else {
        panic!("✗ Track G.2 tests FAILED");
    }
}

fn test_actor_construct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(5)?;
    contract.release(actor)?;
    Ok(true)
}

fn test_actor_current_initial(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(11)?;
    let value = contract.actor_current(actor)?;
    contract.release(actor)?;
    Ok(value == 11)
}

fn test_actor_add_updates(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(0)?;
    let value = contract.actor_add(actor, 4)?;
    contract.release(actor)?;
    Ok(value == 4)
}

fn test_actor_add_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(1)?;
    let _ = contract.actor_add(actor, 2)?;
    let _ = contract.actor_add(actor, 3)?;
    let current = contract.actor_current(actor)?;
    contract.release(actor)?;
    Ok(current == 6)
}

fn test_actor_isolation_validation(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(0)?;
    let ok = contract.actor_validate_isolation(actor)?;
    contract.release(actor)?;
    Ok(ok)
}

fn test_actor_concurrent_final_state(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(10)?;
    let _ = contract.actor_validate_isolation(actor)?;
    let current = contract.actor_current(actor)?;
    contract.release(actor)?;
    Ok(current >= 13)
}

fn test_actor_separate_instances(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let actor_a = contract.construct_actor(3)?;
    let actor_b = contract.construct_actor(7)?;

    let _ = contract.actor_add(actor_a, 5)?;
    let a = contract.actor_current(actor_a)?;
    let b = contract.actor_current(actor_b)?;

    contract.release(actor_a)?;
    contract.release(actor_b)?;

    Ok(a == 8 && b == 7)
}

fn test_actor_release_path(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let actor = contract.construct_actor(99)?;
    contract.release(actor)?;
    Ok(true)
}
