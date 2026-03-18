/// Error handling and introspection probe for Track E.1.
/// Tests: Error creation, description extraction, type identity, code extraction, clearing.
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

    // ValidationError Tests
    println!("\n=== ValidationError Tests ===");

    // Test 1: Create ValidationError
    match test_error_make_validation(&contract) {
        Ok(true) => {
            println!("✓ Create ValidationError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Create ValidationError FAIL");
            tests_failed += 1;
        }
    }

    // Test 2: Get error description
    match test_error_get_description(&contract) {
        Ok(true) => {
            println!("✓ Get error description PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Get error description FAIL");
            tests_failed += 1;
        }
    }

    // Test 3: Get error code
    match test_error_get_code(&contract) {
        Ok(true) => {
            println!("✓ Get error code PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Get error code FAIL");
            tests_failed += 1;
        }
    }

    // Test 4: Type identity - ValidationError
    match test_error_is_validation(&contract) {
        Ok(true) => {
            println!("✓ Type identity ValidationError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Type identity ValidationError FAIL");
            tests_failed += 1;
        }
    }

    // Test 5: Type identity - Not IOError
    match test_error_not_io(&contract) {
        Ok(true) => {
            println!("✓ Type identity Not IOError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Type identity Not IOError FAIL");
            tests_failed += 1;
        }
    }

    // Test 6: ValidationError round-trip
    match test_error_validation_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ ValidationError round-trip (create, check type, extract code) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ ValidationError round-trip (create, check type, extract code) FAIL");
            tests_failed += 1;
        }
    }

    // Test 7: Clear ValidationError
    match test_error_clear(&contract) {
        Ok(true) => {
            println!("✓ Clear error PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Clear error FAIL");
            tests_failed += 1;
        }
    }

    // Test 8: OutOfRange ValidationError
    match test_error_out_of_range(&contract) {
        Ok(true) => {
            println!("✓ OutOfRange ValidationError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ OutOfRange ValidationError FAIL");
            tests_failed += 1;
        }
    }

    // IOError Tests
    println!("\n=== IOError Tests ===");

    // Test 9: Create IOError
    match test_error_make_io(&contract) {
        Ok(true) => {
            println!("✓ Create IOError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Create IOError FAIL");
            tests_failed += 1;
        }
    }

    // Test 10: IOError type identity
    match test_error_is_io(&contract) {
        Ok(true) => {
            println!("✓ Type identity IOError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Type identity IOError FAIL");
            tests_failed += 1;
        }
    }

    // Test 11: IOError is not ValidationError
    match test_error_io_not_validation(&contract) {
        Ok(true) => {
            println!("✓ IOError not ValidationError PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ IOError not ValidationError FAIL");
            tests_failed += 1;
        }
    }

    // Test 12: IOError description and code
    match test_error_io_description_code(&contract) {
        Ok(true) => {
            println!("✓ IOError description and code PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ IOError description and code FAIL");
            tests_failed += 1;
        }
    }

    // Test 13: IOError round-trip
    match test_error_io_roundtrip(&contract) {
        Ok(true) => {
            println!("✓ IOError round-trip (create, check type, extract, clear) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ IOError round-trip (create, check type, extract, clear) FAIL");
            tests_failed += 1;
        }
    }

    // Cross-Error Tests
    println!("\n=== Cross-Error Type Tests ===");

    // Test 14: Switch from ValidationError to IOError
    match test_error_type_switch(&contract) {
        Ok(true) => {
            println!("✓ Error type switching PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Error type switching FAIL");
            tests_failed += 1;
        }
    }

    // Test 15: Error sequence with clear
    match test_error_sequence(&contract) {
        Ok(true) => {
            println!("✓ Error sequence with clears PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Error sequence with clears FAIL");
            tests_failed += 1;
        }
    }

    // Summary
    println!("\n=== Error Introspection Probe Summary ===");
    println!("Tests Passed: {}", tests_passed);
    println!("Tests Failed: {}", tests_failed);

    if tests_failed == 0 {
        println!("✓ All error tests PASSED (Track E.1 complete)");
    } else {
        panic!("✗ Error tests FAILED");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ValidationError Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_error_make_validation(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_make_validation(42)?;
    Ok(true)
}

fn test_error_get_description(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_validation(99)?;
    let desc = contract.error_get_description()?;
    // Should be "Validation failed with code 99"
    Ok(desc.contains("Validation failed") && desc.contains("99"))
}

fn test_error_get_code(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_validation(77)?;
    let code = contract.error_get_code()?;
    Ok(code == 77)
}

fn test_error_is_validation(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_validation(100)?;
    let is_validation = contract.error_is_validation()?;
    Ok(is_validation)
}

fn test_error_not_io(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_validation(200)?;
    let is_io = contract.error_is_io()?;
    Ok(!is_io) // Should NOT be an IOError
}

fn test_error_validation_roundtrip(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let code = 55;
    contract.error_clear()?;
    contract.error_make_validation(code)?;
    let is_validation = contract.error_is_validation()?;
    let extracted_code = contract.error_get_code()?;
    Ok(is_validation && extracted_code == code)
}

fn test_error_clear(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_validation(42)?;
    contract.error_clear()?;

    // After clearing, getting description should fail
    let desc_result = contract.error_get_description();
    Ok(desc_result.is_err())
}

fn test_error_out_of_range(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_out_of_range(88, 1000)?;
    let is_validation = contract.error_is_validation()?;
    let code = contract.error_get_code()?;
    let desc = contract.error_get_description()?;
    Ok(is_validation && code == 88 && desc.contains("out of range"))
}

// ─────────────────────────────────────────────────────────────────────────────
// IOError Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_error_make_io(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_io(404)?;
    Ok(true)
}

fn test_error_is_io(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_io(500)?;
    let is_io = contract.error_is_io()?;
    Ok(is_io)
}

fn test_error_io_not_validation(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_io(403)?;
    let is_validation = contract.error_is_validation()?;
    Ok(!is_validation) // Should NOT be a ValidationError
}

fn test_error_io_description_code(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.error_clear()?;
    contract.error_make_io(413)?;
    let desc = contract.error_get_description()?;
    let code = contract.error_get_code()?;
    Ok(desc.contains("File not found") && code == 413)
}

fn test_error_io_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let code = 505;
    contract.error_clear()?;
    contract.error_make_io(code)?;
    let is_io = contract.error_is_io()?;
    let extracted_code = contract.error_get_code()?;
    let desc = contract.error_get_description()?;
    contract.error_clear()?;
    Ok(is_io && extracted_code == code && desc.contains("File not found"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Cross-Error Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_error_type_switch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // First create a ValidationError
    contract.error_clear()?;
    contract.error_make_validation(111)?;
    let is_validation_first = contract.error_is_validation()?;
    let code_first = contract.error_get_code()?;

    // Then switch to IOError
    contract.error_clear()?;
    contract.error_make_io(222)?;
    let is_io_second = contract.error_is_io()?;
    let code_second = contract.error_get_code()?;

    Ok(is_validation_first && code_first == 111 && is_io_second && code_second == 222)
}

fn test_error_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let codes = vec![10, 20, 30, 40, 50];

    for code in codes {
        contract.error_clear()?;
        contract.error_make_validation(code)?;
        let extracted_code = contract.error_get_code()?;
        if extracted_code != code {
            return Ok(false);
        }
    }

    contract.error_clear()?;
    Ok(true)
}
