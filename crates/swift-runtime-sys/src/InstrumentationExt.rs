#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime instrumentation support.

use core::ffi::c_void;

unsafe extern "C" {
    /// Enable swizzling of allocation and ref-counting functions (Instruments only).
    pub fn swift_enableSwizzlingOfAllocationAndRefCountingFunctions_forInstrumentsOnly();

    /// Validate prespecialized metadata.
    pub fn swift_validatePrespecializedMetadata(metadata: *const c_void) -> bool;

    /// Get an opaque summary of a value.
    pub fn swift_OpaqueSummary(metadata: *const c_void) -> *const u8;

    /// The isa mask used for tagged pointers.
    pub static swift_isaMask: usize;

    /// TSan acquire annotation.
    pub fn swift_tsan_acquire(addr: *const c_void);

    /// TSan release annotation.
    pub fn swift_tsan_release(addr: *const c_void);

    /// Whether TSan is enabled.
    pub static swift_tsan_enabled: bool;
}
