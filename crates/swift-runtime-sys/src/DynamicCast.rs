#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime dynamic casting functions (`as?`, `as!`, `is`).

use core::ffi::{c_char, c_uint, c_void};

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;

/// Flags for `swift_dynamicCast`.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum DynamicCastFlags {
    /// No flags.
    Default = 0,
    /// True if the cast is not permitted to fail.
    Unconditional = 0x1,
    /// True if the cast should destroy the source value on success.
    TakeOnSuccess = 0x2,
    /// True if the cast should destroy the source value on failure.
    DestroyOnFailure = 0x4,
}

unsafe extern "C" {
    /// Perform a checked dynamic cast of a value to a target type.
    pub fn swift_dynamicCast(
        dest: OpaqueValueRef,
        src: OpaqueValueRef,
        src_type: MetadataRef,
        target_type: MetadataRef,
        flags: u32,
    ) -> bool;

    /// Checked dynamic cast to a Swift class type. Returns null on failure.
    pub fn swift_dynamicCastClass(object: *const c_void, target_type: MetadataRef)
        -> *const c_void;

    /// Unconditional checked dynamic cast to a Swift class type. Aborts on failure.
    pub fn swift_dynamicCastClassUnconditional(
        object: *const c_void,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> *const c_void;

    /// Checked Objective-C-style dynamic cast to a class type.
    pub fn swift_dynamicCastObjCClass(
        object: *const c_void,
        target_type: MetadataRef,
    ) -> *const c_void;

    /// Unconditional Objective-C-style dynamic cast to a class type.
    pub fn swift_dynamicCastObjCClassUnconditional(
        object: *const c_void,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> *const c_void;

    /// Checked dynamic cast to a foreign class type.
    pub fn swift_dynamicCastForeignClass(
        object: *const c_void,
        target_type: MetadataRef,
    ) -> *const c_void;

    /// Unconditional dynamic cast to a foreign class type.
    pub fn swift_dynamicCastForeignClassUnconditional(
        object: *const c_void,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> *const c_void;

    /// Checked dynamic cast of a metatype.
    pub fn swift_dynamicCastMetatype(
        source_type: MetadataRef,
        target_type: MetadataRef,
    ) -> MetadataRef;

    /// Unconditional dynamic cast of a metatype.
    pub fn swift_dynamicCastMetatypeUnconditional(
        source_type: MetadataRef,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> MetadataRef;

    /// Checked dynamic cast of a foreign class metatype.
    pub fn swift_dynamicCastForeignClassMetatype(
        source_type: MetadataRef,
        target_type: MetadataRef,
    ) -> MetadataRef;

    /// Unconditional dynamic cast of a foreign class metatype.
    pub fn swift_dynamicCastForeignClassMetatypeUnconditional(
        source_type: MetadataRef,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> MetadataRef;

    /// Checked dynamic cast of an ObjC class metatype.
    pub fn swift_dynamicCastObjCClassMetatype(
        source_type: MetadataRef,
        target_type: MetadataRef,
    ) -> MetadataRef;

    /// Unconditional dynamic cast of an ObjC class metatype.
    pub fn swift_dynamicCastObjCClassMetatypeUnconditional(
        source_type: MetadataRef,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> MetadataRef;

    /// Conditional cast of a metatype to an ObjC object.
    pub fn swift_dynamicCastMetatypeToObjectConditional(source_type: MetadataRef) -> *const c_void;

    /// Unconditional cast of a metatype to an ObjC object.
    pub fn swift_dynamicCastMetatypeToObjectUnconditional(
        source_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> *const c_void;

    /// Checked dynamic cast to an ObjC protocol. Returns null on failure.
    pub fn swift_dynamicCastObjCProtocolConditional(
        object: *const c_void,
        num_protocols: usize,
        protocols: *const *const c_void,
    ) -> *const c_void;

    /// Unconditional dynamic cast to an ObjC protocol.
    pub fn swift_dynamicCastObjCProtocolUnconditional(
        object: *const c_void,
        num_protocols: usize,
        protocols: *const *const c_void,
    ) -> *const c_void;

    /// Conditional cast of a type to an ObjC protocol.
    pub fn swift_dynamicCastTypeToObjCProtocolConditional(
        source_type: MetadataRef,
        num_protocols: usize,
        protocols: *const *const c_void,
    ) -> bool;

    /// Unconditional cast of a type to an ObjC protocol.
    pub fn swift_dynamicCastTypeToObjCProtocolUnconditional(
        source_type: MetadataRef,
        num_protocols: usize,
        protocols: *const *const c_void,
    ) -> bool;

    /// Checked dynamic cast of a class instance pointer to the given type.
    pub fn swift_dynamicCastUnknownClass(
        object: *const c_void,
        target_type: MetadataRef,
    ) -> *const c_void;

    /// Unconditional checked dynamic cast of a class instance pointer.
    pub fn swift_dynamicCastUnknownClassUnconditional(
        object: *const c_void,
        target_type: MetadataRef,
        file: *const c_char,
        line: c_uint,
        column: c_uint,
    ) -> *const c_void;

    /// Fetch the dynamic type of a value.
    pub fn swift_getDynamicType(
        value: OpaqueValueRef,
        self_type: MetadataRef,
        existential_metatype: bool,
    ) -> MetadataRef;
}
