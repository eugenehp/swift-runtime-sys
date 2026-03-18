/// Contract derivation probe for Track L.3.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Contract Derivation from Swift Source (Track L.3) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        ("Derive struct contract includes kind/name", test_derive_struct),
        ("Derive class contract includes class kind", test_derive_class),
        ("Derive protocol contract includes protocol kind", test_derive_protocol),
        ("Validate derived vs matching handwritten", test_validate_matching),
        ("Validate derived vs mismatched handwritten fails", test_validate_mismatch),
        ("Exporter macro sim contains annotation", test_macro_contains_annotation),
        ("Exporter macro sim includes type name", test_macro_contains_name),
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

    println!("\n=== Track L.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track L.3 tests PASSED");
    } else {
        panic!("✗ Track L.3 tests FAILED");
    }
}

fn test_derive_struct(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = contract.l3_derive_contract_from_source("struct User { let id: Int }")?;
    Ok(derived.contains("\"kind\":\"struct\"") && derived.contains("\"name\":\"User\""))
}

fn test_derive_class(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = contract.l3_derive_contract_from_source("class C {}")?;
    Ok(derived.contains("\"kind\":\"class\"") && derived.contains("\"name\":\"C\""))
}

fn test_derive_protocol(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = contract.l3_derive_contract_from_source("protocol P {}")?;
    Ok(derived.contains("\"kind\":\"protocol\"") && derived.contains("\"name\":\"P\""))
}

fn test_validate_matching(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = contract.l3_derive_contract_from_source("struct Box {}")?;
    let handwritten = "{\"name\":\"Box\",\"kind\":\"struct\"}";
    contract.l3_validate_derived_contract(&derived, handwritten)
}

fn test_validate_mismatch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = contract.l3_derive_contract_from_source("class Thing {}")?;
    let handwritten = "{\"name\":\"Thing\",\"kind\":\"struct\"}";
    let valid = contract.l3_validate_derived_contract(&derived, handwritten)?;
    Ok(!valid)
}

fn test_macro_contains_annotation(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let s = contract.l3_exporter_macro_sim("Widget")?;
    Ok(s.contains("@ContractExport") && s.contains("witness"))
}

fn test_macro_contains_name(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.l3_exporter_macro_sim("Widget")?.contains("Widget"))
}
