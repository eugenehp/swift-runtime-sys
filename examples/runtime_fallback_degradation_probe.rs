// Phase B.4 Probe: Fallback & Graceful Degradation
// RuntimeContract-based validation of capability negotiation and fallback handling.

use swift_runtime_sys::RuntimeContract::RuntimeContract;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    println!("=== Phase B.4 Fallback & Degradation Probe ===\n");

    let mut pass = 0;
    let mut total = 0;

    total += 1;
    if contract.b4_is_feature_supported("metadata_introspection") {
        println!("PASS 1 metadata_introspection supported");
        pass += 1;
    } else {
        println!("FAIL 1 metadata_introspection should be supported");
    }

    total += 1;
    if contract.b4_is_feature_supported("witness_table_resolution") {
        println!("PASS 2 witness_table_resolution supported");
        pass += 1;
    } else {
        println!("FAIL 2 witness_table_resolution should be supported");
    }

    total += 1;
    if contract.b4_is_feature_supported("version_adapter") {
        println!("PASS 3 version_adapter supported");
        pass += 1;
    } else {
        println!("FAIL 3 version_adapter should be supported");
    }

    total += 1;
    if !contract.b4_is_feature_supported("__nonexistent_feature_xyz__") {
        println!("PASS 4 nonexistent feature rejected");
        pass += 1;
    } else {
        println!("FAIL 4 nonexistent feature should be unsupported");
    }

    let features = contract.b4_supported_features();

    total += 1;
    if !features.is_empty() {
        println!("PASS 5 non-empty feature list: {} entries", features.len());
        pass += 1;
    } else {
        println!("FAIL 5 feature list should not be empty");
    }

    total += 1;
    if features.len() >= 5 {
        println!("PASS 6 feature list has >= 5 entries");
        pass += 1;
    } else {
        println!("FAIL 6 feature list too short: {}", features.len());
    }

    total += 1;
    if features.contains(&"error_propagation".to_string()) {
        println!("PASS 7 contains error_propagation");
        pass += 1;
    } else {
        println!("FAIL 7 missing error_propagation");
    }

    total += 1;
    if features.contains(&"array_bridging".to_string()) {
        println!("PASS 8 contains array_bridging");
        pass += 1;
    } else {
        println!("FAIL 8 missing array_bridging");
    }

    total += 1;
    let mut sorted = features.clone();
    sorted.sort();
    if sorted == features {
        println!("PASS 9 feature list sorted");
        pass += 1;
    } else {
        println!("FAIL 9 feature list should be sorted");
    }

    total += 1;
    let dump = contract.b4_debug_dump();
    if dump.contains("runtime_version=") && dump.contains("supported_features=") {
        println!("PASS 10 debug dump includes version and features");
        pass += 1;
    } else {
        println!("FAIL 10 malformed debug dump: {}", dump);
    }

    total += 1;
    let missing = contract.b4_check_required_features(&[
        "metadata_introspection",
        "witness_table_resolution",
        "version_adapter",
    ]);
    if missing.is_empty() {
        println!("PASS 11 required features fully supported");
        pass += 1;
    } else {
        println!("FAIL 11 missing required features: {:?}", missing);
    }

    total += 1;
    if contract
        .b4_unsupported_operation::<()>("future_runtime_opcode", "feature not negotiated")
        .is_err()
    {
        println!("PASS 12 unsupported operation emits structured error");
        pass += 1;
    } else {
        println!("FAIL 12 unsupported operation should error");
    }

    println!("\n=== Summary ===");
    println!("Passed: {}/{}", pass, total);
    if pass == total {
        println!("Status: ALL TESTS PASSED");
        std::process::exit(0);
    } else {
        println!("Status: {} TESTS FAILED", total - pass);
        std::process::exit(1);
    }
}
