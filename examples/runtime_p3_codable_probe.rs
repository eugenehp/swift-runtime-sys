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

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== P.3 Codable / Serialization Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("Codable Int32 round-trip 42", test_codable_42),
        ("Codable Int32 round-trip -1001", test_codable_neg),
        ("Codable Int32 round-trip 0", test_codable_zero),
        ("NSCoding integer round-trip 123", test_nscoding_int),
        ("NSCoding integer round-trip -999", test_nscoding_int_neg),
        (
            "NSCoding string round-trip 'hello' length",
            test_nscoding_string_hello,
        ),
        (
            "NSCoding string round-trip 'swift-runtime' length",
            test_nscoding_string_swift_runtime,
        ),
        (
            "NSCopying array mutation independence",
            test_nscopying_independence,
        ),
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

    println!("\n=== P.3 Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!("p3 codable parity => codable_ok=1 nscoding_ok=1 nscopying_ok=1");

    if failed > 0 {
        eprintln!("\n✗ P.3 Codable probe FAILED");
        std::process::exit(1);
    } else {
        println!("\n✓ P.3 Codable probe PASSED");
    }
}

fn test_codable_42(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.constrained_codable_roundtrip(42)? == 42)
}

fn test_codable_neg(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.constrained_codable_roundtrip(-1001)? == -1001)
}

fn test_codable_zero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.constrained_codable_roundtrip(0)? == 0)
}

fn test_nscoding_int(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_integer_roundtrip(123)? == 123)
}

fn test_nscoding_int_neg(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_integer_roundtrip(-999)? == -999)
}

fn test_nscoding_string_hello(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_string_roundtrip("hello")? == 5)
}

fn test_nscoding_string_swift_runtime(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.nscoding_string_roundtrip("swift-runtime")? == 13)
}

fn test_nscopying_independence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.nscopying_array_independence()
}
