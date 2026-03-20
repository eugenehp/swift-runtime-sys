/// O.4 + O.5 Parameter Pack & Span ABI Coverage probe.
///
/// O.4 exercises the fixed-arity @_cdecl wrapper pattern for Swift parameter
/// packs (`repeat each T`), confirming that pack-generic implementations are
/// correctly lowered to C-callable symbols at multiple arities.
///
/// O.5 exercises Swift `Span<T>` as a non-Sequence, index-iterable view over a
/// contiguous buffer supplied from Rust via raw pointer + count.
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

    println!("\n=== O.4/O.5 Parameter Pack & Span ABI Coverage ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        ("O.4: arity-0 pack sum returns 0", test_pack_sum_arity0),
        (
            "O.4: arity-1 pack sum is identity (7 → 7)",
            test_pack_sum_arity1,
        ),
        ("O.4: arity-3 pack sum (1+2+3 → 6)", test_pack_sum_arity3),
        (
            "O.4: arity-3 pack product (2×3×4 → 24)",
            test_pack_product_arity3,
        ),
        (
            "O.4: lowering strategy JSON contains 'strategy' key",
            test_pack_lowering_strategy,
        ),
        ("O.5: span sum [10,20,30] → 60", test_span_sum),
        ("O.5: span length [1,2,3,4,5] → 5", test_span_length),
        ("O.5: span first [99,1,2] → 99", test_span_first),
        (
            "O.5: span contains [1,2,3] needle=3 → 1",
            test_span_contains,
        ),
        (
            "O.5: span bounds_ok idx=2 valid, idx=3 invalid",
            test_span_bounds_ok,
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

    println!("\n=== O.4/O.5 Pack & Span Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    // Structured summary line for parity matrix parsing:
    println!(
        "o4o5 packs-span parity => pack_arity0_ok={} pack_arity1_ok={} pack_arity3_ok={} pack_product_ok={} pack_lowering_ok={} span_sum_ok={} span_length_ok={} span_first_ok={} span_contains_ok={} span_bounds_ok={}",
        passed_flags[0], passed_flags[1], passed_flags[2], passed_flags[3], passed_flags[4],
        passed_flags[5], passed_flags[6], passed_flags[7], passed_flags[8], passed_flags[9],
    );

    if failed == 0 {
        println!("✓ All O.4/O.5 pack & span tests PASSED");
    } else {
        panic!("✗ O.4/O.5 pack & span tests FAILED");
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// O.4 – Parameter Pack tests
// ──────────────────────────────────────────────────────────────────────────────

fn test_pack_sum_arity0(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let v = c.o4_pack_sum_arity0()?;
    Ok(v == 0)
}

fn test_pack_sum_arity1(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let v = c.o4_pack_sum_arity1(7)?;
    Ok(v == 7)
}

fn test_pack_sum_arity3(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let v = c.o4_pack_sum_arity3(1, 2, 3)?;
    Ok(v == 6)
}

fn test_pack_product_arity3(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let v = c.o4_pack_product_arity3(2, 3, 4)?;
    Ok(v == 24)
}

fn test_pack_lowering_strategy(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = c.o4_lowering_strategy_json("pack.sum<Int32>")?;
    Ok(json.contains("\"strategy\""))
}

// ──────────────────────────────────────────────────────────────────────────────
// O.5 – Span tests
// ──────────────────────────────────────────────────────────────────────────────

fn test_span_sum(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let data = [10i32, 20, 30];
    let v = c.o5_span_sum(&data)?;
    Ok(v == 60)
}

fn test_span_length(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let data = [1i32, 2, 3, 4, 5];
    let v = c.o5_span_length(&data)?;
    Ok(v == 5)
}

fn test_span_first(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let data = [99i32, 1, 2];
    let v = c.o5_span_first(&data)?;
    Ok(v == 99)
}

fn test_span_contains(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let data = [1i32, 2, 3];
    let found = c.o5_span_contains(&data, 3)?;
    let not_found = c.o5_span_contains(&data, 42)?;
    Ok(found == 1 && not_found == 0)
}

fn test_span_bounds_ok(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let data = [0i32, 0, 0]; // 3 elements, valid indices 0..=2
    let in_bounds = c.o5_span_bounds_ok(&data, 2)?;
    let out_of_bounds = c.o5_span_bounds_ok(&data, 3)?;
    Ok(in_bounds == 1 && out_of_bounds == 0)
}
