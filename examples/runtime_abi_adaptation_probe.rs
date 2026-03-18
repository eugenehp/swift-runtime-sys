use std::fs;
use std::path::Path;

use serde::Serialize;
use swift_runtime_sys::RuntimeContract::{
    N5AdapterTable, N5DriftReport, N5FeatureProbe, N5SelectedAdapter, RuntimeContract,
    RuntimeContractError,
};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

#[derive(Debug, Serialize)]
struct MatrixRow {
    compiler_family: String,
    optimization_mode: String,
    selected_profile: String,
    compatible: bool,
}

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

    println!("\n=== Cross-Version ABI Adaptation Layer (Track N.5) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 10] = [
        ("Adapter table exposes multiple profiles", test_table_has_multiple_profiles),
        ("Adapter table includes Swift 6.1 profile", test_table_has_swift_6_1),
        ("Adapter table includes Swift 6.2 profile", test_table_has_swift_6_2),
        ("Feature probe reports host macOS arm64 facts", test_probe_reports_host_facts),
        ("Feature probe reports all required features enabled", test_probe_required_features_green),
        ("Auto-selected profile is compatible", test_auto_selected_profile_is_compatible),
        ("Auto-selected profile matches host compiler family", test_auto_selected_matches_family),
        ("Compatibility matrix selects supported profiles", test_matrix_selects_supported_profiles),
        ("Regression checker allows matching snapshot", test_regression_checker_allows_match),
        ("Regression checker flags release drift", test_regression_checker_flags_release_drift),
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

    println!("\n=== Track N.5 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.5 tests PASSED");
    } else {
        panic!("✗ Track N.5 tests FAILED");
    }
}

fn required_features(probe: &N5FeatureProbe) -> [bool; 6] {
    [
        probe.features.swift5_types_scan,
        probe.features.objc_class_scan,
        probe.features.dynamic_symbol_lowering,
        probe.features.recursive_generic_solver,
        probe.features.broker_isolation,
        probe.features.private_type_kind_fallback,
    ]
}

fn synthetic_probe(family: &str, mode: &str) -> N5FeatureProbe {
    N5FeatureProbe {
        compiler_family: family.to_string(),
        platform: "macos".to_string(),
        architecture: "arm64".to_string(),
        os_major: 15,
        os_minor: 0,
        os_patch: 0,
        optimization_mode: mode.to_string(),
        features: swift_runtime_sys::RuntimeContract::N5FeatureFlags {
            swift5_types_scan: true,
            objc_class_scan: true,
            dynamic_symbol_lowering: true,
            recursive_generic_solver: true,
            broker_isolation: true,
            private_type_kind_fallback: true,
        },
    }
}

fn write_matrix_artifact(rows: &[MatrixRow]) -> bool {
    let out_dir = Path::new("target/runtime-probe");
    if fs::create_dir_all(out_dir).is_err() {
        return false;
    }
    let out_path = out_dir.join("n5-compatibility-matrix.json");
    fs::write(out_path, serde_json::to_string_pretty(rows).unwrap()).is_ok()
}

fn test_table_has_multiple_profiles(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n5_adapter_table()?.profiles.len() >= 2)
}

fn test_table_has_swift_6_1(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract
        .n5_adapter_table()?
        .profiles
        .iter()
        .any(|profile| profile.profile_id == "swift_6_1_arm64_macos"))
}

fn test_table_has_swift_6_2(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract
        .n5_adapter_table()?
        .profiles
        .iter()
        .any(|profile| profile.profile_id == "swift_6_2_arm64_macos"))
}

fn test_probe_reports_host_facts(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let probe = contract.n5_feature_probe()?;
    Ok(probe.platform == "macos" && probe.architecture == "arm64" && probe.os_major >= 14)
}

fn test_probe_required_features_green(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let probe = contract.n5_feature_probe()?;
    Ok(required_features(&probe).into_iter().all(|enabled| enabled))
}

fn test_auto_selected_profile_is_compatible(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n5_select_adapter()?.compatible)
}

fn test_auto_selected_matches_family(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let probe = contract.n5_feature_probe()?;
    let selected = contract.n5_select_adapter()?;
    Ok(selected.profile_id.contains(&probe.compiler_family) && selected.compiler_family == probe.compiler_family)
}

fn test_matrix_selects_supported_profiles(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let table: N5AdapterTable = contract.n5_adapter_table()?;
    let mut rows = Vec::new();

    for (family, mode) in [
        ("swift_6_1", "debug"),
        ("swift_6_1", "release"),
        ("swift_6_2", "debug"),
        ("swift_6_2", "release"),
    ] {
        let probe = synthetic_probe(family, mode);
        let selected = RuntimeContract::n5_select_profile_from_table(&table, &probe);
        rows.push(MatrixRow {
            compiler_family: family.to_string(),
            optimization_mode: mode.to_string(),
            selected_profile: selected
                .as_ref()
                .map(|profile| profile.profile_id.clone())
                .unwrap_or_else(|| "missing".to_string()),
            compatible: selected.as_ref().map(|profile| profile.compatible).unwrap_or(false),
        });
    }

    let selected_all = rows.iter().all(|row| row.compatible);
    Ok(selected_all && write_matrix_artifact(&rows))
}

fn test_regression_checker_allows_match(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let selected = contract.n5_select_adapter()?;
    let report = RuntimeContract::n5_regression_report(
        &selected,
        &selected,
        &selected.optimization_mode,
    );
    Ok(!report.drift_detected && report.issues.is_empty())
}

fn test_regression_checker_flags_release_drift(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let expected = contract.n5_select_adapter()?;
    let mut observed: N5SelectedAdapter = expected.clone();
    observed.optimization_mode = "release".to_string();
    observed
        .selected_symbols
        .insert("dynamic_invoke".to_string(), "swift_contract_n2_invoke_auto_release_alias".to_string());
    observed.missing_features = vec!["objc_class_scan".to_string()];

    let report: N5DriftReport = RuntimeContract::n5_regression_report(&expected, &observed, "release");
    Ok(report.drift_detected
        && report.issues.iter().any(|issue| issue.contains("symbol drift for dynamic_invoke"))
        && report.issues.iter().any(|issue| issue.contains("missing required features")))
}