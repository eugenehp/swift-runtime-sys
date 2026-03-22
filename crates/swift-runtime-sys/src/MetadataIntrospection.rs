#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime metadata introspection functions.

use core::ffi::{c_char, c_void};

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;
/// Opaque pointer to a type context descriptor.
pub type TypeContextDescriptorRef = *const c_void;
/// Opaque pointer to a witness table.
pub type WitnessTableRef = *const c_void;
/// Opaque pointer to a protocol descriptor.
pub type ProtocolDescriptorRef = *const c_void;

/// Return type for swift_getTypeName / swift_getMangledTypeName.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TypeNamePair {
    pub data: *const c_char,
    pub length: usize,
}

/// Return type for metadata requests (MetadataResponse).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetadataResponse {
    pub metadata: MetadataRef,
    pub state: usize,
}

/// A metadata request (desired state).
pub type MetadataRequest = usize;

unsafe extern "C" {
    // ── Core metadata ──

    /// Fetch uniqued metadata for a generic nominal type.
    pub fn swift_getGenericMetadata(
        request: MetadataRequest,
        arguments: *const *const c_void,
        description: TypeContextDescriptorRef,
    ) -> MetadataResponse;

    /// Get the kind of a metadata record.
    pub fn swift_getMetadataKind(metadata: MetadataRef) -> usize;

    /// Fetch a uniqued metatype metadata.
    pub fn swift_getMetatypeMetadata(instance_type: MetadataRef) -> MetadataRef;

    /// Get the runtime type metadata of a heap object.
    pub fn swift_getObjectType(object: HeapObjectRef) -> MetadataRef;

    /// Return the human-readable name of a Swift type.
    pub fn swift_getTypeName(metadata: MetadataRef, qualified: bool) -> TypeNamePair;

    /// Return the mangled name of a Swift type.
    pub fn swift_getMangledTypeName(metadata: MetadataRef) -> TypeNamePair;

    /// Get the type context descriptor from metadata.
    pub fn swift_getTypeContextDescriptor(metadata: MetadataRef) -> TypeContextDescriptorRef;

    /// Check that metadata has the right state.
    pub fn swift_checkMetadataState(
        request: MetadataRequest,
        metadata: MetadataRef,
    ) -> MetadataResponse;

    /// Fetch uniqued metadata for a singleton type.
    pub fn swift_getSingletonMetadata(
        request: MetadataRequest,
        description: TypeContextDescriptorRef,
    ) -> MetadataResponse;

    /// Fetch uniqued metadata for a canonical specialized generic type.
    pub fn swift_getCanonicalSpecializedMetadata(
        request: MetadataRequest,
        candidate: MetadataRef,
        cache: *mut MetadataRef,
    ) -> MetadataResponse;

    /// Fetch uniqued metadata for a canonical prespecialized generic type.
    pub fn swift_getCanonicalPrespecializedGenericMetadata(
        request: MetadataRequest,
        arguments: *const *const c_void,
        description: TypeContextDescriptorRef,
        token: *mut usize,
    ) -> MetadataResponse;

    /// Get the ObjC class metadata from type metadata.
    pub fn swift_getObjCClassFromMetadata(metadata: MetadataRef) -> MetadataRef;

    /// Get the ObjC class from a heap object.
    pub fn swift_getObjCClassFromObject(object: HeapObjectRef) -> MetadataRef;

    /// Fetch uniqued type metadata for an ObjC class.
    pub fn swift_getObjCClassMetadata(the_class: MetadataRef) -> MetadataRef;

    /// Get the initialized ObjC class.
    pub fn swift_getInitializedObjCClass(c: *mut c_void) -> *mut c_void;

    /// Check if a type is a class type.
    pub fn swift_isClassType(metadata: MetadataRef) -> bool;

    /// Check if a type is an Optional type.
    pub fn swift_isOptionalType(metadata: MetadataRef) -> MetadataRef;

    /// Check if a type is a class or ObjC existential type.
    pub fn swift_isClassOrObjCExistentialType(metadata: MetadataRef) -> bool;

    /// Check if one class is a subclass of another.
    pub fn swift_class_isSubclass(subclass: MetadataRef, superclass: MetadataRef) -> bool;

    // ── Function type metadata ──

    /// Fetch uniqued metadata for a function type.
    pub fn swift_getFunctionTypeMetadata(
        flags: usize,
        parameters: *const MetadataRef,
        parameter_flags: *const u32,
        result: MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for a 0-parameter function type.
    pub fn swift_getFunctionTypeMetadata0(flags: usize, result: MetadataRef) -> MetadataRef;

    /// Fetch uniqued metadata for a 1-parameter function type.
    pub fn swift_getFunctionTypeMetadata1(
        flags: usize,
        param0: MetadataRef,
        result: MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for a 2-parameter function type.
    pub fn swift_getFunctionTypeMetadata2(
        flags: usize,
        param0: MetadataRef,
        param1: MetadataRef,
        result: MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for a 3-parameter function type.
    pub fn swift_getFunctionTypeMetadata3(
        flags: usize,
        param0: MetadataRef,
        param1: MetadataRef,
        param2: MetadataRef,
        result: MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for a differentiable function type.
    pub fn swift_getFunctionTypeMetadataDifferentiable(
        flags: usize,
        differential_flags: usize,
        parameters: *const MetadataRef,
        parameter_flags: *const u32,
        result: MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for a global-actor function type.
    pub fn swift_getFunctionTypeMetadataGlobalActor(
        flags: usize,
        differential_flags: usize,
        parameters: *const MetadataRef,
        parameter_flags: *const u32,
        result: MetadataRef,
        global_actor: MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for an extended function type.
    pub fn swift_getExtendedFunctionTypeMetadata(
        flags: usize,
        differential_flags: usize,
        parameters: *const MetadataRef,
        parameter_flags: *const u32,
        result: MetadataRef,
        extended_flags: *const c_void,
    ) -> MetadataRef;

    /// Get the number of parameters in a function type.
    pub fn swift_func_getParameterCount(metadata: MetadataRef) -> usize;

    /// Get the type info for a parameter.
    pub fn swift_func_getParameterTypeInfo(metadata: MetadataRef, index: usize) -> MetadataRef;

    /// Get the return type info.
    pub fn swift_func_getReturnTypeInfo(metadata: MetadataRef) -> MetadataRef;

    /// Get a function's full name from its mangled name.
    pub fn swift_getFunctionFullNameFromMangledName(
        mangled_name: *const c_char,
        mangled_name_length: usize,
    ) -> TypeNamePair;

    // ── Tuple type metadata ──

    /// Fetch uniqued metadata for a tuple type.
    pub fn swift_getTupleTypeMetadata(
        request: MetadataRequest,
        flags: usize,
        elements: *const MetadataRef,
        labels: *const c_char,
        proposed_witnesses: *const c_void,
    ) -> MetadataResponse;

    /// Fetch uniqued metadata for a 2-element tuple type.
    pub fn swift_getTupleTypeMetadata2(
        request: MetadataRequest,
        elt0: MetadataRef,
        elt1: MetadataRef,
        labels: *const c_char,
        proposed_witnesses: *const c_void,
    ) -> MetadataResponse;

    /// Fetch uniqued metadata for a 3-element tuple type.
    pub fn swift_getTupleTypeMetadata3(
        request: MetadataRequest,
        elt0: MetadataRef,
        elt1: MetadataRef,
        elt2: MetadataRef,
        labels: *const c_char,
        proposed_witnesses: *const c_void,
    ) -> MetadataResponse;

    /// Compute the layout of a tuple type without full metadata.
    pub fn swift_getTupleTypeLayout(
        result: *mut c_void, // TypeLayout *
        element_offsets: *mut u32,
        flags: usize,
        elements: *const c_void,
    );

    /// Compute layout for a 2-element tuple type.
    pub fn swift_getTupleTypeLayout2(
        result: *mut c_void,
        element_offsets: *mut usize,
        elt0: *const c_void,
        elt1: *const c_void,
    );

    /// Compute layout for a 3-element tuple type.
    pub fn swift_getTupleTypeLayout3(
        result: *mut c_void,
        element_offsets: *mut usize,
        elt0: *const c_void,
        elt1: *const c_void,
        elt2: *const c_void,
    );

    // ── Existential type metadata ──

    /// Fetch uniqued metadata for an existential type.
    pub fn swift_getExistentialTypeMetadata(
        class_constraint: usize,
        superclass: MetadataRef,
        num_protocols: usize,
        protocols: *const *const c_void,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for an existential metatype.
    pub fn swift_getExistentialMetatypeMetadata(instance_type: MetadataRef) -> MetadataRef;

    /// Fetch uniqued metadata for an extended existential type.
    pub fn swift_getExtendedExistentialTypeMetadata(
        shape: *const c_void,
        generalization_args: *const MetadataRef,
    ) -> MetadataRef;

    /// Fetch uniqued metadata for an extended existential type (unique variant).
    pub fn swift_getExtendedExistentialTypeMetadata_unique(
        shape: *const c_void,
        generalization_args: *const MetadataRef,
    ) -> MetadataRef;

    /// Get the shape of an extended existential type.
    pub fn swift_getExtendedExistentialTypeShape(descriptor: *const c_void) -> *const c_void;

    /// Assign a value into an existential container with copy semantics.
    pub fn swift_assignExistentialWithCopy(
        dest: *mut c_void,
        src: *const c_void,
        existential_type: MetadataRef,
    );

    // ── Fixed array metadata ──

    /// Fetch uniqued metadata for a fixed-size array type.
    pub fn swift_getFixedArrayTypeMetadata(
        request: MetadataRequest,
        count: usize,
        element: MetadataRef,
    ) -> MetadataResponse;

    // ── Mangled name lookup ──

    /// Look up a type by mangled name.
    pub fn swift_getTypeByMangledName(
        type_name_start: *const c_char,
        type_name_length: usize,
        generic_args: *const *const c_void,
        generic_args_count: usize,
    ) -> MetadataRef;

    /// Look up a type by mangled name, v2.
    pub fn swift_getTypeByMangledNameInContext2(
        type_name_start: *const u8,
        type_name_length: usize,
        context: *const c_void,
        generic_args: *const *const c_void,
    ) -> MetadataRef;

    /// Look up a type by mangled name in metadata state.
    pub fn swift_getTypeByMangledNameInContextInMetadataState(
        request: MetadataRequest,
        type_name_start: *const u8,
        type_name_length: usize,
        context: *const c_void,
        generic_args: *const *const c_void,
    ) -> MetadataResponse;

    /// Look up a type by mangled name in metadata state, v2.
    pub fn swift_getTypeByMangledNameInContextInMetadataState2(
        request: MetadataRequest,
        type_name_start: *const u8,
        type_name_length: usize,
        context: *const c_void,
        generic_args: *const *const c_void,
    ) -> MetadataResponse;

    /// Look up a type by mangled name in environment metadata state.
    pub fn swift_getTypeByMangledNameInEnvironmentInMetadataState(
        request: MetadataRequest,
        type_name_start: *const u8,
        type_name_length: usize,
        generic_args: *const *const c_void,
    ) -> MetadataResponse;

    /// Look up a type by mangled demangling node.
    pub fn swift_getTypeByMangledNode(
        request: MetadataRequest,
        node: *const c_void,
        arguments: *const *const c_void,
        size: usize,
    ) -> MetadataResponse;

    // ── Foreign type metadata ──

    /// Fetch uniqued metadata for a foreign type.
    pub fn swift_getForeignTypeMetadata(
        request: MetadataRequest,
        non_unique: MetadataRef,
    ) -> MetadataResponse;
}
