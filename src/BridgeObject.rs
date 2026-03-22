#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime bridge object retain/release operations.

use core::ffi::c_void;

/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;

unsafe extern "C" {
    /// Retain a bridge object.
    pub fn swift_bridgeObjectRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Retain a bridge object n times.
    pub fn swift_bridgeObjectRetain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Release a bridge object.
    pub fn swift_bridgeObjectRelease(object: HeapObjectRef);

    /// Release a bridge object n times.
    pub fn swift_bridgeObjectRelease_n(object: HeapObjectRef, n: u32);

    /// Non-atomically retain a bridge object.
    pub fn swift_nonatomic_bridgeObjectRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Non-atomically retain a bridge object n times.
    pub fn swift_nonatomic_bridgeObjectRetain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Non-atomically release a bridge object.
    pub fn swift_nonatomic_bridgeObjectRelease(object: HeapObjectRef);

    /// Non-atomically release a bridge object n times.
    pub fn swift_nonatomic_bridgeObjectRelease_n(object: HeapObjectRef, n: u32);
}
