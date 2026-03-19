/// Phase C.6 probe: dynamic call-lowering coverage and deterministic fallback diagnostics.
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

    println!("\n=== Phase C.6 Dynamic Call-Lowering Expansion ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        (
            "Inout lowering class preserves mutation and result",
            test_c6_inout_lowering,
        ),
        (
            "Indirect return lowering class preserves pair payload",
            test_c6_indirect_pair_lowering,
        ),
        (
            "Auto invoke handles inout-compatible symbol via registry",
            test_c6_auto_supported_symbol,
        ),
        (
            "Shape negotiation reports native strategy for supported C.6 inout",
            test_c6_native_negotiation_inout,
        ),
        (
            "Shape negotiation reports native strategy for supported C.6 indirect return",
            test_c6_native_negotiation_pair,
        ),
        (
            "Fallback hierarchy returns stable reason for indirect+inout mixed unsupported shape",
            test_c6_reason_indirect_inout,
        ),
        (
            "Fallback hierarchy returns stable reason for throws+async mixed unsupported shape",
            test_c6_reason_throws_async,
        ),
        (
            "Unknown symbol invocation is rejected deterministically",
            test_c6_unknown_symbol_rejected,
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

    println!("\n=== C.6 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_c6_inout_lowering(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // initial=10, delta=5 -> updated=15, return=30
    let (updated, out) = c.n2_dynamic_symbol_inout("swift_contract_c6_inout_add_checked", 10, 5)?;
    Ok(updated == 15 && out == 30)
}

fn test_c6_indirect_pair_lowering(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // a=6, b=7 -> pair(sum=13, product=42)
    let pair = c.n2_dynamic_symbol_pair("swift_contract_c6_pair_sum_product", 6, 7)?;
    Ok(pair == (13, 42))
}

fn test_c6_auto_supported_symbol(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Auto path uses shape registry and existing invoke machinery.
    let result = c.n2_invoke_auto("swift_contract_c6_add_offset", 20, 19)?;
    Ok(result == 42)
}

fn test_c6_native_negotiation_inout(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.n2_lowering_strategy_json("c6.inout.checked.i32ptr_i32_to_i32")?;
    Ok(json.contains("\"strategy\":\"native\"")
        && json.contains("\"supported\":true")
        && json.contains("\"reason_code\":0"))
}

fn test_c6_native_negotiation_pair(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.n2_lowering_strategy_json("c6.indirect_ret.pair_sum_product.i32_i32_to_pair")?;
    Ok(json.contains("\"strategy\":\"native\"")
        && json.contains("\"supported\":true")
        && json.contains("\"reason_code\":0"))
}

fn test_c6_reason_indirect_inout(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.n2_lowering_strategy_json("mixed.indirect_ret_inout.combo")?;
    Ok(json.contains("\"strategy\":\"fallback\"")
        && json.contains("\"supported\":false")
        && json.contains("\"reason_code\":-462"))
}

fn test_c6_reason_throws_async(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.n2_lowering_strategy_json("mixed.throws_async.i32_to_i32")?;
    Ok(json.contains("\"strategy\":\"fallback\"")
        && json.contains("\"supported\":false")
        && json.contains("\"reason_code\":-463"))
}

fn test_c6_unknown_symbol_rejected(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let err = c
        .n2_invoke_auto("swift_contract_c6_not_registered", 1, 2)
        .err();
    Ok(matches!(
        err,
        Some(RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 20
        })
    ))
}
