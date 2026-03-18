/// Tuple construction and unpacking probe for Track F.2.
/// Tests: Pair and Triple construction from values, element extraction, round-trips.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut tests_passed = 0;
    let mut tests_failed = 0;

    // Pair Tests
    println!("\n=== Pair (2-Element Tuple) Tests ===");

    // Test 1: Construct a simple Pair
    match test_pair_construct(&contract) {
        Ok(true) => {
            println!("✓ Pair construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Pair construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 2: Extract first element from Pair
    match test_pair_extract_first(&contract) {
        Ok(true) => {
            println!("✓ Pair extract first PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Pair extract first FAIL");
            tests_failed += 1;
        }
    }

    // Test 3: Extract second element from Pair
    match test_pair_extract_second(&contract) {
        Ok(true) => {
            println!("✓ Pair extract second PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Pair extract second FAIL");
            tests_failed += 1;
        }
    }

    // Test 4: Pair round-trip
    match test_pair_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ Pair round-trip (construct, extract both) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Pair round-trip (construct, extract both) FAIL");
            tests_failed += 1;
        }
    }

    // Test 5: Pair with negative values
    match test_pair_negative(&contract) {
        Ok(true) => {
            println!("✓ Pair with negative values PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Pair with negative values FAIL");
            tests_failed += 1;
        }
    }

    // Test 6: Multiple Pairs in sequence
    match test_pair_sequence(&contract) {
        Ok(true) => {
            println!("✓ Multiple Pair construction sequence PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Multiple Pair construction sequence FAIL");
            tests_failed += 1;
        }
    }

    // Triple Tests
    println!("\n=== Triple (3-Element Tuple) Tests ===");

    // Test 7: Construct a simple Triple
    match test_triple_construct(&contract) {
        Ok(true) => {
            println!("✓ Triple construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Triple construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 8: Extract first element from Triple
    match test_triple_extract_first(&contract) {
        Ok(true) => {
            println!("✓ Triple extract first PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Triple extract first FAIL");
            tests_failed += 1;
        }
    }

    // Test 9: Extract second element from Triple
    match test_triple_extract_second(&contract) {
        Ok(true) => {
            println!("✓ Triple extract second PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Triple extract second FAIL");
            tests_failed += 1;
        }
    }

    // Test 10: Extract third element from Triple
    match test_triple_extract_third(&contract) {
        Ok(true) => {
            println!("✓ Triple extract third PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Triple extract third FAIL");
            tests_failed += 1;
        }
    }

    // Test 11: Triple round-trip
    match test_triple_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ Triple round-trip (construct, extract all) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Triple round-trip (construct, extract all) FAIL");
            tests_failed += 1;
        }
    }

    // Test 12: Triple with negative values
    match test_triple_negative(&contract) {
        Ok(true) => {
            println!("✓ Triple with negative values PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Triple with negative values FAIL");
            tests_failed += 1;
        }
    }

    // Test 13: Multiple Triples in sequence
    match test_triple_sequence(&contract) {
        Ok(true) => {
            println!("✓ Multiple Triple construction sequence PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Multiple Triple construction sequence FAIL");
            tests_failed += 1;
        }
    }

    // Test 14: Mixed Pair and Triple operations
    match test_mixed_tuples(&contract) {
        Ok(true) => {
            println!("✓ Mixed Pair and Triple operations PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Mixed Pair and Triple operations FAIL");
            tests_failed += 1;
        }
    }

    // Test 15: Pair with zero values
    match test_pair_zero(&contract) {
        Ok(true) => {
            println!("✓ Pair with zero values PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Pair with zero values FAIL");
            tests_failed += 1;
        }
    }

    // Summary
    println!("\n=== Tuple Unpacking Probe Summary ===");
    println!("Tests Passed: {}", tests_passed);
    println!("Tests Failed: {}", tests_failed);

    if tests_failed == 0 {
        println!("✓ All tuple tests PASSED (Track F.2 complete)");
    } else {
        panic!("✗ Tuple tests FAILED");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pair Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_pair_construct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let pair = contract.construct_tuple_pair(42, 99)?;
    assert_eq!(pair.type_id, 12);
    Ok(true)
}

fn test_pair_extract_first(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let pair = contract.construct_tuple_pair(42, 99)?;
    let first = contract.tuple_pair_get_first(pair)?;
    Ok(first == 42)
}

fn test_pair_extract_second(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let pair = contract.construct_tuple_pair(42, 99)?;
    let second = contract.tuple_pair_get_second(pair)?;
    Ok(second == 99)
}

fn test_pair_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (a, b) = (17, 88);
    let pair = contract.construct_tuple_pair(a, b)?;
    let extracted_a = contract.tuple_pair_get_first(pair)?;
    let extracted_b = contract.tuple_pair_get_second(pair)?;
    Ok(extracted_a == a && extracted_b == b)
}

fn test_pair_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (a, b) = (-99, -77);
    let pair = contract.construct_tuple_pair(a, b)?;
    let extracted_a = contract.tuple_pair_get_first(pair)?;
    let extracted_b = contract.tuple_pair_get_second(pair)?;
    Ok(extracted_a == a && extracted_b == b)
}

fn test_pair_zero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let pair = contract.construct_tuple_pair(0, 0)?;
    let first = contract.tuple_pair_get_first(pair)?;
    let second = contract.tuple_pair_get_second(pair)?;
    Ok(first == 0 && second == 0)
}

fn test_pair_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let pairs: Vec<_> = vec![(1, 2), (3, 4), (5, 6)]
        .into_iter()
        .map(|(a, b)| contract.construct_tuple_pair(a, b))
        .collect::<Result<Vec<_>, _>>()?;

    for p in &pairs {
        if p.type_id != 12 {
            return Ok(false);
        }
    }
    Ok(true)
}

// ─────────────────────────────────────────────────────────────────────────────
// Triple Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_triple_construct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let triple = contract.construct_tuple_triple(42, 99, 77)?;
    assert_eq!(triple.type_id, 13);
    Ok(true)
}

fn test_triple_extract_first(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let triple = contract.construct_tuple_triple(42, 99, 77)?;
    let first = contract.tuple_triple_get_first(triple)?;
    Ok(first == 42)
}

fn test_triple_extract_second(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let triple = contract.construct_tuple_triple(42, 99, 77)?;
    let second = contract.tuple_triple_get_second(triple)?;
    Ok(second == 99)
}

fn test_triple_extract_third(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let triple = contract.construct_tuple_triple(42, 99, 77)?;
    let third = contract.tuple_triple_get_third(triple)?;
    Ok(third == 77)
}

fn test_triple_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (a, b, c) = (11, 22, 33);
    let triple = contract.construct_tuple_triple(a, b, c)?;
    let extracted_a = contract.tuple_triple_get_first(triple)?;
    let extracted_b = contract.tuple_triple_get_second(triple)?;
    let extracted_c = contract.tuple_triple_get_third(triple)?;
    Ok(extracted_a == a && extracted_b == b && extracted_c == c)
}

fn test_triple_negative(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (a, b, c) = (-111, -222, -333);
    let triple = contract.construct_tuple_triple(a, b, c)?;
    let extracted_a = contract.tuple_triple_get_first(triple)?;
    let extracted_b = contract.tuple_triple_get_second(triple)?;
    let extracted_c = contract.tuple_triple_get_third(triple)?;
    Ok(extracted_a == a && extracted_b == b && extracted_c == c)
}

fn test_triple_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let triples: Vec<_> = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)]
        .into_iter()
        .map(|(a, b, c)| contract.construct_tuple_triple(a, b, c))
        .collect::<Result<Vec<_>, _>>()?;

    for t in &triples {
        if t.type_id != 13 {
            return Ok(false);
        }
    }
    Ok(true)
}

fn test_mixed_tuples(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Construct a pair and triple and verify their types
    let pair = contract.construct_tuple_pair(10, 20)?;
    let triple = contract.construct_tuple_triple(30, 40, 50)?;

    let pair_first = contract.tuple_pair_get_first(pair)?;
    let triple_first = contract.tuple_triple_get_first(triple)?;

    Ok(pair.type_id == 12 && triple.type_id == 13 && pair_first == 10 && triple_first == 30)
}
