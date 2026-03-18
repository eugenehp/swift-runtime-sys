/// Universal runtime metadata graph probe for Track N.1.
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

    println!("\n=== Universal Runtime Metadata Graph (Track N.1) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 9] = [
        ("Kind: class node", test_kind_class),
        ("Kind: struct node", test_kind_struct),
        ("Kind: generic instantiation node", test_kind_generic),
        ("Field count: struct node", test_field_count_struct),
        ("Field offset ordering", test_field_offset_order),
        ("Cycle-safe traversal count", test_traverse_count),
        ("Snapshot has user-defined type", test_snapshot_user_type),
        ("Snapshot has stdlib generic", test_snapshot_stdlib_generic),
        ("Snapshot has cycle edge", test_snapshot_cycle_edge),
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

    println!("\n=== Track N.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.1 tests PASSED");
    } else {
        panic!("✗ Track N.1 tests FAILED");
    }
}

fn test_kind_class(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind(1)? == 1)
}

fn test_kind_struct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind(2)? == 2)
}

fn test_kind_generic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_kind(8)? == 8)
}

fn test_field_count_struct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_field_count(2)? == 2)
}

fn test_field_offset_order(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let off0 = contract.n1_metadata_field_offset(2, 0)?;
    let off1 = contract.n1_metadata_field_offset(2, 1)?;
    Ok(off0 == 0 && off1 > off0)
}

fn test_traverse_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_graph_traverse_count()? == 3)
}

fn test_snapshot_user_type(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract
        .n1_metadata_snapshot_json()?
        .contains("N1LayoutStruct"))
}

fn test_snapshot_stdlib_generic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract
        .n1_metadata_snapshot_json()?
        .contains("Array<Int32>"))
}

fn test_snapshot_cycle_edge(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n1_metadata_snapshot_json()?.contains("[3,1]"))
}
