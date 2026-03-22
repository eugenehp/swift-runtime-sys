use std::fs;
use std::path::Path;

use serde::Serialize;
use swift_runtime_sys::RuntimeContract::{
    N7DerivedCallable, N7DerivedContract, N7DerivedType, RuntimeContract, RuntimeContractError,
};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

#[derive(Debug, Serialize)]
struct N7ArtifactSummary {
    binary_path: String,
    module_hint: String,
    callable_count: usize,
    type_count: usize,
    confidence: f64,
    fallback_paths: Vec<String>,
    low_confidence_regions: Vec<String>,
}

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

    println!("\n=== Binary-Driven Contract Derivation (Track N.7) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        (
            "Binary symbol scan discovers callable candidates",
            test_symbol_scan_discovers_callables,
        ),
        (
            "Derived callables include contract surface symbols",
            test_callables_include_contract_symbols,
        ),
        (
            "Demangle stitch pipeline reconstructs readable surfaces",
            test_demangle_stitch_pipeline,
        ),
        (
            "Derived types include runtime-observed module types",
            test_types_include_module_types,
        ),
        (
            "Binary-derived contract artifact is written",
            test_derived_contract_artifact_written,
        ),
        (
            "Validator confirms callable live-observation coverage",
            test_validator_callable_coverage,
        ),
        (
            "Validator confirms type live-observation coverage",
            test_validator_type_coverage,
        ),
        (
            "Confidence scoring remains above promotion floor",
            test_confidence_scoring_floor,
        ),
        (
            "Low-confidence fallback regions are reported",
            test_low_confidence_fallback_reporting,
        ),
        (
            "Exit criterion bootstrap uses binary-only surfaces",
            test_exit_criterion_bootstrap_binary_only,
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

    println!("\n=== Track N.7 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.7 tests PASSED");
    } else {
        panic!("✗ Track N.7 tests FAILED");
    }
}

fn derive(contract: &RuntimeContract) -> Result<N7DerivedContract, RuntimeContractError> {
    contract.n7_derive_contract_from_binary("./libRustBridge.dylib", "RustBridge")
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> bool {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
    }
    fs::write(path, serde_json::to_string_pretty(value).unwrap()).is_ok()
}

fn test_symbol_scan_discovers_callables(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    Ok(!derived.callables.is_empty())
}

fn test_callables_include_contract_symbols(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    Ok(derived
        .callables
        .iter()
        .any(|callable| callable.symbol == "swift_contract_n1_enumerate_all_types_json")
        && derived
            .callables
            .iter()
            .any(|callable| callable.symbol == "swift_contract_n2_symbol_describe"))
}

fn test_demangle_stitch_pipeline(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    Ok(derived
        .callables
        .iter()
        .any(|callable| callable.symbol.starts_with("$s") && callable.demangled != callable.symbol))
}

fn test_types_include_module_types(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    Ok(derived
        .types
        .iter()
        .any(|entry| entry.name.contains("Person"))
        || derived
            .types
            .iter()
            .any(|entry| entry.name.contains("Counter")))
}

fn test_derived_contract_artifact_written(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    let out_path = Path::new("target/runtime-probe/n7-derived-contract.json");
    let summary = N7ArtifactSummary {
        binary_path: derived.binary_path.clone(),
        module_hint: derived.module_hint.clone(),
        callable_count: derived.callables.len(),
        type_count: derived.types.len(),
        confidence: derived.confidence,
        fallback_paths: derived.fallback_paths.clone(),
        low_confidence_regions: derived.low_confidence_regions.clone(),
    };
    Ok(write_json(out_path, &derived)
        && write_json(
            Path::new("target/runtime-probe/n7-derived-summary.json"),
            &summary,
        )
        && out_path.exists())
}

fn test_validator_callable_coverage(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    let report = contract.n7_validate_derived_contract(&derived)?;
    Ok(report.callable_coverage >= 0.7 && report.validated_callables > 0)
}

fn test_validator_type_coverage(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    let report = contract.n7_validate_derived_contract(&derived)?;
    Ok(report.type_coverage >= 0.7 && report.validated_types > 0)
}

fn test_confidence_scoring_floor(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    let report = contract.n7_validate_derived_contract(&derived)?;

    let md = format!(
        "# N.7 Confidence Report\n\n- derived_confidence: {:.3}\n- validated_confidence: {:.3}\n- callable_coverage: {:.3}\n- type_coverage: {:.3}\n- low_confidence_regions: {}\n",
        derived.confidence,
        report.confidence,
        report.callable_coverage,
        report.type_coverage,
        if report.low_confidence_regions.is_empty() {
            "none".to_string()
        } else {
            report.low_confidence_regions.join(",")
        }
    );
    let out_path = Path::new("target/runtime-probe/n7-confidence-report.md");
    let _ = fs::create_dir_all("target/runtime-probe");
    let _ = fs::write(out_path, md);

    Ok(report.confidence >= 0.65)
}

fn synthetic_low_conf_contract(base: &N7DerivedContract) -> N7DerivedContract {
    let mut cloned = base.clone();
    cloned.callables.push(N7DerivedCallable {
        symbol: "swift_contract_n2_unknown_does_not_exist".to_string(),
        demangled: "missing".to_string(),
        observed_runtime: false,
        shape: None,
        confidence: 0.15,
        fallback: "fallback to runtime metadata".to_string(),
    });
    cloned.types.push(N7DerivedType {
        name: "RustBridge.NonexistentType".to_string(),
        kind: "unknown".to_string(),
        field_count: -1,
        observed_runtime: false,
        confidence: 0.15,
        fallback: "fallback to n1_enumerate_all_types_json".to_string(),
    });
    cloned
        .low_confidence_regions
        .push("synthetic:missing-region".to_string());
    cloned
}

fn test_low_confidence_fallback_reporting(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    let synthetic = synthetic_low_conf_contract(&derived);
    let report = contract.n7_validate_derived_contract(&synthetic)?;
    Ok(!report.low_confidence_regions.is_empty() || report.confidence < 0.65)
}

fn test_exit_criterion_bootstrap_binary_only(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let derived = derive(contract)?;
    let report = contract.n7_validate_derived_contract(&derived)?;
    let has_dynamic_unknown = derived.callables.iter().any(|callable| {
        callable.symbol == "swift_contract_n2_unknown_add_offset" && callable.shape.is_some()
    });
    Ok(has_dynamic_unknown
        && report.callable_coverage >= 0.7
        && report.type_coverage >= 0.7
        && report.confidence >= 0.65)
}
