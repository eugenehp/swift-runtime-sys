#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift concurrency runtime bindings (`libswift_Concurrency.dylib`).

use core::ffi::{c_char, c_void};

/// Opaque pointer to a Swift async task.
pub type AsyncTaskRef = *mut c_void;
/// Opaque pointer to an async context.
pub type AsyncContextRef = *mut c_void;
/// Opaque pointer to a Swift executor reference.
pub type ExecutorRef = *const c_void;
/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a job.
pub type JobRef = *mut c_void;
/// Opaque pointer to a task group.
pub type TaskGroupRef = *mut c_void;
/// Opaque pointer to a task option record.
pub type TaskOptionRecordRef = *mut c_void;
/// Opaque pointer to a continuation.
pub type ContinuationRef = *mut c_void;
/// Opaque pointer to an async let.
pub type AsyncLetRef = *mut c_void;
/// Opaque pointer to a default actor.
pub type DefaultActorRef = *mut c_void;
/// Opaque pointer to a Swift error.
pub type SwiftErrorRef = *mut c_void;

/// Return type for task creation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncTaskAndContext {
    pub task: AsyncTaskRef,
    pub initial_context: AsyncContextRef,
}

/// Task creation flags.
pub type TaskCreateFlags = usize;

/// Job flags.
pub type JobFlags = u32;

unsafe extern "C" {
    // ── Task lifecycle ──

    /// Create a new async task.
    pub fn swift_task_create(
        flags: TaskCreateFlags,
        options: TaskOptionRecordRef,
        function: *const c_void,
        result_type: MetadataRef,
    ) -> AsyncTaskAndContext;

    /// Create a new async task (common variant).
    pub fn swift_task_create_common(
        flags: TaskCreateFlags,
        options: TaskOptionRecordRef,
        function: *const c_void,
        result_type: MetadataRef,
        initial_context_size: usize,
    ) -> AsyncTaskAndContext;

    /// Get the current task, or null if not in a task.
    pub fn swift_task_getCurrent() -> AsyncTaskRef;

    /// Cancel a task.
    pub fn swift_task_cancel(task: AsyncTaskRef);

    /// Check if the current task is cancelled.
    pub fn swift_task_isCancelled(task: AsyncTaskRef) -> bool;

    /// Suspend the current task.
    pub fn swift_task_suspend() -> AsyncTaskRef;

    /// Switch to an executor.
    pub fn swift_task_switch(resuming: AsyncContextRef, new_executor: ExecutorRef);

    /// Create and immediately run a task.
    pub fn swift_task_immediate(
        function: *const c_void,
        context: AsyncContextRef,
        flags: TaskCreateFlags,
        options: TaskOptionRecordRef,
    );

    /// Start a task on the main actor.
    pub fn swift_task_startOnMainActor(context: AsyncContextRef);

    /// Get the base priority of a task.
    pub fn swift_task_basePriority(task: AsyncTaskRef) -> u32;

    /// Get the current priority of a task.
    pub fn swift_task_currentPriority(task: AsyncTaskRef) -> u32;

    /// Escalate the priority of a task.
    pub fn swift_task_escalate(task: AsyncTaskRef, priority: u32) -> u32;

    /// Get the job flags of a task.
    pub fn swift_task_getJobFlags(task: AsyncTaskRef) -> JobFlags;

    /// Get the task ID.
    pub fn swift_task_getJobTaskId(task: AsyncTaskRef) -> u64;

    /// Get the current task name.
    pub fn swift_task_getCurrentTaskName() -> *const c_char;

    /// Get the current thread priority.
    pub fn swift_task_getCurrentThreadPriority() -> u32;

    // ── Task scheduling ──

    /// Enqueue a task on its executor.
    pub fn swift_task_enqueue(task: AsyncTaskRef);

    /// Enqueue a job on the global executor.
    pub fn swift_task_enqueueGlobal(job: JobRef);

    /// Enqueue a job on the global executor with a delay.
    pub fn swift_task_enqueueGlobalWithDelay(delay: u64, job: JobRef);

    /// Enqueue a job on the global executor with a deadline.
    pub fn swift_task_enqueueGlobalWithDeadline(
        sec: i64,
        nsec: i64,
        tsec: i64,
        tnsec: i64,
        clock: i32,
        job: JobRef,
    );

    /// Enqueue a job on the main executor.
    pub fn swift_task_enqueueMainExecutor(job: JobRef);

    /// Enqueue a task on a dispatch queue.
    pub fn swift_task_enqueueOnDispatchQueue(job: JobRef, queue: *const c_void);

    /// Enqueue a task on a specific task executor.
    pub fn swift_task_enqueueTaskOnExecutor(task: AsyncTaskRef, executor: ExecutorRef);

    /// Drain the main queue in an async main entry point.
    pub fn swift_task_asyncMainDrainQueue() -> !;

    // ── Executor queries ──

    /// Get the main executor reference.
    pub fn swift_task_getMainExecutor() -> ExecutorRef;

    /// Get the current executor.
    pub fn swift_task_getCurrentExecutor() -> ExecutorRef;

    /// Check if currently on the given executor.
    pub fn swift_task_isCurrentExecutor(executor: ExecutorRef) -> bool;

    /// Check if currently on the given executor (with flags).
    pub fn swift_task_isCurrentExecutorWithFlags(executor: ExecutorRef, flags: usize) -> bool;

    /// Check if currently on the main executor.
    pub fn swift_task_isMainExecutor(executor: ExecutorRef) -> bool;

    /// Check if a task is on an executor.
    pub fn swift_task_isOnExecutor(identity: *const c_void, expected_executor: ExecutorRef)
        -> bool;

    /// Check that the current context is isolated to the expected executor.
    pub fn swift_task_checkIsolated(executor: ExecutorRef);

    /// Check if the current context is isolating.
    pub fn swift_task_isIsolatingCurrentContext(executor: ExecutorRef) -> bool;

    /// Report that the current executor doesn't match the expected one.
    pub fn swift_task_reportUnexpectedExecutor(
        file: *const c_char,
        line: usize,
        executor: ExecutorRef,
    );

    // ── Task memory ──

    /// Allocate memory for a task.
    pub fn swift_task_alloc(size: usize) -> *mut c_void;

    /// Deallocate task memory.
    pub fn swift_task_dealloc(ptr: *mut c_void);

    /// Deallocate through a task memory marker.
    pub fn swift_task_dealloc_through(ptr: *mut c_void);

    // ── Task locals ──

    /// Get a task-local value.
    pub fn swift_task_localValueGet(key: *const c_void) -> *mut c_void;

    /// Push a task-local value.
    pub fn swift_task_localValuePush(
        key: *const c_void,
        value: *mut c_void,
        value_type: MetadataRef,
    );

    /// Pop a task-local value.
    pub fn swift_task_localValuePop();

    /// Copy task locals to another task.
    pub fn swift_task_localsCopyTo(target: AsyncTaskRef);

    /// Report illegal task-local binding within withTaskGroup.
    pub fn swift_task_reportIllegalTaskLocalBindingWithinWithTaskGroup(key: *const c_void);

    // ── Task executor preference ──

    /// Get the preferred task executor.
    pub fn swift_task_getPreferredTaskExecutor() -> ExecutorRef;

    /// Push a task executor preference.
    pub fn swift_task_pushTaskExecutorPreference(executor: ExecutorRef);

    /// Pop a task executor preference.
    pub fn swift_task_popTaskExecutorPreference();

    // ── Futures ──

    /// Wait for a future result.
    pub fn swift_task_future_wait(
        result: *mut c_void,
        callerContext: AsyncContextRef,
        task: AsyncTaskRef,
        resumeFunction: *const c_void,
    );

    /// Wait for a throwing future result.
    pub fn swift_task_future_wait_throwing(
        result: *mut c_void,
        callerContext: AsyncContextRef,
        task: AsyncTaskRef,
        resumeFunction: *const c_void,
    );

    // ── Continuations ──

    /// Initialize a continuation.
    pub fn swift_continuation_init(continuation: ContinuationRef, flags: u32) -> AsyncTaskRef;

    /// Await a continuation.
    pub fn swift_continuation_await(continuation: ContinuationRef);

    /// Resume a continuation with a value.
    pub fn swift_continuation_resume(continuation: ContinuationRef);

    /// Resume a throwing continuation with a value.
    pub fn swift_continuation_throwingResume(continuation: ContinuationRef);

    /// Resume a throwing continuation with an error.
    pub fn swift_continuation_throwingResumeWithError(
        continuation: ContinuationRef,
        error: SwiftErrorRef,
    );

    // ── Task groups ──

    /// Initialize a task group.
    pub fn swift_taskGroup_initialize(flags: usize, group: TaskGroupRef);

    /// Initialize a task group with flags.
    pub fn swift_taskGroup_initializeWithFlags(flags: usize, group: TaskGroupRef);

    /// Initialize a task group with options.
    pub fn swift_taskGroup_initializeWithOptions(
        flags: usize,
        group: TaskGroupRef,
        options: *const c_void,
    );

    /// Destroy a task group.
    pub fn swift_taskGroup_destroy(group: TaskGroupRef);

    /// Add a pending task to the group.
    pub fn swift_taskGroup_addPending(group: TaskGroupRef, unconditionally: bool) -> bool;

    /// Attach a child task to the group.
    pub fn swift_taskGroup_attachChild(group: TaskGroupRef, child: AsyncTaskRef);

    /// Cancel all tasks in the group.
    pub fn swift_taskGroup_cancelAll(group: TaskGroupRef);

    /// Check if the group is cancelled.
    pub fn swift_taskGroup_isCancelled(group: TaskGroupRef) -> bool;

    /// Check if the group is empty.
    pub fn swift_taskGroup_isEmpty(group: TaskGroupRef) -> bool;

    /// Wait for all tasks in the group.
    pub fn swift_taskGroup_waitAll(
        result_buf: *mut c_void,
        callerContext: AsyncContextRef,
        group: TaskGroupRef,
        result_type: MetadataRef,
        resumeFunction: *const c_void,
    );

    /// Wait for the next throwing result.
    pub fn swift_taskGroup_wait_next_throwing(
        result_buf: *mut c_void,
        callerContext: AsyncContextRef,
        group: TaskGroupRef,
        result_type: MetadataRef,
        resumeFunction: *const c_void,
    );

    /// Cancel child tasks in a group.
    pub fn swift_task_cancel_group_child_tasks(group: TaskGroupRef);

    /// Check if a task has a task group status record.
    pub fn swift_task_hasTaskGroupStatusRecord(task: AsyncTaskRef) -> bool;

    // ── Async let ──

    /// Begin an async let.
    pub fn swift_asyncLet_begin(
        async_let: AsyncLetRef,
        options: TaskOptionRecordRef,
        entry_point: *const c_void,
        context: *mut c_void,
    );

    /// Start an async let.
    pub fn swift_asyncLet_start(
        async_let: AsyncLetRef,
        options: TaskOptionRecordRef,
        entry_point: *const c_void,
        context: *mut c_void,
    );

    /// Get the result of an async let.
    pub fn swift_asyncLet_get(
        async_let: AsyncLetRef,
        result: *mut c_void,
        resumeFunction: *const c_void,
    );

    /// Get the result of a throwing async let.
    pub fn swift_asyncLet_get_throwing(
        async_let: AsyncLetRef,
        result: *mut c_void,
        resumeFunction: *const c_void,
    );

    /// Consume the result of an async let.
    pub fn swift_asyncLet_consume(
        async_let: AsyncLetRef,
        result: *mut c_void,
        resumeFunction: *const c_void,
    );

    /// Consume the result of a throwing async let.
    pub fn swift_asyncLet_consume_throwing(
        async_let: AsyncLetRef,
        result: *mut c_void,
        resumeFunction: *const c_void,
    );

    /// Finish an async let.
    pub fn swift_asyncLet_finish(async_let: AsyncLetRef, resumeFunction: *const c_void);

    /// End an async let.
    pub fn swift_asyncLet_end(async_let: AsyncLetRef);

    /// Wait for an async let.
    pub fn swift_asyncLet_wait(async_let: AsyncLetRef, resumeFunction: *const c_void);

    /// Wait for a throwing async let.
    pub fn swift_asyncLet_wait_throwing(async_let: AsyncLetRef, resumeFunction: *const c_void);

    // ── Actors ──

    /// Initialize a default actor.
    pub fn swift_defaultActor_initialize(actor: DefaultActorRef);

    /// Destroy a default actor.
    pub fn swift_defaultActor_destroy(actor: DefaultActorRef);

    /// Deallocate a default actor.
    pub fn swift_defaultActor_deallocate(actor: DefaultActorRef);

    /// Deallocate a resilient default actor.
    pub fn swift_defaultActor_deallocateResilient(actor: DefaultActorRef);

    /// Enqueue a job on a default actor.
    pub fn swift_defaultActor_enqueue(job: JobRef, actor: DefaultActorRef);

    /// Initialize a remote distributed actor.
    pub fn swift_distributedActor_remote_initialize(actor_type: MetadataRef) -> *mut c_void;

    /// Check if a distributed actor is remote.
    pub fn swift_distributed_actor_is_remote(actor: *const c_void) -> bool;

    /// Initialize a non-default distributed actor.
    pub fn swift_nonDefaultDistributedActor_initialize(actor_type: MetadataRef) -> *mut c_void;

    // ── Jobs ──

    /// Run a job.
    pub fn swift_job_run(job: JobRef, executor: ExecutorRef);

    /// Run a job on a serial and task executor.
    pub fn swift_job_run_on_serial_and_task_executor(
        job: JobRef,
        serial_executor: ExecutorRef,
        task_executor: ExecutorRef,
    );

    /// Run a job on a task executor.
    pub fn swift_job_run_on_task_executor(job: JobRef, executor: ExecutorRef);

    /// Allocate a job.
    pub fn swift_job_allocate(flags: JobFlags, function: *const c_void) -> JobRef;

    /// Deallocate a job.
    pub fn swift_job_deallocate(job: JobRef);

    /// Create a nullary continuation job.
    pub fn swift_task_createNullaryContinuationJob(
        priority: u32,
        continuation: ContinuationRef,
    ) -> JobRef;

    /// Deinit on executor.
    pub fn swift_task_deinitOnExecutor(object: *mut c_void, executor: ExecutorRef);

    // ── Clock / sleep ──

    /// Get the current time.
    pub fn swift_get_time(seconds: *mut i64, nanoseconds: *mut i64, clock: i32);

    /// Get the clock resolution.
    pub fn swift_get_clock_res(seconds: *mut i64, nanoseconds: *mut i64, clock: i32);

    /// Sleep for a duration.
    pub fn swift_sleep(
        wake_time_seconds: i64,
        wake_time_nanoseconds: i64,
        tolerance_seconds: i64,
        tolerance_nanoseconds: i64,
        clock: i32,
        context: AsyncContextRef,
        resume_function: *const c_void,
    );

    /// Donate the current thread to the global executor.
    pub fn swift_task_donateThreadToGlobalExecutorUntil(condition: *const c_void);

    // ── Misc concurrency ──

    /// Get the priority of a job.
    pub fn swift_concurrency_jobPriority(job: JobRef) -> u32;

    /// Enter a thread-local execution context.
    pub fn swift_task_enterThreadLocalContext(context: *mut c_void);

    /// Exit a thread-local execution context.
    pub fn swift_task_exitThreadLocalContext(context: *mut c_void);

    /// Register the concurrency runtime.
    pub fn swift_registerConcurrencyRuntime();

    /// Deleted async method error.
    pub fn swift_deletedAsyncMethodError() -> !;

    /// Check if complex executor equality is needed.
    pub fn swift_executor_isComplexEquality(executor: ExecutorRef) -> bool;

    /// Use legacy non-crashing executor checks.
    pub fn swift_bincompat_useLegacyNonCrashingExecutorChecks() -> bool;

    /// Get witness tables for distributed actor.
    pub fn swift_distributed_getWitnessTables(
        actor_type: MetadataRef,
        protocol_type: MetadataRef,
        witness_tables: *mut *const c_void,
    );
}

// ── Executor hooks (global mutable function pointers) ──

// These are global function pointers that can be overridden to customize
// Swift concurrency scheduling. Access via dlsym at runtime.
//
// swift_task_enqueueGlobal_hook
// swift_task_enqueueGlobalWithDelay_hook
// swift_task_enqueueGlobalWithDeadline_hook
// swift_task_enqueueMainExecutor_hook
// swift_task_getMainExecutor_hook
// swift_task_asyncMainDrainQueue_hook
// swift_task_checkIsolated_hook
// swift_task_isIsolatingCurrentContext_hook
// swift_task_isOnExecutor_hook
// swift_task_donateThreadToGlobalExecutorUntil_hook
// swift_task_isMainExecutor_hook
//
// To set these from Rust, use dlsym to find the global and write to it:
//   let hook: *mut Option<unsafe extern "C" fn(...)> = dlsym(RTLD_DEFAULT, "swift_task_enqueueGlobal_hook");

// ── Debug variables (access via dlsym) ──
//
// swift_concurrency_debug_asyncTaskMetadata
// swift_concurrency_debug_asyncTaskSize
// swift_concurrency_debug_asyncTaskSlabMetadata
// swift_concurrency_debug_future_adapter
// swift_concurrency_debug_internal_layout_version
// swift_concurrency_debug_jobMetadata
// swift_concurrency_debug_non_future_adapter
// swift_concurrency_debug_supportsPriorityEscalation
// swift_concurrency_debug_task_future_wait_resume_adapter
// swift_concurrency_debug_task_wait_throwing_resume_adapter
