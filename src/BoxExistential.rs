#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime box and existential allocation.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

/// Return type for swift_allocBox.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoxPair {
    pub object: HeapObjectRef,
    pub buffer: OpaqueValueRef,
}

unsafe extern "C" {
    /// Allocate a heap object that can contain a value of the given type.
    pub fn swift_allocBox(metadata: MetadataRef) -> BoxPair;

    /// Allocate an empty box.
    pub fn swift_allocEmptyBox() -> HeapObjectRef;

    /// Deallocate a box allocated by swift_allocBox.
    pub fn swift_deallocBox(object: HeapObjectRef);

    /// Get the value pointer from a box.
    pub fn swift_projectBox(object: HeapObjectRef) -> OpaqueValueRef;

    /// Perform a uniqueness check on a box pointer; allocate a new box if not unique.
    pub fn swift_makeBoxUnique(
        buffer: OpaqueValueRef,
        metadata: MetadataRef,
        align_mask: usize,
    ) -> BoxPair;
}
