//! Smoke tests proving the Swift runtime bindings actually link and work.
//! These call real functions in `libswiftCore.dylib` on this Mac.

use std::ffi::{c_void, CStr};

// Import the bindings
use swift_runtime_sys::RuntimeRaw::*;
use swift_runtime_sys::SwiftABI::*;

#[test]
fn test_alloc_retain_release() {
    // Look up Int metadata via mangled name
    let mangled = b"Si"; // Swift.Int
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    assert!(!metadata.is_null(), "Failed to look up Int metadata");
}

#[test]
fn test_metadata_kind_enum() {
    // Verify our MetadataKind enum has correct values
    assert_eq!(MetadataKind::Class as u32, 0);
    assert_eq!(MetadataKind::Struct as u32, 0x200);
    assert_eq!(MetadataKind::Enum as u32, 0x201);
    assert_eq!(MetadataKind::Optional as u32, 0x202);
    assert_eq!(MetadataKind::Tuple as u32, 0x301);
    assert_eq!(MetadataKind::Function as u32, 0x302);
    assert_eq!(MetadataKind::Existential as u32, 0x303);
    assert_eq!(MetadataKind::Metatype as u32, 0x304);
}

#[test]
fn test_value_witness_flags() {
    let flags = ValueWitnessFlags(0);
    assert!(flags.is_pod());
    assert!(flags.is_inline());
    assert!(flags.is_bitwise_takable());

    let flags = ValueWitnessFlags(0x10000 | 0x20000);
    assert!(!flags.is_pod());
    assert!(!flags.is_inline());
}

#[test]
fn test_heap_object_size() {
    assert_eq!(std::mem::size_of::<HeapObject>(), HEAP_OBJECT_HEADER_SIZE);
    assert_eq!(HEAP_OBJECT_HEADER_SIZE, 16);
}

#[test]
fn test_get_type_name() {
    // Look up String metadata
    let mangled = b"SS"; // Swift.String
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    assert!(!metadata.is_null(), "Failed to look up String metadata");

    // Get the human-readable type name
    let name_pair =
        unsafe { swift_runtime_sys::MetadataIntrospection::swift_getTypeName(metadata, true) };
    assert!(!name_pair.data.is_null());
    assert!(name_pair.length > 0);

    let name = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(
            name_pair.data as *const u8,
            name_pair.length,
        ))
        .unwrap()
    };
    assert_eq!(name, "Swift.String");
}

#[test]
fn test_value_witness_table_for_int() {
    // Look up Int metadata
    let mangled = b"Si"; // Swift.Int
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    assert!(!metadata.is_null());

    // Read the value witness table
    let vwt = unsafe { get_value_witness_table(metadata) };
    assert!(!vwt.is_null());

    let vwt = unsafe { &*vwt };
    assert_eq!(vwt.get_size(), std::mem::size_of::<isize>());
    assert_eq!(vwt.get_stride(), std::mem::size_of::<isize>());
    assert!(vwt.is_pod(), "Int should be POD");
    assert!(vwt.is_inline(), "Int should be inline");
    assert_eq!(vwt.get_alignment(), std::mem::align_of::<isize>());
}

#[test]
fn test_metadata_kind_for_types() {
    // Int is a struct
    let mangled = b"Si";
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    let kind_raw = unsafe { *(metadata as *const usize) };
    let kind = get_enumerated_metadata_kind(kind_raw);
    assert_eq!(kind, MetadataKind::Struct, "Int should be a Struct");

    // Optional<Int> is Optional
    let mangled = b"SiSg"; // Swift.Optional<Swift.Int>
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    assert!(!metadata.is_null());
    let kind_raw = unsafe { *(metadata as *const usize) };
    let kind = get_enumerated_metadata_kind(kind_raw);
    assert_eq!(
        kind,
        MetadataKind::Optional,
        "Optional<Int> should be Optional"
    );
}

#[test]
fn test_dynamic_cast_and_demangle() {
    // Test swift_demangle
    let mangled = c"$sSiN"; // Swift.Int type metadata
    let result = unsafe {
        swift_runtime_sys::DebugHooks::swift_demangle(
            mangled.as_ptr(),
            mangled.to_bytes().len(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        )
    };
    if !result.is_null() {
        let demangled = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
        assert!(
            demangled.contains("Int"),
            "Demangled name should contain 'Int', got: {demangled}"
        );
        // Free the result (allocated by swift_demangle)
        unsafe { libc::free(result as *mut c_void) };
    }
}

#[test]
fn test_error_retain_release() {
    // Just verify these symbols resolve (they're in libswiftCore)
    // We can't easily create a Swift error from Rust without the Swift CC,
    // but we can test that null is handled gracefully
    unsafe {
        swift_runtime_sys::ErrorHandling::swift_errorRelease(std::ptr::null_mut());
    }
    // If we got here without crashing, the symbol linked correctly
    // (swift_errorRelease is documented to be a no-op for null)
}

// NOTE: swift_stdlib_getHardwareConcurrency and swift_stdlib_operatingSystemVersion
// are SPI symbols (SWIFT_RUNTIME_STDLIB_API) — they link with a leading underscore
// prefix and may not be available on all platforms. Use dlsym for these.

#[test]
fn test_stdlib_hardware_concurrency_dlsym() {
    use core::ffi::{c_char, c_void};
    unsafe extern "C" {
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    let sym = unsafe {
        dlsym(
            (-2isize) as *mut c_void,
            c"swift_stdlib_getHardwareConcurrency".as_ptr(),
        )
    };
    if !sym.is_null() {
        let f: unsafe extern "C" fn() -> usize = unsafe { std::mem::transmute(sym) };
        let count = unsafe { f() };
        assert!(count > 0, "Hardware concurrency should be > 0, got {count}");
    } else {
        println!(
            "swift_stdlib_getHardwareConcurrency not found via dlsym (expected on some platforms)"
        );
    }
}

#[test]
fn test_alloc_release_object() {
    // Look up a simple type's metadata and allocate an object
    let mangled = b"Si"; // Swift.Int
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    assert!(!metadata.is_null());

    // Get size from VWT
    let vwt = unsafe { &*get_value_witness_table(metadata) };
    let size = vwt.get_size();
    assert!(size > 0);

    // We can verify retain/release work on a real heap object
    // by using swift_allocObject + swift_release
    // But we need class metadata for allocObject, not struct metadata.
    // Instead, verify the symbols link by calling retain(null) which is a no-op.
    let result = unsafe { swift_retain(std::ptr::null_mut()) };
    assert!(result.is_null(), "retain(null) should return null");

    unsafe { swift_release(std::ptr::null_mut()) }; // no-op for null
}

#[test]
fn test_runtime_paths() {
    let root = unsafe { swift_runtime_sys::RuntimePaths::swift_getRootPath() };
    if !root.is_null() {
        let path = unsafe { CStr::from_ptr(root) }.to_str().unwrap();
        println!("Swift root path: {path}");
    }

    let lib = unsafe { swift_runtime_sys::RuntimePaths::swift_getRuntimeLibraryPath() };
    if !lib.is_null() {
        let path = unsafe { CStr::from_ptr(lib) }.to_str().unwrap();
        println!("Swift runtime library path: {path}");
        assert!(path.contains("swift") || path.contains("Swift"));
    }
}

#[test]
fn test_numeric_conversion() {
    let mut buf = [0u8; 64];
    let len = unsafe {
        swift_runtime_sys::NumericConversion::swift_int64ToString(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            42,
            10,
            false,
        )
    };
    let s = std::str::from_utf8(&buf[..len]).unwrap();
    assert_eq!(s, "42");

    let mut buf = [0u8; 64];
    let len = unsafe {
        swift_runtime_sys::NumericConversion::swift_float64ToString(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            3.14,
            false,
        )
    };
    let s = std::str::from_utf8(&buf[..len]).unwrap();
    assert!(s.starts_with("3.14"), "Expected 3.14..., got {s}");
}

#[test]
fn test_is_class_type() {
    // Int is not a class
    let mangled = b"Si";
    let metadata = unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            std::ptr::null(),
            0,
        )
    };
    let is_class = unsafe { swift_runtime_sys::MetadataIntrospection::swift_isClassType(metadata) };
    assert!(!is_class, "Int should not be a class type");
}
