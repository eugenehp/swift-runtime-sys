/// Phase C.7 probe: runtime safety guardrails, structured diagnostics, and replay harness.
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

    println!("\n=== Phase C.7 Runtime Safety Guardrails ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        (
            "Preflight allows low-risk operation path",
            test_c7_preflight_allow,
        ),
        (
            "Preflight blocks high-risk operation path",
            test_c7_preflight_block,
        ),
        (
            "Guarded invoke rejects blocked operations deterministically",
            test_c7_guarded_reject,
        ),
        (
            "Guarded invoke executes allowed operation safely",
            test_c7_guarded_allow,
        ),
        (
            "Crash capsule emits structured signal and reason code",
            test_c7_capsule_pair,
        ),
        (
            "Crash capsule context payload is stable and non-zero",
            test_c7_capsule_context,
        ),
        (
            "Replay harness records and executes allowed request",
            test_c7_replay_roundtrip,
        ),
        (
            "Replay harness rejects unknown token",
            test_c7_replay_reject_unknown,
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

    println!("\n=== C.7 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_c7_preflight_allow(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let allowed = c.n2_dynamic_symbol_single("swift_contract_c7_preflight_capability", 1)?;
    Ok(allowed == 1)
}

fn test_c7_preflight_block(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let allowed = c.n2_dynamic_symbol_single("swift_contract_c7_preflight_capability", 2)?;
    Ok(allowed == 0)
}

fn test_c7_guarded_reject(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_i32("swift_contract_c7_guarded_invoke", 2, 99)?;
    Ok(result == i32::MIN)
}

fn test_c7_guarded_allow(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_i32("swift_contract_c7_guarded_invoke", 1, 32)?;
    Ok(result == 42)
}

fn test_c7_capsule_pair(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (signal, reason) =
        c.n2_dynamic_symbol_pair("swift_contract_c7_crash_capsule_pair", 2, 77)?;
    Ok(signal == 11 && reason == -702)
}

fn test_c7_capsule_context(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let context = c.n2_dynamic_symbol_i32("swift_contract_c7_crash_capsule_context", 2, 77)?;
    Ok(context != 0 && context == ((2i32.wrapping_mul(97)) ^ (77i32.wrapping_mul(131))))
}

fn test_c7_replay_roundtrip(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let token = c.n2_dynamic_symbol_i32("swift_contract_c7_replay_record", 1, 15)?;
    let result = c.n2_dynamic_symbol_single("swift_contract_c7_replay_execute", token)?;
    Ok(token > 100 && result == 25)
}

fn test_c7_replay_reject_unknown(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_single("swift_contract_c7_replay_execute", -1)?;
    Ok(result == i32::MIN)
}
