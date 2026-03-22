#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime POD (plain old data) operations.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

unsafe extern "C" {
    /// Copy a POD value.
    pub fn swift_pod_copy(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Destroy a POD value (no-op for PODs).
    pub fn swift_pod_destroy(
        value: OpaqueValueRef,
        self_type: MetadataRef,
    );

    /// Directly initialize a buffer with a copy of a POD buffer.
    pub fn swift_pod_direct_initializeBufferWithCopyOfBuffer(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Indirectly initialize a buffer with a copy of a POD buffer.
    pub fn swift_pod_indirect_initializeBufferWithCopyOfBuffer(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Copy a POD value (alternate entry point).
    pub fn swift_copyPOD(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;
}
