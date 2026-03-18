/// Structured error propagation probe for Track E.3.
/// Tests: context container fields, JSON/string serialization, and JSON round-trip restore.
use serde_json::Value;
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

    println!("\n=== Structured Error Propagation (Track E.3) ===");

    for (name, test_fn) in [
        (
            "Construct validation context",
            test_construct_validation_context
                as fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
        ),
        (
            "Validation context JSON fields",
            test_validation_json_fields,
        ),
        (
            "Validation context chain integrity",
            test_validation_chain_integrity,
        ),
        (
            "Validation user_info and hints",
            test_validation_user_info_hints,
        ),
        (
            "Validation string serialization",
            test_validation_string_serialization,
        ),
        ("Clear context semantics", test_context_clear_semantics),
        ("Construct IO context", test_construct_io_context),
        ("IO context JSON fields", test_io_json_fields),
        ("JSON round-trip restore", test_json_roundtrip_restore),
        ("Context type switching", test_context_type_switching),
    ] {
        match test_fn(&contract) {
            Ok(true) => {
                println!("\u{2713} {name} PASS");
                tests_passed += 1;
            }
            Ok(false) => {
                println!("\u{2717} {name} FAIL");
                tests_failed += 1;
            }
            Err(e) => {
                println!("\u{2717} {name} FAIL ({e:?})");
                tests_failed += 1;
            }
        }
    }

    println!("\n=== Track E.3 Summary ===");
    println!("Tests Passed: {}", tests_passed);
    println!("Tests Failed: {}", tests_failed);

    if tests_failed == 0 {
        println!("\u{2713} All Track E.3 tests PASSED");
    } else {
        panic!("\u{2717} Track E.3 tests FAILED");
    }
}

fn test_construct_validation_context(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.error_context_clear()?;
    contract.error_context_make_validation(422, 1201)?;
    let parsed = contract.error_context_parse()?;
    Ok(parsed.domain == "ValidationError" && parsed.code == 422)
}

fn test_validation_json_fields(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(422, 1201)?;
    let payload = contract.error_context_parse()?;
    Ok(!payload.message.is_empty() && !payload.chain.is_empty())
}

fn test_validation_chain_integrity(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(422, 1201)?;
    let payload = contract.error_context_parse()?;
    Ok(payload.chain.len() >= 2
        && payload.chain[0].contains("ValidationError")
        && payload.chain[1].contains("ConstraintViolation"))
}

fn test_validation_user_info_hints(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(422, 1201)?;
    let payload = contract.error_context_parse()?;
    let has_field = payload
        .user_info
        .get("field")
        .map(|v| v == "age")
        .unwrap_or(false);
    let has_operation = payload
        .user_info
        .get("operation")
        .map(|v| v == "create_user")
        .unwrap_or(false);
    Ok(has_field && has_operation && payload.recovery_hints.len() >= 2)
}

fn test_validation_string_serialization(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(422, 1201)?;
    let text = contract.error_context_get_string()?;
    Ok(text.contains("ValidationError")
        && text.contains("create_user")
        && text.contains("recovery_hints"))
}

fn test_context_clear_semantics(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(422, 1201)?;
    contract.error_context_clear()?;
    Ok(contract.error_context_get_json().is_err())
}

fn test_construct_io_context(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_context_clear()?;
    contract.error_context_make_io(2)?;
    let parsed = contract.error_context_parse()?;
    Ok(parsed.domain == "IOError" && parsed.code == 2)
}

fn test_io_json_fields(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_io(2)?;
    let payload = contract.error_context_parse()?;
    let has_path = payload
        .user_info
        .get("path")
        .map(|v| v.contains("/tmp/runtime-probe"))
        .unwrap_or(false);
    let has_chain = payload.chain.iter().any(|entry| entry.contains("POSIX"));
    Ok(has_path && has_chain)
}

fn test_json_roundtrip_restore(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(409, 2001)?;
    let original = contract.error_context_get_json()?;

    contract.error_context_clear()?;
    contract.error_context_set_json(&original)?;
    let restored = contract.error_context_get_json()?;

    let original_json: Value = serde_json::from_str(&original)
        .map_err(|e| RuntimeContractError::DescriptorParse(e.to_string()))?;
    let restored_json: Value = serde_json::from_str(&restored)
        .map_err(|e| RuntimeContractError::DescriptorParse(e.to_string()))?;

    Ok(original_json == restored_json)
}

fn test_context_type_switching(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.error_context_make_validation(422, 1201)?;
    let first = contract.error_context_parse()?;

    contract.error_context_make_io(13)?;
    let second = contract.error_context_parse()?;

    Ok(first.domain == "ValidationError" && second.domain == "IOError" && second.code == 13)
}
