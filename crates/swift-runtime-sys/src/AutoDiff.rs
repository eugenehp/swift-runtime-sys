#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime AutoDiff support.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;

unsafe extern "C" {
    /// Create a linear map context.
    pub fn swift_autoDiffCreateLinearMapContext(top_level_subcontext_size: usize) -> *mut c_void;

    /// Create a linear map context with type.
    pub fn swift_autoDiffCreateLinearMapContextWithType(
        top_level_subcontext_type: MetadataRef,
    ) -> *mut c_void;

    /// Allocate a subcontext.
    pub fn swift_autoDiffAllocateSubcontext(context: *mut c_void, size: usize) -> *mut c_void;

    /// Allocate a subcontext with type.
    pub fn swift_autoDiffAllocateSubcontextWithType(
        context: *mut c_void,
        subcontext_type: MetadataRef,
    ) -> *mut c_void;

    /// Project the top-level subcontext.
    pub fn swift_autoDiffProjectTopLevelSubcontext(context: *const c_void) -> *mut c_void;
}
