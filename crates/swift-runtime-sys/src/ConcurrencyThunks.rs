#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Arm64 inline-assembly thunks for Swift-CC and swiftasync-CC concurrency functions.
//!
//! On arm64:
//! - `SWIFT_CC(swift)`: args in x0..x7, self/context in x20, returns in x0(,x1)
//!   For most concurrency functions the first few args go in x0..x7 normally;
//!   the CC difference only matters for multi-word struct returns.
//! - `SWIFT_CC(swiftasync)`: async context passed in x22, resume addr in LR.

use core::ffi::{c_char, c_void, CStr};

pub type JobRef = *mut c_void;
pub type AsyncTaskRef = *mut c_void;
pub type AsyncContextRef = *mut c_void;
pub type MetadataRef = *const c_void;
pub type WitnessTableRef = *const c_void;
pub type ExecutorRef = *const c_void;
pub type TaskGroupRef = *mut c_void;
pub type ContinuationRef = *mut c_void;
pub type AsyncLetRef = *mut c_void;
pub type DefaultActorRef = *mut c_void;
pub type SwiftErrorRef = *mut c_void;
pub type TaskOptionRecordRef = *mut c_void;

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;
unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

#[derive(Debug)]
pub enum ThunkError {
    SymbolNotFound(String),
}

fn resolve(name: &CStr) -> Result<*const c_void, ThunkError> {
    let ptr = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) };
    if ptr.is_null() {
        Err(ThunkError::SymbolNotFound(
            name.to_string_lossy().into_owned(),
        ))
    } else {
        Ok(ptr as *const c_void)
    }
}

/// SerialExecutorRef is 2 words on arm64.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SerialExecutorRef {
    pub identity: *const c_void,
    pub implementation: *const c_void,
}

/// AsyncTaskAndContext return type (2 words).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncTaskAndContext {
    pub task: AsyncTaskRef,
    pub initial_context: AsyncContextRef,
}

// ═══════════════════════════════════════════════════════════════════════════
// Generic arm64 asm callers for Swift CC
// On x86_64, Swift CC == C CC so we just transmute.
// ═══════════════════════════════════════════════════════════════════════════

macro_rules! define_thunk_void {
    ($name:ident, $sym:literal $(, $arg:ident : $ty:ty)*) => {
        pub unsafe fn $name($($arg: $ty),*) -> Result<(), ThunkError> {
            let f = resolve(unsafe { CStr::from_bytes_with_nul_unchecked($sym) })?;
            #[cfg(target_arch = "aarch64")] {
                _call_void(f $(, $arg as usize)*);
            }
            #[cfg(target_arch = "x86_64")] {
                type F = unsafe extern "C" fn($($ty),*);
                (core::mem::transmute::<_,F>(f))($($arg),*);
            }
            Ok(())
        }
    };
}

macro_rules! define_thunk_ptr {
    ($name:ident, $sym:literal $(, $arg:ident : $ty:ty)*) => {
        pub unsafe fn $name($($arg: $ty),*) -> Result<*mut c_void, ThunkError> {
            let f = resolve(unsafe { CStr::from_bytes_with_nul_unchecked($sym) })?;
            #[cfg(target_arch = "aarch64")] {
                Ok(_call_ptr(f $(, $arg as usize)*))
            }
            #[cfg(target_arch = "x86_64")] {
                type F = unsafe extern "C" fn($($ty),*) -> *mut c_void;
                Ok((core::mem::transmute::<_,F>(f))($($arg),*))
            }
        }
    };
}

macro_rules! define_thunk_bool {
    ($name:ident, $sym:literal $(, $arg:ident : $ty:ty)*) => {
        pub unsafe fn $name($($arg: $ty),*) -> Result<bool, ThunkError> {
            let f = resolve(unsafe { CStr::from_bytes_with_nul_unchecked($sym) })?;
            #[cfg(target_arch = "aarch64")] {
                Ok(_call_usize(f $(, $arg as usize)*) != 0)
            }
            #[cfg(target_arch = "x86_64")] {
                type F = unsafe extern "C" fn($($ty),*) -> bool;
                Ok((core::mem::transmute::<_,F>(f))($($arg),*))
            }
        }
    };
}

// ── Arm64 asm primitives ──

#[cfg(target_arch = "aarch64")]
unsafe fn _call_void(f: *const c_void, a0: usize) {
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0,
        lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

#[cfg(target_arch = "aarch64")]
unsafe fn _call_void_2(f: *const c_void, a0: usize, a1: usize) {
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, in("x1") a1,
        lateout("x0") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

#[cfg(target_arch = "aarch64")]
unsafe fn _call_void_3(f: *const c_void, a0: usize, a1: usize, a2: usize) {
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, in("x1") a1, in("x2") a2,
        lateout("x0") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

#[cfg(target_arch = "aarch64")]
unsafe fn _call_ptr(f: *const c_void, a0: usize) -> *mut c_void {
    let r: *mut c_void;
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, lateout("x0") r,
        lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "aarch64")]
unsafe fn _call_pair(
    f: *const c_void,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
) -> (*mut c_void, *mut c_void) {
    let r0: *mut c_void;
    let r1: *mut c_void;
    core::arch::asm!("blr {f}", f = in(reg) f,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3,
        lateout("x0") r0, lateout("x1") r1,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    (r0, r1)
}

#[cfg(target_arch = "aarch64")]
unsafe fn _call_usize(f: *const c_void, a0: usize) -> usize {
    let r: usize;
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, lateout("x0") r,
        lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

// Generic N-arg asm callers
#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_2_ret2(f: *const c_void, a0: usize, a1: usize) -> (usize, usize) {
    let r0: usize;
    let r1: usize;
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, in("x1") a1,
        lateout("x0") r0, lateout("x1") r1, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    (r0, r1)
}

#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_0_ret2(f: *const c_void) -> (usize, usize) {
    let r0: usize;
    let r1: usize;
    core::arch::asm!("blr {f}", f = in(reg) f,
        lateout("x0") r0, lateout("x1") r1, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    (r0, r1)
}

#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_2_ret1(f: *const c_void, a0: usize, a1: usize) -> usize {
    let r: usize;
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, in("x1") a1,
        lateout("x0") r, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_3_ret1(f: *const c_void, a0: usize, a1: usize, a2: usize) -> usize {
    let r: usize;
    core::arch::asm!("blr {f}", f = in(reg) f, in("x0") a0, in("x1") a1, in("x2") a2,
        lateout("x0") r, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_6(
    f: *const c_void,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) {
    core::arch::asm!("blr {f}", f = in(reg) f,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3, in("x4") a4, in("x5") a5,
        lateout("x0") _, lateout("x1") _,
        lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

/// Call a swiftasync function: async context goes in x22.
#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_async(f: *const c_void, a0: usize, a1: usize, a2: usize, async_ctx: usize) {
    core::arch::asm!("blr {f}", f = in(reg) f,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x22") async_ctx,
        lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_async_5(
    f: *const c_void,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    async_ctx: usize,
) {
    core::arch::asm!("blr {f}", f = in(reg) f,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3, in("x22") async_ctx,
        lateout("x0") _, lateout("x1") _, lateout("x4") _,
        lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

/// Call a Swift-CC error function: SWIFT_CONTEXT in x20, SWIFT_ERROR_RESULT in x21.
#[cfg(target_arch = "aarch64")]
unsafe fn _asm_call_error(f: *const c_void, ctx: usize, error_ptr: *mut *mut c_void) {
    core::arch::asm!("blr {f}", f = in(reg) f,
        in("x20") ctx, in("x21") error_ptr,
        lateout("x0") _, lateout("x1") _, lateout("x2") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// §37: Concurrency Swift-CC thunks
// ═══════════════════════════════════════════════════════════════════════════

/// `swift_task_create` — returns `AsyncTaskAndContext` (2 words).
pub unsafe fn swift_task_create(
    flags: usize,
    options: TaskOptionRecordRef,
    function: *const c_void,
    result_type: MetadataRef,
) -> Result<AsyncTaskAndContext, ThunkError> {
    let f = resolve(c"swift_task_create")?;
    #[cfg(target_arch = "aarch64")]
    {
        let (t, c) = _asm_call_2_ret2(f, flags, options as usize);
        // task_create actually takes 4 args on some versions; pass via x2,x3 too
        // The actual ABI may vary; using the pair-return pattern
        let _ = (function, result_type); // used in the real call below
                                         // Full 4-arg call:
        let task: *mut c_void;
        let ctx: *mut c_void;
        core::arch::asm!("blr {f}", f = in(reg) f,
            in("x0") flags, in("x1") options, in("x2") function, in("x3") result_type,
            lateout("x0") task, lateout("x1") ctx,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        let _ = (t, c); // suppress unused
        Ok(AsyncTaskAndContext {
            task,
            initial_context: ctx,
        })
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            usize,
            *mut c_void,
            *const c_void,
            *const c_void,
        ) -> AsyncTaskAndContext;
        Ok((core::mem::transmute::<_, F>(f))(
            flags,
            options,
            function,
            result_type,
        ))
    }
}

/// `swift_task_create_common` — returns `AsyncTaskAndContext`.
pub unsafe fn swift_task_create_common(
    flags: usize,
    options: TaskOptionRecordRef,
    function: *const c_void,
    result_type: MetadataRef,
    initial_context_size: usize,
) -> Result<AsyncTaskAndContext, ThunkError> {
    let f = resolve(c"swift_task_create_common")?;
    #[cfg(target_arch = "aarch64")]
    {
        let task: *mut c_void;
        let ctx: *mut c_void;
        core::arch::asm!("blr {f}", f = in(reg) f,
            in("x0") flags, in("x1") options, in("x2") function,
            in("x3") result_type, in("x4") initial_context_size,
            lateout("x0") task, lateout("x1") ctx,
            lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        Ok(AsyncTaskAndContext {
            task,
            initial_context: ctx,
        })
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            usize,
            *mut c_void,
            *const c_void,
            *const c_void,
            usize,
        ) -> AsyncTaskAndContext;
        Ok((core::mem::transmute::<_, F>(f))(
            flags,
            options,
            function,
            result_type,
            initial_context_size,
        ))
    }
}

// Simple void/ptr thunks for single-arg concurrency functions
define_thunk_void!(swift_task_enqueue, b"swift_task_enqueue\0", task: AsyncTaskRef);
define_thunk_void!(swift_task_enqueueGlobal, b"swift_task_enqueueGlobal\0", job: JobRef);
define_thunk_void!(swift_task_enqueueMainExecutor, b"swift_task_enqueueMainExecutor\0", job: JobRef);
define_thunk_void!(swift_task_startOnMainActor, b"swift_task_startOnMainActor\0", ctx: AsyncContextRef);
define_thunk_void!(swift_defaultActor_initialize, b"swift_defaultActor_initialize\0", actor: DefaultActorRef);
define_thunk_void!(swift_defaultActor_destroy, b"swift_defaultActor_destroy\0", actor: DefaultActorRef);
define_thunk_void!(swift_defaultActor_deallocate, b"swift_defaultActor_deallocate\0", actor: DefaultActorRef);
define_thunk_void!(swift_defaultActor_deallocateResilient, b"swift_defaultActor_deallocateResilient\0", actor: DefaultActorRef);
define_thunk_void!(swift_taskGroup_destroy, b"swift_taskGroup_destroy\0", group: TaskGroupRef);
define_thunk_void!(swift_taskGroup_cancelAll, b"swift_taskGroup_cancelAll\0", group: TaskGroupRef);
define_thunk_void!(swift_continuation_resume, b"swift_continuation_resume\0", cont: ContinuationRef);
define_thunk_void!(swift_continuation_throwingResume, b"swift_continuation_throwingResume\0", cont: ContinuationRef);
define_thunk_bool!(swift_taskGroup_isCancelled, b"swift_taskGroup_isCancelled\0", group: TaskGroupRef);
define_thunk_bool!(swift_taskGroup_isEmpty, b"swift_taskGroup_isEmpty\0", group: TaskGroupRef);
define_thunk_ptr!(swift_distributedActor_remote_initialize, b"swift_distributedActor_remote_initialize\0", actor_type: MetadataRef);
define_thunk_ptr!(swift_nonDefaultDistributedActor_initialize, b"swift_nonDefaultDistributedActor_initialize\0", actor_type: MetadataRef);

// 2-arg thunks
pub unsafe fn swift_task_escalate(task: AsyncTaskRef, priority: u32) -> Result<u32, ThunkError> {
    let f = resolve(c"swift_task_escalate")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_2_ret1(f, task as usize, priority as usize) as u32)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, u32) -> u32;
        Ok((core::mem::transmute::<_, F>(f))(task, priority))
    }
}

pub unsafe fn swift_task_enqueueGlobalWithDelay(delay: u64, job: JobRef) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_enqueueGlobalWithDelay")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(f, delay as usize, job as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(u64, *mut c_void);
        (core::mem::transmute::<_, F>(f))(delay, job);
    }
    Ok(())
}

pub unsafe fn swift_task_enqueueOnDispatchQueue(
    job: JobRef,
    queue: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_enqueueOnDispatchQueue")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(f, job as usize, queue as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(job, queue);
    }
    Ok(())
}

pub unsafe fn swift_task_enqueueTaskOnExecutor(
    task: AsyncTaskRef,
    executor: ExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_enqueueTaskOnExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(f, task as usize, executor as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(task, executor);
    }
    Ok(())
}

pub unsafe fn swift_defaultActor_enqueue(
    job: JobRef,
    actor: DefaultActorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_defaultActor_enqueue")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(f, job as usize, actor as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void);
        (core::mem::transmute::<_, F>(f))(job, actor);
    }
    Ok(())
}

pub unsafe fn swift_continuation_throwingResumeWithError(
    cont: ContinuationRef,
    error: SwiftErrorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_continuation_throwingResumeWithError")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(f, cont as usize, error as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void);
        (core::mem::transmute::<_, F>(f))(cont, error);
    }
    Ok(())
}

pub unsafe fn swift_continuation_init(
    cont: ContinuationRef,
    flags: u32,
) -> Result<AsyncTaskRef, ThunkError> {
    let f = resolve(c"swift_continuation_init")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_2_ret1(f, cont as usize, flags as usize) as *mut c_void)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, u32) -> *mut c_void;
        Ok((core::mem::transmute::<_, F>(f))(cont, flags))
    }
}

pub unsafe fn swift_task_switch(
    ctx: AsyncContextRef,
    executor: SerialExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_switch")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(
            f,
            ctx as usize,
            executor.identity as usize,
            executor.implementation as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(ctx, executor.identity, executor.implementation);
    }
    Ok(())
}

// Executor return/arg thunks (SerialExecutorRef is 2 words)
pub unsafe fn swift_task_getMainExecutor() -> Result<SerialExecutorRef, ThunkError> {
    let f = resolve(c"swift_task_getMainExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        let (a, b) = _asm_call_0_ret2(f);
        Ok(SerialExecutorRef {
            identity: a as _,
            implementation: b as _,
        })
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn() -> SerialExecutorRef;
        Ok((core::mem::transmute::<_, F>(f))())
    }
}

pub unsafe fn swift_task_getCurrentExecutor() -> Result<SerialExecutorRef, ThunkError> {
    let f = resolve(c"swift_task_getCurrentExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        let (a, b) = _asm_call_0_ret2(f);
        Ok(SerialExecutorRef {
            identity: a as _,
            implementation: b as _,
        })
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn() -> SerialExecutorRef;
        Ok((core::mem::transmute::<_, F>(f))())
    }
}

pub unsafe fn swift_task_isCurrentExecutor(
    executor: SerialExecutorRef,
) -> Result<bool, ThunkError> {
    let f = resolve(c"swift_task_isCurrentExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_2_ret1(
            f,
            executor.identity as usize,
            executor.implementation as usize,
        ) != 0)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *const c_void) -> bool;
        Ok((core::mem::transmute::<_, F>(f))(
            executor.identity,
            executor.implementation,
        ))
    }
}

pub unsafe fn swift_task_isMainExecutor(executor: SerialExecutorRef) -> Result<bool, ThunkError> {
    let f = resolve(c"swift_task_isMainExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_2_ret1(
            f,
            executor.identity as usize,
            executor.implementation as usize,
        ) != 0)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *const c_void) -> bool;
        Ok((core::mem::transmute::<_, F>(f))(
            executor.identity,
            executor.implementation,
        ))
    }
}

pub unsafe fn swift_task_checkIsolated(executor: SerialExecutorRef) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_checkIsolated")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(
            f,
            executor.identity as usize,
            executor.implementation as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(executor.identity, executor.implementation);
    }
    Ok(())
}

pub unsafe fn swift_task_isIsolatingCurrentContext(
    executor: SerialExecutorRef,
) -> Result<bool, ThunkError> {
    let f = resolve(c"swift_task_isIsolatingCurrentContext")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_2_ret1(
            f,
            executor.identity as usize,
            executor.implementation as usize,
        ) != 0)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *const c_void) -> bool;
        Ok((core::mem::transmute::<_, F>(f))(
            executor.identity,
            executor.implementation,
        ))
    }
}

pub unsafe fn swift_task_pushTaskExecutorPreference(
    executor: SerialExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_pushTaskExecutorPreference")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(
            f,
            executor.identity as usize,
            executor.implementation as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(executor.identity, executor.implementation);
    }
    Ok(())
}

// Task group init (2–3 args)
pub unsafe fn swift_taskGroup_initialize(
    group: TaskGroupRef,
    t: MetadataRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_taskGroup_initialize")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_2(f, group as usize, t as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(group, t);
    }
    Ok(())
}

pub unsafe fn swift_taskGroup_initializeWithFlags(
    flags: usize,
    group: TaskGroupRef,
    t: MetadataRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_taskGroup_initializeWithFlags")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(f, flags, group as usize, t as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(usize, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(flags, group, t);
    }
    Ok(())
}

pub unsafe fn swift_taskGroup_addPending(
    group: TaskGroupRef,
    unconditionally: bool,
) -> Result<bool, ThunkError> {
    let f = resolve(c"swift_taskGroup_addPending")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_2_ret1(f, group as usize, unconditionally as usize) != 0)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, bool) -> bool;
        Ok((core::mem::transmute::<_, F>(f))(group, unconditionally))
    }
}

// Job run with executor
pub unsafe fn swift_job_run(job: JobRef, executor: SerialExecutorRef) -> Result<(), ThunkError> {
    let f = resolve(c"swift_job_run")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(
            f,
            job as usize,
            executor.identity as usize,
            executor.implementation as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(job, executor.identity, executor.implementation);
    }
    Ok(())
}

pub unsafe fn swift_task_deinitOnExecutor(
    object: *mut c_void,
    executor: SerialExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_deinitOnExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(
            f,
            object as usize,
            executor.identity as usize,
            executor.implementation as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(object, executor.identity, executor.implementation);
    }
    Ok(())
}

// Clock
pub unsafe fn swift_get_time(
    seconds: *mut i64,
    nanoseconds: *mut i64,
    clock: i32,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_get_time")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(f, seconds as usize, nanoseconds as usize, clock as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut i64, *mut i64, i32);
        (core::mem::transmute::<_, F>(f))(seconds, nanoseconds, clock);
    }
    Ok(())
}

pub unsafe fn swift_get_clock_res(
    seconds: *mut i64,
    nanoseconds: *mut i64,
    clock: i32,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_get_clock_res")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(f, seconds as usize, nanoseconds as usize, clock as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut i64, *mut i64, i32);
        (core::mem::transmute::<_, F>(f))(seconds, nanoseconds, clock);
    }
    Ok(())
}

pub unsafe fn swift_task_localValuePush(
    key: *const c_void,
    value: *mut c_void,
    value_type: MetadataRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_localValuePush")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(f, key as usize, value as usize, value_type as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(key, value, value_type);
    }
    Ok(())
}

pub unsafe fn swift_task_enqueueGlobalWithDeadline(
    sec: i64,
    nsec: i64,
    tsec: i64,
    tnsec: i64,
    clock: i32,
    job: JobRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_enqueueGlobalWithDeadline")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_6(
            f,
            sec as usize,
            nsec as usize,
            tsec as usize,
            tnsec as usize,
            clock as usize,
            job as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(i64, i64, i64, i64, i32, *mut c_void);
        (core::mem::transmute::<_, F>(f))(sec, nsec, tsec, tnsec, clock, job);
    }
    Ok(())
}

pub unsafe fn swift_task_asyncMainDrainQueue() -> Result<core::convert::Infallible, ThunkError> {
    let f = resolve(c"swift_task_asyncMainDrainQueue")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void(f, 0);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn();
        (core::mem::transmute::<_, F>(f))();
    }
    unreachable!()
}

pub unsafe fn swift_task_donateThreadToGlobalExecutorUntil(
    condition: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_donateThreadToGlobalExecutorUntil")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void(f, condition as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void);
        (core::mem::transmute::<_, F>(f))(condition);
    }
    Ok(())
}

pub unsafe fn swift_task_isOnExecutor(
    executor: *mut c_void,
    self_type: MetadataRef,
    wtable: WitnessTableRef,
) -> Result<bool, ThunkError> {
    let f = resolve(c"swift_task_isOnExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_3_ret1(f, executor as usize, self_type as usize, wtable as usize) != 0)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void) -> bool;
        Ok((core::mem::transmute::<_, F>(f))(
            executor, self_type, wtable,
        ))
    }
}

pub unsafe fn swift_task_isCurrentExecutorWithFlags(
    executor: SerialExecutorRef,
    flags: usize,
) -> Result<bool, ThunkError> {
    let f = resolve(c"swift_task_isCurrentExecutorWithFlags")?;
    #[cfg(target_arch = "aarch64")]
    {
        Ok(_asm_call_3_ret1(
            f,
            executor.identity as usize,
            executor.implementation as usize,
            flags,
        ) != 0)
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_void, *const c_void, usize) -> bool;
        Ok((core::mem::transmute::<_, F>(f))(
            executor.identity,
            executor.implementation,
            flags,
        ))
    }
}

pub unsafe fn swift_task_reportUnexpectedExecutor(
    file: *const c_char,
    line: usize,
    executor: SerialExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_reportUnexpectedExecutor")?;
    #[cfg(target_arch = "aarch64")]
    {
        core::arch::asm!("blr {f}", f = in(reg) f,
            in("x0") file, in("x1") line, in("x2") executor.identity, in("x3") executor.implementation,
            lateout("x0") _, lateout("x1") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const c_char, usize, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(file, line, executor.identity, executor.implementation);
    }
    Ok(())
}

pub unsafe fn swift_job_run_on_serial_and_task_executor(
    job: JobRef,
    serial: SerialExecutorRef,
    task_exec: SerialExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_job_run_on_serial_and_task_executor")?;
    #[cfg(target_arch = "aarch64")]
    {
        core::arch::asm!("blr {f}", f = in(reg) f,
            in("x0") job, in("x1") serial.identity, in("x2") serial.implementation,
            in("x3") task_exec.identity, in("x4") task_exec.implementation,
            lateout("x0") _, lateout("x1") _,
            lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            *mut c_void,
            *const c_void,
            *const c_void,
            *const c_void,
            *const c_void,
        );
        (core::mem::transmute::<_, F>(f))(
            job,
            serial.identity,
            serial.implementation,
            task_exec.identity,
            task_exec.implementation,
        );
    }
    Ok(())
}

pub unsafe fn swift_job_run_on_task_executor(
    job: JobRef,
    executor: SerialExecutorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_job_run_on_task_executor")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(
            f,
            job as usize,
            executor.identity as usize,
            executor.implementation as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(job, executor.identity, executor.implementation);
    }
    Ok(())
}

pub unsafe fn swift_taskGroup_initializeWithOptions(
    flags: usize,
    group: TaskGroupRef,
    t: MetadataRef,
    options: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_taskGroup_initializeWithOptions")?;
    #[cfg(target_arch = "aarch64")]
    {
        core::arch::asm!("blr {f}", f = in(reg) f,
            in("x0") flags, in("x1") group, in("x2") t, in("x3") options,
            lateout("x0") _, lateout("x1") _,
            lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(usize, *mut c_void, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(flags, group, t, options);
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// §37: swiftasync CC thunks (async context in x22)
// ═══════════════════════════════════════════════════════════════════════════

pub unsafe fn swift_task_future_wait(
    result: *mut c_void,
    caller_ctx: AsyncContextRef,
    task: AsyncTaskRef,
    resume: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_future_wait")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(
            f,
            result as usize,
            task as usize,
            resume as usize,
            caller_ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(result, caller_ctx, task, resume);
    }
    Ok(())
}

pub unsafe fn swift_task_future_wait_throwing(
    result: *mut c_void,
    caller_ctx: AsyncContextRef,
    task: AsyncTaskRef,
    resume: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_task_future_wait_throwing")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(
            f,
            result as usize,
            task as usize,
            resume as usize,
            caller_ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(result, caller_ctx, task, resume);
    }
    Ok(())
}

pub unsafe fn swift_taskGroup_waitAll(
    result: *mut c_void,
    ctx: AsyncContextRef,
    group: TaskGroupRef,
    result_type: MetadataRef,
    resume: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_taskGroup_waitAll")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async_5(
            f,
            result as usize,
            group as usize,
            result_type as usize,
            resume as usize,
            ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *const c_void,
            *const c_void,
        );
        (core::mem::transmute::<_, F>(f))(result, ctx, group, result_type, resume);
    }
    Ok(())
}

pub unsafe fn swift_taskGroup_wait_next_throwing(
    result: *mut c_void,
    ctx: AsyncContextRef,
    group: TaskGroupRef,
    result_type: MetadataRef,
    resume: *const c_void,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_taskGroup_wait_next_throwing")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async_5(
            f,
            result as usize,
            group as usize,
            result_type as usize,
            resume as usize,
            ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *const c_void,
            *const c_void,
        );
        (core::mem::transmute::<_, F>(f))(result, ctx, group, result_type, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_get(
    al: AsyncLetRef,
    result: *mut c_void,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_get")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(
            f,
            al as usize,
            result as usize,
            resume as usize,
            ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, result, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_get_throwing(
    al: AsyncLetRef,
    result: *mut c_void,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_get_throwing")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(
            f,
            al as usize,
            result as usize,
            resume as usize,
            ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, result, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_consume(
    al: AsyncLetRef,
    result: *mut c_void,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_consume")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(
            f,
            al as usize,
            result as usize,
            resume as usize,
            ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, result, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_consume_throwing(
    al: AsyncLetRef,
    result: *mut c_void,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_consume_throwing")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(
            f,
            al as usize,
            result as usize,
            resume as usize,
            ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, result, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_begin(
    al: AsyncLetRef,
    options: TaskOptionRecordRef,
    entry: *const c_void,
    context: *mut c_void,
    async_ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_begin")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async_5(
            f,
            al as usize,
            options as usize,
            entry as usize,
            context as usize,
            async_ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void, *mut c_void);
        (core::mem::transmute::<_, F>(f))(al, options, entry, context);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_start(
    al: AsyncLetRef,
    options: TaskOptionRecordRef,
    entry: *const c_void,
    context: *mut c_void,
    async_ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_start")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async_5(
            f,
            al as usize,
            options as usize,
            entry as usize,
            context as usize,
            async_ctx as usize,
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void, *mut c_void);
        (core::mem::transmute::<_, F>(f))(al, options, entry, context);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_finish(
    al: AsyncLetRef,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_finish")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(f, al as usize, resume as usize, 0, ctx as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_wait(
    al: AsyncLetRef,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_wait")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(f, al as usize, resume as usize, 0, ctx as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, resume);
    }
    Ok(())
}

pub unsafe fn swift_asyncLet_wait_throwing(
    al: AsyncLetRef,
    resume: *const c_void,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_asyncLet_wait_throwing")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(f, al as usize, resume as usize, 0, ctx as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(al, resume);
    }
    Ok(())
}

pub unsafe fn swift_continuation_await(
    cont: ContinuationRef,
    ctx: AsyncContextRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_continuation_await")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_async(f, cont as usize, 0, 0, ctx as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void);
        (core::mem::transmute::<_, F>(f))(cont);
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// §41: Swift CC error functions (SWIFT_CONTEXT in x20, SWIFT_ERROR_RESULT in x21)
// ═══════════════════════════════════════════════════════════════════════════

pub unsafe fn swift_willThrow(
    unused: *mut c_void,
    error: *mut SwiftErrorRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_willThrow")?;
    #[cfg(target_arch = "aarch64")]
    {
        _asm_call_error(f, unused as usize, error as *mut *mut c_void);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *mut *mut c_void);
        (core::mem::transmute::<_, F>(f))(unused, error as *mut *mut c_void);
    }
    Ok(())
}

pub unsafe fn swift_willThrowTypedImpl(
    value: *mut c_void,
    error_type: MetadataRef,
    conformance: WitnessTableRef,
) -> Result<(), ThunkError> {
    let f = resolve(c"swift_willThrowTypedImpl")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void_3(f, value as usize, error_type as usize, conformance as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
        (core::mem::transmute::<_, F>(f))(value, error_type, conformance);
    }
    Ok(())
}

pub unsafe fn swift_errorInMain(
    object: SwiftErrorRef,
) -> Result<core::convert::Infallible, ThunkError> {
    let f = resolve(c"swift_errorInMain")?;
    #[cfg(target_arch = "aarch64")]
    {
        _call_void(f, object as usize);
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void);
        (core::mem::transmute::<_, F>(f))(object);
    }
    unreachable!()
}

pub unsafe fn swift_unexpectedError(
    object: SwiftErrorRef,
    filename: *const u8,
    filename_len: isize,
    is_ascii: bool,
    line: usize,
) -> Result<core::convert::Infallible, ThunkError> {
    let f = resolve(c"swift_unexpectedError")?;
    #[cfg(target_arch = "aarch64")]
    {
        core::arch::asm!("blr {f}", f = in(reg) f,
            in("x0") object, in("x1") filename, in("x2") filename_len,
            in("x3") is_ascii as usize, in("x4") line,
            lateout("x0") _, lateout("x1") _,
            lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*mut c_void, *const u8, isize, bool, usize);
        (core::mem::transmute::<_, F>(f))(object, filename, filename_len, is_ascii, line);
    }
    unreachable!()
}
