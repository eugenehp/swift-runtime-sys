/// Foundation NSCoding / NSCopying probe for Track I.4.
/// Tests NSKeyedArchiver round-trips and NSCopying object independence.
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

    println!("\n=== Foundation NSCoding / NSCopying (Track I.4) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 5] = [
        ("NSCoding integer round-trip 42 = 42", test_integer_42),
        ("NSCoding integer round-trip -999 = -999", test_integer_neg),
        ("NSCoding string 'hello' length = 5", test_string_hello),
        ("NSCoding string 'swift' length = 5", test_string_swift),
        ("NSCopying array mutation independence", test_copy_independence),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track I.4 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track I.4 tests PASSED");
    } else {
        panic!("✗ Track I.4 tests FAILED");
    }
}

fn test_integer_42(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_integer_roundtrip(42)? == 42)
}

fn test_integer_neg(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_integer_roundtrip(-999)? == -999)
}

fn test_string_hello(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // "hello" has 5 characters
    Ok(contract.nscoding_string_roundtrip("hello")? == 5)
}

fn test_string_swift(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_string_roundtrip("swift")? == 5)
}

fn test_copy_independence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.nscopying_array_independence()
}
