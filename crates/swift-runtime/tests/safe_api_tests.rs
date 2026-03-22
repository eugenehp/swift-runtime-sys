use swift_runtime::metadata::{lookup_type, Metadata};
use swift_runtime::types;

#[test]
fn test_lookup_int() {
    let m = types::int().expect("Should resolve Int");
    assert_eq!(m.kind(), swift_runtime_sys::SwiftABI::MetadataKind::Struct);
    assert_eq!(m.size(), std::mem::size_of::<isize>());
    assert!(m.is_pod());
}

#[test]
fn test_lookup_string() {
    let m = types::string().expect("Should resolve String");
    assert_eq!(m.size(), 16);
    assert!(!m.is_pod());
}

#[test]
fn test_lookup_bool() {
    let m = types::bool().expect("Should resolve Bool");
    assert_eq!(m.size(), 1);
    assert!(m.is_pod());
}

#[test]
fn test_lookup_double() {
    let m = types::double().expect("Should resolve Double");
    assert_eq!(m.size(), 8);
    assert!(m.is_pod());
}

#[test]
fn test_type_name() {
    let m = types::int().unwrap();
    let name = m.type_name(true).expect("Should have type name");
    assert_eq!(name, "Swift.Int");
}

#[test]
fn test_descriptor_name() {
    let m = types::int().unwrap();
    let name = m.descriptor_name().expect("Should have descriptor name");
    assert_eq!(name, "Int");
}

#[test]
fn test_metadata_debug() {
    let m = types::int().unwrap();
    let dbg = format!("{:?}", m);
    assert!(dbg.contains("Int"), "Debug should contain 'Int': {dbg}");
    assert!(
        dbg.contains("Struct"),
        "Debug should contain 'Struct': {dbg}"
    );
}

#[test]
fn test_optional_metadata() {
    let int_m = types::int().unwrap();
    let opt_m = types::optional(&int_m).expect("Should resolve Optional<Int>");
    assert_eq!(
        opt_m.kind(),
        swift_runtime_sys::SwiftABI::MetadataKind::Optional
    );
}

#[test]
fn test_array_metadata() {
    let int_m = types::int().unwrap();
    let arr_m = types::array(&int_m).expect("Should resolve Array<Int>");
    assert_eq!(
        arr_m.kind(),
        swift_runtime_sys::SwiftABI::MetadataKind::Struct
    );
}

#[test]
fn test_dictionary_metadata() {
    let str_m = types::string().unwrap();
    let int_m = types::int().unwrap();
    let dict_m = types::dictionary(&str_m, &int_m).expect("Should resolve Dictionary<String,Int>");
    assert_eq!(
        dict_m.kind(),
        swift_runtime_sys::SwiftABI::MetadataKind::Struct
    );
}

#[test]
fn test_lookup_by_mangled_name() {
    let m = lookup_type(b"Sf").expect("Should resolve Float via mangled name");
    assert_eq!(m.size(), 4);
    assert!(m.is_pod());
}

#[test]
fn test_string_create_extract() {
    let buf = swift_runtime::string::create_small("hello").expect("Should create small string");
    let extracted = swift_runtime::string::extract_small(&buf).expect("Should extract");
    assert_eq!(extracted, "hello");
}

#[test]
fn test_retain_drop() {
    // Just verify Retained compiles and doesn't crash with null
    // Real test would need a Swift object
    let r = unsafe { swift_runtime::retain::Retained::from_raw(std::ptr::null_mut()) };
    assert!(r.is_none());
}
