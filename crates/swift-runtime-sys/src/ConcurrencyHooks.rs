#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift concurrency executor hooks.
//!
//! These are **global mutable function pointers** exported from
//! `libswift_Concurrency.dylib`. Setting them lets you replace the Swift
//! executor with your own — e.g., a Rust async runtime like tokio.
//!
//! Each hook has the signature:
//!   `fn(...original_args, original: fn(...original_args) -> R) -> R`
//!
//! The hook receives the original function as a parameter so it can call
//! through when it doesn't want to handle the operation itself.
//!
//! # Usage from Rust
//!
//! ```ignore
//! use std::ffi::{CStr, c_void};
//!
//! // Get the hook pointer via dlsym
//! let hook_ptr = dlsym(RTLD_DEFAULT, c"swift_task_enqueueGlobal_hook".as_ptr());
//! let hook = hook_ptr as *mut Option<EnqueueGlobalHook>;
//!
//! // Install your hook
//! unsafe { *hook = Some(my_enqueue_global_hook); }
//! ```

use core::ffi::c_void;

/// Job reference (opaque pointer to a Swift Job).
pub type JobRef = *mut c_void;

/// Serial executor reference (2 words on arm64).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SerialExecutorRef {
    pub identity: *const c_void,
    pub implementation: *const c_void,
}

// ── Hook type definitions ──

/// Original function type for enqueueGlobal.
pub type EnqueueGlobalOriginal = unsafe extern "C" fn(job: JobRef);
/// Hook type for swift_task_enqueueGlobal.
pub type EnqueueGlobalHook = unsafe extern "C" fn(job: JobRef, original: EnqueueGlobalOriginal);

/// Original function type for enqueueGlobalWithDelay.
pub type EnqueueGlobalWithDelayOriginal = unsafe extern "C" fn(delay: u64, job: JobRef);
/// Hook type for swift_task_enqueueGlobalWithDelay.
pub type EnqueueGlobalWithDelayHook =
    unsafe extern "C" fn(delay: u64, job: JobRef, original: EnqueueGlobalWithDelayOriginal);

/// Original function type for enqueueGlobalWithDeadline.
pub type EnqueueGlobalWithDeadlineOriginal =
    unsafe extern "C" fn(sec: i64, nsec: i64, tsec: i64, tnsec: i64, clock: i32, job: JobRef);
/// Hook type for swift_task_enqueueGlobalWithDeadline.
pub type EnqueueGlobalWithDeadlineHook = unsafe extern "C" fn(
    sec: i64,
    nsec: i64,
    tsec: i64,
    tnsec: i64,
    clock: i32,
    job: JobRef,
    original: EnqueueGlobalWithDeadlineOriginal,
);

/// Original function type for enqueueMainExecutor.
pub type EnqueueMainExecutorOriginal = unsafe extern "C" fn(job: JobRef);
/// Hook type for swift_task_enqueueMainExecutor.
pub type EnqueueMainExecutorHook =
    unsafe extern "C" fn(job: JobRef, original: EnqueueMainExecutorOriginal);

/// Original function type for getMainExecutor.
pub type GetMainExecutorOriginal = unsafe extern "C" fn() -> SerialExecutorRef;
/// Hook type for swift_task_getMainExecutor.
pub type GetMainExecutorHook =
    unsafe extern "C" fn(original: GetMainExecutorOriginal) -> SerialExecutorRef;

/// Original function type for checkIsolated.
pub type CheckIsolatedOriginal = unsafe extern "C" fn(executor: SerialExecutorRef);
/// Hook type for swift_task_checkIsolated.
pub type CheckIsolatedHook =
    unsafe extern "C" fn(executor: SerialExecutorRef, original: CheckIsolatedOriginal);

/// Original function type for isOnExecutor.
pub type IsOnExecutorOriginal = unsafe extern "C" fn(
    executor: *mut c_void,
    self_type: *const c_void,
    wtable: *const c_void,
) -> bool;
/// Hook type for swift_task_isOnExecutor.
pub type IsOnExecutorHook = unsafe extern "C" fn(
    executor: *mut c_void,
    self_type: *const c_void,
    wtable: *const c_void,
    original: IsOnExecutorOriginal,
) -> bool;

/// Original function type for isMainExecutor.
pub type IsMainExecutorOriginal = unsafe extern "C" fn(executor: SerialExecutorRef) -> bool;
/// Hook type for swift_task_isMainExecutor.
pub type IsMainExecutorHook =
    unsafe extern "C" fn(executor: SerialExecutorRef, original: IsMainExecutorOriginal) -> bool;

/// Original function type for isIsolatingCurrentContext.
pub type IsIsolatingCurrentContextOriginal =
    unsafe extern "C" fn(executor: SerialExecutorRef) -> i8;
/// Hook type for swift_task_isIsolatingCurrentContext.
pub type IsIsolatingCurrentContextHook = unsafe extern "C" fn(
    executor: SerialExecutorRef,
    original: IsIsolatingCurrentContextOriginal,
) -> i8;

/// Original function type for donateThreadToGlobalExecutorUntil.
pub type DonateThreadOriginal = unsafe extern "C" fn(
    condition: unsafe extern "C" fn(*mut c_void) -> bool,
    context: *mut c_void,
);
/// Hook type for swift_task_donateThreadToGlobalExecutorUntil.
pub type DonateThreadHook = unsafe extern "C" fn(
    condition: unsafe extern "C" fn(*mut c_void) -> bool,
    context: *mut c_void,
    original: DonateThreadOriginal,
);

/// Original function type for asyncMainDrainQueue.
pub type AsyncMainDrainQueueOriginal = unsafe extern "C" fn();
/// Override type for asyncMainDrainQueue.
pub type AsyncMainDrainQueueOverride = unsafe extern "C" fn(original: AsyncMainDrainQueueOriginal);
/// Hook type for swift_task_asyncMainDrainQueue (compatibility hook, slightly different).
pub type AsyncMainDrainQueueHook = unsafe extern "C" fn(
    original: AsyncMainDrainQueueOriginal,
    compat_override: AsyncMainDrainQueueOverride,
);

// ── Helper to install hooks via dlsym ──

use core::ffi::c_char;

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

/// Install a concurrency hook by name.
///
/// # Safety
/// The hook function must have the correct signature for the named hook.
/// This writes to a global mutable function pointer.
///
/// # Example
/// ```ignore
/// unsafe {
///     install_hook(
///         c"swift_task_enqueueGlobal_hook",
///         my_hook as *const c_void,
///     );
/// }
/// ```
pub unsafe fn install_hook(name: &core::ffi::CStr, hook: *const c_void) -> bool {
    let ptr = dlsym(RTLD_DEFAULT, name.as_ptr());
    if ptr.is_null() {
        return false;
    }
    let hook_slot = ptr as *mut *const c_void;
    *hook_slot = hook;
    true
}

/// Read a concurrency hook by name. Returns the current hook value (may be null).
///
/// # Safety
/// The name must be a valid hook symbol.
pub unsafe fn read_hook(name: &core::ffi::CStr) -> *const c_void {
    let ptr = dlsym(RTLD_DEFAULT, name.as_ptr());
    if ptr.is_null() {
        return core::ptr::null();
    }
    let hook_slot = ptr as *const *const c_void;
    *hook_slot
}
