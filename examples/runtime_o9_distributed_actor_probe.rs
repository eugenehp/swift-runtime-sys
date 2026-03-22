/// O.9 distributed actor runtime probe (O14b implementation path).
///
/// Runs deterministic checks for O9.1-O9.4 through concrete Swift bridge
/// exports. This probe is executed only when host watch_status=SUPPORTED and
/// O9_ENABLE_IMPLEMENTATION=1.
use swift_runtime_sys::RuntimeFactory::{RuntimeFactory, RuntimeFactoryError};

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let mut passed = 0;
    let mut failed = 0;
    let mut passed_flags = [0i32; 6];

    println!("\n=== O.9 Distributed Actor Runtime Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeFactory) -> Result<bool, RuntimeFactoryError>,
    ); 6] = [
        ("implementation probe version is v1", test_probe_version),
        (
            "implementation runtime readiness is true",
            test_runtime_readiness,
        ),
        (
            "O9.1 metadata descriptor introspection passes",
            test_o91_metadata_descriptor,
        ),
        (
            "O9.2 distributed invocation boundary passes",
            test_o92_distributed_invocation,
        ),
        (
            "O9.3 result handling/error propagation passes",
            test_o93_result_handling,
        ),
        (
            "O9.4 isolation semantics preservation passes",
            test_o94_isolation_semantics,
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

    println!("\n=== O.9 Runtime Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!(
        "o9 distributed runtime parity => version_ok={} runtime_ready_ok={} o91_ok={} o92_ok={} o93_ok={} o94_ok={}",
        passed_flags[0],
        passed_flags[1],
        passed_flags[2],
        passed_flags[3],
        passed_flags[4],
        passed_flags[5],
    );

    if failed == 0 {
        println!("✓ O.9 distributed runtime probe PASSED");
    } else {
        panic!("✗ O.9 distributed runtime probe FAILED");
    }
}

fn test_probe_version(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_impl_probe_version")? == 1)
}

fn test_runtime_readiness(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_impl_runtime_ready")? == 1)
}

fn test_o91_metadata_descriptor(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_impl_metadata_descriptor_probe")? == 1)
}

fn test_o92_distributed_invocation(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_impl_distributed_invocation_probe")? == 1)
}

fn test_o93_result_handling(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_impl_result_handling_probe")? == 1)
}

fn test_o94_isolation_semantics(factory: &RuntimeFactory) -> Result<bool, RuntimeFactoryError> {
    Ok(factory.call_to_i32("swift_contract_o9_impl_isolation_semantics_probe")? == 1)
}
