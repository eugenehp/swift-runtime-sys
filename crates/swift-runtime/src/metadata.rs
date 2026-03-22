//! Safe metadata introspection.

use core::ffi::c_void;
use swift_runtime_sys::SwiftABI::*;

/// A safe wrapper around a Swift type metadata pointer.
#[derive(Clone, Copy)]
pub struct Metadata(*const c_void);

impl Metadata {
    /// Create from a raw pointer. Returns None if null.
    pub fn from_raw(ptr: *const c_void) -> Option<Self> {
        if ptr.is_null() { None } else { Some(Self(ptr)) }
    }

    /// Get the raw pointer.
    pub fn as_raw(&self) -> *const c_void {
        self.0
    }

    /// Get the metadata kind.
    pub fn kind(&self) -> MetadataKind {
        let raw = unsafe { *(self.0 as *const usize) };
        get_enumerated_metadata_kind(raw)
    }

    /// Get the value witness table.
    pub fn value_witness_table(&self) -> &ValueWitnessTable {
        unsafe { &*get_value_witness_table(self.0) }
    }

    /// Get the size of values of this type.
    pub fn size(&self) -> usize {
        self.value_witness_table().get_size()
    }

    /// Get the stride of values of this type.
    pub fn stride(&self) -> usize {
        self.value_witness_table().get_stride()
    }

    /// Get the alignment of values of this type.
    pub fn alignment(&self) -> usize {
        self.value_witness_table().get_alignment()
    }

    /// Check if this type is POD (trivially copyable/destroyable).
    pub fn is_pod(&self) -> bool {
        self.value_witness_table().is_pod()
    }

    /// Get the human-readable type name.
    pub fn type_name(&self, qualified: bool) -> Option<String> {
        let result = unsafe {
            swift_runtime_sys::SwiftCCThunks::swift_getTypeName(self.0, qualified)
        };
        match result {
            Ok((name, len)) if len > 0 => Some(name.to_string()),
            _ => None,
        }
    }

    /// Get the type context descriptor.
    pub fn descriptor(&self) -> Option<*const c_void> {
        let result = unsafe {
            swift_runtime_sys::SwiftCCThunks::swift_getTypeContextDescriptor(self.0)
        };
        match result {
            Ok(ptr) if !ptr.is_null() => Some(ptr),
            _ => None,
        }
    }

    /// Get the descriptor name (e.g., "Int", "String").
    pub fn descriptor_name(&self) -> Option<String> {
        let desc = self.descriptor()?;
        let tcd = unsafe { &*(desc as *const TypeContextDescriptor) };
        let name = unsafe { tcd.get_name() };
        name.to_str().ok().map(|s| s.to_string())
    }
}

impl std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.type_name(true).unwrap_or_else(|| format!("{:?}", self.0));
        write!(f, "Metadata({}, kind={:?}, size={})", name, self.kind(), self.size())
    }
}

/// Look up a type by mangled name.
pub fn lookup_type(mangled: &[u8]) -> Option<Metadata> {
    let ptr = unsafe {
        swift_runtime_sys::RuntimeRaw::swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            core::ptr::null(),
            0,
        )
    };
    Metadata::from_raw(ptr)
}
