/// Struct layout introspection and construction probe for Track F.1.
/// Tests: struct size/stride/alignment, field offset discovery, construction from bytes, field access.
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

    // Layout Introspection Tests
    println!("\n=== Struct Layout Introspection Tests ===");

    // Test 1: Get struct size
    match test_struct_size(&contract) {
        Ok(true) => {
            println!("✓ TestPayload size query PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload size query FAIL");
            tests_failed += 1;
        }
    }

    // Test 2: Get struct stride
    match test_struct_stride(&contract) {
        Ok(true) => {
            println!("✓ TestPayload stride query PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload stride query FAIL");
            tests_failed += 1;
        }
    }

    // Test 3: Get struct alignment
    match test_struct_alignment(&contract) {
        Ok(true) => {
            println!("✓ TestPayload alignment query PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload alignment query FAIL");
            tests_failed += 1;
        }
    }

    // Test 4: Get field_a offset
    match test_struct_offset_a(&contract) {
        Ok(true) => {
            println!("✓ TestPayload field_a offset PASS (offset=0)");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload field_a offset FAIL");
            tests_failed += 1;
        }
    }

    // Test 5: Get field_b offset
    match test_struct_offset_b(&contract) {
        Ok(true) => {
            println!("✓ TestPayload field_b offset PASS (offset=8)");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload field_b offset FAIL");
            tests_failed += 1;
        }
    }

    // Test 6: Get field_c offset
    match test_struct_offset_c(&contract) {
        Ok(true) => {
            println!("✓ TestPayload field_c offset PASS (offset=16)");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload field_c offset FAIL");
            tests_failed += 1;
        }
    }

    // Struct Construction Tests
    println!("\n=== Struct Construction & Field Access Tests ===");

    // Test 7: Construct struct with simple values
    match test_struct_construct_simple(&contract) {
        Ok(true) => {
            println!("✓ TestPayload construction (simple values) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload construction (simple values) FAIL");
            tests_failed += 1;
        }
    }

    // Test 8: Extract field_a from constructed struct
    match test_struct_extract_field_a(&contract) {
        Ok(true) => {
            println!("✓ TestPayload field_a extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload field_a extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 9: Extract field_b from constructed struct
    match test_struct_extract_field_b(&contract) {
        Ok(true) => {
            println!("✓ TestPayload field_b extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload field_b extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 10: Extract field_c from constructed struct
    match test_struct_extract_field_c(&contract) {
        Ok(true) => {
            println!("✓ TestPayload field_c extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload field_c extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 11: Round-trip (construct, extract all fields)
    match test_struct_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ TestPayload round-trip (construct, extract all) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload round-trip (construct, extract all) FAIL");
            tests_failed += 1;
        }
    }

    // Test 12: Multiple struct instances in sequence
    match test_struct_multiple_sequence(&contract) {
        Ok(true) => {
            println!("✓ Multiple struct construction sequence PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Multiple struct construction sequence FAIL");
            tests_failed += 1;
        }
    }

    // Test 13: Struct with large Int64 field
    match test_struct_large_int64(&contract) {
        Ok(true) => {
            println!("✓ TestPayload with large Int64 value PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload with large Int64 value FAIL");
            tests_failed += 1;
        }
    }

    // Test 14: Struct with negative values
    match test_struct_negative_values(&contract) {
        Ok(true) => {
            println!("✓ TestPayload with negative values PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ TestPayload with negative values FAIL");
            tests_failed += 1;
        }
    }

    // Cleanup and summary
    println!("\n=== Struct Layout Introspection Probe Summary ===");
    println!("Tests Passed: {}", tests_passed);
    println!("Tests Failed: {}", tests_failed);

    if tests_failed == 0 {
        println!("✓ All struct introspection tests PASSED (Track F.1 complete)");
    } else {
        panic!("✗ Struct introspection tests FAILED");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Layout Introspection Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_struct_size(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let size = contract.struct_testpayload_size()?;
    // TestPayload: Int32 (4) + padding (4) + Int64 (8) + Int32 (4) = 20 bytes minimum
    // But actual size depends on Swift's layout rules
    println!("  [DEBUG] TestPayload size: {}", size);
    Ok(size > 0)
}

fn test_struct_stride(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let stride = contract.struct_testpayload_stride()?;
    println!("  [DEBUG] TestPayload stride: {}", stride);
    Ok(stride > 0)
}

fn test_struct_alignment(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let alignment = contract.struct_testpayload_alignment()?;
    println!("  [DEBUG] TestPayload alignment: {}", alignment);
    Ok(alignment > 0 && alignment <= 8)
}

fn test_struct_offset_a(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let offset = contract.struct_testpayload_offset_field_a()?;
    println!("  [DEBUG] field_a offset: {}", offset);
    Ok(offset == 0)
}

fn test_struct_offset_b(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let offset = contract.struct_testpayload_offset_field_b()?;
    println!("  [DEBUG] field_b offset: {}", offset);
    Ok(offset == 8)
}

fn test_struct_offset_c(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let offset = contract.struct_testpayload_offset_field_c()?;
    println!("  [DEBUG] field_c offset: {}", offset);
    Ok(offset == 16)
}

// ─────────────────────────────────────────────────────────────────────────────
// Struct Construction & Field Access Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_struct_construct_simple(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let payload = contract.construct_struct_testpayload(42, 123456789, 99)?;
    assert_eq!(payload.type_id, 11);
    Ok(true)
}

fn test_struct_extract_field_a(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let payload = contract.construct_struct_testpayload(42, 123456789, 99)?;
    let value = contract.struct_testpayload_get_field_a(payload)?;
    Ok(value == 42)
}

fn test_struct_extract_field_b(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let payload = contract.construct_struct_testpayload(42, 123456789, 99)?;
    let value = contract.struct_testpayload_get_field_b(payload)?;
    Ok(value == 123456789)
}

fn test_struct_extract_field_c(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let payload = contract.construct_struct_testpayload(42, 123456789, 99)?;
    let value = contract.struct_testpayload_get_field_c(payload)?;
    Ok(value == 99)
}

fn test_struct_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (a, b, c) = (17, 9876543210i64, 88);
    let payload = contract.construct_struct_testpayload(a, b, c)?;

    let extracted_a = contract.struct_testpayload_get_field_a(payload)?;
    let extracted_b = contract.struct_testpayload_get_field_b(payload)?;
    let extracted_c = contract.struct_testpayload_get_field_c(payload)?;

    Ok(extracted_a == a && extracted_b == b && extracted_c == c)
}

fn test_struct_multiple_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Construct multiple structs in sequence
    let structs: Vec<_> = vec![(10, 100i64, 1), (20, 200i64, 2), (30, 300i64, 3)]
        .into_iter()
        .map(|(a, b, c)| contract.construct_struct_testpayload(a, b, c))
        .collect::<Result<Vec<_>, _>>()?;

    // Verify all were constructed
    for s in &structs {
        if s.type_id != 11 {
            return Ok(false);
        }
    }

    Ok(true)
}

fn test_struct_large_int64(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let large_value = 9223372036854775807i64; // i64::MAX
    let payload = contract.construct_struct_testpayload(0, large_value, 0)?;
    let extracted = contract.struct_testpayload_get_field_b(payload)?;
    Ok(extracted == large_value)
}

fn test_struct_negative_values(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (a, b, c) = (-99, -888888i64, -77);
    let payload = contract.construct_struct_testpayload(a, b, c)?;

    let extracted_a = contract.struct_testpayload_get_field_a(payload)?;
    let extracted_b = contract.struct_testpayload_get_field_b(payload)?;
    let extracted_c = contract.struct_testpayload_get_field_c(payload)?;

    Ok(extracted_a == a && extracted_b == b && extracted_c == c)
}
