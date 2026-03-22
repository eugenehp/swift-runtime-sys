#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift in-process reflection mirror functions (different from RemoteMirror).

use core::ffi::{c_char, c_void};

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

unsafe extern "C" {
    /// Get the number of children for a value of the given type.
    pub fn swift_reflectionMirror_count(value: OpaqueValueRef, metadata: MetadataRef) -> isize;

    /// Get the total recursive count of children.
    pub fn swift_reflectionMirror_recursiveCount(metadata: MetadataRef) -> isize;

    /// Get a child of a reflected value by index.
    pub fn swift_reflectionMirror_subscript(
        result: *mut c_void,
        index: isize,
        value: OpaqueValueRef,
        metadata: MetadataRef,
        out_name: *mut *const c_char,
        out_free_func: *mut *const c_void,
    );

    /// Get the display style of a reflected value.
    pub fn swift_reflectionMirror_displayStyle(metadata: MetadataRef) -> u8;

    /// Get the normalized type for reflection.
    pub fn swift_reflectionMirror_normalizedType(
        value: OpaqueValueRef,
        metadata: MetadataRef,
    ) -> MetadataRef;

    /// Get metadata for a recursive child.
    pub fn swift_reflectionMirror_recursiveChildMetadata(
        metadata: MetadataRef,
        index: isize,
        out_name: *mut *const c_char,
        out_free_func: *mut *const c_void,
    ) -> MetadataRef;

    /// Get the offset for a recursive child.
    pub fn swift_reflectionMirror_recursiveChildOffset(
        metadata: MetadataRef,
        index: isize,
    ) -> isize;

    /// Get a quick-look object for a reflected value.
    pub fn swift_reflectionMirror_quickLookObject(
        value: OpaqueValueRef,
        metadata: MetadataRef,
    ) -> *mut c_void;
}
