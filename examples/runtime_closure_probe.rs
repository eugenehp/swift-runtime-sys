/// Closure and function pointer probe for Track F.3.
/// Tests: Closure construction with capture, single/multi-arg invocation, capture extraction, round-trips.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut tests_passed = 0;
    let mut tests_failed = 0;

    // Single-Argument Closure (Adder) Tests
    println!("\n=== Single-Argument Closure (Adder) Tests ===");

    // Test 1: Construct an adder closure
    match test_closure_adder_construct(&contract) {
        Ok(true) => {
            println!("✓ Closure adder construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure adder construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 2: Invoke adder closure with positive argument
    match test_closure_adder_invoke_positive(&contract) {
        Ok(true) => {
            println!("✓ Closure adder invoke positive PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure adder invoke positive FAIL");
            tests_failed += 1;
        }
    }

    // Test 3: Invoke adder closure with negative argument
    match test_closure_adder_invoke_negative(&contract) {
        Ok(true) => {
            println!("✓ Closure adder invoke negative PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure adder invoke negative FAIL");
            tests_failed += 1;
        }
    }

    // Test 4: Extract capture from adder closure
    match test_closure_adder_get_capture(&contract) {
        Ok(true) => {
            println!("✓ Closure adder get capture PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure adder get capture FAIL");
            tests_failed += 1;
        }
    }

    // Test 5: Adder closure round-trip
    match test_closure_adder_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ Closure adder round-trip (construct, invoke, extract) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure adder round-trip (construct, invoke, extract) FAIL");
            tests_failed += 1;
        }
    }

    // Test 6: Multiple adder closures with different deltas
    match test_closure_adder_sequence(&contract) {
        Ok(true) => {
            println!("✓ Multiple adder closures sequence PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Multiple adder closures sequence FAIL");
            tests_failed += 1;
        }
    }

    // Test 7: Adder closure with zero delta
    match test_closure_adder_zero_delta(&contract) {
        Ok(true) => {
            println!("✓ Adder closure with zero delta PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Adder closure with zero delta FAIL");
            tests_failed += 1;
        }
    }

    // Test 8: Adder closure with extreme values
    match test_closure_adder_extreme(&contract) {
        Ok(true) => {
            println!("✓ Adder closure with extreme values PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Adder closure with extreme values FAIL");
            tests_failed += 1;
        }
    }

    // Multi-Argument Closure Tests
    println!("\n=== Multi-Argument Closure Tests ===");

    // Test 9: Construct a multi-arg closure
    match test_closure_multi_construct(&contract) {
        Ok(true) => {
            println!("✓ Closure multi construct PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure multi construct FAIL");
            tests_failed += 1;
        }
    }

    // Test 10: Invoke multi-arg closure
    match test_closure_multi_invoke(&contract) {
        Ok(true) => {
            println!("✓ Closure multi invoke PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure multi invoke FAIL");
            tests_failed += 1;
        }
    }

    // Test 11: Extract factor from multi closure
    match test_closure_multi_get_factor(&contract) {
        Ok(true) => {
            println!("✓ Closure multi get factor PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure multi get factor FAIL");
            tests_failed += 1;
        }
    }

    // Test 12: Extract offset from multi closure
    match test_closure_multi_get_offset(&contract) {
        Ok(true) => {
            println!("✓ Closure multi get offset PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure multi get offset FAIL");
            tests_failed += 1;
        }
    }

    // Test 13: Multi closure round-trip
    match test_closure_multi_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ Closure multi round-trip (construct, invoke, extract) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Closure multi round-trip (construct, invoke, extract) FAIL");
            tests_failed += 1;
        }
    }

    // Test 14: Multiple multi-arg closures
    match test_closure_multi_sequence(&contract) {
        Ok(true) => {
            println!("✓ Multiple multi-arg closures sequence PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Multiple multi-arg closures sequence FAIL");
            tests_failed += 1;
        }
    }

    // Test 15: Mixed adder and multi closures
    match test_closure_mixed(&contract) {
        Ok(true) => {
            println!("✓ Mixed adder and multi closures PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Mixed adder and multi closures FAIL");
            tests_failed += 1;
        }
    }

    // Summary
    println!("\n=== Closure Probe Summary ===");
    println!("Tests Passed: {}", tests_passed);
    println!("Tests Failed: {}", tests_failed);

    if tests_failed == 0 {
        println!("✓ All closure tests PASSED (Track F.3 complete)");
    } else {
        panic!("✗ Closure tests FAILED");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Single-Argument Closure Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_closure_adder_construct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_adder(10)?;
    assert_eq!(closure.type_id, 14);
    Ok(true)
}

fn test_closure_adder_invoke_positive(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_adder(5)?;
    let result = contract.closure_invoke_adder(closure, 10)?;
    Ok(result == 15) // 10 + 5 = 15
}

fn test_closure_adder_invoke_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_adder(-3)?;
    let result = contract.closure_invoke_adder(closure, 7)?;
    Ok(result == 4) // 7 + (-3) = 4
}

fn test_closure_adder_get_capture(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let delta = 42;
    let closure = contract.construct_closure_adder(delta)?;
    let captured = contract.closure_get_capture(closure)?;
    Ok(captured == delta)
}

fn test_closure_adder_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let delta = 17;
    let arg = 33;
    let closure = contract.construct_closure_adder(delta)?;
    let result = contract.closure_invoke_adder(closure, arg)?;
    let captured = contract.closure_get_capture(closure)?;
    Ok(result == (arg + delta) && captured == delta)
}

fn test_closure_adder_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closures: Vec<_> = vec![1, 2, 3, 4, 5]
        .into_iter()
        .map(|delta| contract.construct_closure_adder(delta))
        .collect::<Result<Vec<_>, _>>()?;

    for (i, closure) in closures.iter().enumerate() {
        if closure.type_id != 14 {
            return Ok(false);
        }
        let result = contract.closure_invoke_adder(*closure, 10)?;
        if result != 10 + (i as i32 + 1) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn test_closure_adder_zero_delta(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_adder(0)?;
    let result = contract.closure_invoke_adder(closure, 42)?;
    Ok(result == 42) // 42 + 0 = 42
}

fn test_closure_adder_extreme(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_adder(i32::MAX - 100)?;
    let result = contract.closure_invoke_adder(closure, 50)?;
    // Test that extreme values compute correctly
    Ok(result == (i32::MAX - 50))
}

// ─────────────────────────────────────────────────────────────────────────────
// Multi-Argument Closure Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_closure_multi_construct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_multi(2, 3)?;
    assert_eq!(closure.type_id, 15);
    Ok(true)
}

fn test_closure_multi_invoke(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closure = contract.construct_closure_multi(2, 3)?;
    let result = contract.closure_invoke_multi(closure, 5, 7)?;
    // Result = (5 * 2) + (7 * 3) = 10 + 21 = 31
    Ok(result == 31)
}

fn test_closure_multi_get_factor(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let factor = 12;
    let closure = contract.construct_closure_multi(factor, 5)?;
    let captured_factor = contract.closure_get_factor(closure)?;
    Ok(captured_factor == factor)
}

fn test_closure_multi_get_offset(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let offset = 8;
    let closure = contract.construct_closure_multi(2, offset)?;
    let captured_offset = contract.closure_get_offset(closure)?;
    Ok(captured_offset == offset)
}

fn test_closure_multi_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (factor, offset) = (3, 4);
    let (a, b) = (6, 2);
    let closure = contract.construct_closure_multi(factor, offset)?;
    let result = contract.closure_invoke_multi(closure, a, b)?;
    let captured_factor = contract.closure_get_factor(closure)?;
    let captured_offset = contract.closure_get_offset(closure)?;
    // Result = (6 * 3) + (2 * 4) = 18 + 8 = 26
    Ok(result == 26 && captured_factor == factor && captured_offset == offset)
}

fn test_closure_multi_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let closures: Vec<_> = vec![(1, 1), (2, 2), (3, 3)]
        .into_iter()
        .map(|(f, o)| contract.construct_closure_multi(f, o))
        .collect::<Result<Vec<_>, _>>()?;

    for closure in closures {
        if closure.type_id != 15 {
            return Ok(false);
        }
    }
    Ok(true)
}

fn test_closure_mixed(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let adder = contract.construct_closure_adder(10)?;
    let multi = contract.construct_closure_multi(2, 3)?;

    let adder_result = contract.closure_invoke_adder(adder, 5)?; // 5 + 10 = 15
    let multi_result = contract.closure_invoke_multi(multi, 4, 6)?; // (4 * 2) + (6 * 3) = 8 + 18 = 26

    Ok(adder.type_id == 14 && multi.type_id == 15 && adder_result == 15 && multi_result == 26)
}
