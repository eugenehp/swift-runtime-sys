#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime memory and allocation primitives.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;

unsafe extern "C" {
    /// Allocate memory with the given size and alignment.
    pub fn swift_slowAlloc(size: usize, align_mask: usize) -> *mut c_void;

    /// Deallocate memory previously allocated with swift_slowAlloc.
    pub fn swift_slowDealloc(ptr: *mut c_void, size: usize, align_mask: usize);

    /// Allocate a buffer.
    pub fn swift_bufferAllocate(
        buffer: *mut c_void,
        capacity: usize,
        alignment_mask: usize,
    ) -> *mut c_void;

    /// Initialize an object header for a stack-allocated object.
    pub fn swift_initStackObject(metadata: MetadataRef, object: HeapObjectRef) -> HeapObjectRef;

    /// Initialize an object header for a statically allocated object.
    pub fn swift_initStaticObject(metadata: MetadataRef, object: HeapObjectRef) -> HeapObjectRef;

    /// Initialize an inert heap object.
    pub fn swift_instantiateInertHeapObject(
        metadata: MetadataRef,
        object: HeapObjectRef,
    ) -> HeapObjectRef;

    /// Check if an object is being deallocated.
    pub fn swift_isDeallocating(object: HeapObjectRef) -> bool;

    /// Set the deallocating flag on an object.
    pub fn swift_setDeallocating(object: HeapObjectRef);

    /// Verify that a stack-allocated object's lifetime has ended.
    pub fn swift_verifyEndOfLifetime(object: HeapObjectRef);

    /// Deallocate an object.
    pub fn swift_deallocObject(
        object: HeapObjectRef,
        allocated_size: usize,
        allocated_align_mask: usize,
    );

    /// Deallocate an uninitialized object.
    pub fn swift_deallocUninitializedObject(
        object: HeapObjectRef,
        allocated_size: usize,
        allocated_align_mask: usize,
    );

    /// Is this pointer a non-null unique reference?
    pub fn swift_isUniquelyReferenced(object: *const c_void) -> bool;

    /// Is this non-null pointer a unique reference (native Swift)?
    pub fn swift_isUniquelyReferenced_native(object: *const c_void) -> bool;

    /// Is this non-null pointer a unique reference?
    pub fn swift_isUniquelyReferenced_nonNull(object: *const c_void) -> bool;

    /// Is this non-null native pointer a unique reference?
    pub fn swift_isUniquelyReferenced_nonNull_native(object: *const c_void) -> bool;

    /// Is this non-null BridgeObject a unique reference?
    pub fn swift_isUniquelyReferenced_nonNull_bridgeObject(bits: usize) -> bool;

    /// Is this pointer a non-null unique reference to a non-ObjC object?
    pub fn swift_isUniquelyReferencedNonObjC(object: *const c_void) -> bool;

    /// Is this non-null pointer a unique reference to a non-ObjC object?
    pub fn swift_isUniquelyReferencedNonObjC_nonNull(object: *const c_void) -> bool;

    /// Is this non-null BridgeObject a unique reference to a non-ObjC object?
    pub fn swift_isUniquelyReferencedNonObjC_nonNull_bridgeObject(bits: usize) -> bool;

    /// Check if COW checks are enabled.
    pub fn swift_COWChecksEnabled() -> bool;

    /// Securely clear sensitive memory.
    pub fn swift_clearSensitive(ptr: *mut c_void, size: usize);

    /// Try to retain an object. Returns null if the object is being deallocated.
    pub fn swift_tryRetain(object: HeapObjectRef) -> HeapObjectRef;

    /// Retain an object n times.
    pub fn swift_retain_n(object: HeapObjectRef, n: u32) -> HeapObjectRef;

    /// Release an object n times.
    pub fn swift_release_n(object: HeapObjectRef, n: u32);

    /// Get the weak reference count.
    pub fn swift_weakRetainCount(object: HeapObjectRef) -> usize;
}
