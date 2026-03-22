#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime function replacement and dynamic dispatch.

use core::ffi::c_void;

unsafe extern "C" {
    /// Get the replacement for a replaceable function.
    pub fn swift_getFunctionReplacement(
        table: *const *const c_void,
        original: *const c_void,
    ) -> *const c_void;

    /// Get the original of a replaceable function.
    pub fn swift_getOrigOfReplaceable(table: *const *const c_void) -> *const c_void;

    /// Enable a dynamic replacement scope.
    pub fn swift_enableDynamicReplacementScope(scope: *const c_void);

    /// Disable a dynamic replacement scope.
    pub fn swift_disableDynamicReplacementScope(scope: *const c_void);

    /// Look up a class method.
    pub fn swift_lookUpClassMethod(
        metadata: *const c_void,
        method: *const c_void,
        description: *const c_void,
    ) -> *const c_void;

    /// Error for calling a deleted method.
    pub fn swift_deletedMethodError() -> !;

    /// Error for calling a deleted callee-allocated coroutine method.
    pub fn swift_deletedCalleeAllocatedCoroutineMethodError() -> !;
}
