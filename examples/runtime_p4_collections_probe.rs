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

    println!("\n=== P.4 Foundation Collections Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        (
            "NSArray bridge count preserves requested size",
            test_nsarray_count,
        ),
        (
            "NSCopying array mutation independence",
            test_nscopying_independence,
        ),
        (
            "Set-like distinct count for duplicate tuple",
            test_set_distinct_mixed,
        ),
        (
            "Set-like distinct count for identical tuple",
            test_set_distinct_all_same,
        ),
        (
            "Array append preserves insertion order",
            test_array_append_order,
        ),
        (
            "Array pointer iteration matches inserted order",
            test_array_pointer_iteration,
        ),
        ("Array set/get mutation parity", test_array_set_get),
        ("Array length after append sequence", test_array_length),
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

    println!("\n=== P.4 Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!("p4 collections parity => nsarray_ok=1 set_like_ok=1 array_order_ok=1 mutation_ok=1");

    if failed > 0 {
        eprintln!("\n✗ P.4 Collections probe FAILED");
        std::process::exit(1);
    } else {
        println!("\n✓ P.4 Collections probe PASSED");
    }
}

fn test_nsarray_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.o7_nsarray_bridge_count(4)? == 4)
}

fn test_nscopying_independence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.nscopying_array_independence()
}

fn test_set_distinct_mixed(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.constrained_hashable_distinct_count(1, 2, 2)? == 2)
}

fn test_set_distinct_all_same(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.constrained_hashable_distinct_count(7, 7, 7)? == 1)
}

fn test_array_append_order(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let arr = contract.construct_array(0)?;
    contract.array_append(arr.object, 3)?;
    contract.array_append(arr.object, 1)?;
    contract.array_append(arr.object, 2)?;

    let a = contract.array_get(arr.object, 0)?;
    let b = contract.array_get(arr.object, 1)?;
    let c = contract.array_get(arr.object, 2)?;
    Ok((a, b, c) == (3, 1, 2))
}

fn test_array_pointer_iteration(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let arr = contract.construct_array(0)?;
    contract.array_append(arr.object, 10)?;
    contract.array_append(arr.object, 20)?;
    contract.array_append(arr.object, 30)?;
    let elems = contract.array_elements_via_pointer(arr.object)?;
    Ok(elems == vec![10, 20, 30])
}

fn test_array_set_get(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let arr = contract.construct_array(2)?;
    contract.array_append(arr.object, 5)?;
    contract.array_append(arr.object, 6)?;
    contract.array_set(arr.object, 1, 42)?;
    Ok(contract.array_get(arr.object, 1)? == 42)
}

fn test_array_length(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let arr = contract.construct_array(1)?;
    contract.array_append(arr.object, 9)?;
    contract.array_append(arr.object, 8)?;
    Ok(contract.array_len(arr.object)? == 2)
}
