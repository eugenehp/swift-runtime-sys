use std::collections::BTreeMap;

use crate::RuntimeContract::{RuntimeContract, RuntimeContractError};
use crate::RuntimeFactory::{RuntimeFactory, RuntimeFactoryError};

#[derive(Debug)]
pub enum ConcurrencyAbiError {
    Factory(RuntimeFactoryError),
    Contract(RuntimeContractError),
    Invariant(String),
}

impl From<RuntimeFactoryError> for ConcurrencyAbiError {
    fn from(value: RuntimeFactoryError) -> Self {
        Self::Factory(value)
    }
}

impl From<RuntimeContractError> for ConcurrencyAbiError {
    fn from(value: RuntimeContractError) -> Self {
        Self::Contract(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcurrencyControlPath {
    BridgeContract,
    RawThunkBridgeHybrid,
}

#[derive(Debug, Clone)]
pub struct ConcurrencyControlReadiness {
    pub path: ConcurrencyControlPath,
    pub reason: String,
    pub required_fingerprint: String,
    pub optional_fingerprint: String,
}

#[derive(Debug, Clone)]
pub struct RawTaskThunkSnapshot {
    pub current_task: usize,
    pub current_executor: usize,
    pub main_executor: usize,
    pub alloc_probe_status: i32,
    pub task_context_alloc_status: i32,
    pub task_context_executor_status: i32,
    pub task_context_current_task_status: i32,
    pub task_context_cancel_status: i32,
    pub task_context_child_cancel_status: i32,
    pub task_context_asynclet_status: i32,
    pub task_context_job_order_status: i32,
    pub orchestration_policy_status: i32,
    pub main_executor_identity_status: i32,
    pub task_context_direct_swiftcc_ordering_status: i32,
}

#[derive(Debug, Clone)]
pub struct BridgeControlSmoke {
    pub task_spawn_sum: i32,
    pub task_spawn_chain: i32,
    pub continuation_roundtrip: i32,
    pub continuation_resume_count_before: i32,
    pub continuation_resume_count_after: i32,
    pub continuation_resume_once_ok: bool,
    pub actor_initial: i32,
    pub actor_after_add: i32,
    pub actor_isolation_ok: bool,
    pub stream_first: Option<i32>,
    pub stream_sum: i32,
    pub task_local_default: i32,
    pub task_local_scoped: i32,
    pub task_local_isolation_ok: bool,
}

#[derive(Debug, Clone)]
pub struct ConcurrencyAbiProfile {
    pub required: BTreeMap<String, usize>,
    pub optional: BTreeMap<String, usize>,
    pub missing_required: Vec<String>,
}

impl ConcurrencyAbiProfile {
    pub fn required_count(&self) -> usize {
        self.required.len()
    }

    pub fn optional_count(&self) -> usize {
        self.optional.len()
    }

    pub fn has_all_required(&self) -> bool {
        self.missing_required.is_empty()
    }

    pub fn required_fingerprint(&self) -> String {
        format!("{}/{}", self.required_count(), required_symbols().len())
    }

    pub fn optional_fingerprint(&self) -> String {
        format!("{}/{}", self.optional_count(), optional_symbols().len())
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.required.contains_key(symbol) || self.optional.contains_key(symbol)
    }
}

pub fn required_symbols() -> &'static [&'static str] {
    &[
        "swift_task_getCurrent",
        "swift_task_alloc",
        "swift_task_dealloc",
        "swift_task_cancel",
        "swift_task_create",
        "swift_continuation_init",
        "swift_continuation_resume",
        "swift_continuation_throwingResume",
        "swift_asyncLet_start",
        "swift_asyncLet_begin",
        "swift_asyncLet_end",
        "swift_job_run",
    ]
}

pub fn optional_symbols() -> &'static [&'static str] {
    &[
        "swift_task_run_inline",
        "swift_continuation_throwingResumeWithError",
        "swift_task_getMainExecutor",
        "swift_task_getCurrentExecutor",
        "swift_task_dealloc_through",
    ]
}

pub fn collect_profile(
    factory: &RuntimeFactory,
) -> Result<ConcurrencyAbiProfile, RuntimeFactoryError> {
    let mut required = BTreeMap::new();
    let mut optional = BTreeMap::new();
    let mut missing_required = Vec::new();

    for symbol in required_symbols() {
        match factory.symbol_address(symbol) {
            Ok(addr) => {
                required.insert((*symbol).to_string(), addr as usize);
            }
            Err(_) => {
                missing_required.push((*symbol).to_string());
            }
        }
    }

    for symbol in optional_symbols() {
        if let Ok(addr) = factory.symbol_address(symbol) {
            optional.insert((*symbol).to_string(), addr as usize);
        }
    }

    Ok(ConcurrencyAbiProfile {
        required,
        optional,
        missing_required,
    })
}

pub fn raw_swiftcc_direct_invocation_reason() -> &'static str {
    "swift_task_*/swift_continuation_*/swift_asyncLet_*/swift_job_* use SWIFT_CC(swift); this crate currently executes O.2 control semantics through versioned bridge exports rather than direct raw Swift calling-convention invocation"
}

pub fn raw_swiftcc_hybrid_reason() -> &'static str {
    "direct raw SwiftCC thunk path is available for safe task inspection primitives (current task/current executor/main executor plus guarded alloc/dealloc probe); higher-risk task create/cancel/async-let/job orchestration still uses bridge-contract fallback"
}

pub fn direct_thunk_symbols() -> &'static [&'static str] {
    &[
        "runtime_thunk_swift_task_get_current",
        "runtime_thunk_swift_task_get_current_executor",
        "runtime_thunk_swift_task_get_main_executor",
        "runtime_thunk_swift_task_alloc",
        "runtime_thunk_swift_task_dealloc",
        "runtime_thunk_swift_task_alloc_probe",
        "runtime_thunk_swift_task_cancel_current_probe",
        "runtime_thunk_swift_task_cancel_task",
        "runtime_thunk_swift_concurrency_orchestration_policy_status",
        "runtime_thunk_swift_main_executor_identity_probe",
        "runtime_thunk_swift_task_direct_ordering_probe",
    ]
}

pub fn collect_control_readiness(
    factory: &RuntimeFactory,
) -> Result<ConcurrencyControlReadiness, ConcurrencyAbiError> {
    let profile = collect_profile(factory)?;
    let direct_thunks_ready = direct_thunk_symbols()
        .iter()
        .all(|symbol| factory.symbol_address(symbol).is_ok());
    Ok(ConcurrencyControlReadiness {
        path: if direct_thunks_ready {
            ConcurrencyControlPath::RawThunkBridgeHybrid
        } else {
            ConcurrencyControlPath::BridgeContract
        },
        reason: if direct_thunks_ready {
            raw_swiftcc_hybrid_reason().to_string()
        } else {
            raw_swiftcc_direct_invocation_reason().to_string()
        },
        required_fingerprint: profile.required_fingerprint(),
        optional_fingerprint: profile.optional_fingerprint(),
    })
}

pub fn collect_raw_task_thunk_snapshot(
    factory: &RuntimeFactory,
) -> Result<RawTaskThunkSnapshot, ConcurrencyAbiError> {
    Ok(RawTaskThunkSnapshot {
        current_task: factory.call_to_ptr("runtime_thunk_swift_task_get_current")? as usize,
        current_executor: factory.call_to_ptr("runtime_thunk_swift_task_get_current_executor")?
            as usize,
        main_executor: factory.call_to_ptr("runtime_thunk_swift_task_get_main_executor")? as usize,
        alloc_probe_status: factory
            .call_usize_to_i32("runtime_thunk_swift_task_alloc_probe", 64)?,
        task_context_alloc_status: factory
            .call_to_i32("swift_contract_task_context_raw_alloc_status")?,
        task_context_executor_status: factory
            .call_to_i32("swift_contract_task_context_executor_status")?,
        task_context_current_task_status: factory
            .call_to_i32("swift_contract_task_context_current_task_status")?,
        task_context_cancel_status: factory
            .call_to_i32("swift_contract_task_context_cancel_status")?,
        task_context_child_cancel_status: factory
            .call_to_i32("swift_contract_task_context_child_cancel_status")?,
        task_context_asynclet_status: factory
            .call_to_i32("swift_contract_task_context_asynclet_status")?,
        task_context_job_order_status: factory
            .call_to_i32("swift_contract_task_context_job_order_status")?,
        orchestration_policy_status: factory
            .call_to_i32("runtime_thunk_swift_concurrency_orchestration_policy_status")?,
        main_executor_identity_status: factory
            .call_to_i32("runtime_thunk_swift_main_executor_identity_probe")?,
        task_context_direct_swiftcc_ordering_status: factory
            .call_to_i32("swift_contract_direct_swiftcc_ordering_probe")?,
    })
}

pub fn run_bridge_control_smoke(
    factory: &RuntimeFactory,
) -> Result<BridgeControlSmoke, ConcurrencyAbiError> {
    let contract = RuntimeContract::new(factory);

    let task_spawn_sum = contract.task_spawn_sum(20, 22)?;
    if task_spawn_sum != 42 {
        return Err(ConcurrencyAbiError::Invariant(format!(
            "task_spawn_sum expected 42, got {task_spawn_sum}"
        )));
    }

    let task_spawn_chain = contract.task_spawn_chain(5, 3)?;

    contract.continuation_reset()?;
    let before = contract.continuation_resume_count()?;
    let continuation_roundtrip = contract.continuation_roundtrip(77)?;
    let after = contract.continuation_resume_count()?;
    if continuation_roundtrip != 77 {
        return Err(ConcurrencyAbiError::Invariant(format!(
            "continuation_roundtrip expected 77, got {continuation_roundtrip}"
        )));
    }
    if after < (before + 1) {
        return Err(ConcurrencyAbiError::Invariant(format!(
            "continuation resume count did not increase: before={before}, after={after}"
        )));
    }

    let continuation_resume_once_ok = contract.continuation_validate_resume_once()?;
    if !continuation_resume_once_ok {
        return Err(ConcurrencyAbiError::Invariant(
            "continuation resume-once validation returned false".to_string(),
        ));
    }

    let actor = contract.construct_actor(10)?;
    let actor_initial = contract.actor_current(actor)?;
    if actor_initial != 10 {
        let _ = contract.release(actor);
        return Err(ConcurrencyAbiError::Invariant(format!(
            "actor current expected 10, got {actor_initial}"
        )));
    }
    let actor_after_add = contract.actor_add(actor, 5)?;
    if actor_after_add != 15 {
        let _ = contract.release(actor);
        return Err(ConcurrencyAbiError::Invariant(format!(
            "actor add expected 15, got {actor_after_add}"
        )));
    }
    let actor_isolation_ok = contract.actor_validate_isolation(actor)?;
    contract.release(actor)?;
    if !actor_isolation_ok {
        return Err(ConcurrencyAbiError::Invariant(
            "actor isolation validation returned false".to_string(),
        ));
    }

    let stream = contract.construct_stream(3, 4)?;
    let stream_first = contract.stream_next(stream)?;
    let expected_first = Some(3);
    if stream_first != expected_first {
        let _ = contract.release(stream);
        return Err(ConcurrencyAbiError::Invariant(format!(
            "stream first expected {:?}, got {:?}",
            expected_first, stream_first
        )));
    }
    contract.release(stream)?;
    // 3 + 4 + 5 + 6 = 18
    let stream_sum = contract.stream_collect_sum(3, 4)?;
    if stream_sum != 18 {
        return Err(ConcurrencyAbiError::Invariant(format!(
            "stream sum expected 18, got {stream_sum}"
        )));
    }

    let task_local_default = contract.task_local_get_default()?;
    if task_local_default != -1 {
        return Err(ConcurrencyAbiError::Invariant(format!(
            "task-local default expected -1, got {task_local_default}"
        )));
    }
    let task_local_scoped = contract.task_local_run_with(30, 12)?;
    if task_local_scoped != 42 {
        return Err(ConcurrencyAbiError::Invariant(format!(
            "task-local scoped expected 42, got {task_local_scoped}"
        )));
    }
    let task_local_isolation_ok = contract.task_local_isolation_check(17)?;
    if !task_local_isolation_ok {
        return Err(ConcurrencyAbiError::Invariant(
            "task-local isolation check returned false".to_string(),
        ));
    }

    Ok(BridgeControlSmoke {
        task_spawn_sum,
        task_spawn_chain,
        continuation_roundtrip,
        continuation_resume_count_before: before,
        continuation_resume_count_after: after,
        continuation_resume_once_ok,
        actor_initial,
        actor_after_add,
        actor_isolation_ok,
        stream_first,
        stream_sum,
        task_local_default,
        task_local_scoped,
        task_local_isolation_ok,
    })
}
