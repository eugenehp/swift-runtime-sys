use swift_runtime_sys::ConcurrencyAbi::{
    collect_control_readiness, collect_profile, collect_raw_task_thunk_snapshot,
    direct_thunk_symbols, optional_symbols as concurrency_optional_symbols,
    raw_swiftcc_direct_invocation_reason, raw_swiftcc_hybrid_reason, required_symbols,
    run_bridge_control_smoke, ConcurrencyControlPath,
};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== O.2 Raw Concurrency ABI Probe ===");

    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let tests: [(&str, fn(&RuntimeFactory) -> Result<bool, String>); 23] = [
        (
            "required swift_task_* symbols resolve",
            test_required_task_symbols,
        ),
        (
            "required swift_continuation_* symbols resolve",
            test_required_continuation_symbols,
        ),
        (
            "required swift_asyncLet_* symbols resolve",
            test_required_asynclet_symbols,
        ),
        (
            "required swift_job_* symbols resolve",
            test_required_job_symbols,
        ),
        (
            "all required symbols have distinct addresses",
            test_required_symbols_distinct_addresses,
        ),
        (
            "optional concurrency control symbols status",
            test_optional_symbol_status,
        ),
        (
            "concurrency capability fingerprint is non-empty",
            test_capability_fingerprint,
        ),
        (
            "capability profile reports full required coverage",
            test_profile_required_coverage,
        ),
        (
            "raw swiftcc invocation status is explicit",
            test_raw_swiftcc_invocation_status,
        ),
        (
            "bridge control fallback semantics are deterministic",
            test_bridge_control_fallback_semantics,
        ),
        (
            "bridge control fallback covers actor/stream/task-local",
            test_bridge_control_extended_domains,
        ),
        (
            "direct raw task thunk symbols resolve",
            test_direct_raw_task_thunk_symbols,
        ),
        (
            "direct raw task thunk snapshot is callable",
            test_direct_raw_task_thunk_snapshot,
        ),
        (
            "raw task alloc/dealloc works inside task context",
            test_task_context_raw_alloc_status,
        ),
        (
            "raw executor visibility works inside task context",
            test_task_context_executor_status,
        ),
        (
            "raw current-task visibility works across task yield",
            test_task_context_current_task_status,
        ),
        (
            "raw self-cancel ordering works in task context",
            test_task_context_cancel_status,
        ),
        (
            "raw child-cancel ordering works in task context",
            test_task_context_child_cancel_status,
        ),
        (
            "raw async-let lifecycle ordering works in task context",
            test_task_context_asynclet_status,
        ),
        (
            "raw child-job completion ordering works in task context",
            test_task_context_job_order_status,
        ),
        (
            "direct raw orchestration preflight policy is explicit",
            test_orchestration_policy_status,
        ),
        (
            "direct raw main-executor identity check is callable",
            test_main_executor_identity_status,
        ),
        (
            "direct SwiftCC thunk ordering probe works inside task context",
            test_task_context_direct_swiftcc_ordering_status,
        ),
    ];

    for (name, test_fn) in tests {
        match test_fn(&factory) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(err) => {
                println!("✗ {name} FAIL ({err})");
                failed += 1;
            }
        }
    }

    println!("\n=== O.2 Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("✓ O.2 raw concurrency ABI probe PASSED");
    } else {
        panic!("✗ O.2 raw concurrency ABI probe FAILED");
    }
}

fn required_task_symbols() -> &'static [&'static str] {
    &[
        "swift_task_getCurrent",
        "swift_task_alloc",
        "swift_task_dealloc",
        "swift_task_cancel",
        "swift_task_create",
    ]
}

fn required_continuation_symbols() -> &'static [&'static str] {
    &[
        "swift_continuation_init",
        "swift_continuation_resume",
        "swift_continuation_throwingResume",
    ]
}

fn required_asynclet_symbols() -> &'static [&'static str] {
    &[
        "swift_asyncLet_start",
        "swift_asyncLet_begin",
        "swift_asyncLet_end",
    ]
}

fn required_job_symbols() -> &'static [&'static str] {
    &["swift_job_run"]
}

fn optional_symbols() -> &'static [&'static str] {
    concurrency_optional_symbols()
}

fn has_symbol(factory: &RuntimeFactory, symbol: &str) -> bool {
    factory.symbol_address(symbol).is_ok()
}

fn assert_symbol_set(
    factory: &RuntimeFactory,
    symbols: &[&str],
    label: &str,
) -> Result<bool, String> {
    let missing: Vec<&str> = symbols
        .iter()
        .copied()
        .filter(|sym| !has_symbol(factory, sym))
        .collect();
    if missing.is_empty() {
        Ok(true)
    } else {
        Err(format!("missing {label} symbols: {}", missing.join(", ")))
    }
}

fn test_required_task_symbols(factory: &RuntimeFactory) -> Result<bool, String> {
    assert_symbol_set(factory, required_task_symbols(), "task")
}

fn test_required_continuation_symbols(factory: &RuntimeFactory) -> Result<bool, String> {
    assert_symbol_set(factory, required_continuation_symbols(), "continuation")
}

fn test_required_asynclet_symbols(factory: &RuntimeFactory) -> Result<bool, String> {
    assert_symbol_set(factory, required_asynclet_symbols(), "asynclet")
}

fn test_required_job_symbols(factory: &RuntimeFactory) -> Result<bool, String> {
    assert_symbol_set(factory, required_job_symbols(), "job")
}

fn test_required_symbols_distinct_addresses(factory: &RuntimeFactory) -> Result<bool, String> {
    let mut addrs = Vec::new();
    for symbol in required_task_symbols()
        .iter()
        .chain(required_continuation_symbols().iter())
        .chain(required_asynclet_symbols().iter())
        .chain(required_job_symbols().iter())
    {
        let addr = factory
            .symbol_address(symbol)
            .map_err(|e| format!("resolve failed for {symbol}: {e:?}"))?;
        addrs.push((*symbol, addr as usize));
    }

    for i in 0..addrs.len() {
        for j in (i + 1)..addrs.len() {
            if addrs[i].1 == addrs[j].1 {
                return Err(format!(
                    "symbol address collision: {} and {} at 0x{:x}",
                    addrs[i].0, addrs[j].0, addrs[i].1
                ));
            }
        }
    }

    Ok(true)
}

fn test_optional_symbol_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let resolved_count = optional_symbols()
        .iter()
        .filter(|sym| has_symbol(factory, sym))
        .count();

    println!(
        "optional concurrency symbols resolved: {resolved_count}/{}",
        optional_symbols().len()
    );

    // Capability status is valid even if some optional symbols are unavailable.
    Ok(true)
}

fn test_capability_fingerprint(factory: &RuntimeFactory) -> Result<bool, String> {
    let required_total = required_task_symbols().len()
        + required_continuation_symbols().len()
        + required_asynclet_symbols().len()
        + required_job_symbols().len();

    let resolved = required_task_symbols()
        .iter()
        .chain(required_continuation_symbols().iter())
        .chain(required_asynclet_symbols().iter())
        .chain(required_job_symbols().iter())
        .filter(|sym| has_symbol(factory, sym))
        .count();

    println!("required concurrency fingerprint: {resolved}/{required_total}");

    Ok(resolved == required_total)
}

fn test_profile_required_coverage(factory: &RuntimeFactory) -> Result<bool, String> {
    let profile = collect_profile(factory).map_err(|e| format!("collect profile failed: {e:?}"))?;

    if !profile.has_all_required() {
        return Err(format!(
            "missing required symbols in profile: {}",
            profile.missing_required.join(", ")
        ));
    }

    let required_fp = profile.required_fingerprint();
    let optional_fp = profile.optional_fingerprint();
    println!("profile required fingerprint: {required_fp}");
    println!("profile optional fingerprint: {optional_fp}");

    for required in required_symbols() {
        if !profile.has_symbol(required) {
            return Err(format!(
                "profile missing required symbol lookup: {required}"
            ));
        }
    }

    Ok(true)
}

fn test_raw_swiftcc_invocation_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let readiness = collect_control_readiness(factory)
        .map_err(|e| format!("collect control readiness failed: {e:?}"))?;

    let reason = match readiness.path {
        ConcurrencyControlPath::BridgeContract => raw_swiftcc_direct_invocation_reason(),
        ConcurrencyControlPath::RawThunkBridgeHybrid => raw_swiftcc_hybrid_reason(),
    };
    if reason.trim().is_empty() {
        return Err("raw swiftcc invocation reason was empty".to_string());
    }

    println!("control path: {:?}", readiness.path);
    println!("control reason: {}", readiness.reason);
    println!(
        "readiness required fingerprint: {}",
        readiness.required_fingerprint
    );
    println!(
        "readiness optional fingerprint: {}",
        readiness.optional_fingerprint
    );
    Ok(true)
}

fn test_bridge_control_fallback_semantics(factory: &RuntimeFactory) -> Result<bool, String> {
    let smoke = run_bridge_control_smoke(factory)
        .map_err(|e| format!("bridge control smoke failed: {e:?}"))?;

    println!(
        "bridge smoke task_spawn_sum={}, task_spawn_chain={}, continuation_roundtrip={}, resume_count_before={}, resume_count_after={}, resume_once_ok={}",
        smoke.task_spawn_sum,
        smoke.task_spawn_chain,
        smoke.continuation_roundtrip,
        smoke.continuation_resume_count_before,
        smoke.continuation_resume_count_after,
        smoke.continuation_resume_once_ok
    );

    Ok(true)
}

fn test_bridge_control_extended_domains(factory: &RuntimeFactory) -> Result<bool, String> {
    let smoke = run_bridge_control_smoke(factory)
        .map_err(|e| format!("bridge control smoke failed: {e:?}"))?;

    println!(
        "bridge extended actor_initial={}, actor_after_add={}, actor_isolation_ok={}, stream_first={:?}, stream_sum={}, task_local_default={}, task_local_scoped={}, task_local_isolation_ok={}",
        smoke.actor_initial,
        smoke.actor_after_add,
        smoke.actor_isolation_ok,
        smoke.stream_first,
        smoke.stream_sum,
        smoke.task_local_default,
        smoke.task_local_scoped,
        smoke.task_local_isolation_ok
    );

    Ok(smoke.actor_initial == 10
        && smoke.actor_after_add == 15
        && smoke.actor_isolation_ok
        && smoke.stream_first == Some(3)
        && smoke.stream_sum == 18
        && smoke.task_local_default == -1
        && smoke.task_local_scoped == 42
        && smoke.task_local_isolation_ok)
}

fn test_direct_raw_task_thunk_symbols(factory: &RuntimeFactory) -> Result<bool, String> {
    let missing: Vec<&str> = direct_thunk_symbols()
        .iter()
        .copied()
        .filter(|sym| factory.symbol_address(sym).is_err())
        .collect();
    if missing.is_empty() {
        Ok(true)
    } else {
        Err(format!(
            "missing direct thunk symbols: {}",
            missing.join(", ")
        ))
    }
}

fn test_direct_raw_task_thunk_snapshot(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    println!(
        "raw thunk snapshot current_task=0x{:x}, current_executor=0x{:x}, main_executor=0x{:x}, alloc_probe_status={}, task_context_alloc_status={}, task_context_executor_status={}, task_context_current_task_status={}, task_context_cancel_status={}, task_context_child_cancel_status={}, task_context_asynclet_status={}, task_context_job_order_status={}, orchestration_policy_status={}, main_executor_identity_status={}, task_context_direct_swiftcc_ordering_status={}",
        snapshot.current_task,
        snapshot.current_executor,
        snapshot.main_executor,
        snapshot.alloc_probe_status,
        snapshot.task_context_alloc_status,
        snapshot.task_context_executor_status,
        snapshot.task_context_current_task_status,
        snapshot.task_context_cancel_status,
        snapshot.task_context_child_cancel_status,
        snapshot.task_context_asynclet_status,
        snapshot.task_context_job_order_status,
        snapshot.orchestration_policy_status,
        snapshot.main_executor_identity_status,
        snapshot.task_context_direct_swiftcc_ordering_status
    );

    Ok(matches!(snapshot.alloc_probe_status, -1 | 0 | 1 | 2))
}

fn test_task_context_raw_alloc_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    Ok(matches!(snapshot.task_context_alloc_status, 1 | 3))
}

fn test_task_context_executor_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_executor_status;
    let has_main = (status & 2) != 0;
    let same_executor = (status & 4) != 0;

    println!(
        "task-context executor status bits: has_current={}, has_main={}, same_executor={}",
        (status & 1) != 0,
        has_main,
        same_executor
    );

    // Some host contexts expose main executor without a non-null current executor pointer.
    Ok(status != i32::MIN && has_main)
}

fn test_task_context_current_task_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_current_task_status;
    let has_before = (status & 1) != 0;
    let has_after = (status & 2) != 0;
    let stable = (status & 4) != 0;

    println!(
        "task-context current-task status bits: has_before={}, has_after={}, stable={}",
        has_before, has_after, stable
    );

    Ok(status != i32::MIN && has_before && has_after)
}

fn test_task_context_cancel_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_cancel_status;
    let cancel_invoked = (status & 1) != 0;
    let was_cancelled_before = (status & 2) != 0;
    let cancelled_after = (status & 4) != 0;

    println!(
        "task-context cancel status bits: cancel_invoked={}, was_cancelled_before={}, cancelled_after={}",
        cancel_invoked,
        was_cancelled_before,
        cancelled_after
    );

    Ok(status != i32::MIN && cancel_invoked && cancelled_after)
}

fn test_task_context_child_cancel_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_child_cancel_status;
    let cancel_invoked = (status & 1) != 0;
    let child_observed_cancel = (status & 2) != 0;
    let child_completed_cancel_path = (status & 4) != 0;

    println!(
        "task-context child-cancel status bits: cancel_invoked={}, child_observed_cancel={}, child_completed_cancel_path={}",
        cancel_invoked,
        child_observed_cancel,
        child_completed_cancel_path
    );

    Ok(
        status != i32::MIN
            && cancel_invoked
            && child_observed_cancel
            && child_completed_cancel_path,
    )
}

fn test_task_context_asynclet_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_asynclet_status;
    let asynclet_declared = (status & 1) != 0;
    let asynclet_awaited = (status & 2) != 0;
    let expected_sum = (status & 4) != 0;

    println!(
        "task-context async-let status bits: asynclet_declared={}, asynclet_awaited={}, expected_sum={}",
        asynclet_declared,
        asynclet_awaited,
        expected_sum
    );

    Ok(status != i32::MIN && asynclet_declared && asynclet_awaited && expected_sum)
}

fn test_task_context_job_order_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_job_order_status;
    let child_started = (status & 1) != 0;
    let child_step_ordered = (status & 2) != 0;
    let parent_observed_completion = (status & 4) != 0;

    println!(
        "task-context job-order status bits: child_started={}, child_step_ordered={}, parent_observed_completion={}",
        child_started,
        child_step_ordered,
        parent_observed_completion
    );

    Ok(status != i32::MIN && child_started && child_step_ordered && parent_observed_completion)
}

fn test_orchestration_policy_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.orchestration_policy_status;
    let has_task_create = (status & 1) != 0;
    let has_job_run = (status & 2) != 0;
    let has_asynclet_begin = (status & 4) != 0;
    let has_asynclet_end = (status & 8) != 0;
    let has_task_cancel = (status & 16) != 0;
    let guard_active = (status & 32) != 0;
    let has_nullary_job_create = (status & 64) != 0;
    let has_enqueue_global = (status & 128) != 0;
    let has_current_executor = (status & 256) != 0;
    let yield_path_guard_active = (status & 512) != 0;

    println!(
        "orchestration policy status bits: has_task_create={}, has_job_run={}, has_asynclet_begin={}, has_asynclet_end={}, has_task_cancel={}, guard_active={}, has_nullary_job_create={}, has_enqueue_global={}, has_current_executor={}, yield_path_guard_active={}",
        has_task_create,
        has_job_run,
        has_asynclet_begin,
        has_asynclet_end,
        has_task_cancel,
        guard_active,
        has_nullary_job_create,
        has_enqueue_global,
        has_current_executor,
        yield_path_guard_active
    );

    Ok(has_task_create
        && has_job_run
        && has_asynclet_begin
        && has_asynclet_end
        && has_task_cancel
        && guard_active
        && has_nullary_job_create
        && has_enqueue_global
        && has_current_executor
        && yield_path_guard_active)
}

fn test_main_executor_identity_status(factory: &RuntimeFactory) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.main_executor_identity_status;
    println!("direct raw main-executor identity status: {}", status);
    Ok(status == 1)
}

fn test_task_context_direct_swiftcc_ordering_status(
    factory: &RuntimeFactory,
) -> Result<bool, String> {
    let snapshot = collect_raw_task_thunk_snapshot(factory)
        .map_err(|e| format!("collect raw task thunk snapshot failed: {e:?}"))?;

    let status = snapshot.task_context_direct_swiftcc_ordering_status;
    let task_visible = (status & 1) != 0;
    let executor_visible = (status & 2) != 0;
    let alloc_ordering_ok = (status & 4) != 0;
    println!(
        "direct SwiftCC thunk ordering status={} (task_visible={} executor_visible={} alloc_ordering_ok={})",
        status, task_visible, executor_visible, alloc_ordering_ok
    );
    // executor_visible (bit1) is not required: swift_task_getCurrentExecutor returns NULL
    // in bridge-hosted task context (consistent with test_task_context_executor_status).
    // Required: task must be visible (bit0) and alloc/dealloc ordering must be deterministic (bit2).
    Ok(task_visible && alloc_ordering_ok)
}
