/// Foundation Data / UUID / CharacterSet probe for Track I.2.
/// Tests byte-sum checksum, UUID generation/parsing/roundtrip, and CharacterSet membership.
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

    println!("\n=== Foundation Data / UUID / CharacterSet (Track I.2) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        ("Data empty checksum = 0", test_data_empty),
        ("Data [1,2,3,4] checksum = 10", test_data_sum),
        ("UUID new string length = 36", test_uuid_new_length),
        ("UUID new string contains '-'", test_uuid_new_dashes),
        ("UUID parse valid RFC 4122", test_uuid_parse_valid),
        ("UUID parse invalid string → false", test_uuid_parse_invalid),
        ("UUID round-trip consistent", test_uuid_roundtrip),
        ("CharacterSet: 'A' (65) is letter", test_charset_letter),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track I.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track I.2 tests PASSED");
    } else {
        panic!("✗ Track I.2 tests FAILED");
    }
}

fn test_data_empty(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.data_from_bytes_checksum(&[])? == 0)
}

fn test_data_sum(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 1 + 2 + 3 + 4 = 10
    Ok(contract.data_from_bytes_checksum(&[1u8, 2, 3, 4])? == 10)
}

fn test_uuid_new_length(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // UUID string format: 8-4-4-4-12 plus four '-' = 36 characters
    Ok(contract.uuid_new_string()?.len() == 36)
}

fn test_uuid_new_dashes(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.uuid_new_string()?.contains('-'))
}

fn test_uuid_parse_valid(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Well-known RFC 4122 UUID
    contract.uuid_parse_validate("6BA7B810-9DAD-11D1-80B4-00C04FD430C8")
}

fn test_uuid_parse_invalid(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let valid = contract.uuid_parse_validate("not-a-uuid")?;
    Ok(!valid)
}

fn test_uuid_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.uuid_roundtrip()
}

fn test_charset_letter(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 'A' = codepoint 65 is a letter
    contract.charset_is_letter(65)
}
