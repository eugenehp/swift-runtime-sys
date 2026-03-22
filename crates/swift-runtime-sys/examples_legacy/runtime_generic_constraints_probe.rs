/// Constrained generic bounds probe for Track H.3.
/// Tests generic functions and types bounded by Equatable, Comparable,
/// Hashable, AdditiveArithmetic, Codable, and multi-constraint combinations.
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

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Constrained Generic Bounds (Track H.3) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 10] = [
        ("Equatable: equal values", test_equatable_equal),
        ("Equatable: different values", test_equatable_not_equal),
        ("Comparable: less-than", test_comparable_lt),
        ("Comparable: greater-than", test_comparable_gt),
        ("Comparable: equal comparison", test_comparable_eq),
        ("Hashable: all distinct", test_hashable_distinct_all),
        ("Hashable: duplicate collapses", test_hashable_distinct_dup),
        ("AdditiveArithmetic: sum", test_additive_sum),
        ("Codable: round-trip", test_codable_roundtrip),
        ("Multi-constraint (Comparable & Hashable): min", test_multi_min),
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

    println!("\n=== Track H.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track H.3 tests PASSED");
    } else {
        panic!("✗ Track H.3 tests FAILED");
    }
}

// --- Equatable constraint ---

fn test_equatable_equal(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 42 == 42 → 1
    let result = contract.constrained_equatable_equal(42, 42)?;
    Ok(result == 1)
}

fn test_equatable_not_equal(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 42 != 99 → 0
    let result = contract.constrained_equatable_equal(42, 99)?;
    Ok(result == 0)
}

// --- Comparable constraint ---

fn test_comparable_lt(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 5 < 10 → -1
    let result = contract.constrained_comparable_cmp(5, 10)?;
    Ok(result == -1)
}

fn test_comparable_gt(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 10 > 5 → 1
    let result = contract.constrained_comparable_cmp(10, 5)?;
    Ok(result == 1)
}

fn test_comparable_eq(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 7 == 7 → 0
    let result = contract.constrained_comparable_cmp(7, 7)?;
    Ok(result == 0)
}

// --- Hashable constraint ---

fn test_hashable_distinct_all(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // {1, 2, 3} → 3 distinct
    let result = contract.constrained_hashable_distinct_count(1, 2, 3)?;
    Ok(result == 3)
}

fn test_hashable_distinct_dup(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // {5, 5, 9} → 2 distinct (duplicate collapses)
    let result = contract.constrained_hashable_distinct_count(5, 5, 9)?;
    Ok(result == 2)
}

// --- AdditiveArithmetic constraint ---

fn test_additive_sum(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 100 + 23 = 123
    let result = contract.constrained_additive_sum(100, 23)?;
    Ok(result == 123)
}

// --- Codable constraint ---

fn test_codable_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // JSON encode then decode 12345 → should preserve value exactly
    let result = contract.constrained_codable_roundtrip(12345)?;
    Ok(result == 12345)
}

// --- Multi-constraint (Comparable & Hashable) ---

fn test_multi_min(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // min(17, 42) = 17
    let result = contract.constrained_multi_min(17, 42)?;
    Ok(result == 17)
}
