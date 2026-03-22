#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Helpers for accessing Swift standard library types from Rust.
//!
//! Uses `swift_getTypeByMangledNameInEnvironment` with known mangled names
//! to resolve stdlib type metadata at runtime.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;

unsafe extern "C" {
    fn swift_getTypeByMangledNameInEnvironment(
        type_name: *const u8,
        type_name_length: usize,
        generic_args: *const *const c_void,
        generic_args_count: usize,
    ) -> MetadataRef;
}

/// Resolve a Swift type by its mangled name.
///
/// # Safety
/// The mangled name must be valid.
unsafe fn resolve_type(mangled: &[u8]) -> Option<MetadataRef> {
    let result = swift_getTypeByMangledNameInEnvironment(
        mangled.as_ptr(),
        mangled.len(),
        core::ptr::null(),
        0,
    );
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}

/// Resolve a generic type with one type argument.
///
/// # Safety
/// The mangled name must be valid and expect exactly one generic argument.
unsafe fn resolve_generic_1(mangled: &[u8], arg: MetadataRef) -> Option<MetadataRef> {
    let args = [arg];
    let result =
        swift_getTypeByMangledNameInEnvironment(mangled.as_ptr(), mangled.len(), args.as_ptr(), 1);
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}

/// Resolve a generic type with two type arguments.
///
/// # Safety
/// The mangled name must be valid and expect exactly two generic arguments.
unsafe fn resolve_generic_2(
    mangled: &[u8],
    arg0: MetadataRef,
    arg1: MetadataRef,
) -> Option<MetadataRef> {
    let args = [arg0, arg1];
    let result =
        swift_getTypeByMangledNameInEnvironment(mangled.as_ptr(), mangled.len(), args.as_ptr(), 2);
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Primitive type metadata
// ═══════════════════════════════════════════════════════════════════════════

/// Get metadata for `Swift.Int`.
pub fn int_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"Si") }
}

/// Get metadata for `Swift.UInt`.
pub fn uint_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"Su") }
}

/// Get metadata for `Swift.Int8`.
pub fn int8_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s4Int8V") }
}

/// Get metadata for `Swift.Int16`.
pub fn int16_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s5Int16V") }
}

/// Get metadata for `Swift.Int32`.
pub fn int32_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s5Int32V") }
}

/// Get metadata for `Swift.Int64`.
pub fn int64_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s5Int64V") }
}

/// Get metadata for `Swift.UInt8`.
pub fn uint8_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s5UInt8V") }
}

/// Get metadata for `Swift.UInt16`.
pub fn uint16_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s6UInt16V") }
}

/// Get metadata for `Swift.UInt32`.
pub fn uint32_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s6UInt32V") }
}

/// Get metadata for `Swift.UInt64`.
pub fn uint64_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"s6UInt64V") }
}

/// Get metadata for `Swift.Bool`.
pub fn bool_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"Sb") }
}

/// Get metadata for `Swift.Float`.
pub fn float_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"Sf") }
}

/// Get metadata for `Swift.Double`.
pub fn double_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"Sd") }
}

/// Get metadata for `Swift.String`.
pub fn string_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"SS") }
}

/// Get metadata for `Swift.Character`.
pub fn character_metadata() -> Option<MetadataRef> {
    unsafe { resolve_type(b"SJ") }
}

// ═══════════════════════════════════════════════════════════════════════════
// Generic type metadata
// ═══════════════════════════════════════════════════════════════════════════

/// Get metadata for `Swift.Optional<Element>`.
///
/// Uses the fully-specified mangled name with a symbolic reference byte (0x01)
/// pointing to the element metadata.
pub fn optional_metadata(element: MetadataRef) -> Option<MetadataRef> {
    // Optional<Int> = "SiSg" etc, but for arbitrary element we need
    // swift_getTypeByMangledNameInContext. Use a simpler approach: dlsym the accessor.
    use core::ffi::c_char;
    unsafe extern "C" {
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    let rtld = (-2isize) as *mut c_void;
    // Use swift_getGenericMetadata approach via metadata accessor
    let accessor = unsafe { dlsym(rtld, c"$sSqMa".as_ptr()) };
    if accessor.is_null() {
        return None;
    }
    // Metadata accessor signature: (MetadataRequest, Element) -> MetadataResponse
    // On arm64 this is Swift CC but for metadata accessors the pattern is:
    // (request: usize) -> (metadata, state) with generic args in subsequent registers
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let metadata: *const c_void;
        let _state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) accessor,
            in("x0") 0usize, // MetadataRequest::Complete
            in("x1") element,
            lateout("x0") metadata,
            lateout("x1") _state,
            lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
            lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _,
            lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _,
            lateout("x14") _, lateout("x15") _, lateout("x16") _, lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        if metadata.is_null() {
            None
        } else {
            Some(metadata)
        }
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        #[repr(C)]
        struct Resp(*const c_void, usize);
        type F = unsafe extern "C" fn(usize, MetadataRef) -> Resp;
        let f: F = core::mem::transmute(accessor);
        let resp = f(0, element);
        if resp.0.is_null() {
            None
        } else {
            Some(resp.0)
        }
    }
}

/// Get metadata for `Swift.Array<Element>`.
pub fn array_metadata(element: MetadataRef) -> Option<MetadataRef> {
    use core::ffi::c_char;
    unsafe extern "C" {
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    let accessor = unsafe { dlsym((-2isize) as *mut c_void, c"$sSaMa".as_ptr()) };
    if accessor.is_null() {
        return None;
    }
    call_metadata_accessor_1(accessor, element)
}

/// Get metadata for `Swift.Set<Element>`.
pub fn set_metadata(element: MetadataRef) -> Option<MetadataRef> {
    use core::ffi::c_char;
    unsafe extern "C" {
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    let accessor = unsafe { dlsym((-2isize) as *mut c_void, c"$sShMa".as_ptr()) };
    if accessor.is_null() {
        return None;
    }
    call_metadata_accessor_1(accessor, element)
}

/// Get metadata for `Swift.Dictionary<Key, Value>`.
pub fn dictionary_metadata(key: MetadataRef, value: MetadataRef) -> Option<MetadataRef> {
    use core::ffi::c_char;
    unsafe extern "C" {
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    let accessor = unsafe { dlsym((-2isize) as *mut c_void, c"$sSDMa".as_ptr()) };
    if accessor.is_null() {
        return None;
    }
    call_metadata_accessor_2(accessor, key, value)
}

fn call_metadata_accessor_1(accessor: *mut c_void, arg: MetadataRef) -> Option<MetadataRef> {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let metadata: *const c_void;
        let _state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) accessor,
            in("x0") 0usize,
            in("x1") arg,
            lateout("x0") metadata,
            lateout("x1") _state,
            lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
            lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _,
            lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _,
            lateout("x14") _, lateout("x15") _, lateout("x16") _, lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        if metadata.is_null() {
            None
        } else {
            Some(metadata)
        }
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        #[repr(C)]
        struct Resp(*const c_void, usize);
        type F = unsafe extern "C" fn(usize, MetadataRef) -> Resp;
        let f: F = core::mem::transmute(accessor);
        let resp = f(0, arg);
        if resp.0.is_null() {
            None
        } else {
            Some(resp.0)
        }
    }
}

fn call_metadata_accessor_2(
    accessor: *mut c_void,
    arg0: MetadataRef,
    arg1: MetadataRef,
) -> Option<MetadataRef> {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let metadata: *const c_void;
        let _state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) accessor,
            in("x0") 0usize,
            in("x1") arg0,
            in("x2") arg1,
            lateout("x0") metadata,
            lateout("x1") _state,
            lateout("x3") _, lateout("x4") _, lateout("x5") _,
            lateout("x6") _, lateout("x7") _, lateout("x8") _, lateout("x9") _,
            lateout("x10") _, lateout("x11") _, lateout("x12") _, lateout("x13") _,
            lateout("x14") _, lateout("x15") _, lateout("x16") _, lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        if metadata.is_null() {
            None
        } else {
            Some(metadata)
        }
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        #[repr(C)]
        struct Resp(*const c_void, usize);
        type F = unsafe extern "C" fn(usize, MetadataRef, MetadataRef) -> Resp;
        let f: F = core::mem::transmute(accessor);
        let resp = f(0, arg0, arg1);
        if resp.0.is_null() {
            None
        } else {
            Some(resp.0)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Type info helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Get the size, stride, and alignment of a Swift type.
pub fn type_layout(metadata: MetadataRef) -> Option<(usize, usize, usize)> {
    if metadata.is_null() {
        return None;
    }
    let vwt = unsafe { crate::SwiftABI::get_value_witness_table(metadata) };
    if vwt.is_null() {
        return None;
    }
    let vwt = unsafe { &*vwt };
    Some((vwt.get_size(), vwt.get_stride(), vwt.get_alignment()))
}

/// Check if a Swift type is POD (trivially copyable/destroyable).
pub fn is_pod(metadata: MetadataRef) -> Option<bool> {
    if metadata.is_null() {
        return None;
    }
    let vwt = unsafe { crate::SwiftABI::get_value_witness_table(metadata) };
    if vwt.is_null() {
        return None;
    }
    Some(unsafe { &*vwt }.is_pod())
}

/// Get the metadata kind of a type.
pub fn metadata_kind(metadata: MetadataRef) -> Option<crate::SwiftABI::MetadataKind> {
    if metadata.is_null() {
        return None;
    }
    let kind_raw = unsafe { *(metadata as *const usize) };
    Some(crate::SwiftABI::get_enumerated_metadata_kind(kind_raw))
}

// ═══════════════════════════════════════════════════════════════════════════
// Protocol descriptor access via mangled symbols
// ═══════════════════════════════════════════════════════════════════════════

use core::ffi::c_char;
unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}
const RTLD: *mut c_void = (-2isize) as *mut c_void;

/// Resolve a symbol from the Swift runtime.
fn sym(name: &core::ffi::CStr) -> Option<*const c_void> {
    let p = unsafe { dlsym(RTLD, name.as_ptr()) };
    if p.is_null() {
        None
    } else {
        Some(p as *const c_void)
    }
}

/// Get the `Swift.Error` protocol descriptor.
pub fn error_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$ss5ErrorMp")
}

/// Get the `Swift.Hashable` protocol descriptor (`SH`).
pub fn hashable_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$sSHMp")
}

/// Get the `Swift.Equatable` protocol descriptor (`SQ`).
pub fn equatable_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$sSQMp")
}

/// Get the `Swift.Comparable` protocol descriptor (`SL`).
pub fn comparable_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$sSLMp")
}

/// Get the `Swift.CodingKey` protocol descriptor.
pub fn coding_key_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$ss9CodingKeyMp")
}

/// Get the `Swift.Sendable` protocol descriptor.
pub fn sendable_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$ss8SendableMp")
}

/// Get the `Swift.Actor` protocol descriptor.
pub fn actor_protocol_descriptor() -> Option<*const c_void> {
    sym(c"$sScAMp")
}

/// Get the direct type metadata pointer for `Swift.Int`.
pub fn int_metadata_direct() -> Option<*const c_void> {
    sym(c"$sSiN")
}

/// Get the direct type metadata pointer for `Swift.Double`.
pub fn double_metadata_direct() -> Option<*const c_void> {
    sym(c"$sSdN")
}

/// Get the direct type metadata pointer for `Swift.Bool`.
pub fn bool_metadata_direct() -> Option<*const c_void> {
    sym(c"$sSbN")
}

/// Get the direct type metadata pointer for `Swift.String`.
pub fn string_metadata_direct() -> Option<*const c_void> {
    sym(c"$sSSN")
}

// ═══════════════════════════════════════════════════════════════════════════
// String construction / extraction helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Create a Swift String from a Rust `&str`.
///
/// Returns (string_buffer, metadata) where string_buffer is a 16-byte
/// inline Swift String representation that can be passed to Swift functions.
///
/// # Safety
/// The returned buffer is only valid while the input string data is valid
/// (for small strings the data is copied inline; for large strings it
/// references the original bytes).
pub fn create_swift_string(s: &str) -> Option<[u8; 16]> {
    // Swift small string representation on 64-bit little-endian:
    // If len <= 15, the string is stored inline in the 16-byte buffer.
    // Layout: bytes[0..len] = UTF-8 data, bytes[15] = (len << 4) | discriminator
    // The discriminator for small strings has the high nibble set.
    if s.len() <= 15 {
        let mut buf = [0u8; 16];
        buf[..s.len()].copy_from_slice(s.as_bytes());
        // Small string discriminator: length in high nibble of last byte
        // On 64-bit LE: byte 15 = (count << 4) | 0xe0 (small string tag)
        // Small string tag: high nibble = 0xe, low nibble = count
        buf[15] = 0xe0 | (s.len() as u8);
        Some(buf)
    } else {
        // Large strings require calling into the Swift runtime.
        // Use swift_stdlib_NSStringFromUTF8 or bridge through the runtime.
        // For now, only support small strings.
        None
    }
}

/// Extract the contents of a Swift small string as a Rust `&str`.
///
/// Returns None if the string is not a small string (i.e., heap-allocated).
pub fn extract_small_string(buf: &[u8; 16]) -> Option<&str> {
    let discriminator = buf[15];
    // Small string: bit 7 is set (isSmall flag)
    if discriminator & 0x80 == 0 {
        return None;
    }
    // Count is stored in the low 4 bits
    let count = (discriminator & 0x0F) as usize;
    if count > 15 {
        return None;
    }
    core::str::from_utf8(&buf[..count]).ok()
}
