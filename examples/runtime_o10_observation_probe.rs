/// O.10 Observation Runtime Surface probe.
///
/// Exercises deterministic `@Observable` model semantics exported through
/// stable C-callable bridge wrappers, including mutation, snapshot, tracking,
/// and lowering-strategy introspection.
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
    let mut passed_flags = [0i32; 10];

    println!("\n=== O.10 Observation Runtime Surface ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        ("observable increment 5 + 7 = 12", test_increment_basic),
        (
            "observable increment supports negative delta",
            test_increment_negative,
        ),
        ("observable sum3 1+2+3 = 6", test_sum3_basic),
        ("observable sum3 with negative term", test_sum3_negative),
        (
            "observable snapshot returns original",
            test_snapshot_identity,
        ),
        ("observation tracking hits = 1", test_tracking_hits_once),
        (
            "observation tracking hits stable across inputs",
            test_tracking_hits_stable,
        ),
        (
            "lowering strategy JSON has strategy key",
            test_lowering_json_shape,
        ),
        (
            "lowering strategy supports observation signature",
            test_lowering_supported,
        ),
        (
            "lowering strategy unknown shape is unsupported",
            test_lowering_unknown_shape,
        ),
    ];

    for (i, (name, test_fn)) in tests.iter().enumerate() {
        match test_fn(&contract) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
                passed_flags[i] = 1;
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

    println!("\n=== O.10 Observation Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    println!(
        "o10 observation parity => inc_basic_ok={} inc_neg_ok={} sum3_basic_ok={} sum3_neg_ok={} snapshot_ok={} track_once_ok={} track_stable_ok={} lower_shape_ok={} lower_supported_ok={} lower_unknown_ok={}",
        passed_flags[0], passed_flags[1], passed_flags[2], passed_flags[3], passed_flags[4],
        passed_flags[5], passed_flags[6], passed_flags[7], passed_flags[8], passed_flags[9],
    );

    if failed == 0 {
        println!("✓ All O.10 observation tests PASSED");
    } else {
        panic!("✗ O.10 observation tests FAILED");
    }
}

fn test_increment_basic(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o10_observable_increment(5, 7)? == 12)
}

fn test_increment_negative(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o10_observable_increment(5, -2)? == 3)
}

fn test_sum3_basic(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o10_observable_sum3(1, 2, 3)? == 6)
}

fn test_sum3_negative(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o10_observable_sum3(10, -4, 1)? == 7)
}

fn test_snapshot_identity(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o10_observable_snapshot(42)? == 42)
}

fn test_tracking_hits_once(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o10_observation_tracking_hits(3, 1)? == 1)
}

fn test_tracking_hits_stable(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let a = c.o10_observation_tracking_hits(100, 5)?;
    let b = c.o10_observation_tracking_hits(-2, 9)?;
    Ok(a == 1 && b == 1)
}

fn test_lowering_json_shape(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.o10_lowering_strategy_json("observation.observable")?;
    Ok(json.contains("\"strategy\""))
}

fn test_lowering_supported(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.o10_lowering_strategy_json("observation.observable.increment")?;
    Ok(json.contains("\"supported\":true"))
}

fn test_lowering_unknown_shape(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.o10_lowering_strategy_json("unknown.shape")?;
    Ok(json.contains("\"supported\":false"))
}
