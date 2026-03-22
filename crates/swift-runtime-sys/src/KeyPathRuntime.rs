#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime KeyPath operations.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

unsafe extern "C" {
    /// Instantiate a key path object from a pattern.
    pub fn swift_getKeyPath(pattern: *const c_void, arguments: *const c_void) -> *const c_void;

    /// Instantiate a key path (implementation variant).
    pub fn swift_getKeyPathImpl(
        pattern: *const c_void,
        arguments: *const c_void,
        arguments_size: usize,
    ) -> *const c_void;

    /// Read a value at a key path.
    pub fn swift_getAtKeyPath(result: OpaqueValueRef, root: *const c_void, key_path: *const c_void);

    /// Read a value at an AnyKeyPath.
    pub fn swift_getAtAnyKeyPath(
        result: OpaqueValueRef,
        root: *const c_void,
        key_path: *const c_void,
    );

    /// Read a value at a PartialKeyPath.
    pub fn swift_getAtPartialKeyPath(
        result: OpaqueValueRef,
        root: *const c_void,
        key_path: *const c_void,
    );

    /// Read a value through a key path.
    pub fn swift_readAtKeyPath(
        result: OpaqueValueRef,
        root: *const c_void,
        key_path: *const c_void,
    );

    /// Set a value at a WritableKeyPath.
    pub fn swift_setAtWritableKeyPath(
        root: OpaqueValueRef,
        key_path: *const c_void,
        value: *const c_void,
    );

    /// Set a value at a ReferenceWritableKeyPath.
    pub fn swift_setAtReferenceWritableKeyPath(
        root: *const c_void,
        key_path: *const c_void,
        value: *const c_void,
    );

    /// Modify a value at a WritableKeyPath.
    pub fn swift_modifyAtWritableKeyPath(root: OpaqueValueRef, key_path: *const c_void);

    /// Modify implementation for WritableKeyPath.
    pub fn swift_modifyAtWritableKeyPath_impl(root: OpaqueValueRef, key_path: *const c_void);

    /// Modify a value at a ReferenceWritableKeyPath.
    pub fn swift_modifyAtReferenceWritableKeyPath(root: *const c_void, key_path: *const c_void);

    /// Modify implementation for ReferenceWritableKeyPath.
    pub fn swift_modifyAtReferenceWritableKeyPath_impl(
        root: *const c_void,
        key_path: *const c_void,
    );

    /// Copy trivial indices for a key path.
    pub fn swift_copyKeyPathTrivialIndices(dest: *mut c_void, src: *const c_void, bytes: usize);

    /// The generic witness table for key paths.
    pub static swift_keyPathGenericWitnessTable: *const c_void;
}
