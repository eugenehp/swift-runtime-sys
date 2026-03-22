#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime nonatomic reference counting operations.
//! Performance-critical single-threaded paths.

use core::ffi::c_void;

/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;

unsafe extern "C" {
    /// Non-atomically retain an object.
    pub fn swift_nonatomic_retain(object: HeapObjectRef) -> HeapObjectRef;

    /// Non-atomically retain an object n times.
    pub fn swift_nonatomic_retain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Non-atomically release an object.
    pub fn swift_nonatomic_release(object: HeapObjectRef);

    /// Non-atomically release an object n times.
    pub fn swift_nonatomic_release_n(object: HeapObjectRef, n: u32);

    /// Non-atomically retain an unknown (Swift or ObjC) object.
    pub fn swift_nonatomic_unknownObjectRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Non-atomically retain an unknown object n times.
    pub fn swift_nonatomic_unknownObjectRetain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Non-atomically release an unknown object.
    pub fn swift_nonatomic_unknownObjectRelease(object: HeapObjectRef);

    /// Non-atomically release an unknown object n times.
    pub fn swift_nonatomic_unknownObjectRelease_n(object: HeapObjectRef, n: u32);

    /// Non-atomically increment unowned reference count.
    pub fn swift_nonatomic_unownedRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Non-atomically increment unowned reference count by n.
    pub fn swift_nonatomic_unownedRetain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Non-atomically decrement unowned reference count.
    pub fn swift_nonatomic_unownedRelease(object: HeapObjectRef);

    /// Non-atomically decrement unowned reference count by n.
    pub fn swift_nonatomic_unownedRelease_n(object: HeapObjectRef, n: u32);

    /// Non-atomically increment unowned and then strong reference count.
    pub fn swift_nonatomic_unownedRetainStrong(object: HeapObjectRef) -> HeapObjectRef;

    /// Non-atomically increment strong reference count and decrement unowned.
    pub fn swift_nonatomic_unownedRetainStrongAndRelease(object: HeapObjectRef) -> HeapObjectRef;
}
