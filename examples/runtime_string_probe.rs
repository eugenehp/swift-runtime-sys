/// String construction and UTF-8 validation probe for Track C.1.
/// Tests: empty strings, ASCII, UTF-8 multibyte, null-termination safety.
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

    // Test 1: Empty String
    match test_empty_string(&contract) {
        Ok(true) => {
            println!("✓ Empty string test PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Empty string test FAIL");
            tests_failed += 1;
        }
    }

    // Test 2: ASCII String
    match test_ascii_string(&contract) {
        Ok(true) => {
            println!("✓ ASCII string test PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ ASCII string test FAIL");
            tests_failed += 1;
        }
    }

    // Test 3: UTF-8 Multibyte String
    match test_utf8_multibyte(&contract) {
        Ok(true) => {
            println!("✓ UTF-8 multibyte test PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ UTF-8 multibyte test FAIL");
            tests_failed += 1;
        }
    }

    // Test 4: String Length Validation
    match test_string_length(&contract) {
        Ok(true) => {
            println!("✓ String length test PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ String length test FAIL");
            tests_failed += 1;
        }
    }

    // Test 5: Buffer Truncation Safety
    match test_buffer_truncation(&contract) {
        Ok(true) => {
            println!("✓ Buffer truncation safety test PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Buffer truncation safety test FAIL");
            tests_failed += 1;
        }
    }

    println!();
    println!(
        "String parity probe results: {}/{} PASS",
        tests_passed,
        tests_passed + tests_failed
    );

    if tests_failed > 0 {
        std::process::exit(1);
    }
}

fn test_empty_string(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let _str_obj = contract.construct_string_owned(b"")?;
    Ok(true)
}

fn test_ascii_string(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let test_bytes = b"Hello, Rust!";
    let str_obj = contract.construct_string_owned(test_bytes)?;

    let len = contract.string_len(str_obj.as_object().object)?;
    if len != test_bytes.len() as i32 {
        eprintln!(
            "ASCII length mismatch: expected {}, got {}",
            test_bytes.len(),
            len
        );
        return Ok(false);
    }

    let mut buffer = vec![0u8; test_bytes.len()];
    let actual_count = contract.string_bytes(str_obj.as_object().object, &mut buffer)?;

    if actual_count != test_bytes.len() as i32 {
        eprintln!(
            "ASCII byte count mismatch: expected {}, got {}",
            test_bytes.len(),
            actual_count
        );
        return Ok(false);
    }

    if buffer != test_bytes {
        eprintln!("ASCII content mismatch");
        return Ok(false);
    }

    Ok(true)
}

fn test_utf8_multibyte(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // "Hello 世界" in UTF-8
    let test_str = "Hello 世界";
    let test_bytes = test_str.as_bytes();

    let str_obj = contract.construct_string_owned(test_bytes)?;

    let len = contract.string_len(str_obj.as_object().object)?;
    if len != test_bytes.len() as i32 {
        eprintln!(
            "UTF-8 length mismatch: expected {}, got {}",
            test_bytes.len(),
            len
        );
        return Ok(false);
    }

    let mut buffer = vec![0u8; test_bytes.len()];
    let actual_count = contract.string_bytes(str_obj.as_object().object, &mut buffer)?;

    if actual_count != test_bytes.len() as i32 {
        eprintln!(
            "UTF-8 byte count mismatch: expected {}, got {}",
            test_bytes.len(),
            actual_count
        );
        return Ok(false);
    }

    if buffer != test_bytes {
        eprintln!("UTF-8 content mismatch");
        return Ok(false);
    }

    // Verify round-trip string interpretation
    let reconstructed = String::from_utf8(buffer).expect("invalid utf8");
    if reconstructed != test_str {
        eprintln!(
            "UTF-8 round-trip mismatch: expected '{}', got '{}'",
            test_str, reconstructed
        );
        return Ok(false);
    }

    Ok(true)
}

fn test_string_length(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let test_cases = vec![("", 0), ("a", 1), ("hello", 5), ("hello world", 11)];

    for (test_str, expected_len) in test_cases {
        let str_obj = contract.construct_string_owned(test_str.as_bytes())?;
        let len = contract.string_len(str_obj.as_object().object)?;

        if len != expected_len {
            eprintln!(
                "Length test failed for '{}': expected {}, got {}",
                test_str, expected_len, len
            );
            return Ok(false);
        }
    }

    Ok(true)
}

fn test_buffer_truncation(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let test_bytes = b"0123456789ABCDEF";
    let str_obj = contract.construct_string_owned(test_bytes)?;

    // Try to read with a smaller buffer
    let mut small_buffer = vec![0u8; 5];
    let actual_count = contract.string_bytes(str_obj.as_object().object, &mut small_buffer)?;

    // Should return actual total count even though we truncated
    if actual_count != test_bytes.len() as i32 {
        eprintln!(
            "Truncation return value wrong: expected {}, got {}",
            test_bytes.len(),
            actual_count
        );
        return Ok(false);
    }

    // But the buffer should only contain what fits
    if small_buffer != &test_bytes[0..5] {
        eprintln!("Truncation buffer content wrong");
        return Ok(false);
    }

    Ok(true)
}
