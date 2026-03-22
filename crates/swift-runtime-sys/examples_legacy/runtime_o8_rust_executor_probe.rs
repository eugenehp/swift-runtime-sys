/// O.8 Rust-owned executor integration probe.
///
/// Starts an experimental, default-off Rust worker queue that executes existing
/// Swift task bridge calls on a dedicated Rust-owned thread. This is the first
/// Wave O9 scaffold: fairness is FIFO on the Rust side, cancellation is visible
/// both as queued-job cancellation and via existing Swift task cancel probes,
/// and shutdown remains bounded.
use std::env;
use std::time::Duration;

use swift_runtime_sys::RustExecutorInterop::{
    o8_executor_enabled_from_env, RustExecutorInteropConfig, RustExecutorJob, RustOwnedExecutor,
    O8_ENABLE_ENV,
};

fn main() {
    let mut passed = 0;
    let mut failed = 0;
    let mut passed_flags = [0i32; 6];

    println!("\n=== O.8 Rust-Owned Executor Integration Probe ===");

    let tests: [(&str, fn() -> Result<bool, String>); 6] = [
        ("opt-in flag is disabled by default", test_default_opt_in_disabled),
        (
            "enabled executor runs Swift task bridge work",
            test_enabled_executor_runs_bridge_work,
        ),
        ("executor preserves FIFO fairness", test_fifo_fairness),
        (
            "queued job cancellation is deterministic",
            test_queued_job_cancellation,
        ),
        (
            "Swift cancellation visibility remains available through executor jobs",
            test_swift_cancellation_visibility,
        ),
        (
            "executor shutdown completes within bound",
            test_bounded_shutdown,
        ),
    ];

    for (index, (name, test_fn)) in tests.iter().enumerate() {
        match test_fn() {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
                passed_flags[index] = 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(error) => {
                println!("✗ {name} FAIL ({error})");
                failed += 1;
            }
        }
    }

    println!("\n=== O.8 Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!(
        "o8 rust executor parity => default_off_ok={} run_ok={} fairness_ok={} queue_cancel_ok={} swift_cancel_visibility_ok={} shutdown_ok={}",
        passed_flags[0],
        passed_flags[1],
        passed_flags[2],
        passed_flags[3],
        passed_flags[4],
        passed_flags[5],
    );

    if failed == 0 {
        println!("✓ O.8 rust-owned executor probe PASSED");
    } else {
        panic!("✗ O.8 rust-owned executor probe FAILED");
    }
}

fn test_default_opt_in_disabled() -> Result<bool, String> {
    env::remove_var(O8_ENABLE_ENV);
    Ok(!o8_executor_enabled_from_env())
}

fn test_enabled_executor_runs_bridge_work() -> Result<bool, String> {
    let executor = spawn_enabled_executor()?;
    let handle = executor
        .submit(RustExecutorJob::TaskSpawnSum { lhs: 20, rhs: 22 })
        .map_err(|error| format!("submit failed: {error:?}"))?;
    let report = handle
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await result failed: {error:?}"))?;
    let shutdown_elapsed = executor
        .shutdown()
        .map_err(|error| format!("shutdown failed: {error:?}"))?;
    env::remove_var(O8_ENABLE_ENV);

    Ok(
        report.status == "completed"
            && report.value == 42
            && report.kind.as_str() == "task_spawn_sum"
            && shutdown_elapsed <= Duration::from_secs(2),
    )
}

fn test_fifo_fairness() -> Result<bool, String> {
    let executor = spawn_enabled_executor()?;
    let first = executor
        .submit(RustExecutorJob::TaskSpawnSum { lhs: 20, rhs: 22 })
        .map_err(|error| format!("submit first failed: {error:?}"))?;
    let second = executor
        .submit(RustExecutorJob::TaskSpawnChain { base: 5, steps: 3 })
        .map_err(|error| format!("submit second failed: {error:?}"))?;
    let third = executor
        .submit(RustExecutorJob::TaskSpawnSum { lhs: -3, rhs: 9 })
        .map_err(|error| format!("submit third failed: {error:?}"))?;

    let first_report = first
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await first failed: {error:?}"))?;
    let second_report = second
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await second failed: {error:?}"))?;
    let third_report = third
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await third failed: {error:?}"))?;

    executor
        .shutdown()
        .map_err(|error| format!("shutdown failed: {error:?}"))?;
    env::remove_var(O8_ENABLE_ENV);

    Ok(
        first_report.completion_order == 1
            && second_report.completion_order == 2
            && third_report.completion_order == 3
            && first_report.value == 42
            && second_report.value == 8
            && third_report.value == 6,
    )
}

fn test_queued_job_cancellation() -> Result<bool, String> {
    let executor = spawn_enabled_executor()?;
    let delay = executor
        .submit(RustExecutorJob::Delay { millis: 100 })
        .map_err(|error| format!("submit delay failed: {error:?}"))?;
    let cancelled = executor
        .submit(RustExecutorJob::TaskSpawnSum { lhs: 1, rhs: 2 })
        .map_err(|error| format!("submit cancelled job failed: {error:?}"))?;
    cancelled.cancel();

    let delay_report = delay
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await delay failed: {error:?}"))?;
    let cancelled_report = cancelled
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await cancelled job failed: {error:?}"))?;

    executor
        .shutdown()
        .map_err(|error| format!("shutdown failed: {error:?}"))?;
    env::remove_var(O8_ENABLE_ENV);

    Ok(
        delay_report.status == "completed"
            && cancelled_report.status == "cancelled"
            && cancelled_report.completion_order == 2,
    )
}

fn test_swift_cancellation_visibility() -> Result<bool, String> {
    let executor = spawn_enabled_executor()?;
    let handle = executor
        .submit(RustExecutorJob::CancellationVisibility)
        .map_err(|error| format!("submit visibility job failed: {error:?}"))?;
    let report = handle
        .await_result(executor.result_timeout())
        .map_err(|error| format!("await visibility job failed: {error:?}"))?;

    executor
        .shutdown()
        .map_err(|error| format!("shutdown failed: {error:?}"))?;
    env::remove_var(O8_ENABLE_ENV);

    let status = report.value;
    let cancel_invoked = (status & 1) != 0;
    let cancelled_after = (status & 4) != 0;
    Ok(report.status == "completed" && cancel_invoked && cancelled_after)
}

fn test_bounded_shutdown() -> Result<bool, String> {
    let executor = spawn_enabled_executor()?;
    let elapsed = executor
        .shutdown()
        .map_err(|error| format!("shutdown failed: {error:?}"))?;
    env::remove_var(O8_ENABLE_ENV);
    Ok(elapsed <= Duration::from_secs(2))
}

fn spawn_enabled_executor() -> Result<RustOwnedExecutor, String> {
    env::set_var(O8_ENABLE_ENV, "1");
    RustOwnedExecutor::spawn_with_config(
        RustExecutorInteropConfig {
            enabled: true,
            result_timeout: Duration::from_secs(3),
            shutdown_timeout: Duration::from_secs(2),
        },
        "./libRustBridge.dylib",
        "./libRuntimeThunks.dylib",
    )
    .map_err(|error| format!("spawn executor failed: {error:?}"))
}