/// Runtime generic/witness instantiation probe for Track N.3.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Arbitrary Generic/Witness Instantiation (Track N.3) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 21] = [
        (
            "Build context: unconstrained generic box",
            test_build_context_unconstrained_box,
        ),
        (
            "Build context: constrained Int32",
            test_build_context_constrained_i32,
        ),
        (
            "Build context: derived ContractGenericBox<String>",
            test_build_context_box_string,
        ),
        (
            "H.1 rejects Array<String> but N.3 supports it",
            test_n3_beyond_h1_array_string,
        ),
        (
            "Witness resolve: Array<Int32> Sequence",
            test_witness_array_sequence,
        ),
        (
            "Witness resolve: Dictionary<String,Int32> Sequence",
            test_witness_dict_sequence,
        ),
        (
            "Witness resolve: Array<String> Sequence",
            test_witness_array_string_sequence,
        ),
        (
            "Witness resolve: nested Array<Array<Int32>> Sequence",
            test_witness_nested_array_sequence,
        ),
        (
            "Witness resolve: unsupported Int32 Sequence",
            test_witness_unsupported_sequence,
        ),
        (
            "Requirement solver: Array Element==Int32 passes",
            test_requirements_array_pass,
        ),
        (
            "Requirement solver: Array Element==String fails",
            test_requirements_array_fail_assoc,
        ),
        (
            "Requirement solver: Int32 Sequence fails",
            test_requirements_i32_fail_protocol,
        ),
        (
            "Requirement solver: Dictionary<String,String> passes",
            test_requirements_dict_string_string_pass,
        ),
        ("Dispatch generic box make/get", test_dispatch_box_make_get),
        (
            "Dispatch derived generic box String metric",
            test_dispatch_box_string_metric,
        ),
        ("Dispatch constrained Equatable op", test_dispatch_equatable),
        ("Dispatch constrained Additive op", test_dispatch_additive),
        (
            "Dispatch Array<Int32> Sequence op",
            test_dispatch_array_sequence,
        ),
        (
            "Dispatch Array<String> Sequence op",
            test_dispatch_array_string_sequence,
        ),
        (
            "Dispatch Dictionary<String,Int32> Sequence op",
            test_dispatch_dict_sequence,
        ),
        (
            "Dispatch Dictionary<String,String> Sequence op",
            test_dispatch_dict_string_string_sequence,
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

    println!("\n=== Track N.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.3 tests PASSED");
    } else {
        panic!("✗ Track N.3 tests FAILED");
    }
}

fn test_build_context_unconstrained_box(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_build_context_json("ContractGenericBox<Int32>", "")?;
    Ok(json.contains("\"supported\":true")
        && json.contains("\"generic_base\":\"ContractGenericBox\"")
        && json.contains("\"Int32\""))
}

fn test_build_context_constrained_i32(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_build_context_json("Int32", "Equatable;Comparable")?;
    Ok(json.contains("\"supported\":true")
        && json.contains("\"name\":\"Equatable\",\"satisfied\":true")
        && json.contains("\"name\":\"Comparable\",\"satisfied\":true"))
}

fn test_build_context_box_string(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_build_context_json("ContractGenericBox<String>", "")?;
    Ok(json.contains("\"supported\":true")
        && json.contains("\"generic_base\":\"ContractGenericBox\"")
        && json.contains("\"String\""))
}

fn test_n3_beyond_h1_array_string(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let h1_supported = contract.generic_validate_substitution("Array<String>")?;
    let n3_json = contract.n3_build_context_json("Array<String>", "Sequence;Element==String")?;
    Ok(!h1_supported && n3_json.contains("\"supported\":true"))
}

fn test_witness_array_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_resolve_witness_json("Array<Int32>", "Sequence", "Element==Int32")?;
    Ok(json.contains("\"supported\":true") && !json.contains("\"token\":0"))
}

fn test_witness_dict_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_resolve_witness_json(
        "Dictionary<String,Int32>",
        "Sequence",
        "Key==String;Value==Int32",
    )?;
    Ok(json.contains("\"supported\":true") && !json.contains("\"token\":0"))
}

fn test_witness_array_string_sequence(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_resolve_witness_json("Array<String>", "Sequence", "Element==String")?;
    Ok(json.contains("\"supported\":true") && !json.contains("\"token\":0"))
}

fn test_witness_nested_array_sequence(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_resolve_witness_json(
        "Array<Array<Int32>>",
        "Sequence",
        "Element==Array<Int32>",
    )?;
    Ok(json.contains("\"supported\":true") && !json.contains("\"token\":0"))
}

fn test_witness_unsupported_sequence(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_resolve_witness_json("Int32", "Sequence", "Element==Int32")?;
    Ok(json.contains("\"supported\":false") && json.contains("unsupported_protocol"))
}

fn test_requirements_array_pass(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_validate_requirements_json("Array<Int32>", "Sequence;Element==Int32")?;
    Ok(json.contains("\"supported\":true") && json.contains("\"failures\":[]"))
}

fn test_requirements_array_fail_assoc(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json =
        contract.n3_validate_requirements_json("Array<Int32>", "Sequence;Element==String")?;
    Ok(json.contains("\"supported\":false") && json.contains("associated_type_mismatch"))
}

fn test_requirements_i32_fail_protocol(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_validate_requirements_json("Int32", "Sequence")?;
    Ok(json.contains("\"supported\":false") && json.contains("unsupported_protocol"))
}

fn test_requirements_dict_string_string_pass(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let json = contract.n3_validate_requirements_json(
        "Dictionary<String,String>",
        "Sequence;Key==String;Value==String",
    )?;
    Ok(json.contains("\"supported\":true") && json.contains("\"failures\":[]"))
}

fn test_dispatch_box_make_get(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(
        contract.n3_invoke_generic_i32("ContractGenericBox<Int32>", "", "box_make_get", 73, 0)?
            == 73,
    )
}

fn test_dispatch_box_string_metric(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.n3_invoke_generic_i32(
        "ContractGenericBox<String>",
        "",
        "box.sample_metric",
        12,
        0,
    )? == 12)
}

fn test_dispatch_equatable(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n3_invoke_generic_i32("Int32", "Equatable", "equatable.equal", 9, 9)? == 1)
}

fn test_dispatch_additive(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(
        contract.n3_invoke_generic_i32("Int32", "AdditiveArithmetic", "additive.sum", 100, 23)?
            == 123,
    )
}

fn test_dispatch_array_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n3_invoke_generic_i32(
        "Array<Int32>",
        "Sequence;Element==Int32",
        "sequence.sum_range",
        5,
        4,
    )? == 26)
}

fn test_dispatch_array_string_sequence(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.n3_invoke_generic_i32(
        "Array<String>",
        "Sequence;Element==String",
        "sequence.sample_metric",
        3,
        4,
    )? == 18)
}

fn test_dispatch_dict_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n3_invoke_generic_i32(
        "Dictionary<String,Int32>",
        "Sequence;Key==String;Value==Int32",
        "sequence.sum_values",
        11,
        0,
    )? == 36)
}

fn test_dispatch_dict_string_string_sequence(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.n3_invoke_generic_i32(
        "Dictionary<String,String>",
        "Sequence;Key==String;Value==String",
        "sequence.sample_metric",
        5,
        0,
    )? == 18)
}
