use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use swift_runtime_sys::RuntimeContract::{
    ContractErrorContextPayload, N6Execution, N6Program, N6Result, RuntimeContract,
    RuntimeContractError,
};
use swift_runtime_sys::RuntimeFactory::{RuntimeFactory, ThrowsResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct N6Mismatch {
    index: usize,
    reason: String,
    native: N6Result,
    rust: N6Result,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct N6TriageReport {
    seed: i64,
    mismatch_count: usize,
    mismatches: Vec<N6Mismatch>,
    swift_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct N6CampaignSummary {
    runs: usize,
    fragment_count: i32,
    mismatches: usize,
    corpus_dir: String,
    seeds: Vec<i64>,
    oracles: Vec<String>,
    divergence_artifacts_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct N6CrossOracleReport {
    seed: i64,
    native_repeat_match: bool,
    native_vs_rust_match: bool,
    native_result_count: i32,
    rust_result_count: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(String::as_str) == Some("--seed-check") {
        let seed = args
            .get(2)
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(1);
        let fragment_count = args
            .get(3)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(12);
        let out_dir = args
            .get(4)
            .cloned()
            .unwrap_or_else(|| "target/runtime-probe/n6-corpus".to_string());
        let factory = build_factory();
        let contract = RuntimeContract::new(&factory);
        let outcome = run_seed_check(&factory, &contract, seed, fragment_count, Path::new(&out_dir))
            .unwrap_or_else(|e| panic!("seed check failed: {e:?}"));
        if !outcome.pass {
            panic!(
                "seed check failed for seed {} (artifacts_complete={})",
                outcome.seed, outcome.divergence_artifacts_complete
            );
        }
        return;
    }

    if args.get(1).map(String::as_str) == Some("--campaign") {
        let runs = args
            .get(2)
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(16);
        let fragment_count = args
            .get(3)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(12);
        let out_dir = args
            .get(4)
            .cloned()
            .unwrap_or_else(|| "target/runtime-probe/n6-corpus".to_string());
        let factory = build_factory();
        let contract = RuntimeContract::new(&factory);
        run_campaign(
            &factory,
            &contract,
            runs,
            fragment_count,
            Path::new(&out_dir),
        )
        .unwrap_or_else(|e| panic!("campaign failed: {e:?}"));
        return;
    }

    let factory = build_factory();
    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Differential Fuzzing & Semantic Oracle (Track N.6) ===");

    let tests: [(
        &str,
        fn(&RuntimeFactory, &RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 10] = [
        (
            "Program generator is deterministic",
            test_generator_is_deterministic,
        ),
        (
            "Program generator emits Swift source",
            test_generator_emits_source,
        ),
        (
            "Native Swift executor runs generated program",
            test_native_executor_runs_program,
        ),
        (
            "Rust-driven executor matches native Swift",
            test_rust_matches_native,
        ),
        (
            "Comparator stays clean on matching traces",
            test_matching_trace_has_no_mismatch,
        ),
        (
            "Comparator detects injected mismatch",
            test_injected_mismatch_detected,
        ),
        ("Triage artifact is written", test_triage_artifact_written),
        (
            "Corpus minimizer shrinks mismatch",
            test_corpus_minimizer_shrinks,
        ),
        (
            "Campaign passes multi-seed differential run",
            test_campaign_passes,
        ),
        (
            "Campaign summary artifact is written",
            test_campaign_summary_written,
        ),
    ];

    for (name, test_fn) in tests {
        match test_fn(&factory, &contract) {
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

    println!("\n=== Track N.6 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.6 tests PASSED");
    } else {
        panic!("✗ Track N.6 tests FAILED");
    }
}

fn build_factory() -> RuntimeFactory {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));
    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));
    factory
}

fn compare_executions(native: &N6Execution, rust: &N6Execution) -> Vec<N6Mismatch> {
    let mut mismatches = Vec::new();
    let len = native.results.len().min(rust.results.len());

    for index in 0..len {
        if native.results[index] != rust.results[index] {
            mismatches.push(N6Mismatch {
                index,
                reason: "semantic mismatch".to_string(),
                native: native.results[index].clone(),
                rust: rust.results[index].clone(),
            });
        }
    }

    if native.results.len() != rust.results.len() {
        let index = len;
        mismatches.push(N6Mismatch {
            index,
            reason: format!(
                "result-count mismatch native={} rust={}",
                native.results.len(),
                rust.results.len()
            ),
            native: native
                .results
                .get(index)
                .cloned()
                .unwrap_or_else(|| N6Result {
                    id: -1,
                    kind: "missing".to_string(),
                    status: "missing".to_string(),
                    value: None,
                    error: None,
                    side_effect: None,
                }),
            rust: rust
                .results
                .get(index)
                .cloned()
                .unwrap_or_else(|| N6Result {
                    id: -1,
                    kind: "missing".to_string(),
                    status: "missing".to_string(),
                    value: None,
                    error: None,
                    side_effect: None,
                }),
        });
    }

    mismatches
}

fn error_side_effect(payload: &ContractErrorContextPayload) -> String {
    format!(
        "{}|{}|chain={}",
        payload.domain,
        payload.message,
        payload.chain.len()
    )
}

fn rust_execute_program(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
    program: &N6Program,
) -> Result<N6Execution, RuntimeContractError> {
    let mut results = Vec::new();

    for fragment in &program.fragments {
        let result = match fragment.kind.as_str() {
            "add" => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "value".to_string(),
                value: Some(
                    factory
                        .call_i32_i32_to_i32("swift_add", fragment.a, fragment.b)
                        .map_err(RuntimeContractError::from)?,
                ),
                error: None,
                side_effect: None,
            },
            "safe_divide" => match factory
                .call_throws_i32_i32(
                    "$s10RustBridge10safeDivideys5Int32VAD_ADtKF",
                    fragment.a,
                    fragment.b,
                )
                .map_err(RuntimeContractError::from)?
            {
                ThrowsResult::Ok(value) => N6Result {
                    id: fragment.id,
                    kind: fragment.kind.clone(),
                    status: "value".to_string(),
                    value: Some(value),
                    error: None,
                    side_effect: None,
                },
                ThrowsResult::Threw(error_obj) => {
                    let _ = factory.release(error_obj);
                    N6Result {
                        id: fragment.id,
                        kind: fragment.kind.clone(),
                        status: "error".to_string(),
                        value: None,
                        error: Some("division_by_zero".to_string()),
                        side_effect: None,
                    }
                }
            },
            "async_add" => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "value".to_string(),
                value: Some(
                    factory
                        .call_i32_i32_to_i32("swift_async_add_blocking", fragment.a, fragment.b)
                        .map_err(RuntimeContractError::from)?,
                ),
                error: None,
                side_effect: None,
            },
            "task_local" => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "value".to_string(),
                value: Some(contract.task_local_run_with(fragment.a, fragment.b)?),
                error: None,
                side_effect: Some(format!("inherited={}|detached=-1", fragment.a)),
            },
            "error_context_validation" => {
                contract.error_context_make_validation(fragment.a, fragment.b)?;
                let payload = contract.error_context_parse()?;
                contract.error_context_clear()?;
                N6Result {
                    id: fragment.id,
                    kind: fragment.kind.clone(),
                    status: "context".to_string(),
                    value: Some(payload.code),
                    error: None,
                    side_effect: Some(error_side_effect(&payload)),
                }
            }
            "generic_array_i32" => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "value".to_string(),
                value: Some(contract.n3_invoke_generic_i32(
                    "Array<Int32>",
                    "Sequence;Element==Int32",
                    "sequence.sum_range",
                    fragment.a,
                    fragment.b,
                )?),
                error: None,
                side_effect: None,
            },
            "generic_array_string" => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "value".to_string(),
                value: Some(contract.n3_invoke_generic_i32(
                    "Array<String>",
                    "Sequence;Element==String",
                    "sequence.sample_metric",
                    fragment.a,
                    fragment.b,
                )?),
                error: None,
                side_effect: None,
            },
            "generic_sequence_witness" => {
                let witness = contract.n3_resolve_witness_json(
                    "Array<String>",
                    "Sequence",
                    "Element==String",
                )?;
                let parsed: Value = serde_json::from_str(&witness)
                    .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))?;
                let supported = parsed
                    .get("supported")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let token = parsed.get("token").and_then(Value::as_u64).unwrap_or(0);
                N6Result {
                    id: fragment.id,
                    kind: fragment.kind.clone(),
                    status: "value".to_string(),
                    value: Some(if supported && token > 0 { 1 } else { 0 }),
                    error: None,
                    side_effect: Some("Sequence<Element=String>".to_string()),
                }
            }
            "generic_box_string" => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "value".to_string(),
                value: Some(contract.n3_invoke_generic_i32(
                    "ContractGenericBox<String>",
                    "",
                    "box.sample_metric",
                    fragment.a,
                    fragment.b,
                )?),
                error: None,
                side_effect: None,
            },
            _ => N6Result {
                id: fragment.id,
                kind: fragment.kind.clone(),
                status: "error".to_string(),
                value: None,
                error: Some("unsupported_fragment".to_string()),
                side_effect: None,
            },
        };
        results.push(result);
    }

    Ok(N6Execution {
        seed: program.seed,
        result_count: results.len() as i32,
        results,
    })
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> bool {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
    }
    fs::write(path, serde_json::to_string_pretty(value).unwrap()).is_ok()
}

fn minimize_program<F>(mut program: N6Program, mut mismatches_for: F) -> N6Program
where
    F: FnMut(&N6Program) -> Result<Vec<N6Mismatch>, RuntimeContractError>,
{
    let mut index = 0;
    while index < program.fragments.len() {
        if program.fragments.len() == 1 {
            break;
        }
        let mut candidate = program.clone();
        candidate.fragments.remove(index);
        candidate.swift_source = candidate
            .fragments
            .iter()
            .map(|fragment| fragment.source.clone())
            .collect::<Vec<_>>()
            .join("\n");
        if mismatches_for(&candidate)
            .map(|m| !m.is_empty())
            .unwrap_or(false)
        {
            program = candidate;
        } else {
            index += 1;
        }
    }
    program
}

fn injected_mismatch(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
    program: &N6Program,
) -> Result<Vec<N6Mismatch>, RuntimeContractError> {
    let native = contract.n6_execute_program(program)?;
    let mut rust = rust_execute_program(factory, contract, program)?;
    if let Some(first) = rust.results.first_mut() {
        first.side_effect = Some("injected_mismatch".to_string());
    }
    Ok(compare_executions(&native, &rust))
}

fn run_campaign(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
    runs: usize,
    fragment_count: i32,
    out_dir: &Path,
) -> Result<(), RuntimeContractError> {
    fs::create_dir_all(out_dir)
        .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))?;
    let mismatches = 0usize;
    let mut seeds = Vec::with_capacity(runs);
    let mut divergence_artifacts_complete = true;

    for seed in 1..=runs {
        let outcome = run_seed_check(factory, contract, seed as i64, fragment_count, out_dir)?;
        seeds.push(outcome.seed);
        if !outcome.divergence_artifacts_complete {
            divergence_artifacts_complete = false;
        }
        if !outcome.pass {
            return Err(RuntimeContractError::DescriptorParse(format!(
                "differential mismatch at seed {} (artifacts_complete={})",
                outcome.seed, outcome.divergence_artifacts_complete
            )));
        }
    }

    let summary = N6CampaignSummary {
        runs,
        fragment_count,
        mismatches,
        corpus_dir: out_dir.to_string_lossy().into_owned(),
        seeds,
        oracles: vec![
            "native_swift".to_string(),
            "native_swift_replay".to_string(),
            "rust_runtime".to_string(),
        ],
        divergence_artifacts_complete,
    };
    let summary_path = out_dir.join("campaign-summary.json");
    let _ = write_json(&summary_path, &summary);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct N6SeedOutcome {
    seed: i64,
    pass: bool,
    divergence_artifacts_complete: bool,
}

fn run_seed_check(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
    seed: i64,
    fragment_count: i32,
    out_dir: &Path,
) -> Result<N6SeedOutcome, RuntimeContractError> {
    fs::create_dir_all(out_dir)
        .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))?;
    let program = contract.n6_generate_program(seed, fragment_count)?;
    let native = contract.n6_execute_program(&program)?;
    let native_repeat = contract.n6_execute_program(&program)?;
    let rust = rust_execute_program(factory, contract, &program)?;
    let native_repeat_diff = compare_executions(&native, &native_repeat);
    let diff = compare_executions(&native, &rust);
    let cross_oracle = N6CrossOracleReport {
        seed: program.seed,
        native_repeat_match: native_repeat_diff.is_empty(),
        native_vs_rust_match: diff.is_empty(),
        native_result_count: native.result_count,
        rust_result_count: rust.result_count,
    };
    let oracle_path = out_dir.join(format!("seed-{}-cross-oracle.json", program.seed));
    let _ = write_json(&oracle_path, &cross_oracle);

    if !native_repeat_diff.is_empty() {
        let triage = N6TriageReport {
            seed: program.seed,
            mismatch_count: native_repeat_diff.len(),
            mismatches: native_repeat_diff,
            swift_source: program.swift_source.clone(),
        };
        let corpus_path = out_dir.join(format!("seed-{}-corpus.json", program.seed));
        let triage_path = out_dir.join(format!("seed-{}-triage.json", program.seed));
        let minimized = minimize_program(program.clone(), |candidate| {
            let native_candidate = contract.n6_execute_program(candidate)?;
            let native_repeat_candidate = contract.n6_execute_program(candidate)?;
            Ok(compare_executions(&native_candidate, &native_repeat_candidate))
        });
        let minimized_path = out_dir.join(format!("seed-{}-minimized.json", program.seed));
        let corpus_ok = write_json(&corpus_path, &program);
        let triage_ok = write_json(&triage_path, &triage);
        let minimized_ok = write_json(&minimized_path, &minimized);
        return Ok(N6SeedOutcome {
            seed: program.seed,
            pass: false,
            divergence_artifacts_complete: corpus_ok && triage_ok && minimized_ok,
        });
    }

    if !diff.is_empty() {
        let triage = N6TriageReport {
            seed: program.seed,
            mismatch_count: diff.len(),
            mismatches: diff,
            swift_source: program.swift_source.clone(),
        };
        let corpus_path = out_dir.join(format!("seed-{}-corpus.json", program.seed));
        let triage_path = out_dir.join(format!("seed-{}-triage.json", program.seed));
        let minimized = minimize_program(program.clone(), |candidate| {
            let native_candidate = contract.n6_execute_program(candidate)?;
            let rust_candidate = rust_execute_program(factory, contract, candidate)?;
            Ok(compare_executions(&native_candidate, &rust_candidate))
        });
        let minimized_path = out_dir.join(format!("seed-{}-minimized.json", program.seed));
        let corpus_ok = write_json(&corpus_path, &program);
        let triage_ok = write_json(&triage_path, &triage);
        let minimized_ok = write_json(&minimized_path, &minimized);
        return Ok(N6SeedOutcome {
            seed: program.seed,
            pass: false,
            divergence_artifacts_complete: corpus_ok && triage_ok && minimized_ok,
        });
    }

    Ok(N6SeedOutcome {
        seed: program.seed,
        pass: true,
        divergence_artifacts_complete: true,
    })
}

fn test_generator_is_deterministic(
    _: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let first = contract.n6_generate_program(77, 12)?;
    let second = contract.n6_generate_program(77, 12)?;
    Ok(first == second)
}

fn test_generator_emits_source(
    _: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(101, 16)?;
    Ok(!program.swift_source.is_empty()
        && program.swift_source.contains("let r")
        && (program.swift_source.contains("await") || program.swift_source.contains("try?")))
}

fn test_native_executor_runs_program(
    _: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(5, 10)?;
    let native = contract.n6_execute_program(&program)?;
    Ok(native.result_count == program.fragments.len() as i32)
}

fn test_rust_matches_native(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(9, 18)?;
    let native = contract.n6_execute_program(&program)?;
    let rust = rust_execute_program(factory, contract, &program)?;
    Ok(compare_executions(&native, &rust).is_empty())
}

fn test_matching_trace_has_no_mismatch(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(33, 14)?;
    let native = contract.n6_execute_program(&program)?;
    let rust = rust_execute_program(factory, contract, &program)?;
    Ok(compare_executions(&native, &rust).is_empty())
}

fn test_injected_mismatch_detected(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(55, 6)?;
    Ok(!injected_mismatch(factory, contract, &program)?.is_empty())
}

fn test_triage_artifact_written(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(61, 7)?;
    let diff = injected_mismatch(factory, contract, &program)?;
    let report = N6TriageReport {
        seed: program.seed,
        mismatch_count: diff.len(),
        mismatches: diff,
        swift_source: program.swift_source.clone(),
    };
    let out_path = Path::new("target/runtime-probe/n6-triage-report.json");
    Ok(write_json(out_path, &report) && out_path.exists())
}

fn test_corpus_minimizer_shrinks(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let program = contract.n6_generate_program(73, 9)?;
    let original_len = program.fragments.len();
    let minimized = minimize_program(program.clone(), |candidate| {
        injected_mismatch(factory, contract, candidate)
    });
    let out_path = Path::new("target/runtime-probe/n6-minimized-corpus.json");
    Ok(minimized.fragments.len() < original_len
        && minimized.fragments.len() == 1
        && write_json(out_path, &minimized)
        && out_path.exists())
}

fn test_campaign_passes(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let out_dir = Path::new("target/runtime-probe/n6-campaign-pass");
    run_campaign(factory, contract, 8, 10, out_dir)?;
    Ok(true)
}

fn test_campaign_summary_written(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let out_dir = PathBuf::from("target/runtime-probe/n6-campaign-summary");
    run_campaign(factory, contract, 4, 8, &out_dir)?;
    Ok(out_dir.join("campaign-summary.json").exists())
}
