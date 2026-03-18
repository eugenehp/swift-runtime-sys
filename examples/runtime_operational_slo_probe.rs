use std::fs;
use std::path::Path;

use serde::Serialize;
use swift_runtime_sys::RuntimeContract::{
    N8BenchmarkSample, N8BudgetGate, N8OperationalReport, N8SloBudget, RuntimeContract,
    RuntimeContractError,
};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

#[derive(Debug, Serialize)]
struct N8GateArtifact {
    budgets: Vec<N8SloBudget>,
    samples: Vec<N8BenchmarkSample>,
    gates: Vec<N8BudgetGate>,
    alerts: Vec<String>,
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

    println!("\n=== Operational Guarantees & SLOs (Track N.8) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 10] = [
        ("Default SLO table includes required operation budgets", test_default_slos_cover_required_ops),
        ("Dynamic invoke benchmark returns measurable throughput", test_dynamic_invoke_benchmark),
        ("Metadata traversal benchmark returns measurable throughput", test_metadata_traversal_benchmark),
        ("Graph benchmark returns measurable latency and memory", test_graph_benchmark),
        ("Budget gate passes for healthy benchmark report", test_budget_gate_passes),
        ("Budget gate emits failures for synthetic regressions", test_budget_gate_regression_alert),
        ("CI budget artifact and alert report are written", test_ci_budget_artifacts_written),
        ("Degraded mode runbook is generated", test_degraded_mode_runbook_generated),
        ("Degraded mode runbook includes fallback actions", test_degraded_mode_contains_actions),
        ("Exit criterion confirms operational report readiness", test_exit_criterion_operational_readiness),
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

    println!("\n=== Track N.8 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.8 tests PASSED");
    } else {
        panic!("✗ Track N.8 tests FAILED");
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> bool {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
    }
    fs::write(path, serde_json::to_string_pretty(value).unwrap()).is_ok()
}

fn report(contract: &RuntimeContract) -> Result<N8OperationalReport, RuntimeContractError> {
    contract.n8_operational_report(1000, 160, 80)
}

fn test_default_slos_cover_required_ops(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let slos = contract.n8_default_slos();
    Ok(slos.iter().any(|entry| entry.operation == "dynamic_invoke")
        && slos.iter().any(|entry| entry.operation == "metadata_traversal")
        && slos.iter().any(|entry| entry.operation == "graph_operations"))
}

fn test_dynamic_invoke_benchmark(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let sample = contract.n8_measure_dynamic_invoke(600)?;
    Ok(sample.p95_latency_ms > 0.0 && sample.throughput_ops_per_sec > 100.0)
}

fn test_metadata_traversal_benchmark(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let sample = contract.n8_measure_metadata_traversal(80)?;
    Ok(sample.p95_latency_ms > 0.0 && sample.throughput_ops_per_sec > 10.0)
}

fn test_graph_benchmark(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let sample = contract.n8_measure_graph_operations(40)?;
    Ok(sample.p50_latency_ms > 0.0 && sample.rss_bytes > 0)
}

fn test_budget_gate_passes(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let op = report(contract)?;
    Ok(op.gates.iter().all(|gate| gate.passed))
}

fn test_budget_gate_regression_alert(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let healthy = report(contract)?;
    let strict_budget = vec![N8SloBudget {
        operation: "dynamic_invoke".to_string(),
        p50_latency_ms_budget: 0.0001,
        p95_latency_ms_budget: 0.0001,
        min_throughput_ops_per_sec: f64::MAX,
        max_rss_bytes: 1,
    }];
    let gates = contract.n8_evaluate_budget_gates(&healthy.samples, &strict_budget);
    let alerts = contract.n8_ci_budget_alerts(&gates);
    Ok(gates.iter().any(|gate| !gate.passed) && !alerts.is_empty())
}

fn test_ci_budget_artifacts_written(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let op = report(contract)?;
    let out = Path::new("target/runtime-probe/n8-budget-gates.json");
    let alerts_path = Path::new("target/runtime-probe/n8-alerts.txt");
    let artifact = N8GateArtifact {
        budgets: op.budgets.clone(),
        samples: op.samples.clone(),
        gates: op.gates.clone(),
        alerts: op.alerts.clone(),
    };
    let alerts_body = if op.alerts.is_empty() {
        "N8 ALERTS: none\n".to_string()
    } else {
        format!("{}\n", op.alerts.join("\n"))
    };
    Ok(write_json(out, &artifact) && fs::write(alerts_path, alerts_body).is_ok() && out.exists())
}

fn test_degraded_mode_runbook_generated(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let op = report(contract)?;
    let out = Path::new("target/runtime-probe/n8-degraded-mode-runbook.md");
    let _ = fs::create_dir_all("target/runtime-probe");
    let _ = fs::write(out, &op.degraded_mode_runbook);
    Ok(op.degraded_mode_runbook.contains("N.8 Degraded Mode Runbook") && out.exists())
}

fn test_degraded_mode_contains_actions(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let op = report(contract)?;
    Ok(op.degraded_mode_runbook.contains("action:")
        || op.degraded_mode_runbook.contains("Incident Response"))
}

fn test_exit_criterion_operational_readiness(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let op = report(contract)?;
    let all_measured = op.samples.len() >= 3;
    let gates_green = op.gates.iter().all(|gate| gate.passed);
    Ok(all_measured
        && gates_green
        && op
            .degraded_mode_runbook
            .contains("When capability probes fail"))
}
