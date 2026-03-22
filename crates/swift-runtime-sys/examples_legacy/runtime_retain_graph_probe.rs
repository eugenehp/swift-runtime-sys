/// Retain-count inspection and graph probe for Track K.2.
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

    println!("\n=== Retain Count & Graph (Track K.2) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 6] = [
        ("Retain delta is positive", test_retain_delta_positive),
        ("Type infer class path", test_type_infer_class),
        ("Type infer value path", test_type_infer_value),
        ("Type infer metatype path", test_type_infer_metatype),
        ("Graph DOT has digraph header", test_dot_header),
        ("Graph DOT has cycle edges", test_dot_edges),
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

    println!("\n=== Track K.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track K.2 tests PASSED");
    } else {
        panic!("✗ Track K.2 tests FAILED");
    }
}

fn test_retain_delta_positive(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.k2_retain_delta()? >= 1)
}

fn test_type_infer_class(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.k2_reference_type_infer(1)? == 1)
}

fn test_type_infer_value(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.k2_reference_type_infer(2)? == 2)
}

fn test_type_infer_metatype(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.k2_reference_type_infer(3)? == 3)
}

fn test_dot_header(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.k2_reference_graph_dot()?.contains("digraph"))
}

fn test_dot_edges(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dot = contract.k2_reference_graph_dot()?;
    Ok(dot.contains("A -> B") && dot.contains("B -> A"))
}
