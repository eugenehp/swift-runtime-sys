#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime enum operations.

use core::ffi::{c_char, c_uint, c_void};

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;
/// Opaque pointer to enum metadata.
pub type EnumMetadataRef = *const c_void;
/// Opaque pointer to a type layout.
pub type TypeLayoutRef = *const c_void;

/// Enum layout flags.
pub type EnumLayoutFlags = u32;

/// Callback type for getExtraInhabitantTag.
pub type GetExtraInhabitantTagFn = unsafe extern "C" fn(
    value: *const c_void,
    num_extra: c_uint,
    payload_type: MetadataRef,
) -> c_uint;

/// Callback type for storeExtraInhabitantTag.
pub type StoreExtraInhabitantTagFn =
    unsafe extern "C" fn(value: OpaqueValueRef, which_case: c_uint, payload_type: MetadataRef);

unsafe extern "C" {
    /// Get the name of an enum case.
    pub fn swift_EnumCaseName(metadata: MetadataRef, tag: c_uint) -> *const c_char;

    /// Initialize enum metadata for a single-case enum.
    pub fn swift_initEnumMetadataSingleCase(
        enum_type: EnumMetadataRef,
        flags: EnumLayoutFlags,
        payload: TypeLayoutRef,
    );

    /// Initialize enum metadata for a single-case enum with layout string.
    pub fn swift_initEnumMetadataSingleCaseWithLayoutString(
        enum_type: EnumMetadataRef,
        flags: EnumLayoutFlags,
        payload_type: MetadataRef,
    );

    /// Initialize enum metadata for a single-payload enum.
    pub fn swift_initEnumMetadataSinglePayload(
        enum_type: EnumMetadataRef,
        flags: EnumLayoutFlags,
        payload: TypeLayoutRef,
        empty_cases: c_uint,
    );

    /// Initialize enum metadata for a single-payload enum with layout string.
    pub fn swift_initEnumMetadataSinglePayloadWithLayoutString(
        enum_type: EnumMetadataRef,
        flags: EnumLayoutFlags,
        payload: MetadataRef,
        empty_cases: c_uint,
    );

    /// Initialize enum metadata for a multi-payload enum.
    pub fn swift_initEnumMetadataMultiPayload(
        enum_type: EnumMetadataRef,
        flags: EnumLayoutFlags,
        num_payloads: c_uint,
        payload_types: *const TypeLayoutRef,
    );

    /// Initialize enum metadata for a multi-payload enum with layout string.
    pub fn swift_initEnumMetadataMultiPayloadWithLayoutString(
        enum_type: EnumMetadataRef,
        flags: EnumLayoutFlags,
        num_payloads: c_uint,
        payload_types: *const MetadataRef,
    );

    /// Get the enum tag for a single-payload generic enum.
    pub fn swift_getEnumTagSinglePayloadGeneric(
        value: *const c_void,
        empty_cases: c_uint,
        payload_type: MetadataRef,
        get_tag: GetExtraInhabitantTagFn,
    ) -> c_uint;

    /// Store the enum tag for a single-payload generic enum.
    pub fn swift_storeEnumTagSinglePayloadGeneric(
        value: OpaqueValueRef,
        which_case: c_uint,
        empty_cases: c_uint,
        payload_type: MetadataRef,
        store_tag: StoreExtraInhabitantTagFn,
    );

    /// Get the case index for a multi-payload enum.
    pub fn swift_getEnumCaseMultiPayload(
        value: *const c_void,
        enum_type: EnumMetadataRef,
    ) -> c_uint;

    /// Store the tag for a multi-payload enum.
    pub fn swift_storeEnumTagMultiPayload(
        value: OpaqueValueRef,
        enum_type: EnumMetadataRef,
        which_case: c_uint,
    );

    /// Get the single-payload tag for multi-payload enums.
    pub fn swift_getMultiPayloadEnumTagSinglePayload(
        value: *const c_void,
        num_extra_cases: u32,
        enum_type: MetadataRef,
    ) -> c_uint;

    /// Store the single-payload tag for multi-payload enums.
    pub fn swift_storeMultiPayloadEnumTagSinglePayload(
        value: OpaqueValueRef,
        index: u32,
        num_extra_cases: u32,
        enum_type: MetadataRef,
    );

    /// Simple enum: get tag.
    pub fn swift_enumSimple_getEnumTag(value: *const c_void, metadata: MetadataRef) -> c_uint;

    /// Simple enum: destructively inject tag.
    pub fn swift_enumSimple_destructiveInjectEnumTag(
        value: OpaqueValueRef,
        tag: c_uint,
        metadata: MetadataRef,
    );

    /// Enum function-based: get tag.
    pub fn swift_enumFn_getEnumTag(value: *const c_void, metadata: MetadataRef) -> c_uint;

    /// Single-payload enum generic: get tag.
    pub fn swift_singlePayloadEnumGeneric_getEnumTag(
        value: *const c_void,
        metadata: MetadataRef,
    ) -> c_uint;

    /// Single-payload enum generic: destructively inject tag.
    pub fn swift_singlePayloadEnumGeneric_destructiveInjectEnumTag(
        value: OpaqueValueRef,
        tag: c_uint,
        metadata: MetadataRef,
    );

    /// Multi-payload enum generic: get tag.
    pub fn swift_multiPayloadEnumGeneric_getEnumTag(
        value: *const c_void,
        metadata: MetadataRef,
    ) -> c_uint;

    /// Multi-payload enum generic: destructively inject tag.
    pub fn swift_multiPayloadEnumGeneric_destructiveInjectEnumTag(
        value: OpaqueValueRef,
        tag: c_uint,
        metadata: MetadataRef,
    );

    /// Singleton enum: get tag.
    pub fn swift_singletonEnum_getEnumTag(value: *const c_void, metadata: MetadataRef) -> c_uint;

    /// Singleton enum: destructively inject tag.
    pub fn swift_singletonEnum_destructiveInjectEnumTag(
        value: OpaqueValueRef,
        tag: c_uint,
        metadata: MetadataRef,
    );
}
