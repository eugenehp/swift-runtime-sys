/// Universal call-lowering and invocation probe for Track N.2.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

const N2_CAP_INDIRECT_RET: u32 = 1 << 0;
const N2_CAP_INOUT: u32 = 1 << 1;
const N2_CAP_THROWING: u32 = 1 << 2;
const N2_CAP_ASYNC: u32 = 1 << 3;
const N2_CAP_RESILIENT_ARGS: u32 = 1 << 4;

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

    println!("\n=== Universal Call Lowering & Invocation (Track N.2) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 18] = [
        (
            "Capability mask advertises matrix support",
            test_capability_mask,
        ),
        ("Direct i32 add invocation", test_direct_add),
        ("Inout mutation invocation", test_inout_add_assign),
        ("Throwing success path", test_throwing_success),
        ("Throwing error path", test_throwing_error),
        ("Async invocation path", test_async_double),
        ("Indirect return pair", test_indirect_pair),
        ("Resilient argument path", test_resilient_args),
        (
            "Unknown symbol i32_i32_to_i32",
            test_dynamic_unknown_symbol_i32,
        ),
        (
            "Unknown symbol i32ptr_i32_to_i32",
            test_dynamic_unknown_symbol_inout,
        ),
        (
            "Unknown symbol i32_i32_to_pair",
            test_dynamic_unknown_symbol_pair,
        ),
        (
            "Unknown symbol rejects unknown shape",
            test_dynamic_unknown_shape_reject,
        ),
        (
            "Lowering strategy fallback",
            test_lowering_strategy_fallback,
        ),
        ("Dynamic single-arg i32_to_i32", test_dynamic_single_arg),
        ("Dynamic void_to_i32 constant", test_dynamic_void_to_i32),
        ("Symbol describe: known shape", test_symbol_describe_known),
        (
            "Symbol describe: unregistered returns unsupported",
            test_symbol_describe_unregistered,
        ),
        (
            "Describe and invoke unknown callable (N.2 exit criterion)",
            test_describe_and_invoke,
        ),
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

    println!("\n=== Track N.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.2 tests PASSED");
    } else {
        panic!("✗ Track N.2 tests FAILED");
    }
}

fn test_capability_mask(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let mask = contract.n2_capability_mask()?;
    let required =
        N2_CAP_INDIRECT_RET | N2_CAP_INOUT | N2_CAP_THROWING | N2_CAP_ASYNC | N2_CAP_RESILIENT_ARGS;
    Ok((mask & required) == required)
}

fn test_direct_add(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_direct_add(12, 30)? == 42)
}

fn test_inout_add_assign(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (updated, returned) = contract.n2_inout_add_assign(5, 7)?;
    Ok(updated == 12 && returned == 12)
}

fn test_throwing_success(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_throwing_require_non_negative(44)? == 44)
}

fn test_throwing_error(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_throwing_require_non_negative(-1).is_err())
}

fn test_async_double(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_async_double(21)? == 42)
}

fn test_indirect_pair(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (sum, diff) = contract.n2_indirect_pair_sum_diff(11, 4)?;
    Ok(sum == 15 && diff == 7)
}

fn test_resilient_args(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_resilient_counter_addpair(9, 8)? == 17)
}

fn test_dynamic_unknown_symbol_i32(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    // (10 + 20 + 3) via runtime symbol lookup
    Ok(contract.n2_dynamic_symbol_i32("swift_contract_n2_unknown_add_offset", 10, 20)? == 33)
}

fn test_dynamic_unknown_symbol_inout(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    // inout starts at 9, then += 5
    let (updated, returned) =
        contract.n2_dynamic_symbol_inout("swift_contract_n2_unknown_inout_accumulate", 9, 5)?;
    Ok(updated == 14 && returned == 14)
}

fn test_dynamic_unknown_symbol_pair(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let (sum, diff) =
        contract.n2_dynamic_symbol_pair("swift_contract_n2_unknown_pair_sum_diff", 12, 7)?;
    Ok(sum == 19 && diff == 5)
}

fn test_dynamic_unknown_shape_reject(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.n2_dynamic_symbol_rejects_unknown_shape("swift_contract_n2_unknown_add_offset")
}

fn test_lowering_strategy_fallback(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let known = contract.n2_lowering_strategy_json("direct.add.i32_i32_to_i32")?;
    let dynamic = contract.n2_lowering_strategy_json("dynamic.symbol.i32_i32_to_i32")?;
    let unknown = contract.n2_lowering_strategy_json("unknown.signature.shape")?;
    Ok(known.contains("\"supported\":true")
        && known.contains("\"strategy\":\"native\"")
        && dynamic.contains("\"supported\":true")
        && dynamic.contains("\"strategy\":\"native\"")
        && unknown.contains("\"supported\":false")
        && unknown.contains("\"strategy\":\"fallback\""))
}

fn test_dynamic_single_arg(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // negate(15) = 0 - 15 = -15
    Ok(contract.n2_dynamic_symbol_single("swift_contract_n2_unknown_negate", 15)? == -15)
}

fn test_dynamic_void_to_i32(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_dynamic_symbol_const("swift_contract_n2_unknown_const42")? == 42)
}

fn test_symbol_describe_known(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (shape, supported) = contract.n2_symbol_describe("swift_contract_n2_unknown_negate")?;
    Ok(supported && shape == "i32_to_i32")
}

fn test_symbol_describe_unregistered(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let (_shape, supported) = contract.n2_symbol_describe("swift_not_a_real_symbol")?;
    Ok(!supported)
}

fn test_describe_and_invoke(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Rust knows only the symbol name — shape is discovered at runtime.
    // negate(7, _) = -7
    Ok(contract.n2_describe_and_invoke("swift_contract_n2_unknown_negate", 7, 0)? == -7)
}
