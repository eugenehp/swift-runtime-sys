#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime unknown-object (ObjC-bridged) retain/release and
//! unowned/weak reference operations.

use core::ffi::c_void;

/// Opaque pointer to a heap object (Swift or ObjC).
pub type HeapObjectRef = *mut c_void;
/// Opaque pointer to an unowned reference storage.
pub type UnownedRefRef = *mut c_void;
/// Opaque pointer to a weak reference storage.
pub type WeakRefRef = *mut c_void;

unsafe extern "C" {
    // ── Strong ──

    /// Retain an unknown object (Swift or ObjC).
    pub fn swift_unknownObjectRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Retain an unknown object n times.
    pub fn swift_unknownObjectRetain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Release an unknown object.
    pub fn swift_unknownObjectRelease(object: HeapObjectRef);

    /// Release an unknown object n times.
    pub fn swift_unknownObjectRelease_n(object: HeapObjectRef, n: u32);

    // ── Unowned ──

    /// Initialize an unowned reference to an unknown object.
    pub fn swift_unknownObjectUnownedInit(dest: UnownedRefRef, object: HeapObjectRef);

    /// Assign an unknown object to an unowned reference.
    pub fn swift_unknownObjectUnownedAssign(dest: UnownedRefRef, object: HeapObjectRef);

    /// Copy-initialize an unowned reference.
    pub fn swift_unknownObjectUnownedCopyInit(dest: UnownedRefRef, src: UnownedRefRef);

    /// Copy-assign an unowned reference.
    pub fn swift_unknownObjectUnownedCopyAssign(dest: UnownedRefRef, src: UnownedRefRef);

    /// Take-initialize an unowned reference (moves from src).
    pub fn swift_unknownObjectUnownedTakeInit(dest: UnownedRefRef, src: UnownedRefRef);

    /// Take-assign an unowned reference (moves from src).
    pub fn swift_unknownObjectUnownedTakeAssign(dest: UnownedRefRef, src: UnownedRefRef);

    /// Destroy an unowned reference.
    pub fn swift_unknownObjectUnownedDestroy(ref_storage: UnownedRefRef);

    /// Load a strong reference from an unowned reference.
    pub fn swift_unknownObjectUnownedLoadStrong(ref_storage: UnownedRefRef) -> HeapObjectRef;

    /// Check if two unowned references are equal.
    pub fn swift_unknownObjectUnownedIsEqual(
        lhs: UnownedRefRef,
        rhs: HeapObjectRef,
    ) -> bool;

    // ── Weak ──

    /// Initialize a weak reference to an unknown object.
    pub fn swift_unknownObjectWeakInit(dest: WeakRefRef, object: HeapObjectRef);

    /// Assign an unknown object to a weak reference.
    pub fn swift_unknownObjectWeakAssign(dest: WeakRefRef, object: HeapObjectRef);

    /// Copy-initialize a weak reference.
    pub fn swift_unknownObjectWeakCopyInit(dest: WeakRefRef, src: WeakRefRef);

    /// Copy-assign a weak reference.
    pub fn swift_unknownObjectWeakCopyAssign(dest: WeakRefRef, src: WeakRefRef);

    /// Take-initialize a weak reference (moves from src).
    pub fn swift_unknownObjectWeakTakeInit(dest: WeakRefRef, src: WeakRefRef);

    /// Take-assign a weak reference (moves from src).
    pub fn swift_unknownObjectWeakTakeAssign(dest: WeakRefRef, src: WeakRefRef);

    /// Destroy a weak reference.
    pub fn swift_unknownObjectWeakDestroy(ref_storage: WeakRefRef);

    /// Load a strong reference from a weak reference. Returns null if deallocated.
    pub fn swift_unknownObjectWeakLoadStrong(ref_storage: WeakRefRef) -> HeapObjectRef;

    /// Take a strong reference from a weak reference, destroying the weak ref.
    pub fn swift_unknownObjectWeakTakeStrong(ref_storage: WeakRefRef) -> HeapObjectRef;
}
