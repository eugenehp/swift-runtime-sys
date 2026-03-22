#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime unowned reference operations.

use core::ffi::c_void;

/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;

unsafe extern "C" {
    /// Increment unowned reference count.
    pub fn swift_unownedRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Decrement unowned reference count.
    pub fn swift_unownedRelease(object: HeapObjectRef);

    /// Increment unowned reference count by n.
    pub fn swift_unownedRetain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Decrement unowned reference count by n.
    pub fn swift_unownedRelease_n(object: HeapObjectRef, n: u32);

    /// Increment unowned reference count and then increment strong reference count.
    pub fn swift_unownedRetainStrong(object: HeapObjectRef) -> HeapObjectRef;

    /// Increment strong reference count and then decrement unowned reference count.
    pub fn swift_unownedRetainStrongAndRelease(object: HeapObjectRef) -> HeapObjectRef;

    /// Assert that the object is not deallocated (for unowned references).
    pub fn swift_unownedCheck(object: HeapObjectRef);

    /// Get the unowned reference count.
    pub fn swift_unownedRetainCount(object: HeapObjectRef) -> usize;
}
