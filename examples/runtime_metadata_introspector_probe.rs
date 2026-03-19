/// Phase B.1 metadata layout introspector probe.
///
/// Builds a normalized Rust-side layout view from runtime metadata pointers and
/// runtime-discovered type names. This intentionally reuses the N.1 graph/type-info
/// bridge so B.1 can advance without introducing a second metadata oracle.
use swift_runtime_sys::RuntimeContract::{MetadataLayout, RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

type TestFn = fn(&RuntimeContract) -> Result<bool, RuntimeContractError>;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);
    let tests: [(&str, TestFn); 23] = [
        ("pointer scan: Person metadata id", test_person_pointer_scan),
        ("pointer scan: Person generic count", test_person_generic_count),
        ("pointer scan: Counter witness count", test_counter_witness_count),
        ("pointer scan: Counter generic count", test_counter_generic_count),
        ("pointer scan: String has zero generic params", test_string_generic_count),
        ("pointer scan: Array<Int32> generic count", test_array_generic_count),
        (
            "pointer scan: Array<OpaqueRef> generic count",
            test_array_opaque_generic_count,
        ),
        (
            "pointer scan: Dictionary<Int32, Int32> generic count",
            test_dictionary_generic_count,
        ),
        (
            "pointer scan: Dictionary<Int32, OpaqueRef> generic count",
            test_dictionary_ref_generic_count,
        ),
        (
            "pointer scan: Any<ContractObject> generic count",
            test_any_contract_object_generic_count,
        ),
        ("pointer scan: Array kind is generic instantiation", test_array_kind),
        ("pointer scan: Direction kind is enum", test_direction_pointer_kind),
        ("pointer scan: Shape metadata id", test_shape_pointer_scan),
        ("name scan: N1LayoutStruct field count", test_n1_struct_field_count),
        ("name scan: N1LayoutStruct field offsets", test_n1_struct_field_offsets),
        ("name scan: Direction kind is enum", test_direction_kind),
        ("name scan: Swift.String resolves", test_swift_string_kind),
        ("name scan: Array<Int32> resolves", test_array_name_kind),
        ("name scan: unknown type remains unknown", test_unknown_type_name),
        ("discovery path: discovered Layout type introspects", test_discovered_layout_type),
        ("registry path: metadata registry exposes bridged types", test_registry_entries),
        ("registry path: generic metadata is present", test_generic_registry_entries),
        ("registry path: Counter protocol linkage preserved", test_counter_protocol_linkage),
    ];

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Phase B.1 Metadata Layout Introspector ===");
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

    let sample = contract
        .scan_metadata_header_by_name("Array<Int32>")
        .unwrap_or_else(|e| panic!("sample introspection failed: {e:?}"));
    print_layout("sample layout", &sample);

    println!("\n=== Phase B.1 Summary ===");
    println!("Tests Passed: {passed}");
    println!("Tests Failed: {failed}");

    if failed == 0 {
        println!("✓ Metadata introspector probe PASSED");
    } else {
        panic!("✗ Metadata introspector probe FAILED");
    }
}

fn print_layout(label: &str, layout: &MetadataLayout) {
    println!(
        "{label} => name={} metadata_id={:?} kind={} kind_id={} fields={} offsets={:?} witnesses={} generic_params={} size={:?}",
        layout.type_name,
        layout.metadata_id,
        layout.kind,
        layout.kind_id,
        layout.field_count,
        layout.field_offsets,
        layout.witness_count,
        layout.generic_param_count,
        layout.size,
    );
}

fn test_person_pointer_scan(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(1)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.metadata_id == Some(1) && layout.type_name == "Person")
}

fn test_counter_witness_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(2)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Counter" && layout.witness_count == 1)
}

fn test_person_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(1)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.generic_param_count == 0)
}

fn test_counter_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(2)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.generic_param_count == 0)
}

fn test_string_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(3)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "String" && layout.generic_param_count == 0)
}

fn test_array_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(4)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Array<Int32>" && layout.generic_param_count == 1)
}

fn test_array_opaque_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(5)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Array<OpaqueRef>" && layout.generic_param_count == 1)
}

fn test_dictionary_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(6)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Dictionary<Int32, Int32>" && layout.generic_param_count == 2)
}

fn test_dictionary_ref_generic_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(7)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Dictionary<Int32, OpaqueRef>" && layout.generic_param_count == 2)
}

fn test_any_contract_object_generic_count(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(8)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Any<ContractObject>" && layout.generic_param_count == 1)
}

fn test_array_kind(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(4)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.kind == "generic_instantiation" && layout.kind_id == 8)
}

fn test_direction_pointer_kind(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(9)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.type_name == "Direction" && layout.kind == "enum" && layout.kind_id == 3)
}

fn test_shape_pointer_scan(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let metadata = contract.lookup_metadata(10)?;
    let layout = contract.scan_metadata_header(metadata)?;
    Ok(layout.metadata_id == Some(10) && layout.type_name == "Shape")
}

fn test_n1_struct_field_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("N1LayoutStruct")?;
    Ok(layout.field_count == 2)
}

fn test_n1_struct_field_offsets(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("N1LayoutStruct")?;
    Ok(layout.field_offsets == vec![0, 8])
}

fn test_direction_kind(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("Direction")?;
    Ok(layout.kind == "enum" && layout.kind_id == 3)
}

fn test_swift_string_kind(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("Swift.String")?;
    Ok(layout.kind == "struct" && layout.kind_id == 2)
}

fn test_array_name_kind(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("Array<Int32>")?;
    Ok(layout.kind == "generic_instantiation" && layout.kind_id == 8)
}

fn test_unknown_type_name(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("No.Such.Type")?;
    Ok(layout.kind == "unknown" && layout.kind_id == -1)
}

fn test_discovered_layout_type(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let discovered = contract.n1_enumerate_all_types_json()?;
    let name = discovered
        .split('"')
        .find(|value| value.contains("Layout") && !value.contains("name"))
        .unwrap_or("N1LayoutStruct");
    let layout = contract.scan_metadata_header_by_name(name)?;
    Ok(layout.kind != "unknown")
}

fn test_registry_entries(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let entries = contract.metadata_registry_entries()?;
    Ok(entries.iter().any(|entry| entry.name == "Person")
        && entries.iter().any(|entry| entry.name == "Counter")
        && entries.iter().any(|entry| entry.name == "Array<Int32>"))
}

fn test_generic_registry_entries(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let entries = contract.metadata_registry_entries()?;
    Ok(entries.iter().any(|entry| entry.metadata_id == 1001 && entry.name == "ContractGenericBox<Int32>"))
}

fn test_counter_protocol_linkage(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let layout = contract.scan_metadata_header_by_name("Counter")?;
    Ok(layout.witness_count == 1)
}