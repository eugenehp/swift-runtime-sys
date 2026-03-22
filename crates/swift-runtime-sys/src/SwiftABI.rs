#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Core Swift ABI struct layouts — the actual binary representation of
//! metadata, heap objects, value witness tables, and type descriptors.
//!
//! These layouts allow reading/writing Swift runtime data structures directly
//! from Rust, instead of treating everything as opaque `*const c_void`.

use core::ffi::c_void;

// ═══════════════════════════════════════════════════════════════════════════
// Metadata Kind
// ═══════════════════════════════════════════════════════════════════════════

/// Bit flags used in metadata kind values.
pub const METADATA_KIND_IS_NON_TYPE: u32 = 0x400;
pub const METADATA_KIND_IS_NON_HEAP: u32 = 0x200;
pub const METADATA_KIND_IS_RUNTIME_PRIVATE: u32 = 0x100;

/// Swift metadata kind enumeration.
/// For class types the Kind field is actually the isa pointer.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataKind {
    Class = 0,
    Struct = 0 | METADATA_KIND_IS_NON_HEAP,
    Enum = 1 | METADATA_KIND_IS_NON_HEAP,
    Optional = 2 | METADATA_KIND_IS_NON_HEAP,
    ForeignClass = 3 | METADATA_KIND_IS_NON_HEAP,
    ForeignReferenceType = 4 | METADATA_KIND_IS_NON_HEAP,
    Opaque = 0 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    Tuple = 1 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    Function = 2 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    Existential = 3 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    Metatype = 4 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    ObjCClassWrapper = 5 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    ExistentialMetatype = 6 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    ExtendedExistential = 7 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    FixedArray = 8 | METADATA_KIND_IS_RUNTIME_PRIVATE | METADATA_KIND_IS_NON_HEAP,
    HeapLocalVariable = 0 | METADATA_KIND_IS_NON_TYPE,
    HeapGenericLocalVariable = 0 | METADATA_KIND_IS_NON_TYPE | METADATA_KIND_IS_RUNTIME_PRIVATE,
    ErrorObject = 1 | METADATA_KIND_IS_NON_TYPE | METADATA_KIND_IS_RUNTIME_PRIVATE,
    Task = 2 | METADATA_KIND_IS_NON_TYPE | METADATA_KIND_IS_RUNTIME_PRIVATE,
    Job = 3 | METADATA_KIND_IS_NON_TYPE | METADATA_KIND_IS_RUNTIME_PRIVATE,
}

pub const LAST_ENUMERATED_METADATA_KIND: u32 = 0x7FF;

/// Interpret a raw kind value: if > LAST_ENUMERATED, it's a class isa pointer.
pub fn get_enumerated_metadata_kind(kind: usize) -> MetadataKind {
    if kind > LAST_ENUMERATED_METADATA_KIND as usize {
        MetadataKind::Class
    } else {
        // Safety: we've checked bounds
        unsafe { core::mem::transmute(kind as u32) }
    }
}

pub fn is_heap_metadata_kind(kind: MetadataKind) -> bool {
    (kind as u32) & METADATA_KIND_IS_NON_HEAP == 0
}

pub fn is_type_metadata_kind(kind: MetadataKind) -> bool {
    (kind as u32) & METADATA_KIND_IS_NON_TYPE == 0
}

// ═══════════════════════════════════════════════════════════════════════════
// Heap Object Header
// ═══════════════════════════════════════════════════════════════════════════

/// The Swift heap object header. Every Swift class instance starts with this.
///
/// Layout (64-bit):
///   offset 0: metadata pointer (8 bytes)
///   offset 8: InlineRefCounts (8 bytes)
/// Total: 16 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HeapObject {
    /// Pointer to the type metadata (isa pointer for classes).
    pub metadata: *const c_void,
    /// Inline reference counts (strong + unowned packed into 8 bytes).
    pub ref_counts: u64,
}

pub const HEAP_OBJECT_HEADER_SIZE: usize = 16;

impl HeapObject {
    /// Read the HeapObject header from a raw object pointer.
    ///
    /// # Safety
    /// `ptr` must point to a valid Swift heap object.
    pub unsafe fn from_ptr(ptr: *const c_void) -> &'static HeapObject {
        &*(ptr as *const HeapObject)
    }

    /// Get the metadata kind for this object.
    pub fn metadata_kind(&self) -> MetadataKind {
        // For class objects, the metadata pointer IS the isa pointer,
        // and the Kind field is at offset 0 of the metadata.
        let kind_raw = unsafe { *(self.metadata as *const usize) };
        get_enumerated_metadata_kind(kind_raw)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Metadata Header (lives BEFORE the metadata pointer)
// ═══════════════════════════════════════════════════════════════════════════

/// The value witness table pointer lives at (metadata_ptr - pointer_size).
/// This function retrieves it.
///
/// # Safety
/// `metadata` must point to valid Swift type metadata.
pub unsafe fn get_value_witness_table(metadata: *const c_void) -> *const ValueWitnessTable {
    let ptr = metadata as *const *const ValueWitnessTable;
    *ptr.offset(-1)
}

// ═══════════════════════════════════════════════════════════════════════════
// Value Witness Table
// ═══════════════════════════════════════════════════════════════════════════

/// Function pointer types for value witnesses.
pub type InitializeBufferWithCopyOfBufferFn = unsafe extern "C" fn(
    dest: *mut c_void,
    src: *mut c_void,
    metadata: *const c_void,
) -> *mut c_void;
pub type DestroyFn = unsafe extern "C" fn(value: *mut c_void, metadata: *const c_void);
pub type InitializeWithCopyFn = unsafe extern "C" fn(
    dest: *mut c_void,
    src: *mut c_void,
    metadata: *const c_void,
) -> *mut c_void;
pub type AssignWithCopyFn = unsafe extern "C" fn(
    dest: *mut c_void,
    src: *mut c_void,
    metadata: *const c_void,
) -> *mut c_void;
pub type InitializeWithTakeFn = unsafe extern "C" fn(
    dest: *mut c_void,
    src: *mut c_void,
    metadata: *const c_void,
) -> *mut c_void;
pub type AssignWithTakeFn = unsafe extern "C" fn(
    dest: *mut c_void,
    src: *mut c_void,
    metadata: *const c_void,
) -> *mut c_void;
pub type GetEnumTagSinglePayloadFn =
    unsafe extern "C" fn(value: *const c_void, empty_cases: u32, metadata: *const c_void) -> u32;
pub type StoreEnumTagSinglePayloadFn = unsafe extern "C" fn(
    value: *mut c_void,
    which_case: u32,
    empty_cases: u32,
    metadata: *const c_void,
);
pub type GetEnumTagFn = unsafe extern "C" fn(value: *const c_void, metadata: *const c_void) -> u32;
pub type DestructiveProjectEnumDataFn =
    unsafe extern "C" fn(value: *mut c_void, metadata: *const c_void);
pub type DestructiveInjectEnumTagFn =
    unsafe extern "C" fn(value: *mut c_void, tag: u32, metadata: *const c_void);

/// The value witness table. This is the actual binary layout.
/// Lives at metadata_pointer[-1].
///
/// Offsets (64-bit):
///   0x00: initializeBufferWithCopyOfBuffer
///   0x08: destroy
///   0x10: initializeWithCopy
///   0x18: assignWithCopy
///   0x20: initializeWithTake
///   0x28: assignWithTake
///   0x30: getEnumTagSinglePayload
///   0x38: storeEnumTagSinglePayload
///   0x40: size
///   0x48: stride
///   0x50: flags
///   0x54: extraInhabitantCount
#[repr(C)]
pub struct ValueWitnessTable {
    pub initialize_buffer_with_copy_of_buffer: InitializeBufferWithCopyOfBufferFn,
    pub destroy: DestroyFn,
    pub initialize_with_copy: InitializeWithCopyFn,
    pub assign_with_copy: AssignWithCopyFn,
    pub initialize_with_take: InitializeWithTakeFn,
    pub assign_with_take: AssignWithTakeFn,
    pub get_enum_tag_single_payload: GetEnumTagSinglePayloadFn,
    pub store_enum_tag_single_payload: StoreEnumTagSinglePayloadFn,
    pub size: usize,
    pub stride: usize,
    pub flags: ValueWitnessFlags,
    pub extra_inhabitant_count: u32,
}

/// Enum-specific value witness table (extends ValueWitnessTable).
#[repr(C)]
pub struct EnumValueWitnessTable {
    pub base: ValueWitnessTable,
    pub get_enum_tag: GetEnumTagFn,
    pub destructive_project_enum_data: DestructiveProjectEnumDataFn,
    pub destructive_inject_enum_tag: DestructiveInjectEnumTagFn,
}

/// Value witness flags packed into a usize.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ValueWitnessFlags(pub usize);

impl ValueWitnessFlags {
    const ALIGNMENT_MASK: usize = 0xFF;
    const IS_NON_POD: usize = 0x10000;
    const IS_NON_INLINE: usize = 0x20000;
    const HAS_SPARE_BITS: usize = 0x80000;
    const IS_NON_BITWISE_TAKABLE: usize = 0x100000;
    const HAS_ENUM_WITNESSES: usize = 0x200000;
    const INCOMPLETE: usize = 0x400000;

    /// The required alignment of the first byte of an object of this type,
    /// expressed as a mask of the low bits that must not be set in the pointer.
    /// (alignment - 1)
    pub fn alignment_mask(&self) -> usize {
        self.0 & Self::ALIGNMENT_MASK
    }

    /// The required alignment in bytes.
    pub fn alignment(&self) -> usize {
        self.alignment_mask() + 1
    }

    /// True if the type is not plain old data (has non-trivial copy/destroy).
    pub fn is_non_pod(&self) -> bool {
        self.0 & Self::IS_NON_POD != 0
    }

    /// True if the type does not fit in 3 pointer-sized words inline.
    pub fn is_non_inline(&self) -> bool {
        self.0 & Self::IS_NON_INLINE != 0
    }

    /// True if values of this type have spare bits.
    pub fn has_spare_bits(&self) -> bool {
        self.0 & Self::HAS_SPARE_BITS != 0
    }

    /// True if the type is not bitwise-takable.
    pub fn is_non_bitwise_takable(&self) -> bool {
        self.0 & Self::IS_NON_BITWISE_TAKABLE != 0
    }

    /// True if this is an enum type with enum witnesses.
    pub fn has_enum_witnesses(&self) -> bool {
        self.0 & Self::HAS_ENUM_WITNESSES != 0
    }

    /// True if the type layout is incomplete.
    pub fn is_incomplete(&self) -> bool {
        self.0 & Self::INCOMPLETE != 0
    }

    /// True if the type is POD (trivially copyable and destroyable).
    pub fn is_pod(&self) -> bool {
        !self.is_non_pod()
    }

    /// True if values fit inline in an existential container.
    pub fn is_inline(&self) -> bool {
        !self.is_non_inline()
    }

    /// True if the type is bitwise-takable.
    pub fn is_bitwise_takable(&self) -> bool {
        !self.is_non_bitwise_takable()
    }
}

impl ValueWitnessTable {
    /// Get size of a value of this type.
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Get stride of a value of this type.
    pub fn get_stride(&self) -> usize {
        self.stride
    }

    /// Get alignment of a value of this type (in bytes).
    pub fn get_alignment(&self) -> usize {
        self.flags.alignment()
    }

    /// Check if the type is POD.
    pub fn is_pod(&self) -> bool {
        self.flags.is_pod()
    }

    /// Check if values fit inline in an existential container.
    pub fn is_inline(&self) -> bool {
        self.flags.is_inline()
    }

    /// Check if this VWT has enum witnesses.
    pub fn has_enum_witnesses(&self) -> bool {
        self.flags.has_enum_witnesses()
    }

    /// Get the number of extra inhabitants.
    pub fn get_extra_inhabitant_count(&self) -> u32 {
        self.extra_inhabitant_count
    }

    /// Destroy a value using this witness table.
    ///
    /// # Safety
    /// `value` must point to a valid, initialized value of this type.
    /// `metadata` must be the metadata this VWT belongs to.
    pub unsafe fn destroy_value(&self, value: *mut c_void, metadata: *const c_void) {
        (self.destroy)(value, metadata);
    }

    /// Copy a value using this witness table.
    ///
    /// # Safety
    /// `dest` must point to uninitialized memory of sufficient size.
    /// `src` must point to a valid, initialized value.
    pub unsafe fn copy_value(
        &self,
        dest: *mut c_void,
        src: *mut c_void,
        metadata: *const c_void,
    ) -> *mut c_void {
        (self.initialize_with_copy)(dest, src, metadata)
    }

    /// Move a value using this witness table (take semantics).
    ///
    /// # Safety
    /// `dest` must point to uninitialized memory. `src` will be left uninitialized.
    pub unsafe fn move_value(
        &self,
        dest: *mut c_void,
        src: *mut c_void,
        metadata: *const c_void,
    ) -> *mut c_void {
        (self.initialize_with_take)(dest, src, metadata)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Type Metadata Layouts
// ═══════════════════════════════════════════════════════════════════════════

/// The full metadata header layout for value types.
/// In memory: `VWT_pointer | Kind`
/// The metadata pointer points at the Kind field.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FullTypeMetadata {
    /// Value witness table pointer (at offset -1 from metadata pointer).
    pub vwt: *const ValueWitnessTable,
    /// The metadata kind (at offset 0 = the metadata pointer itself).
    pub kind: usize,
}

/// Struct metadata layout.
/// `VWT_ptr | Kind | TypeDescriptor | FieldOffsets...`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StructMetadata {
    pub kind: usize,
    pub descriptor: *const c_void,
}

/// Enum metadata layout.
/// `VWT_ptr | Kind | TypeDescriptor`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EnumMetadata {
    pub kind: usize,
    pub descriptor: *const c_void,
}

/// Tuple metadata element.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TupleElement {
    pub metadata: *const c_void,
    pub offset: usize,
}

/// Tuple metadata layout.
/// `VWT_ptr | Kind | NumElements | Labels | Elements...`
#[repr(C)]
#[derive(Debug)]
pub struct TupleMetadata {
    pub kind: usize,
    pub num_elements: usize,
    pub labels: *const u8,
    // Followed by `num_elements` TupleElement entries
}

impl TupleMetadata {
    /// Get the elements slice.
    ///
    /// # Safety
    /// The metadata must be valid and have `num_elements` elements after it.
    pub unsafe fn elements(&self) -> &[TupleElement] {
        let base = (self as *const TupleMetadata).add(1) as *const TupleElement;
        core::slice::from_raw_parts(base, self.num_elements)
    }
}

/// Function metadata layout.
/// `VWT_ptr | Kind | Flags | ResultType | Parameters...`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FunctionMetadata {
    pub kind: usize,
    pub flags: usize,
    pub result_type: *const c_void,
    // Followed by parameter types
}

/// Existential container layout (for protocol types like `any P`).
/// For inline storage: [value buffer (3 words)] [type metadata] [witness tables...]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExistentialContainer {
    pub buffer: [usize; 3],
    pub metadata: *const c_void,
    // Followed by witness table pointers
}

/// Opaque existential container (with N witness tables).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OpaqueExistentialContainer {
    pub buffer: [usize; 3],
    pub metadata: *const c_void,
}

/// Class existential container (for `any AnyObject & P`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClassExistentialContainer {
    pub object: *mut c_void,
    // Followed by witness table pointers
}

// ═══════════════════════════════════════════════════════════════════════════
// Dynamic Cast Flags (complete)
// ═══════════════════════════════════════════════════════════════════════════

/// Flags for `swift_dynamicCast`. Bitwise-OR these together.
pub mod DynamicCastFlags {
    pub const DEFAULT: usize = 0x0;
    pub const UNCONDITIONAL: usize = 0x1;
    pub const TAKE_ON_SUCCESS: usize = 0x2;
    pub const DESTROY_ON_FAILURE: usize = 0x4;
    pub const PROHIBIT_ISOLATED_CONFORMANCES: usize = 0x8;
}

// ═══════════════════════════════════════════════════════════════════════════
// Existential Type Flags
// ═══════════════════════════════════════════════════════════════════════════

/// Class constraint for existential types.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistentialClassConstraint {
    /// The existential type is class-constrained (reference type only).
    Class = 0,
    /// The existential type has no class constraint (any value type or reference type).
    Any = 1,
}

// ═══════════════════════════════════════════════════════════════════════════
// Context Descriptor Kind
// ═══════════════════════════════════════════════════════════════════════════

/// The kind of a context descriptor.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextDescriptorKind {
    Module = 0,
    Extension = 1,
    Anonymous = 2,
    Protocol = 3,
    OpaqueType = 4,
    Class = 16,
    Struct = 17,
    Enum = 18,
}

/// Flags for a context descriptor, packed into a u32.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ContextDescriptorFlags(pub u32);

impl ContextDescriptorFlags {
    /// Get the kind of context descriptor.
    pub fn kind(&self) -> ContextDescriptorKind {
        let raw = (self.0 & 0x1F) as u8;
        unsafe { core::mem::transmute(raw) }
    }

    /// Get the raw kind value (safe for unknown kinds).
    pub fn kind_raw(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    /// Whether this context is generic.
    pub fn is_generic(&self) -> bool {
        (self.0 >> 7) & 1 != 0
    }

    /// Whether this context descriptor is unique (not shared across images).
    pub fn is_unique(&self) -> bool {
        (self.0 >> 6) & 1 != 0
    }

    /// Get the kind-specific flags (upper 16 bits).
    pub fn kind_specific_flags(&self) -> u16 {
        (self.0 >> 16) as u16
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Relative Pointers
// ═══════════════════════════════════════════════════════════════════════════

/// A relative pointer: a 32-bit offset from the field's own address.
/// This is how Swift ABI stores pointers in descriptors to be PIC.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct RelativePointer(pub i32);

impl RelativePointer {
    /// Resolve the relative pointer to an absolute address.
    ///
    /// # Safety
    /// The resolved address must point to valid data.
    pub unsafe fn resolve(&self) -> *const c_void {
        let self_addr = self as *const Self as *const u8;
        self_addr.offset(self.0 as isize) as *const c_void
    }

    /// Check if this relative pointer is null (offset == 0 means null).
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

/// A relative pointer that may be null.
/// Null is represented by an offset of 0.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct NullableRelativePointer(pub i32);

impl NullableRelativePointer {
    /// Resolve the relative pointer, returning None if null.
    ///
    /// # Safety
    /// If non-null, the resolved address must point to valid data.
    pub unsafe fn resolve(&self) -> Option<*const c_void> {
        if self.0 == 0 {
            None
        } else {
            let self_addr = self as *const Self as *const u8;
            Some(self_addr.offset(self.0 as isize) as *const c_void)
        }
    }
}

/// A relative pointer with the low bit used as a flag (indirect pointer).
/// If the low bit is set, the pointer resolves to a pointer-to-pointer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct RelativeIndirectablePointer(pub i32);

impl RelativeIndirectablePointer {
    /// Whether this is an indirect pointer.
    pub fn is_indirect(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Resolve the relative pointer, following indirection if needed.
    ///
    /// # Safety
    /// The resolved address must point to valid data.
    pub unsafe fn resolve(&self) -> *const c_void {
        let offset = self.0 & !1; // clear low bit
        let self_addr = self as *const Self as *const u8;
        let target = self_addr.offset(offset as isize) as *const c_void;
        if self.is_indirect() {
            *(target as *const *const c_void)
        } else {
            target
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Context Descriptor (base)
// ═══════════════════════════════════════════════════════════════════════════

/// The base layout of all context descriptors.
/// Every type descriptor, module descriptor, etc. starts with this.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ContextDescriptor {
    /// Flags describing the context.
    pub flags: ContextDescriptorFlags,
    /// Relative pointer to the parent context, or null if top-level.
    pub parent: RelativeIndirectablePointer,
}

impl ContextDescriptor {
    pub fn kind(&self) -> ContextDescriptorKind {
        self.flags.kind()
    }

    pub fn is_generic(&self) -> bool {
        self.flags.is_generic()
    }

    /// Resolve the parent descriptor.
    ///
    /// # Safety
    /// The parent pointer must be valid if non-null.
    pub unsafe fn parent(&self) -> Option<*const ContextDescriptor> {
        if self.parent.0 == 0 {
            None
        } else {
            Some(self.parent.resolve() as *const ContextDescriptor)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Type Context Descriptor (extends ContextDescriptor)
// ═══════════════════════════════════════════════════════════════════════════

/// Layout shared by all type (Class/Struct/Enum) context descriptors.
///
/// In memory:
///   ContextDescriptor fields    4 bytes
///   ContextDescriptor fields   4 bytes (relative)
///   Name —                        4 bytes (relative pointer to C string)
///   AccessFunction —              4 bytes (relative pointer or null)
///   Fields —                      4 bytes (relative pointer to FieldDescriptor or null)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TypeContextDescriptor {
    /// Base context descriptor fields.
    pub base: ContextDescriptor,
    /// Relative pointer to the null-terminated type name string.
    pub name: RelativePointer,
    /// Relative pointer to the metadata access function (or null).
    pub access_function: NullableRelativePointer,
    /// Relative pointer to the field descriptor (or null).
    pub fields: NullableRelativePointer,
}

impl TypeContextDescriptor {
    /// Get the type name as a C string.
    ///
    /// # Safety
    /// The descriptor must be valid.
    pub unsafe fn get_name(&self) -> &core::ffi::CStr {
        let ptr = self.name.resolve() as *const core::ffi::c_char;
        core::ffi::CStr::from_ptr(ptr)
    }

    /// Get the field descriptor, if present.
    ///
    /// # Safety
    /// The descriptor must be valid.
    pub unsafe fn get_fields(&self) -> Option<*const FieldDescriptor> {
        self.fields.resolve().map(|p| p as *const FieldDescriptor)
    }
}

/// Struct-specific descriptor (extends TypeContextDescriptor).
///
/// After the base TypeContextDescriptor fields:
///   NumFields —          4 bytes
///   FieldOffsetVectorOffset —  4 bytes (relative pointer)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StructDescriptor {
    pub base: TypeContextDescriptor,
    /// Number of stored properties.
    pub num_fields: u32,
    /// Offset in the metadata to the vector of field offsets.
    pub field_offset_vector_offset: u32,
}

/// Class-specific descriptor (extends TypeContextDescriptor).
///
/// After the base TypeContextDescriptor fields:
///   SuperclassType —                  4 bytes (relative pointer or null)
///   MetadataNegativeSizeInWords —     4 bytes (union with ResilientMetadataBounds)
///   MetadataPositiveSizeInWords —     4 bytes (union with ExtraClassFlags)
///   NumImmediateMembers —             4 bytes
///   NumFields —                       4 bytes
///   FieldOffsetVectorOffset —         4 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClassDescriptor {
    pub base: TypeContextDescriptor,
    /// Relative pointer to the superclass type (mangled name, or null).
    pub superclass_type: NullableRelativePointer,
    /// Negative size of metadata objects of this class (in words).
    pub metadata_negative_size_in_words: u32,
    /// Positive size of metadata objects of this class (in words).
    pub metadata_positive_size_in_words: u32,
    /// Number of additional members added by this class to the class metadata.
    pub num_immediate_members: u32,
    /// Number of stored properties.
    pub num_fields: u32,
    /// Offset in the metadata to the vector of field offsets
    /// (0 if the class has no fields).
    pub field_offset_vector_offset: u32,
}

/// Enum-specific descriptor (extends TypeContextDescriptor).
///
/// After the base TypeContextDescriptor fields:
///   NumPayloadCasesAndPayloadSizeOffset —   4 bytes
///   NumEmptyCases —                         4 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EnumDescriptor {
    pub base: TypeContextDescriptor,
    /// Upper 24 bits: number of payload cases.
    /// Lower 8 bits: payload size offset in the metadata (in words).
    pub num_payload_cases_and_payload_size_offset: u32,
    /// Number of empty (no associated value) cases.
    pub num_empty_cases: u32,
}

impl EnumDescriptor {
    /// Get the number of payload cases.
    pub fn num_payload_cases(&self) -> u32 {
        self.num_payload_cases_and_payload_size_offset >> 8
    }

    /// Get the total number of cases.
    pub fn num_cases(&self) -> u32 {
        self.num_payload_cases() + self.num_empty_cases
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Protocol Descriptor
// ═══════════════════════════════════════════════════════════════════════════

/// Protocol context descriptor.
///
///   ContextDescriptor fields
///   Name —                         4 bytes (relative pointer to C string)
///   NumRequirements fields   4 bytes
///   NumRequirements fields              4 bytes
///   AssociatedTypeNames —          4 bytes (relative pointer or null)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProtocolDescriptor {
    pub base: ContextDescriptor,
    /// Relative pointer to the protocol name.
    pub name: RelativePointer,
    /// Number of generic requirements in the requirement signature.
    pub num_requirements_in_signature: u32,
    /// Number of protocol requirements.
    pub num_requirements: u32,
    /// Relative pointer to associated type names (space-separated, or null).
    pub associated_type_names: NullableRelativePointer,
}

impl ProtocolDescriptor {
    /// # Safety
    /// The descriptor must be valid.
    pub unsafe fn get_name(&self) -> &core::ffi::CStr {
        let ptr = self.name.resolve() as *const core::ffi::c_char;
        core::ffi::CStr::from_ptr(ptr)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Protocol Conformance Descriptor
// ═══════════════════════════════════════════════════════════════════════════

/// A protocol conformance descriptor.
///
///   Protocol —             4 bytes (relative indirectable pointer)
///   TypeRef —              4 bytes (relative indirectable pointer)
///   WitnessTablePattern —  4 bytes (relative pointer)
///   Flags —                4 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProtocolConformanceDescriptor {
    /// Relative pointer to the protocol descriptor.
    pub protocol_descriptor: RelativeIndirectablePointer,
    /// Relative pointer to the conforming type reference.
    pub type_ref: RelativeIndirectablePointer,
    /// Relative pointer to the witness table pattern.
    pub witness_table_pattern: RelativePointer,
    /// Conformance flags.
    pub flags: u32,
}

// ═══════════════════════════════════════════════════════════════════════════
// Generic Context
// ═══════════════════════════════════════════════════════════════════════════

/// Generic context header, present when ContextDescriptorFlags::is_generic().
/// Follows immediately after the specific descriptor (Struct/Class/Enum).
///
///   NumParams —                2 bytes
///   NumRequirements fields          2 bytes
///   NumKeyArguments —          2 bytes
///   NumExtraArguments —        2 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GenericContextDescriptorHeader {
    /// Number of generic parameters.
    pub num_params: u16,
    /// Number of generic requirements.
    pub num_requirements: u16,
    /// Number of key generic arguments.
    pub num_key_arguments: u16,
    /// Number of extra generic arguments.
    pub num_extra_arguments: u16,
}

impl GenericContextDescriptorHeader {
    /// Total number of generic arguments in the metadata.
    pub fn num_generic_arguments(&self) -> u32 {
        self.num_key_arguments as u32 + self.num_extra_arguments as u32
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Field Descriptor & Field Record
// ═══════════════════════════════════════════════════════════════════════════

/// The kind of field descriptor.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldDescriptorKind {
    Struct = 0,
    Class = 1,
    Enum = 2,
    MultiPayloadEnum = 3,
    Protocol = 4,
    ClassProtocol = 5,
    ObjCProtocol = 6,
    ObjCClass = 7,
}

/// A field descriptor: describes all the fields/cases of a type.
///
///   MangledTypeName —    4 bytes (relative pointer)
///   Superclass —         4 bytes (relative pointer or null)
///   Kind —               2 bytes
///   FieldRecordSize —    2 bytes
///   NumFields —          4 bytes
///   FieldRecords —    NumFields × FieldRecordSize bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FieldDescriptor {
    /// Relative pointer to the mangled type name.
    pub mangled_type_name: RelativePointer,
    /// Relative pointer to the superclass name (or null).
    pub superclass: NullableRelativePointer,
    /// Kind of field descriptor.
    pub kind: FieldDescriptorKind,
    /// Size of each field record in bytes.
    pub field_record_size: u16,
    /// Number of fields/cases.
    pub num_fields: u32,
}

impl FieldDescriptor {
    /// Get the field records as a slice.
    ///
    /// # Safety
    /// The descriptor must be valid with `num_fields` records following it.
    pub unsafe fn fields(&self) -> &[FieldRecord] {
        let base = (self as *const FieldDescriptor).add(1) as *const FieldRecord;
        core::slice::from_raw_parts(base, self.num_fields as usize)
    }
}

/// A single field record (within a FieldDescriptor).
///
///   Flags —              4 bytes
///   MangledTypeName —    4 bytes (relative pointer)
///   FieldName —          4 bytes (relative pointer)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FieldRecord {
    /// Field flags.
    pub flags: FieldRecordFlags,
    /// Relative pointer to the mangled field type name.
    pub mangled_type_name: NullableRelativePointer,
    /// Relative pointer to the field name C string.
    pub field_name: RelativePointer,
}

impl FieldRecord {
    /// Get the field name.
    ///
    /// # Safety
    /// The record must be valid.
    pub unsafe fn get_name(&self) -> &core::ffi::CStr {
        let ptr = self.field_name.resolve() as *const core::ffi::c_char;
        core::ffi::CStr::from_ptr(ptr)
    }

    /// Get the mangled type name of the field (if present).
    ///
    /// # Safety
    /// The record must be valid.
    pub unsafe fn get_mangled_type_name(&self) -> Option<*const u8> {
        self.mangled_type_name.resolve().map(|p| p as *const u8)
    }
}

/// Field record flags.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct FieldRecordFlags(pub u32);

impl FieldRecordFlags {
    /// Whether this is an indirect enum case.
    pub fn is_indirect_case(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Whether this field is a `var` (as opposed to `let`).
    pub fn is_var(&self) -> bool {
        self.0 & 2 != 0
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Class Metadata (full layout)
// ═══════════════════════════════════════════════════════════════════════════

/// Full class metadata layout on 64-bit with ObjC interop (macOS arm64).
///
///   offset -8:  VWT pointer (via FullTypeMetadata header)
///   offset  0:  Kind / isa pointer (to metaclass)
///   offset  8:  Superclass metadata pointer
///   offset 16:  Cache data (ObjC, 2 words)
///   offset 32:  Data / rodata (ObjC)
///   offset 40:  ClassFlags
///   offset 44:  InstanceAddressPoint
///   offset 48:  InstanceSize
///   offset 52:  InstanceAlignMask
///   offset 54:  Reserved
///   offset 56:  ClassSize
///   offset 60:  ClassAddressPoint
///   offset 64:  Descriptor (pointer, signed)
///   offset 72:  IVarDestroyer (pointer)
///   ... followed by vtable entries
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClassMetadata {
    // TargetAnyClassMetadataObjCInterop part:
    /// isa pointer (to metaclass, or MetadataKind for non-ObjC classes).
    pub kind: usize,
    /// Superclass metadata pointer (null for root class).
    pub superclass: *const c_void,
    /// ObjC cache data (2 words, opaque).
    pub cache_data: [*const c_void; 2],
    /// ObjC rodata pointer.
    pub data: usize,

    // TargetClassMetadata specific fields:
    /// Swift-specific class flags.
    pub class_flags: u32,
    /// Address point of instances.
    pub instance_address_point: u32,
    /// Required size of instances.
    pub instance_size: u32,
    /// Alignment mask for instances.
    pub instance_align_mask: u16,
    /// Reserved.
    pub reserved: u16,
    /// Total class object size.
    pub class_size: u32,
    /// Address point within the class object.
    pub class_address_point: u32,
    /// Pointer to the class descriptor.
    pub descriptor: *const ClassDescriptor,
    /// Pointer to the ivar destroyer function (or null).
    pub ivar_destroyer: *const c_void,
    // Followed by: parent metadata ref, generic args, vtable entries
}

impl ClassMetadata {
    /// Check if this is valid Swift type metadata (not a pure ObjC class).
    pub fn is_type_metadata(&self) -> bool {
        // Swift type metadata has a non-null descriptor.
        // Pure ObjC classes have no descriptor.
        !self.descriptor.is_null()
    }

    /// Get the class descriptor.
    pub fn get_descriptor(&self) -> Option<&ClassDescriptor> {
        if self.descriptor.is_null() {
            None
        } else {
            Some(unsafe { &*self.descriptor })
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Existential Type Metadata (full layout)
// ═══════════════════════════════════════════════════════════════════════════

/// Flags for an existential type.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ExistentialTypeFlags(pub u32);

impl ExistentialTypeFlags {
    /// Number of witness tables.
    pub fn num_witness_tables(&self) -> u32 {
        self.0 & 0x00FFFFFF
    }

    /// Class constraint.
    pub fn class_constraint(&self) -> ExistentialClassConstraint {
        if (self.0 >> 31) & 1 != 0 {
            ExistentialClassConstraint::Any
        } else {
            ExistentialClassConstraint::Class
        }
    }

    /// Whether there is a superclass constraint.
    pub fn has_superclass_constraint(&self) -> bool {
        (self.0 >> 30) & 1 != 0
    }

    /// Whether this is a special protocol (Error, etc.).
    pub fn special_protocol_raw(&self) -> u32 {
        (self.0 >> 24) & 0x3F
    }
}

/// Existential type metadata layout.
///
///   offset 0: Kind (MetadataKind::Existential = 0x303)
///   offset 8: Flags (ExistentialTypeFlags)
///   offset 12: NumProtocols
///   ... followed by optional superclass pointer, then protocol descriptor refs
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExistentialTypeMetadata {
    /// Metadata kind.
    pub kind: usize,
    /// Existential type flags.
    pub flags: ExistentialTypeFlags,
    /// Number of protocol constraints.
    pub num_protocols: u32,
    // Followed by:
    //   - If has_superclass_constraint: 1 pointer to superclass metadata
    //   - NumProtocols protocol descriptor references
}

// ═══════════════════════════════════════════════════════════════════════════
// Function Type Flags
// ═══════════════════════════════════════════════════════════════════════════

/// Flags for function type metadata, packed into a usize.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct FunctionTypeFlags(pub usize);

impl FunctionTypeFlags {
    const NUM_PARAMETERS_MASK: usize = 0x0000FFFF;
    const CONVENTION_MASK: usize = 0x00FF0000;
    const CONVENTION_SHIFT: usize = 16;
    const THROWS: usize = 0x01000000;
    const PARAM_FLAGS: usize = 0x02000000;
    const ESCAPING: usize = 0x04000000;
    const DIFFERENTIABLE: usize = 0x08000000;
    const GLOBAL_ACTOR: usize = 0x10000000;
    const ASYNC: usize = 0x20000000;
    const SENDABLE: usize = 0x40000000;
    const EXTENDED_FLAGS: usize = 0x80000000;

    /// Number of parameters.
    pub fn num_parameters(&self) -> usize {
        self.0 & Self::NUM_PARAMETERS_MASK
    }

    /// Function calling convention (0=swift, 1=block, 2=thin, 3=cfunc).
    pub fn convention(&self) -> u8 {
        ((self.0 & Self::CONVENTION_MASK) >> Self::CONVENTION_SHIFT) as u8
    }

    /// Whether the function throws.
    pub fn is_throws(&self) -> bool {
        self.0 & Self::THROWS != 0
    }

    /// Whether parameter flags are present.
    pub fn has_parameter_flags(&self) -> bool {
        self.0 & Self::PARAM_FLAGS != 0
    }

    /// Whether the function is escaping.
    pub fn is_escaping(&self) -> bool {
        self.0 & Self::ESCAPING != 0
    }

    /// Whether the function is differentiable.
    pub fn is_differentiable(&self) -> bool {
        self.0 & Self::DIFFERENTIABLE != 0
    }

    /// Whether the function has a global actor.
    pub fn has_global_actor(&self) -> bool {
        self.0 & Self::GLOBAL_ACTOR != 0
    }

    /// Whether the function is async.
    pub fn is_async(&self) -> bool {
        self.0 & Self::ASYNC != 0
    }

    /// Whether the function is Sendable.
    pub fn is_sendable(&self) -> bool {
        self.0 & Self::SENDABLE != 0
    }

    /// Whether extended flags are present.
    pub fn has_extended_flags(&self) -> bool {
        self.0 & Self::EXTENDED_FLAGS != 0
    }
}

/// Function calling convention.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionConvention {
    Swift = 0,
    Block = 1,
    Thin = 2,
    CFunctionPointer = 3,
}

// ═══════════════════════════════════════════════════════════════════════════
// §38: VTable Dispatch
// ═══════════════════════════════════════════════════════════════════════════

/// VTable descriptor header — found after ClassDescriptor + generic context.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VTableDescriptorHeader {
    /// Offset of the vtable in the class metadata, in words.
    pub vtable_offset: u32,
    /// Number of vtable entries.
    pub vtable_size: u32,
}

/// Method descriptor flags.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct MethodDescriptorFlags(pub u32);

impl MethodDescriptorFlags {
    pub fn kind(&self) -> MethodDescriptorKind {
        match self.0 & 0x0F {
            0 => MethodDescriptorKind::Method,
            1 => MethodDescriptorKind::Init,
            2 => MethodDescriptorKind::Getter,
            3 => MethodDescriptorKind::Setter,
            4 => MethodDescriptorKind::ModifyCoroutine,
            5 => MethodDescriptorKind::ReadCoroutine,
            _ => MethodDescriptorKind::Method,
        }
    }
    pub fn is_instance(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn is_dynamic(&self) -> bool {
        self.0 & 0x20 != 0
    }
    pub fn is_async(&self) -> bool {
        self.0 & 0x40 != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodDescriptorKind {
    Method,
    Init,
    Getter,
    Setter,
    ModifyCoroutine,
    ReadCoroutine,
}

/// A method descriptor — describes a single vtable entry.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MethodDescriptor {
    pub flags: MethodDescriptorFlags,
    /// Relative pointer to the method implementation.
    pub impl_ptr: RelativePointer,
}

/// Override table header — follows the vtable entries in the descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OverrideTableHeader {
    pub num_entries: u32,
}

/// A single override table entry.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MethodOverrideDescriptor {
    /// Relative pointer to the class being overridden.
    pub override_class: RelativeIndirectablePointer,
    /// Relative pointer to the method being overridden.
    pub override_method: RelativePointer,
    /// Relative pointer to the overriding implementation.
    pub impl_ptr: RelativePointer,
}

/// Read a method function pointer from a class vtable.
///
/// # Safety
/// `metadata` must be valid ClassMetadata. `vtable_offset` (in words from
/// the metadata pointer) and `method_index` must be within bounds.
pub unsafe fn read_vtable_entry(
    metadata: *const c_void,
    vtable_offset_words: u32,
    method_index: u32,
) -> *const c_void {
    let base = metadata as *const *const c_void;
    let entry = base.add((vtable_offset_words + method_index) as usize);
    *entry
}

// ═══════════════════════════════════════════════════════════════════════════
// §39: Witness Table Dispatch
// ═══════════════════════════════════════════════════════════════════════════

/// Protocol requirement flags.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ProtocolRequirementFlags(pub u32);

impl ProtocolRequirementFlags {
    pub fn kind(&self) -> ProtocolRequirementKind {
        match self.0 & 0x0F {
            0 => ProtocolRequirementKind::BaseProtocol,
            1 => ProtocolRequirementKind::Method,
            2 => ProtocolRequirementKind::Init,
            3 => ProtocolRequirementKind::Getter,
            4 => ProtocolRequirementKind::Setter,
            5 => ProtocolRequirementKind::ReadCoroutine,
            6 => ProtocolRequirementKind::ModifyCoroutine,
            7 => ProtocolRequirementKind::AssociatedTypeAccessFunction,
            8 => ProtocolRequirementKind::AssociatedConformanceAccessFunction,
            _ => ProtocolRequirementKind::Method,
        }
    }
    pub fn is_instance(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn is_async(&self) -> bool {
        self.0 & 0x40 != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolRequirementKind {
    BaseProtocol,
    Method,
    Init,
    Getter,
    Setter,
    ReadCoroutine,
    ModifyCoroutine,
    AssociatedTypeAccessFunction,
    AssociatedConformanceAccessFunction,
}

/// A protocol requirement descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProtocolRequirement {
    pub flags: ProtocolRequirementFlags,
    /// Relative pointer to the default implementation (or null).
    pub default_impl: NullableRelativePointer,
}

/// Read a function pointer from a witness table at a given offset.
///
/// # Safety
/// `witness_table` must point to a valid witness table.
/// `requirement_index` must be in bounds (starting after base conformance entries).
pub unsafe fn read_witness_method(
    witness_table: *const c_void,
    requirement_index: usize,
) -> *const c_void {
    // Witness table layout: entry 0 = = protocol conformance descriptor,
    // then [1..] = witness entries (function pointers or associated types).
    let base = witness_table as *const *const c_void;
    *base.add(1 + requirement_index)
}

/// Call a protocol witness method with (self, metadata, witness_table).
///
/// # Safety
/// `func` must be a valid function pointer from the witness table.
/// On arm64, this uses the Swift CC where self goes in x20 for instance methods.
/// For simplicity, this version passes self in x0 (works for most C-bridged cases).
#[cfg(target_arch = "aarch64")]
pub unsafe fn call_witness_method_self_to_isize(
    func: *const c_void,
    self_val: *const c_void,
    metadata: *const c_void,
    witness_table: *const c_void,
) -> isize {
    let r: isize;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") self_val, in("x20") self_val, // pass self in both x0 and x20 for safety
        in("x1") metadata, // self metadata
        in("x2") witness_table,
        lateout("x0") r,
        lateout("x1") _, lateout("x3") _,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _,
        lateout("x8") _, lateout("x9") _, lateout("x10") _, lateout("x11") _,
        lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
        lateout("x16") _, lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn call_witness_method_self_to_isize(
    func: *const c_void,
    self_val: *const c_void,
    metadata: *const c_void,
    witness_table: *const c_void,
) -> isize {
    type F = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void) -> isize;
    (core::mem::transmute::<_, F>(func))(self_val, metadata, witness_table)
}

// ═══════════════════════════════════════════════════════════════════════════
// §40: Async Function Entry Points
// ═══════════════════════════════════════════════════════════════════════════

/// Async function pointer layout. This is what `swift_task_create` expects.
///
/// In Swift's ABI, an async function is represented as a pair:
/// - A function pointer (the entry point)
/// - The size of the initial async context required
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncFunctionPointer {
    /// Relative pointer to the async function entry point.
    pub function: RelativePointer,
    /// Expected initial context size.
    pub expected_context_size: u32,
}

/// The base async context layout.
///
///   offset 0: Parent context pointer (pointer to caller's async context)
///   offset 8: Resume function pointer (where to continue after await)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncContext {
    /// Parent async context (or null for top-level).
    pub parent: *mut AsyncContext,
    /// Resume function pointer.
    pub resume_fn: *const c_void,
}

/// Extended async context with an error slot.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThrowingAsyncContext {
    pub base: AsyncContext,
    /// Error slot — set by the callee if it throws.
    pub error: *mut c_void,
}

/// Future result context — async context + space for the result.
#[repr(C)]
pub struct FutureAsyncContext<T> {
    pub base: AsyncContext,
    /// Error result (null if no error).
    pub error: *mut c_void,
    /// The result value (valid only if error is null).
    pub result: core::mem::MaybeUninit<T>,
}
