/// Phase C.3 probe: enum layout, multi-payload, recursive, and resilient evolution.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Phase C.3 Enum Layout & Evolution ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("Multi-payload enum: no-value case", test_c3_multi_novalue),
        ("Multi-payload enum: one-value case", test_c3_multi_onevalue),
        (
            "Multi-payload enum: two-value case sum",
            test_c3_multi_twovalue,
        ),
        ("Recursive enum: leaf node", test_c3_recursive_leaf),
        ("Recursive enum: tree with branches", test_c3_recursive_tree),
        ("Resilient enum: stable case V1", test_c3_resilient_v1),
        ("Resilient enum: stable case V2", test_c3_resilient_v2),
        (
            "Resilient enum: unknown case rejection",
            test_c3_resilient_unknown,
        ),
    ];

    for (name, f) in tests {
        match f(&contract) {
            Ok(true) => {
                println!("PASS: {name}");
                passed += 1;
            }
            Ok(false) => {
                println!("FAIL: {name}");
                failed += 1;
            }
            Err(err) => {
                println!("FAIL: {name} ({err:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== C.3 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_c3_multi_novalue(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Multi-payload: no-value case (tag only, no associated data).
    let result = c.n2_dynamic_symbol_const("swift_contract_c3_multi_payload_novalue")?;
    Ok(result == 1)
}

fn test_c3_multi_onevalue(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Multi-payload: one-value case.
    let result = c.n2_dynamic_symbol_single("swift_contract_c3_multi_payload_onevalue", 99)?;
    Ok(result == 99)
}

fn test_c3_multi_twovalue(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Multi-payload: two-value case sum.
    let result = c.n2_dynamic_symbol_i32("swift_contract_c3_multi_payload_sumtwo", 30, 12)?;
    Ok(result == 42)
}

fn test_c3_recursive_leaf(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Recursive enum: single leaf node.
    let result = c.n2_dynamic_symbol_single("swift_contract_c3_recursive_leaf", 55)?;
    Ok(result == 55)
}

fn test_c3_recursive_tree(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Recursive enum: tree with root and two leaf children.
    // tree = branch(100, leaf(20), leaf(20)) => sum = 100 + 20 + 20 = 140
    let result = c.n2_dynamic_symbol_i32("swift_contract_c3_recursive_tree", 100, 20)?;
    Ok(result == 140)
}

fn test_c3_resilient_v1(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Resilient enum: stable version 1 case (tag=0 => returns 10).
    let result = c.n2_dynamic_symbol_single("swift_contract_c3_resilient_match", 0)?;
    Ok(result == 10)
}

fn test_c3_resilient_v2(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Resilient enum: stable version 2 case (tag=1 => returns 20).
    let result = c.n2_dynamic_symbol_single("swift_contract_c3_resilient_match", 1)?;
    Ok(result == 20)
}

fn test_c3_resilient_unknown(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Resilient enum: unknown/future case (tag=99 => returns Int32.min).
    let result = c.n2_dynamic_symbol_single("swift_contract_c3_resilient_match", 99)?;
    Ok(result == i32::MIN)
}
