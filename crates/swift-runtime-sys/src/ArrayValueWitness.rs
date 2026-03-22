#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime array value-witness operations.
//! Bulk value operations on contiguous Swift arrays.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

unsafe extern "C" {
    /// Initialize array elements with copies.
    pub fn swift_arrayInitWithCopy(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Initialize array by taking elements from back to front.
    pub fn swift_arrayInitWithTakeBackToFront(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Initialize array by taking elements from front to back.
    pub fn swift_arrayInitWithTakeFrontToBack(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Initialize array by taking elements (no aliasing).
    pub fn swift_arrayInitWithTakeNoAlias(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Assign array elements with copies from back to front.
    pub fn swift_arrayAssignWithCopyBackToFront(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Assign array elements with copies from front to back.
    pub fn swift_arrayAssignWithCopyFrontToBack(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Assign array elements with copies (no aliasing).
    pub fn swift_arrayAssignWithCopyNoAlias(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Assign array elements by taking.
    pub fn swift_arrayAssignWithTake(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        count: usize,
        self_type: MetadataRef,
    );

    /// Destroy all elements in an array.
    pub fn swift_arrayDestroy(begin: OpaqueValueRef, count: usize, self_type: MetadataRef);
}
