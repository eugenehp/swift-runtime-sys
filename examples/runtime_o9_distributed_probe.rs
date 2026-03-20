/// O.9 Distributed actor surface scaffold probe.
///
/// Validates dormant Swift bridge exports that define the concrete O14b probe
/// surface without attempting real distributed-runtime execution on blocked hosts.
use swift_runtime_sys::RuntimeFactory::{RuntimeFactory, RuntimeFactoryError};

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let mut passed = 0;
    let mut failed = 0;
    let mut passed_flags = [0i32; 6];

    println!("\n=== O.9 Distributed Actor Surface Scaffold Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeFactory) -> Result<bool, RuntimeFactoryError>,
    ); 6] = [
        ("manifest version is v1", test_manifest_version),
        ("manifest JSON includes planned probes", test_manifest_json_shape),
        ("manifest lists four O9 probe ids", test_manifest_lists_four_probes),
        ("scaffold flags cover all dormant probes", test_scaffold_flags),
        (
            "distributed import capability matches manifest field",
            test_manifest_distributed_field,
        ),
        (
            "all dormant O9 probe entrypoints are exported",
            test_individual_probe_status_exports,
        ),
    ];

    for (index, (name, test_fn)) in tests.iter().enumerate() {
        match test_fn(&factory) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
                passed_flags[index] = 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(error) => {
                println!("✗ {name} FAIL ({error:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== O.9 Scaffold Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!(
        "o9 distributed scaffold parity => manifest_version_ok={} manifest_json_ok={} probe_ids_ok={} flags_ok={} import_field_ok={} exports_ok={}",
        passed_flags[0],
        passed_flags[1],
        passed_flags[2],
        passed_flags[3],
        passed_flags[4],
        passed_flags[5],
    );

    if failed == 0 {
        println!("✓ O.9 distributed scaffold probe PASSED");
    } else {
        panic!("✗ O.9 distributed scaffold probe FAILED");
    }
}

fn test_manifest_version(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_probe_manifest_version")? == 1)
}

fn test_manifest_json_shape(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    let json = factory.call_to_cstring("swift_contract_o9_probe_manifest_json")?;
    Ok(json.contains("\"planned_probes\"") && json.contains("\"activation_requirements\""))
}

fn test_manifest_lists_four_probes(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    let json = factory.call_to_cstring("swift_contract_o9_probe_manifest_json")?;
    let count = [
        "\"id\":\"O9.1\"",
        "\"id\":\"O9.2\"",
        "\"id\":\"O9.3\"",
        "\"id\":\"O9.4\"",
    ]
    .iter()
    .filter(|needle| json.contains(**needle))
    .count();
    Ok(count == 4)
}

fn test_scaffold_flags(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_scaffold_probe_flags")? == 0b1111)
}

fn test_manifest_distributed_field(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    let json = factory.call_to_cstring("swift_contract_o9_probe_manifest_json")?;
    let can_import = factory.call_to_i32("swift_contract_o9_host_can_import_distributed")? == 1;
    Ok(
        (can_import && json.contains("\"host_can_import_distributed\":true"))
            || (!can_import && json.contains("\"host_can_import_distributed\":false")),
    )
}

fn test_individual_probe_status_exports(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    let statuses = [
        factory.call_to_i32("swift_contract_o9_metadata_descriptor_status")?,
        factory.call_to_i32("swift_contract_o9_distributed_invocation_status")?,
        factory.call_to_i32("swift_contract_o9_result_handling_status")?,
        factory.call_to_i32("swift_contract_o9_isolation_semantics_status")?,
    ];
    Ok(statuses.iter().all(|status| *status == 1))
}