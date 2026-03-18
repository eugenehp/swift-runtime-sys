/// Runtime metadata discovery probe for Track N.1 dynamic discovery path.
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

    println!("\n=== Runtime Metadata Discovery (Track N.1) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("Kind by name for Swift.Int", test_kind_by_name_int),
        ("Kind by name for Swift.String", test_kind_by_name_string),
        (
            "Kind by name for generic instantiation",
            test_kind_by_name_array,
        ),
        (
            "Field count by name for N1LayoutStruct",
            test_field_count_by_name,
        ),
        (
            "Discover JSON includes user-defined type",
            test_discover_json_user_type,
        ),
        (
            "Discover JSON includes stdlib type",
            test_discover_json_stdlib,
        ),
        (
            "Discovery traversal count positive",
            test_discovery_traverse_count,
        ),
        ("Unknown type lookup errors", test_unknown_type_errors),
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

    println!("\n=== Track N.1 Discovery Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.1 discovery tests PASSED");
    } else {
        panic!("✗ Track N.1 discovery tests FAILED");
    }
}

fn test_kind_by_name_int(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind_by_name("Swift.Int")? == 2)
}

fn test_kind_by_name_string(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind_by_name("Swift.String")? == 2)
}

fn test_kind_by_name_array(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind_by_name("Swift.Array<Swift.Int32>")? == 8)
}

fn test_field_count_by_name(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_field_count_by_name("N1LayoutStruct")? == 2)
}

fn test_discover_json_user_type(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract
        .n1_metadata_discover_types_json()?
        .contains("N1LayoutStruct"))
}

fn test_discover_json_stdlib(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = contract.n1_metadata_discover_types_json()?;
    Ok(json.contains("Swift.Int") || json.contains("Swift.String"))
}

fn test_discovery_traverse_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_graph_traverse_discovered_count()? > 0)
}

fn test_unknown_type_errors(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind_by_name("No.Such.Type").is_err())
}
