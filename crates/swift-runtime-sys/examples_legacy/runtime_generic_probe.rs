/// Generic metadata accessor and instantiation probe for Track H.1.
/// Tests generic metadata lookup, substitution validation, and deterministic specializations.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

const METADATA_ARRAY_I32: i32 = 4;
const METADATA_GENERIC_BOX_I32: i32 = 1001;
const METADATA_DICT_STRING_I32: i32 = 1002;

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

    println!("\n=== Generic Metadata Accessor Chains (Track H.1) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 9] = [
        ("Metadata lookup Array<Int32>", test_metadata_array_i32),
        (
            "Metadata lookup ContractGenericBox<Int32>",
            test_metadata_generic_box_i32,
        ),
        (
            "Metadata lookup Dictionary<String,Int32>",
            test_metadata_dict_string_i32,
        ),
        ("Substitution supports Int32", test_substitution_i32),
        ("Substitution supports Array<Int32>", test_substitution_array_i32),
        (
            "Substitution supports Dictionary<String,Int32>",
            test_substitution_dict_string_i32,
        ),
        ("Generic box round-trip", test_generic_box_roundtrip),
        ("Generic Array<Int32> instantiation", test_generic_array_sum),
        (
            "Generic Dictionary<String,Int32> instantiation",
            test_generic_dict_sum,
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

    println!("\n=== Track H.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track H.1 tests PASSED");
    } else {
        panic!("✗ Track H.1 tests FAILED");
    }
}

fn test_metadata_array_i32(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(!contract.lookup_metadata(METADATA_ARRAY_I32)?.is_null())
}

fn test_metadata_generic_box_i32(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(!contract.lookup_metadata(METADATA_GENERIC_BOX_I32)?.is_null())
}

fn test_metadata_dict_string_i32(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(!contract.lookup_metadata(METADATA_DICT_STRING_I32)?.is_null())
}

fn test_substitution_i32(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.generic_validate_substitution("Int32")
}

fn test_substitution_array_i32(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.generic_validate_substitution("Array<Int32>")
}

fn test_substitution_dict_string_i32(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    contract.generic_validate_substitution("Dictionary<String,Int32>")
}

fn test_generic_box_roundtrip(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let boxed = contract.construct_generic_box_i32(73)?;
    let value = contract.generic_box_i32_get(boxed)?;
    contract.release(boxed)?;
    Ok(value == 73)
}

fn test_generic_array_sum(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 5 + 6 + 7 + 8 = 26
    Ok(contract.generic_array_i32_sum(5, 4)? == 26)
}

fn test_generic_dict_sum(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // base + (base+1) + (base+2) => 3*base + 3
    let base = 11;
    Ok(contract.generic_dict_string_i32_sum(base)? == (3 * base + 3))
}
