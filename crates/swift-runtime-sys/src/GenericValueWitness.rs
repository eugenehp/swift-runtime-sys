#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime generic value-witness operations.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

unsafe extern "C" {
    /// Destroy a value of the given type.
    pub fn swift_generic_destroy(value: OpaqueValueRef, self_type: MetadataRef);

    /// Initialize a value with a copy.
    pub fn swift_generic_initWithCopy(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Initialize a value by taking from source.
    pub fn swift_generic_initWithTake(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Assign a value with a copy.
    pub fn swift_generic_assignWithCopy(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Assign a value by taking from source.
    pub fn swift_generic_assignWithTake(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Initialize a buffer with a copy of a buffer.
    pub fn swift_generic_initializeBufferWithCopyOfBuffer(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        self_type: MetadataRef,
    ) -> OpaqueValueRef;

    /// Instantiate a layout string.
    pub fn swift_generic_instantiateLayoutString(
        layout_string: *const c_void,
        metadata: MetadataRef,
    );
}
