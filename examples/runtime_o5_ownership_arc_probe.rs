/// O.5 Wave probe: O.6 Ownership-Convention Closures + O.7 ObjC Bridge ARC.
///
/// O.6 exercises `borrowing` and `consuming` parameter convention annotations
/// on `@_cdecl` exported Swift functions, confirming the value-semantic ABI
/// lowering path is reachable and correct from Rust.
///
/// O.7 exercises the ObjC bridge ARC paths: NSString↔String, NSNumber↔Int,
/// and NSMutableArray bridging, verifying retain/release balance across the
/// Swift↔ObjC boundary.
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

    println!("\n=== O.5 Wave: O.6 Ownership + O.7 ObjC Bridge ARC ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        ("O.6: borrow_identity(42) = 42", test_o6_borrow_identity),
        ("O.6: consume_double(7) = 14", test_o6_consume_double),
        ("O.6: borrow_sum(10, 32) = 42", test_o6_borrow_sum),
        ("O.6: consume_negate(99) = -99", test_o6_consume_negate),
        (
            "O.6: lowering strategy JSON has 'strategy' key",
            test_o6_lowering_strategy,
        ),
        (
            "O.7: NSString bridge roundtrip length matches",
            test_o7_nsstring_roundtrip,
        ),
        (
            "O.7: NSMutableArray bridge count = 5",
            test_o7_nsarray_count,
        ),
        ("O.7: NSString ARC balance = 1", test_o7_bridge_arc_balance),
        (
            "O.7: NSNumber Int32 roundtrip = 12345",
            test_o7_nsnumber_roundtrip,
        ),
        (
            "O.7: NSString UTF-8 length matches Swift",
            test_o7_nsstring_utf8_match,
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

    println!("\n=== O.5 Wave Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    // Structured summary line for parity matrix parsing:
    println!(
        "o5wave ownership-arc parity => o6_borrow_id_ok={} o6_consume_double_ok={} o6_borrow_sum_ok={} o6_consume_negate_ok={} o6_lowering_ok={} o7_ns_roundtrip_ok={} o7_nsarray_ok={} o7_arc_balance_ok={} o7_nsnumber_ok={} o7_utf8_ok={}",
        passed_flags[0], passed_flags[1], passed_flags[2], passed_flags[3], passed_flags[4],
        passed_flags[5], passed_flags[6], passed_flags[7], passed_flags[8], passed_flags[9],
    );

    if failed == 0 {
        println!("✓ All O.5-wave tests PASSED");
    } else {
        panic!("✗ O.5-wave tests FAILED");
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// O.6 – Ownership Convention tests
// ──────────────────────────────────────────────────────────────────────────────

fn test_o6_borrow_identity(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o6_borrow_identity(42)? == 42)
}

fn test_o6_consume_double(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o6_consume_double(7)? == 14)
}

fn test_o6_borrow_sum(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o6_borrow_sum(10, 32)? == 42)
}

fn test_o6_consume_negate(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o6_consume_negate(99)? == -99)
}

fn test_o6_lowering_strategy(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.o6_lowering_strategy_json("borrowing.Int32.returning.Int32")?;
    Ok(json.contains("\"strategy\""))
}

// ──────────────────────────────────────────────────────────────────────────────
// O.7 – ObjC Bridge ARC tests
// ──────────────────────────────────────────────────────────────────────────────

fn test_o7_nsstring_roundtrip(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // "hello" has length 5 in NSString (UTF-16 code units)
    let len = c.o7_nsstring_bridge_roundtrip("hello")?;
    Ok(len == 5)
}

fn test_o7_nsarray_count(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o7_nsarray_bridge_count(5)? == 5)
}

fn test_o7_bridge_arc_balance(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o7_bridge_arc_balance()? == 1)
}

fn test_o7_nsnumber_roundtrip(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.o7_nsnumber_bridge_roundtrip(12345)? == 12345)
}

fn test_o7_nsstring_utf8_match(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // ASCII: UTF-8 len == code unit count
    Ok(c.o7_nsstring_utf8_match("swift-runtime-sys")? == 1)
}
