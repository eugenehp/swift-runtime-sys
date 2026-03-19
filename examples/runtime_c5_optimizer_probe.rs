/// Phase C.5 probe: optimizer-sensitive equivalence across debug/release builds.
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

    println!("\n=== Phase C.5 Optimizer-Sensitive Equivalence ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        ("Inline equivalence positive path", test_c5_inline_positive),
        ("Inline equivalence negative path", test_c5_inline_negative),
        ("Devirtualized dispatch equivalence", test_c5_devirt_equiv),
        (
            "Generic specialized and unspecialized equivalence",
            test_c5_generic_equiv,
        ),
        (
            "Generic equivalence wrapping arithmetic",
            test_c5_generic_wrap,
        ),
        ("ARC reset reports success", test_c5_arc_reset),
        ("ARC sequence value semantics", test_c5_arc_sequence),
        ("ARC deinit side effects are preserved", test_c5_arc_deinit),
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

    println!("\n=== C.5 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_c5_inline_positive(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_single("swift_contract_c5_inline_equiv", 9)?;
    Ok(result == ((9i32.wrapping_mul(3)).wrapping_add(7)))
}

fn test_c5_inline_negative(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_single("swift_contract_c5_inline_equiv", -4)?;
    Ok(result == ((-4i32).wrapping_mul(3).wrapping_add(7)))
}

fn test_c5_devirt_equiv(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_single("swift_contract_c5_devirt_equiv", 31)?;
    Ok(result == 42)
}

fn test_c5_generic_equiv(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_i32("swift_contract_c5_generic_equiv", 17, 25)?;
    Ok(result == 42)
}

fn test_c5_generic_wrap(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_i32("swift_contract_c5_generic_equiv", i32::MAX, 1)?;
    Ok(result == i32::MIN)
}

fn test_c5_arc_reset(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let result = c.n2_dynamic_symbol_const("swift_contract_c5_arc_reset")?;
    Ok(result == 1)
}

fn test_c5_arc_sequence(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.n2_dynamic_symbol_const("swift_contract_c5_arc_reset")?;
    let result = c.n2_dynamic_symbol_single("swift_contract_c5_arc_sequence", 21)?;
    Ok(result == 42)
}

fn test_c5_arc_deinit(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.n2_dynamic_symbol_const("swift_contract_c5_arc_reset")?;
    c.n2_dynamic_symbol_single("swift_contract_c5_arc_sequence", 5)?;
    c.n2_dynamic_symbol_single("swift_contract_c5_arc_sequence", 7)?;
    let count = c.n2_dynamic_symbol_const("swift_contract_c5_arc_deinit_count")?;
    Ok(count == 2)
}
