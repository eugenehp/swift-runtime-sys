#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Runtime resolution of Swift stdlib SPI symbols via dlsym.
//!
//! Some `swift_stdlib_*` functions are marked `SWIFT_RUNTIME_STDLIB_API`
//! (SPI) and may not link directly. This module resolves them lazily at
//! runtime via dlsym, with graceful fallback when unavailable.

use core::ffi::{c_char, c_void};
use std::sync::OnceLock;

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// Resolve a symbol from the default search path.
fn sym(name: &core::ffi::CStr) -> Option<*const c_void> {
    let ptr = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr as *const c_void)
    }
}

// ── Resolved SPI functions ──

type HardwareConcurrencyFn = unsafe extern "C" fn() -> usize;
type OsVersionFn = unsafe extern "C" fn() -> crate::StdlibUtils::OSVersion;
type RandomFn = unsafe extern "C" fn(*mut c_void, usize);
type ReadLineFn = unsafe extern "C" fn(*mut *mut u8, *mut usize) -> bool;
type StackBoundsFn = unsafe extern "C" fn(*mut *mut c_void, *mut *mut c_void);
type IsStackSafeFn = unsafe extern "C" fn(usize, usize) -> bool;
type ImmortalizeFn = unsafe extern "C" fn(*mut c_void);
type PutcStderrFn = unsafe extern "C" fn(u32);
type IsNSStringFn = unsafe extern "C" fn(*const c_void) -> bool;

static S_HARDWARE_CONCURRENCY: OnceLock<Option<HardwareConcurrencyFn>> = OnceLock::new();
static S_OS_VERSION: OnceLock<Option<OsVersionFn>> = OnceLock::new();
static S_RANDOM: OnceLock<Option<RandomFn>> = OnceLock::new();
static S_READ_LINE: OnceLock<Option<ReadLineFn>> = OnceLock::new();
static S_STACK_BOUNDS: OnceLock<Option<StackBoundsFn>> = OnceLock::new();
static S_IS_STACK_SAFE: OnceLock<Option<IsStackSafeFn>> = OnceLock::new();
static S_IMMORTALIZE: OnceLock<Option<ImmortalizeFn>> = OnceLock::new();
static S_PUTC_STDERR: OnceLock<Option<PutcStderrFn>> = OnceLock::new();
static S_IS_NSSTRING: OnceLock<Option<IsNSStringFn>> = OnceLock::new();

fn resolve_fn<T: Copy>(lock: &OnceLock<Option<T>>, name: &core::ffi::CStr) -> Option<T> {
    *lock.get_or_init(|| sym(name).map(|p| unsafe { core::mem::transmute_copy(&p) }))
}

// ── Public API ──

/// Get the number of hardware threads. Returns None if symbol unavailable.
pub fn get_hardware_concurrency() -> Option<usize> {
    resolve_fn(
        &S_HARDWARE_CONCURRENCY,
        c"_swift_stdlib_getHardwareConcurrency",
    )
    .map(|f| unsafe { f() })
}

/// Get the OS version. Returns None if symbol unavailable.
pub fn get_os_version() -> Option<crate::StdlibUtils::OSVersion> {
    resolve_fn(&S_OS_VERSION, c"_swift_stdlib_operatingSystemVersion").map(|f| unsafe { f() })
}

/// Fill a buffer with random bytes. Returns false if symbol unavailable.
pub fn random(buf: &mut [u8]) -> bool {
    match resolve_fn(&S_RANDOM, c"_swift_stdlib_random") {
        Some(f) => {
            unsafe { f(buf.as_mut_ptr() as _, buf.len()) };
            true
        }
        None => false,
    }
}

/// Read a line from stdin. Returns None if symbol unavailable.
pub fn read_line_stdin() -> Option<Option<Vec<u8>>> {
    resolve_fn(&S_READ_LINE, c"_swift_stdlib_readLine_stdin").map(|f| {
        let mut ptr: *mut u8 = core::ptr::null_mut();
        let mut len: usize = 0;
        let ok = unsafe { f(&mut ptr, &mut len) };
        if ok && !ptr.is_null() && len > 0 {
            let v = unsafe { Vec::from_raw_parts(ptr, len, len) };
            Some(v)
        } else {
            None
        }
    })
}

/// Get the current stack bounds. Returns None if symbol unavailable.
pub fn get_current_stack_bounds() -> Option<(*mut c_void, *mut c_void)> {
    resolve_fn(&S_STACK_BOUNDS, c"_swift_stdlib_getCurrentStackBounds").map(|f| {
        let mut begin: *mut c_void = core::ptr::null_mut();
        let mut end: *mut c_void = core::ptr::null_mut();
        unsafe { f(&mut begin, &mut end) };
        (begin, end)
    })
}

/// Check if a stack allocation of the given size/alignment is safe.
pub fn is_stack_allocation_safe(size: usize, alignment: usize) -> Option<bool> {
    resolve_fn(&S_IS_STACK_SAFE, c"_swift_stdlib_isStackAllocationSafe")
        .map(|f| unsafe { f(size, alignment) })
}

/// Immortalize a Swift object (make it never deallocated).
pub fn immortalize(object: *mut c_void) -> bool {
    match resolve_fn(&S_IMMORTALIZE, c"_swift_stdlib_immortalize") {
        Some(f) => {
            unsafe { f(object) };
            true
        }
        None => false,
    }
}

/// Write a character to stderr.
pub fn putc_stderr(c: u32) -> bool {
    match resolve_fn(&S_PUTC_STDERR, c"_swift_stdlib_putc_stderr") {
        Some(f) => {
            unsafe { f(c) };
            true
        }
        None => false,
    }
}

/// Check if a value is an NSString.
pub fn is_nsstring(object: *const c_void) -> Option<bool> {
    resolve_fn(&S_IS_NSSTRING, c"_swift_stdlib_isNSString").map(|f| unsafe { f(object) })
}
