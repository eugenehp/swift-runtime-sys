/// Generic protocol witness lookup probe for Track H.2.
/// Tests generic protocol support and generic subscript/lookup behavior.
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

    println!("\n=== Generic Protocol Witness Lookup (Track H.2) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 8] = [
        (
            "Array<Int32> Sequence support",
            test_array_i32_sequence_supported,
        ),
        ("Array<Int32> subscript index 0", test_array_i32_subscript_0),
        ("Array<Int32> subscript index 3", test_array_i32_subscript_3),
        (
            "Array<Int32> witness token non-zero",
            test_array_i32_witness_token,
        ),
        (
            "Dictionary<String,Int32> support",
            test_dict_string_i32_supported,
        ),
        (
            "Dictionary<String,Int32> lookup alpha",
            test_dict_string_i32_lookup_alpha,
        ),
        (
            "Dictionary<String,Int32> lookup gamma",
            test_dict_string_i32_lookup_gamma,
        ),
        (
            "Dictionary<String,Int32> missing key errors",
            test_dict_string_i32_lookup_missing,
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

    println!("\n=== Track H.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track H.2 tests PASSED");
    } else {
        panic!("✗ Track H.2 tests FAILED");
    }
}

fn test_array_i32_sequence_supported(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.generic_protocol_array_i32_sequence_supported()
}

fn test_array_i32_subscript_0(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.generic_protocol_array_i32_subscript(0)? == 10)
}

fn test_array_i32_subscript_3(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.generic_protocol_array_i32_subscript(3)? == 40)
}

fn test_array_i32_witness_token(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.generic_protocol_array_i32_witness_token()? != 0)
}

fn test_dict_string_i32_supported(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.generic_protocol_dict_string_i32_supported()
}

fn test_dict_string_i32_lookup_alpha(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.generic_protocol_dict_string_i32_lookup("alpha")? == 101)
}

fn test_dict_string_i32_lookup_gamma(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.generic_protocol_dict_string_i32_lookup("gamma")? == 303)
}

fn test_dict_string_i32_lookup_missing(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract
        .generic_protocol_dict_string_i32_lookup("missing")
        .is_err())
}
