#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime class and struct metadata initialization.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a type context descriptor.
pub type TypeContextDescriptorRef = *const c_void;
/// Opaque pointer to a class descriptor.
pub type ClassDescriptorRef = *const c_void;
/// Opaque pointer to a value type descriptor.
pub type ValueTypeDescriptorRef = *const c_void;

unsafe extern "C" {
    /// Initialize class metadata.
    pub fn swift_initClassMetadata(
        metadata: MetadataRef,
        flags: usize,
        num_fields: usize,
        field_types: *const MetadataRef,
        field_offsets: *mut usize,
    );

    /// Initialize class metadata (v2).
    pub fn swift_initClassMetadata2(
        metadata: MetadataRef,
        flags: usize,
        num_fields: usize,
        field_types: *const *const c_void,
        field_offsets: *mut usize,
    );

    /// Initialize struct metadata.
    pub fn swift_initStructMetadata(
        metadata: MetadataRef,
        flags: usize,
        num_fields: usize,
        field_types: *const *const c_void,
        field_offsets: *mut u32,
    );

    /// Initialize struct metadata with layout string.
    pub fn swift_initStructMetadataWithLayoutString(
        metadata: MetadataRef,
        flags: usize,
        num_fields: usize,
        field_types: *const *const c_void,
        field_offsets: *mut u32,
    );

    /// Initialize raw struct metadata.
    pub fn swift_initRawStructMetadata(
        metadata: MetadataRef,
        flags: usize,
        field_types: *const *const c_void,
        field_offsets: *mut u32,
    );

    /// Initialize raw struct metadata (v2).
    pub fn swift_initRawStructMetadata2(
        metadata: MetadataRef,
        flags: usize,
        field_types: *const *const c_void,
        field_offsets: *mut u32,
    );

    /// Update class metadata after initialization.
    pub fn swift_updateClassMetadata(
        metadata: MetadataRef,
        flags: usize,
        num_fields: usize,
        field_types: *const *const c_void,
        field_offsets: *mut usize,
    );

    /// Update class metadata (v2).
    pub fn swift_updateClassMetadata2(
        metadata: MetadataRef,
        flags: usize,
        num_fields: usize,
        field_types: *const *const c_void,
        field_offsets: *mut usize,
    );

    /// Relocate class metadata for a resilient superclass.
    pub fn swift_relocateClassMetadata(metadata: MetadataRef, superclass: MetadataRef);

    /// Set the class metadata for an object.
    pub fn swift_setClassMetadata(metadata: MetadataRef, superclass: MetadataRef);

    /// Update pure ObjC class metadata.
    pub fn swift_updatePureObjCClassMetadata(
        cls: *mut c_void,
        flags: usize,
        num_fields: usize,
        field_offsets: *mut usize,
    );

    /// Allocate generic class metadata.
    pub fn swift_allocateGenericClassMetadata(
        description: ClassDescriptorRef,
        arguments: *const c_void,
        pattern: *const c_void,
    ) -> MetadataRef;

    /// Allocate generic class metadata with layout string.
    pub fn swift_allocateGenericClassMetadataWithLayoutString(
        description: ClassDescriptorRef,
        arguments: *const c_void,
        pattern: *const c_void,
    ) -> MetadataRef;

    /// Allocate generic value metadata.
    pub fn swift_allocateGenericValueMetadata(
        description: ValueTypeDescriptorRef,
        arguments: *const c_void,
        pattern: *const c_void,
        extra_data_size: usize,
    ) -> MetadataRef;

    /// Allocate generic value metadata with layout string.
    pub fn swift_allocateGenericValueMetadataWithLayoutString(
        description: ValueTypeDescriptorRef,
        arguments: *const c_void,
        pattern: *const c_void,
        extra_data_size: usize,
    ) -> MetadataRef;

    /// Allocate a metadata pack.
    pub fn swift_allocateMetadataPack(
        elements: *const MetadataRef,
        num_elements: usize,
    ) -> *const c_void;

    /// Allocate a witness table pack.
    pub fn swift_allocateWitnessTablePack(
        tables: *const *const c_void,
        num_tables: usize,
    ) -> *const c_void;

    /// Instantiate an ObjC class.
    pub fn swift_instantiateObjCClass(cls: *mut c_void);
}
