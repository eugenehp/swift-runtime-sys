#![allow(non_camel_case_types)]

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;

/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;

/// Opaque pointer returned by runtime type lookup APIs.
pub type TypeLookupResultRef = *const c_void;

unsafe extern "C" {
    /// Allocates a Swift class instance for the given metadata.
    ///
    /// This only allocates memory and initializes object header state.
    /// It does not run Swift initializers.
    pub fn swift_allocObject(
        metadata: MetadataRef,
        required_size: usize,
        required_alignment_mask: usize,
    ) -> HeapObjectRef;

    /// Deallocates a class instance allocated by Swift runtime allocation paths.
    pub fn swift_deallocClassInstance(
        object: HeapObjectRef,
        allocated_size: usize,
        allocated_alignment_mask: usize,
    );

    /// Increments strong reference count.
    pub fn swift_retain(object: HeapObjectRef) -> HeapObjectRef;

    /// Decrements strong reference count.
    pub fn swift_release(object: HeapObjectRef);

    /// Looks up a type by mangled name in context.
    ///
    /// ABI details are unstable and inputs must match Swift runtime expectations.
    pub fn swift_getTypeByMangledNameInContext(
        type_name_start: *const u8,
        type_name_length: usize,
        context: *const c_void,
        generic_args: *const *const c_void,
    ) -> TypeLookupResultRef;

    /// Looks up a type by mangled name in process environment.
    pub fn swift_getTypeByMangledNameInEnvironment(
        type_name_start: *const u8,
        type_name_length: usize,
        generic_args: *const *const c_void,
        generic_args_count: usize,
    ) -> TypeLookupResultRef;
}

/// Unsafe helper around `swift_retain` with null check.
#[inline]
pub unsafe fn retain_if_nonnull(object: HeapObjectRef) -> HeapObjectRef {
    if object.is_null() {
        object
    } else {
        unsafe { swift_retain(object) }
    }
}

/// Unsafe helper around `swift_release` with null check.
#[inline]
pub unsafe fn release_if_nonnull(object: HeapObjectRef) {
    if !object.is_null() {
        unsafe { swift_release(object) }
    }
}

/// Runtime-level class allocation wrapper.
///
/// Safety requirements:
/// - `metadata` must be valid class metadata pointer.
/// - `required_size` and `required_alignment_mask` must match class layout.
/// - Returned object must either be initialized correctly or destroyed safely.
#[inline]
pub unsafe fn alloc_class_raw(
    metadata: MetadataRef,
    required_size: usize,
    required_alignment_mask: usize,
) -> HeapObjectRef {
    unsafe { swift_allocObject(metadata, required_size, required_alignment_mask) }
}
