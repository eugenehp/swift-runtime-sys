use std::env;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::ConcurrencyAbi::{collect_raw_task_thunk_snapshot, ConcurrencyAbiError};
use crate::RuntimeContract::{RuntimeContract, RuntimeContractError};
use crate::RuntimeFactory::{RuntimeFactory, RuntimeFactoryError};

pub const O8_ENABLE_ENV: &str = "SWIFT_RUNTIME_O8_ENABLE";

#[derive(Debug)]
pub enum RustExecutorInteropError {
    Disabled(String),
    Factory(RuntimeFactoryError),
    Contract(RuntimeContractError),
    Abi(ConcurrencyAbiError),
    ChannelClosed,
    Worker(String),
    Timeout(String),
}

impl From<RuntimeFactoryError> for RustExecutorInteropError {
    fn from(value: RuntimeFactoryError) -> Self {
        Self::Factory(value)
    }
}

impl From<RuntimeContractError> for RustExecutorInteropError {
    fn from(value: RuntimeContractError) -> Self {
        Self::Contract(value)
    }
}

impl From<ConcurrencyAbiError> for RustExecutorInteropError {
    fn from(value: ConcurrencyAbiError) -> Self {
        Self::Abi(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustExecutorJobKind {
    Delay,
    TaskSpawnSum,
    TaskSpawnChain,
    CancellationVisibility,
}

impl RustExecutorJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Delay => "delay",
            Self::TaskSpawnSum => "task_spawn_sum",
            Self::TaskSpawnChain => "task_spawn_chain",
            Self::CancellationVisibility => "cancellation_visibility",
        }
    }
}

#[derive(Debug)]
pub enum RustExecutorJob {
    Delay { millis: u64 },
    TaskSpawnSum { lhs: i32, rhs: i32 },
    TaskSpawnChain { base: i32, steps: i32 },
    CancellationVisibility,
}

impl RustExecutorJob {
    fn kind(&self) -> RustExecutorJobKind {
        match self {
            Self::Delay { .. } => RustExecutorJobKind::Delay,
            Self::TaskSpawnSum { .. } => RustExecutorJobKind::TaskSpawnSum,
            Self::TaskSpawnChain { .. } => RustExecutorJobKind::TaskSpawnChain,
            Self::CancellationVisibility => RustExecutorJobKind::CancellationVisibility,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustExecutorJobReport {
    pub sequence: u64,
    pub completion_order: u64,
    pub kind: RustExecutorJobKind,
    pub status: String,
    pub value: i32,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct RustExecutorInteropConfig {
    pub enabled: bool,
    pub result_timeout: Duration,
    pub shutdown_timeout: Duration,
}

impl Default for RustExecutorInteropConfig {
    fn default() -> Self {
        Self {
            enabled: o8_executor_enabled_from_env(),
            result_timeout: Duration::from_secs(3),
            shutdown_timeout: Duration::from_secs(2),
        }
    }
}

pub fn o8_executor_enabled_from_env() -> bool {
    match env::var(O8_ENABLE_ENV) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}

pub fn o8_executor_policy_reason() -> &'static str {
    "O.8 stays default-off and out of required parity until an explicit opt-in flag enables the experimental Rust-owned executor path."
}

struct WorkItem {
    sequence: u64,
    job: RustExecutorJob,
    cancelled: Arc<AtomicBool>,
    result_tx: Sender<Result<RustExecutorJobReport, RustExecutorInteropError>>,
}

enum WorkerMessage {
    Execute(WorkItem),
    Shutdown(Sender<Result<(), RustExecutorInteropError>>),
}

pub struct RustExecutorJobHandle {
    pub sequence: u64,
    cancelled: Arc<AtomicBool>,
    result_rx: Receiver<Result<RustExecutorJobReport, RustExecutorInteropError>>,
}

impl RustExecutorJobHandle {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn await_result(
        self,
        timeout: Duration,
    ) -> Result<RustExecutorJobReport, RustExecutorInteropError> {
        match self.result_rx.recv_timeout(timeout) {
            Ok(result) => result,
            Err(RecvTimeoutError::Timeout) => Err(RustExecutorInteropError::Timeout(format!(
                "timed out waiting for executor job {}",
                self.sequence
            ))),
            Err(RecvTimeoutError::Disconnected) => Err(RustExecutorInteropError::ChannelClosed),
        }
    }
}

pub struct RustOwnedExecutor {
    sender: Sender<WorkerMessage>,
    worker: Option<JoinHandle<Result<(), RustExecutorInteropError>>>,
    next_sequence: AtomicU64,
    result_timeout: Duration,
    shutdown_timeout: Duration,
}

impl RustOwnedExecutor {
    pub fn spawn(
        swift_library_path: &str,
        thunk_library_path: &str,
    ) -> Result<Self, RustExecutorInteropError> {
        Self::spawn_with_config(
            RustExecutorInteropConfig::default(),
            swift_library_path,
            thunk_library_path,
        )
    }

    pub fn spawn_with_config(
        config: RustExecutorInteropConfig,
        swift_library_path: &str,
        thunk_library_path: &str,
    ) -> Result<Self, RustExecutorInteropError> {
        if !config.enabled {
            return Err(RustExecutorInteropError::Disabled(
                o8_executor_policy_reason().to_string(),
            ));
        }

        let (tx, rx) = mpsc::channel::<WorkerMessage>();
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), RustExecutorInteropError>>();
        let swift_library_path = swift_library_path.to_string();
        let thunk_library_path = thunk_library_path.to_string();

        let worker = thread::Builder::new()
            .name("o8-rust-executor".to_string())
            .spawn(move || {
                let factory =
                    RuntimeFactory::with_thunk_library(&swift_library_path, &thunk_library_path)
                        .or_else(|_| RuntimeFactory::new(&swift_library_path))
                        .map_err(RustExecutorInteropError::Factory)?;
                factory
                    .validate_runtime_contract(1)
                    .map_err(RustExecutorInteropError::Factory)?;
                let contract = RuntimeContract::new(&factory);
                let _ = ready_tx.send(Ok(()));

                let mut completion_order = 0u64;
                while let Ok(message) = rx.recv() {
                    match message {
                        WorkerMessage::Execute(item) => {
                            completion_order += 1;
                            if item.cancelled.load(Ordering::SeqCst) {
                                let _ = item.result_tx.send(Ok(RustExecutorJobReport {
                                    sequence: item.sequence,
                                    completion_order,
                                    kind: item.job.kind(),
                                    status: "cancelled".to_string(),
                                    value: 0,
                                    detail: "job cancelled before dispatch".to_string(),
                                }));
                                continue;
                            }

                            let result = execute_job(
                                &factory,
                                &contract,
                                item.sequence,
                                completion_order,
                                item.job,
                            );
                            let _ = item.result_tx.send(result);
                        }
                        WorkerMessage::Shutdown(ack_tx) => {
                            let _ = ack_tx.send(Ok(()));
                            break;
                        }
                    }
                }

                Ok(())
            })
            .map_err(|error| RustExecutorInteropError::Worker(error.to_string()))?;

        match ready_rx.recv_timeout(config.result_timeout) {
            Ok(Ok(())) => Ok(Self {
                sender: tx,
                worker: Some(worker),
                next_sequence: AtomicU64::new(1),
                result_timeout: config.result_timeout,
                shutdown_timeout: config.shutdown_timeout,
            }),
            Ok(Err(error)) => {
                let _ = worker.join();
                Err(error)
            }
            Err(RecvTimeoutError::Timeout) => {
                let _ = worker.join();
                Err(RustExecutorInteropError::Timeout(
                    "timed out waiting for O.8 executor worker initialization".to_string(),
                ))
            }
            Err(RecvTimeoutError::Disconnected) => {
                let _ = worker.join();
                Err(RustExecutorInteropError::ChannelClosed)
            }
        }
    }

    pub fn submit(
        &self,
        job: RustExecutorJob,
    ) -> Result<RustExecutorJobHandle, RustExecutorInteropError> {
        let sequence = self.next_sequence.fetch_add(1, Ordering::SeqCst);
        let cancelled = Arc::new(AtomicBool::new(false));
        let (result_tx, result_rx) = mpsc::channel();
        self.sender
            .send(WorkerMessage::Execute(WorkItem {
                sequence,
                job,
                cancelled: cancelled.clone(),
                result_tx,
            }))
            .map_err(|_| RustExecutorInteropError::ChannelClosed)?;
        Ok(RustExecutorJobHandle {
            sequence,
            cancelled,
            result_rx,
        })
    }

    pub fn result_timeout(&self) -> Duration {
        self.result_timeout
    }

    pub fn shutdown(mut self) -> Result<Duration, RustExecutorInteropError> {
        let started = Instant::now();
        let (ack_tx, ack_rx) = mpsc::channel();
        self.sender
            .send(WorkerMessage::Shutdown(ack_tx))
            .map_err(|_| RustExecutorInteropError::ChannelClosed)?;

        match ack_rx.recv_timeout(self.shutdown_timeout) {
            Ok(Ok(())) => {}
            Ok(Err(error)) => return Err(error),
            Err(RecvTimeoutError::Timeout) => {
                return Err(RustExecutorInteropError::Timeout(
                    "timed out waiting for O.8 executor shutdown ack".to_string(),
                ));
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Err(RustExecutorInteropError::ChannelClosed);
            }
        }

        if let Some(worker) = self.worker.take() {
            match worker.join() {
                Ok(Ok(())) => Ok(started.elapsed()),
                Ok(Err(error)) => Err(error),
                Err(_) => Err(RustExecutorInteropError::Worker(
                    "O.8 executor worker panicked".to_string(),
                )),
            }
        } else {
            Ok(started.elapsed())
        }
    }
}

fn execute_job(
    factory: &RuntimeFactory,
    contract: &RuntimeContract,
    sequence: u64,
    completion_order: u64,
    job: RustExecutorJob,
) -> Result<RustExecutorJobReport, RustExecutorInteropError> {
    match job {
        RustExecutorJob::Delay { millis } => {
            thread::sleep(Duration::from_millis(millis));
            Ok(RustExecutorJobReport {
                sequence,
                completion_order,
                kind: RustExecutorJobKind::Delay,
                status: "completed".to_string(),
                value: millis as i32,
                detail: format!("delay completed after {} ms", millis),
            })
        }
        RustExecutorJob::TaskSpawnSum { lhs, rhs } => {
            let value = contract.task_spawn_sum(lhs, rhs)?;
            Ok(RustExecutorJobReport {
                sequence,
                completion_order,
                kind: RustExecutorJobKind::TaskSpawnSum,
                status: "completed".to_string(),
                value,
                detail: format!("swift task sum {} + {} = {}", lhs, rhs, value),
            })
        }
        RustExecutorJob::TaskSpawnChain { base, steps } => {
            let value = contract.task_spawn_chain(base, steps)?;
            Ok(RustExecutorJobReport {
                sequence,
                completion_order,
                kind: RustExecutorJobKind::TaskSpawnChain,
                status: "completed".to_string(),
                value,
                detail: format!(
                    "swift task chain base={} steps={} => {}",
                    base, steps, value
                ),
            })
        }
        RustExecutorJob::CancellationVisibility => {
            let snapshot = collect_raw_task_thunk_snapshot(factory)?;
            let status = snapshot.task_context_cancel_status;
            Ok(RustExecutorJobReport {
                sequence,
                completion_order,
                kind: RustExecutorJobKind::CancellationVisibility,
                status: "completed".to_string(),
                value: status,
                detail: format!(
                    "cancel_invoked={} cancelled_before={} cancelled_after={}",
                    (status & 1) != 0,
                    (status & 2) != 0,
                    (status & 4) != 0,
                ),
            })
        }
    }
}
