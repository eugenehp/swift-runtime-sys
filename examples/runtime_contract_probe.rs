use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    // Prefer loading both bridge and thunk libraries, but this probe only needs bridge symbols.
    let factory = RuntimeFactory::with_thunk_library("libRustBridge.dylib", "libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let contract = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    println!(
        "runtime contract => version={} bytes={} has_counter={} has_person={}",
        contract.version,
        contract.json.as_bytes().len(),
        contract.json.contains("\"Counter\""),
        contract.json.contains("\"Person\"")
    );
}
