/// Async stream probe for Track G.3.
/// Tests: iterator next(), exhaustion, deterministic ordering, and aggregate sum.
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

    println!("\n=== Async Streams & AsyncSequence (Track G.3) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("Stream construct", test_stream_construct),
        ("Stream next first value", test_stream_next_first),
        ("Stream deterministic sequence", test_stream_sequence),
        ("Stream exhaustion", test_stream_exhaustion),
        ("Stream collect sum", test_stream_collect_sum),
        ("Stream zero count", test_stream_zero_count),
        (
            "Stream independent instances",
            test_stream_independent_instances,
        ),
        ("Stream release path", test_stream_release_path),
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

    println!("\n=== Track G.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track G.3 tests PASSED");
    } else {
        panic!("✗ Track G.3 tests FAILED");
    }
}

fn test_stream_construct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stream = contract.construct_stream(10, 3)?;
    contract.release(stream)?;
    Ok(true)
}

fn test_stream_next_first(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stream = contract.construct_stream(7, 4)?;
    let first = contract.stream_next(stream)?;
    contract.release(stream)?;
    Ok(first == Some(7))
}

fn test_stream_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stream = contract.construct_stream(3, 5)?;
    let mut values = Vec::new();
    while let Some(v) = contract.stream_next(stream)? {
        values.push(v);
    }
    contract.release(stream)?;
    Ok(values == vec![3, 4, 5, 6, 7])
}

fn test_stream_exhaustion(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stream = contract.construct_stream(1, 2)?;
    let _ = contract.stream_next(stream)?;
    let _ = contract.stream_next(stream)?;
    let exhausted = contract.stream_next(stream)?;
    contract.release(stream)?;
    Ok(exhausted.is_none())
}

fn test_stream_collect_sum(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 2 + 3 + 4 + 5 = 14
    Ok(contract.stream_collect_sum(2, 4)? == 14)
}

fn test_stream_zero_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stream = contract.construct_stream(100, 0)?;
    let first = contract.stream_next(stream)?;
    contract.release(stream)?;
    Ok(first.is_none() && contract.stream_collect_sum(100, 0)? == 0)
}

fn test_stream_independent_instances(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let a = contract.construct_stream(0, 3)?;
    let b = contract.construct_stream(10, 3)?;

    let a_values = vec![
        contract.stream_next(a)?.unwrap_or(-1),
        contract.stream_next(a)?.unwrap_or(-1),
        contract.stream_next(a)?.unwrap_or(-1),
    ];
    let b_values = vec![
        contract.stream_next(b)?.unwrap_or(-1),
        contract.stream_next(b)?.unwrap_or(-1),
        contract.stream_next(b)?.unwrap_or(-1),
    ];

    contract.release(a)?;
    contract.release(b)?;

    Ok(a_values == vec![0, 1, 2] && b_values == vec![10, 11, 12])
}

fn test_stream_release_path(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stream = contract.construct_stream(-5, 1)?;
    contract.release(stream)?;
    Ok(true)
}
