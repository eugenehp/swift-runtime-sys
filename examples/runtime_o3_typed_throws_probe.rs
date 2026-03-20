/// O.3 Typed-Throws ABI Coverage probe.
///
/// Exercises the `throws(ConcreteError)` typed-throws lowering path introduced
/// in Swift 6.x, which passes the error via a value register rather than boxing
/// it as an `any Error` existential — a distinct ABI path from plain `throws`.
///
/// Deterministic error encoding convention (O3Error):
///   bits [31:16] = case tag:  0x0001 = negativeInput, 0x0002 = outOfRange, 0x0003 = combinedFailure
///   bits [15:0]  = 16-bit truncated payload (value or combined code)
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

    println!("\n=== O.3 Typed-Throws ABI Coverage ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        (
            "typed-throws success path returns doubled value",
            test_success_path,
        ),
        (
            "typed-throws success path error_out is zero",
            test_success_no_error,
        ),
        (
            "typed-throws negative input fires O3Error::negativeInput",
            test_concrete_error_negative,
        ),
        (
            "typed-throws error case tag is 0x0001 (negativeInput)",
            test_error_case_tag_negative,
        ),
        (
            "typed-throws out-of-range fires O3Error::outOfRange",
            test_concrete_error_out_of_range,
        ),
        (
            "typed-throws error identity: two distinct concrete cases",
            test_error_identity,
        ),
        (
            "typed-throws combined-failure on i32 overflow",
            test_combined_failure_overflow,
        ),
        (
            "typed-throws combined-failure: no-overflow success path",
            test_combined_no_overflow,
        ),
        ("typed-throws async success path", test_async_success),
        (
            "O.3 lowering strategy: typed_throws signatures are native",
            test_lowering_strategy,
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

    println!("\n=== O.3 Typed-Throws Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    // Structured summary line for parity matrix parsing:
    println!(
        "o3 typed-throws parity => success_path_ok={} no_error_ok={} concrete_error_ok={} tag_negative_ok={} out_of_range_ok={} identity_ok={} overflow_ok={} no_overflow_ok={} async_ok={} lowering_ok={}",
        passed_flags[0], passed_flags[1], passed_flags[2], passed_flags[3], passed_flags[4],
        passed_flags[5], passed_flags[6], passed_flags[7], passed_flags[8], passed_flags[9],
    );

    if failed == 0 {
        println!("✓ All O.3 typed-throws tests PASSED");
    } else {
        panic!("✗ O.3 typed-throws tests FAILED");
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────────────────────

/// Extract the 16-bit case tag from an encoded O3Error.
fn error_case_tag(error_out: i32) -> u32 {
    ((error_out as u32) >> 16) & 0xFFFF
}

// ──────────────────────────────────────────────────────────────────────────────
// Test functions
// ──────────────────────────────────────────────────────────────────────────────

fn test_success_path(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (ret, error_out) = contract.o3_typed_throws_success(21)?;
    println!("  typed_throws_success(21) -> ret={ret}, error_out={error_out}");
    Ok(ret == 42 && error_out == 0)
}

fn test_success_no_error(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (ret, error_out) = contract.o3_typed_throws_success(0)?;
    println!("  typed_throws_success(0) -> ret={ret}, error_out={error_out}");
    Ok(ret == 0 && error_out == 0)
}

fn test_concrete_error_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (ret, error_out) = contract.o3_typed_throws_concrete_error(-5)?;
    println!("  typed_throws_concrete_error(-5) -> ret={ret}, error_out=0x{error_out:08x}");
    // Should have thrown — ret must be 0 and error_out non-zero
    Ok(ret == 0 && error_out != 0)
}

fn test_error_case_tag_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (_ret, error_out) = contract.o3_typed_throws_concrete_error(-99)?;
    let tag = error_case_tag(error_out);
    println!("  typed_throws_concrete_error(-99) error case tag=0x{tag:04x}");
    // negativeInput tag = 0x0001
    Ok(tag == 0x0001)
}

fn test_concrete_error_out_of_range(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let (ret, error_out) = contract.o3_typed_throws_concrete_error(9999)?;
    let tag = error_case_tag(error_out);
    println!("  typed_throws_concrete_error(9999) -> ret={ret}, tag=0x{tag:04x}");
    // outOfRange tag = 0x0002
    Ok(ret == 0 && tag == 0x0002)
}

fn test_error_identity(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (_, error_neg) = contract.o3_typed_throws_error_identity(-1)?;
    let (_, error_oor) = contract.o3_typed_throws_error_identity(2000)?;
    let tag_neg = error_case_tag(error_neg);
    let tag_oor = error_case_tag(error_oor);
    println!("  identity(-1) tag=0x{tag_neg:04x}  identity(2000) tag=0x{tag_oor:04x}");
    // Two distinct concrete error case tags
    Ok(tag_neg == 0x0001 && tag_oor == 0x0002 && tag_neg != tag_oor)
}

fn test_combined_failure_overflow(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    // i32::MAX + 1 overflows
    let (ret, error_out) = contract.o3_typed_throws_combined_failure(i32::MAX, 1)?;
    let tag = error_case_tag(error_out);
    println!("  combined_failure(MAX, 1) -> ret={ret}, tag=0x{tag:04x}");
    // combinedFailure tag = 0x0003
    Ok(ret == 0 && tag == 0x0003)
}

fn test_combined_no_overflow(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (ret, error_out) = contract.o3_typed_throws_combined_failure(100, 200)?;
    println!("  combined_failure(100, 200) -> ret={ret}, error_out={error_out}");
    Ok(ret == 300 && error_out == 0)
}

fn test_async_success(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (ret, error_out) = contract.o3_typed_throws_async(11)?;
    println!("  typed_throws_async(11) -> ret={ret}, error_out={error_out}");
    Ok(ret == 22 && error_out == 0)
}

fn test_lowering_strategy(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let native_sync = contract.o3_lowering_strategy_json("typed_throws.i32_to_i32")?;
    let native_async = contract.o3_lowering_strategy_json("typed_throws_async.i32_to_i32")?;
    let native_comb =
        contract.o3_lowering_strategy_json("typed_throws.combined_failure.i32_i32_to_i32")?;
    let fallback = contract.o3_lowering_strategy_json("unknown.typed_throws.shape")?;
    println!("  lowering[typed_throws.i32_to_i32]={native_sync}");
    println!("  lowering[typed_throws_async.i32_to_i32]={native_async}");
    println!("  lowering[combined_failure]={native_comb}");
    println!("  lowering[unknown.typed_throws.shape]={fallback}");
    Ok(native_sync.contains("\"supported\":true")
        && native_sync.contains("\"strategy\":\"native\"")
        && native_async.contains("\"supported\":true")
        && native_comb.contains("\"supported\":true")
        && fallback.contains("\"supported\":false")
        && fallback.contains("\"reason_code\":-461"))
}
