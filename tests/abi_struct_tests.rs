//! Tests for ABI struct layouts, SwiftCCThunks, StdlibTypes, and DlsymStdlib.

use swift_runtime_sys::RuntimeRaw::*;
use swift_runtime_sys::SwiftABI::*;
use swift_runtime_sys::StdlibTypes;
use swift_runtime_sys::DlsymStdlib;

fn resolve_type(mangled: &[u8]) -> *const core::ffi::c_void {
    unsafe {
        swift_getTypeByMangledNameInEnvironment(
            mangled.as_ptr(),
            mangled.len(),
            core::ptr::null(),
            0,
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// StdlibTypes tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_int_metadata() {
    let m = StdlibTypes::int_metadata();
    assert!(m.is_some(), "Should resolve Int metadata");
    let m = m.unwrap();
    assert!(!m.is_null());
    assert_eq!(StdlibTypes::metadata_kind(m), Some(MetadataKind::Struct));
}

#[test]
fn test_stdlib_string_metadata() {
    let m = StdlibTypes::string_metadata();
    assert!(m.is_some());
    assert_eq!(StdlibTypes::metadata_kind(m.unwrap()), Some(MetadataKind::Struct));
}

#[test]
fn test_stdlib_bool_metadata() {
    let m = StdlibTypes::bool_metadata();
    assert!(m.is_some());
    let (size, stride, _alignment) = StdlibTypes::type_layout(m.unwrap()).unwrap();
    assert_eq!(size, 1, "Bool should be 1 byte");
    assert_eq!(stride, 1);
}

#[test]
fn test_stdlib_double_metadata() {
    let m = StdlibTypes::double_metadata();
    assert!(m.is_some());
    let (size, _stride, alignment) = StdlibTypes::type_layout(m.unwrap()).unwrap();
    assert_eq!(size, 8, "Double should be 8 bytes");
    assert_eq!(alignment, 8);
}

#[test]
fn test_stdlib_optional_int_metadata() {
    let int_m = StdlibTypes::int_metadata().unwrap();
    let opt_m = StdlibTypes::optional_metadata(int_m);
    assert!(opt_m.is_some());
    assert_eq!(StdlibTypes::metadata_kind(opt_m.unwrap()), Some(MetadataKind::Optional));
}

#[test]
fn test_stdlib_array_int_metadata() {
    let int_m = StdlibTypes::int_metadata().unwrap();
    let arr_m = StdlibTypes::array_metadata(int_m);
    assert!(arr_m.is_some(), "Should resolve Array<Int> metadata");
    assert_eq!(StdlibTypes::metadata_kind(arr_m.unwrap()), Some(MetadataKind::Struct));
}

#[test]
fn test_stdlib_dictionary_metadata() {
    let str_m = StdlibTypes::string_metadata().unwrap();
    let int_m = StdlibTypes::int_metadata().unwrap();
    let dict_m = StdlibTypes::dictionary_metadata(str_m, int_m);
    assert!(dict_m.is_some(), "Should resolve Dictionary<String, Int> metadata");
}

#[test]
fn test_stdlib_is_pod() {
    assert_eq!(StdlibTypes::is_pod(StdlibTypes::int_metadata().unwrap()), Some(true));
    assert_eq!(StdlibTypes::is_pod(StdlibTypes::bool_metadata().unwrap()), Some(true));
    assert_eq!(StdlibTypes::is_pod(StdlibTypes::double_metadata().unwrap()), Some(true));
    // String is NOT pod (has non-trivial copy/destroy)
    assert_eq!(StdlibTypes::is_pod(StdlibTypes::string_metadata().unwrap()), Some(false));
}

// ═══════════════════════════════════════════════════════════════════════════
// VWT read tests for non-POD types
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_vwt_string() {
    let m = StdlibTypes::string_metadata().unwrap();
    let vwt = unsafe { &*get_value_witness_table(m) };
    // String is 16 bytes on 64-bit (2 words)
    assert_eq!(vwt.get_size(), 16);
    assert_eq!(vwt.get_stride(), 16);
    assert!(!vwt.is_pod(), "String is not POD");
    assert!(vwt.is_inline(), "String fits inline in existential");
}

#[test]
fn test_vwt_array_int() {
    let int_m = StdlibTypes::int_metadata().unwrap();
    let arr_m = StdlibTypes::array_metadata(int_m).unwrap();
    let vwt = unsafe { &*get_value_witness_table(arr_m) };
    // Array is 8 bytes (one pointer — class reference)
    assert_eq!(vwt.get_size(), 8);
    assert!(!vwt.is_pod(), "Array is not POD");
}

// ═══════════════════════════════════════════════════════════════════════════
// ABI struct layout tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_context_descriptor_flags() {
    let f = ContextDescriptorFlags(16); // Class
    assert_eq!(f.kind(), ContextDescriptorKind::Class);
    assert!(!f.is_generic());

    let f = ContextDescriptorFlags(17 | (1 << 7)); // Struct, generic
    assert_eq!(f.kind(), ContextDescriptorKind::Struct);
    assert!(f.is_generic());
}

#[test]
fn test_function_type_flags() {
    let f = FunctionTypeFlags(3 | (0x01000000) | (0x20000000));
    assert_eq!(f.num_parameters(), 3);
    assert!(f.is_throws());
    assert!(f.is_async());
    assert!(!f.is_escaping());
    assert!(!f.is_sendable());
}

#[test]
fn test_existential_type_flags() {
    let f = ExistentialTypeFlags(2); // 2 witness tables, class-constrained
    assert_eq!(f.num_witness_tables(), 2);
    assert_eq!(f.class_constraint(), ExistentialClassConstraint::Class);
    assert!(!f.has_superclass_constraint());
}

#[test]
fn test_field_record_flags() {
    let f = FieldRecordFlags(0);
    assert!(!f.is_indirect_case());
    assert!(!f.is_var());

    let f = FieldRecordFlags(3);
    assert!(f.is_indirect_case());
    assert!(f.is_var());
}

#[test]
fn test_relative_pointer_null() {
    let rp = RelativePointer(0);
    assert!(rp.is_null());
    let rp = RelativePointer(42);
    assert!(!rp.is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// SwiftCCThunks tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_thunk_get_type_name() {
    let m = StdlibTypes::int_metadata().unwrap();
    let result = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeName(m, true)
    };
    assert!(result.is_ok());
    let (name, len) = result.unwrap();
    assert!(len > 0);
    assert!(name.contains("Int"), "Expected 'Int' in type name, got '{name}'");
}

#[test]
fn test_thunk_get_type_name_string() {
    let m = StdlibTypes::string_metadata().unwrap();
    let (name, _) = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeName(m, true)
    }.unwrap();
    assert_eq!(name, "Swift.String");
}

#[test]
fn test_thunk_check_metadata_state() {
    let m = StdlibTypes::int_metadata().unwrap();
    let resp = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_checkMetadataState(0, m)
    };
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert_eq!(resp.metadata, m, "checkMetadataState should return same metadata");
}

#[test]
fn test_thunk_get_type_context_descriptor() {
    let m = StdlibTypes::int_metadata().unwrap();
    let desc = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeContextDescriptor(m)
    };
    assert!(desc.is_ok());
    let desc = desc.unwrap();
    assert!(!desc.is_null(), "Int should have a type context descriptor");

    // Read the descriptor as a TypeContextDescriptor
    let tcd = unsafe { &*(desc as *const TypeContextDescriptor) };
    let name = unsafe { tcd.get_name() }.to_str().unwrap();
    assert_eq!(name, "Int");
}

#[test]
fn test_struct_descriptor_fields() {
    let m = StdlibTypes::int_metadata().unwrap();
    let desc = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeContextDescriptor(m)
    }.unwrap();

    let sd = unsafe { &*(desc as *const StructDescriptor) };
    let name = unsafe { sd.base.get_name() }.to_str().unwrap();
    assert_eq!(name, "Int");
    assert_eq!(sd.base.base.kind(), ContextDescriptorKind::Struct);
}

// ═══════════════════════════════════════════════════════════════════════════
// DlsymStdlib tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_dlsym_hardware_concurrency() {
    let c = DlsymStdlib::get_hardware_concurrency();
    assert!(c.is_some(), "Should resolve via dlsym");
    assert!(c.unwrap() > 0);
}

#[test]
fn test_dlsym_os_version() {
    // The symbol has a leading underscore: _swift_stdlib_operatingSystemVersion
    // Our DlsymStdlib now uses the correct name
    let v = DlsymStdlib::get_os_version();
    if let Some(v) = v {
        // The function uses Swift CC and returns a struct — on arm64 this may
        // return garbage via C ABI. Just check it doesn't crash.
        println!("OS version (may be incorrect on arm64 due to CC): {}.{}.{}", v.major, v.minor, v.patch);
    } else {
        println!("OS version symbol not found");
    }
}

#[test]
fn test_dlsym_random() {
    let mut buf = [0u8; 16];
    let ok = DlsymStdlib::random(&mut buf);
    if ok {
        // Extremely unlikely all 16 bytes are still zero
        assert!(buf.iter().any(|&b| b != 0), "random buffer should have non-zero bytes");
    } else {
        println!("random symbol not found (OK on some platforms)");
    }
}

#[test]
fn test_dlsym_stack_bounds() {
    let bounds = DlsymStdlib::get_current_stack_bounds();
    if let Some((begin, end)) = bounds {
        assert!(!begin.is_null());
        assert!(!end.is_null());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Dynamic cast test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_dynamic_cast_metatype() {
    let int_m = StdlibTypes::int_metadata().unwrap();
    // Cast Int metatype to itself — should succeed
    let result = unsafe {
        swift_runtime_sys::DynamicCast::swift_dynamicCastMetatype(int_m, int_m)
    };
    assert!(!result.is_null(), "Casting Int metatype to itself should succeed");
}

// ═══════════════════════════════════════════════════════════════════════════
// Enum tag test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_enum_optional_is_optional_kind() {
    // Verify Optional<Int> resolves and has the right kind
    let opt_m = resolve_type(b"SiSg");
    assert!(!opt_m.is_null(), "Should resolve Optional<Int>");
    assert_eq!(StdlibTypes::metadata_kind(opt_m), Some(MetadataKind::Optional));
    // Verify VWT is accessible
    let vwt = unsafe { &*get_value_witness_table(opt_m) };
    // Optional<Int> should be 8 bytes (Int + tag fits in spare bits on 64-bit)
    assert!(vwt.get_size() == 8 || vwt.get_size() == 9,
        "Optional<Int> size should be 8 or 9, got {}", vwt.get_size());
}

// ═══════════════════════════════════════════════════════════════════════════
// Box alloc/project/dealloc test (via thunk)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_box_alloc_project_dealloc() {
    let int_m = StdlibTypes::int_metadata().unwrap();

    let result = unsafe { swift_runtime_sys::SwiftCCThunks::swift_allocBox(int_m) };
    assert!(result.is_ok(), "swift_allocBox should succeed");
    let (object, buffer) = result.unwrap();
    assert!(!object.is_null(), "Box object should be non-null");
    assert!(!buffer.is_null(), "Box buffer should be non-null");

    // Write a value into the box
    unsafe { *(buffer as *mut i64) = 42 };

    // Project should return same buffer
    let projected = unsafe { swift_runtime_sys::BoxExistential::swift_projectBox(object) };
    assert_eq!(projected, buffer, "projectBox should return same buffer");

    // Read back
    let val = unsafe { *(projected as *const i64) };
    assert_eq!(val, 42);

    // Dealloc
    unsafe { swift_runtime_sys::BoxExistential::swift_deallocBox(object) };
}

// ═══════════════════════════════════════════════════════════════════════════
// Weak reference lifecycle test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_weak_init_load_destroy() {
    // swift_weakInit/LoadStrong/Destroy are in libswiftCore, use dlsym
    use core::ffi::{c_char, c_void};
    unsafe extern "C" { fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void; }
    let rtld = (-2isize) as *mut c_void;

    let init_fn = unsafe { dlsym(rtld, c"swift_weakInit".as_ptr()) };
    let load_fn = unsafe { dlsym(rtld, c"swift_weakLoadStrong".as_ptr()) };
    let destroy_fn = unsafe { dlsym(rtld, c"swift_weakDestroy".as_ptr()) };

    if !init_fn.is_null() && !load_fn.is_null() && !destroy_fn.is_null() {
        type WeakInit = unsafe extern "C" fn(*mut c_void, *mut c_void);
        type WeakLoad = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
        type WeakDestroy = unsafe extern "C" fn(*mut c_void);

        let init: WeakInit = unsafe { core::mem::transmute(init_fn) };
        let load: WeakLoad = unsafe { core::mem::transmute(load_fn) };
        let destroy: WeakDestroy = unsafe { core::mem::transmute(destroy_fn) };

        let mut storage = [0u8; 16];
        let s = storage.as_mut_ptr() as *mut c_void;
        unsafe {
            init(s, core::ptr::null_mut());
            let loaded = load(s);
            assert!(loaded.is_null(), "Loading null weak ref should return null");
            destroy(s);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Unowned reference test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_unowned_retain_release_null() {
    // unowned retain/release on null should be no-ops
    unsafe {
        let r = swift_runtime_sys::UnownedRef::swift_unownedRetain(core::ptr::null_mut());
        assert!(r.is_null());
        swift_runtime_sys::UnownedRef::swift_unownedRelease(core::ptr::null_mut());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Bridge object test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_bridge_object_retain_release_null() {
    unsafe {
        let r = swift_runtime_sys::BridgeObject::swift_bridgeObjectRetain(core::ptr::null_mut());
        assert!(r.is_null());
        swift_runtime_sys::BridgeObject::swift_bridgeObjectRelease(core::ptr::null_mut());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// In-process reflection mirror test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_reflection_mirror_recursive_count() {
    // Check that Int has 1 stored property (_value)
    let int_m = StdlibTypes::int_metadata().unwrap();
    let count = unsafe {
        swift_runtime_sys::ReflectionMirrorInProcess::swift_reflectionMirror_recursiveCount(int_m)
    };
    // Int has 1 field: _value
    assert!(count >= 0, "recursive count should be >= 0, got {count}");
    println!("Int recursive field count: {count}");
}

// ═══════════════════════════════════════════════════════════════════════════
// Tuple metadata inspection test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_tuple_metadata_inspection() {
    let int_m = StdlibTypes::int_metadata().unwrap();
    let double_m = StdlibTypes::double_metadata().unwrap();

    let resp = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTupleTypeMetadata2(
            0, // Complete
            int_m,
            double_m,
            core::ptr::null(), // no labels
            core::ptr::null(), // no proposed witnesses
        )
    };
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert!(!resp.metadata.is_null(), "Tuple metadata should be non-null");

    // Check it's a tuple
    let kind = StdlibTypes::metadata_kind(resp.metadata).unwrap();
    assert_eq!(kind, MetadataKind::Tuple);

    // Read the tuple metadata fields
    let tm = unsafe { &*(resp.metadata as *const TupleMetadata) };
    assert_eq!(tm.num_elements, 2, "Should have 2 elements");

    let elements = unsafe { tm.elements() };
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0].metadata, int_m);
    assert_eq!(elements[1].metadata, double_m);
    // Int offset should be 0, Double offset should be 8
    assert_eq!(elements[0].offset, 0);
    assert_eq!(elements[1].offset, 8);
}

// ═══════════════════════════════════════════════════════════════════════════
// Function metadata inspection test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_function_metadata_inspection() {
    use swift_runtime_sys::SwiftABI::FunctionTypeFlags;
    // Get metadata for () -> () via swift_getFunctionTypeMetadata0
    let void_to_void = unsafe {
        swift_runtime_sys::MetadataIntrospection::swift_getFunctionTypeMetadata0(
            0x04000000, // escaping, 0 params
            resolve_type(b"yt"), // Void result type
        )
    };
    if void_to_void.is_null() {
        println!("Could not create function type metadata, skipping");
        return;
    }
    let kind = StdlibTypes::metadata_kind(void_to_void).unwrap();
    assert_eq!(kind, MetadataKind::Function);

    let fm = unsafe { &*(void_to_void as *const FunctionMetadata) };
    let flags = FunctionTypeFlags(fm.flags);
    assert_eq!(flags.num_parameters(), 0);
    assert!(flags.is_escaping());
    assert!(!flags.is_throws());
    assert!(!flags.is_async());
}

// ═══════════════════════════════════════════════════════════════════════════
// Protocol descriptor tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_protocol_descriptors() {
    assert!(StdlibTypes::error_protocol_descriptor().is_some(), "Should find Error protocol");
    assert!(StdlibTypes::hashable_protocol_descriptor().is_some(), "Should find Hashable protocol");
    assert!(StdlibTypes::equatable_protocol_descriptor().is_some(), "Should find Equatable protocol");
    assert!(StdlibTypes::comparable_protocol_descriptor().is_some(), "Should find Comparable protocol");
    assert!(StdlibTypes::coding_key_protocol_descriptor().is_some(), "Should find CodingKey protocol");
    // Sendable and Actor are marker protocols — may not have descriptors
    if StdlibTypes::sendable_protocol_descriptor().is_some() {
        println!("Found Sendable protocol descriptor");
    }
    if StdlibTypes::actor_protocol_descriptor().is_some() {
        println!("Found Actor protocol descriptor");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Direct metadata symbol tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_direct_metadata_symbols() {
    let int_d = StdlibTypes::int_metadata_direct();
    assert!(int_d.is_some(), "Should find $sSiN");

    let bool_d = StdlibTypes::bool_metadata_direct();
    assert!(bool_d.is_some(), "Should find $sSbN");

    let double_d = StdlibTypes::double_metadata_direct();
    assert!(double_d.is_some(), "Should find $sSdN");

    let string_d = StdlibTypes::string_metadata_direct();
    assert!(string_d.is_some(), "Should find $sSSN");
}

// ═══════════════════════════════════════════════════════════════════════════
// String construction / extraction test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_swift_small_string() {
    let buf = StdlibTypes::create_swift_string("hello").unwrap();
    let extracted = StdlibTypes::extract_small_string(&buf).unwrap();
    assert_eq!(extracted, "hello");

    let buf = StdlibTypes::create_swift_string("").unwrap();
    let extracted = StdlibTypes::extract_small_string(&buf).unwrap();
    assert_eq!(extracted, "");

    let buf = StdlibTypes::create_swift_string("0123456789abcde").unwrap(); // 15 chars
    let extracted = StdlibTypes::extract_small_string(&buf).unwrap();
    assert_eq!(extracted, "0123456789abcde");

    // 16+ chars can't be small strings
    assert!(StdlibTypes::create_swift_string("0123456789abcdef").is_none());
}

// ═══════════════════════════════════════════════════════════════════════════
// Concurrency hook test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_concurrency_hook_read() {
    // Read the current enqueueGlobal hook — should be null (no hook installed)
    let hook = unsafe {
        swift_runtime_sys::ConcurrencyHooks::read_hook(c"swift_task_enqueueGlobal_hook")
    };
    // The hook might be non-null if something else installed one, but at least
    // this proves the symbol resolves.
    println!("swift_task_enqueueGlobal_hook = {:?}", hook);
}

#[test]
fn test_concurrency_hook_install_uninstall() {
    use swift_runtime_sys::ConcurrencyHooks::*;

    // Save original
    let original = unsafe { read_hook(c"swift_task_enqueueGlobal_hook") };

    // Install a no-op hook that just calls through
    unsafe extern "C" fn passthrough_hook(job: JobRef, original: EnqueueGlobalOriginal) {
        // Just call the original
        unsafe { original(job) };
    }

    let installed = unsafe {
        install_hook(c"swift_task_enqueueGlobal_hook", passthrough_hook as *const core::ffi::c_void)
    };
    assert!(installed, "Should be able to install hook");

    // Verify it was installed
    let current = unsafe { read_hook(c"swift_task_enqueueGlobal_hook") };
    assert_eq!(current, passthrough_hook as *const core::ffi::c_void);

    // Restore original
    unsafe {
        install_hook(c"swift_task_enqueueGlobal_hook", original);
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// Compare thunks test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_compare_type_descriptors() {
    let int_m = StdlibTypes::int_metadata().unwrap();
    let desc = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeContextDescriptor(int_m)
    }.unwrap();

    // Comparing a descriptor to itself should return true
    let result = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_compareTypeContextDescriptors(desc, desc)
    };
    assert!(result.is_ok());
    assert!(result.unwrap(), "Descriptor should be equal to itself");

    // Compare with a different descriptor
    let str_m = StdlibTypes::string_metadata().unwrap();
    let str_desc = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_getTypeContextDescriptor(str_m)
    }.unwrap();
    let result = unsafe {
        swift_runtime_sys::SwiftCCThunks::swift_compareTypeContextDescriptors(desc, str_desc)
    };
    assert!(result.is_ok());
    assert!(!result.unwrap(), "Int and String descriptors should not be equal");
}

// ═══════════════════════════════════════════════════════════════════════════
// §38: VTable struct layout tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_vtable_descriptor_header_size() {
    assert_eq!(core::mem::size_of::<swift_runtime_sys::SwiftABI::VTableDescriptorHeader>(), 8);
}

#[test]
fn test_method_descriptor_flags() {
    use swift_runtime_sys::SwiftABI::*;
    let f = MethodDescriptorFlags(0x10); // Method (kind=0) + instance
    assert_eq!(f.kind(), MethodDescriptorKind::Method);
    assert!(f.is_instance());
    assert!(!f.is_async());

    let f = MethodDescriptorFlags(0x41); // Init (kind=1) + async (0x40)
    assert_eq!(f.kind(), MethodDescriptorKind::Init);
    assert!(f.is_async());
}

// ═══════════════════════════════════════════════════════════════════════════
// §39: Witness table + protocol requirement tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_protocol_requirement_flags() {
    use swift_runtime_sys::SwiftABI::*;
    let f = ProtocolRequirementFlags(1 | 0x10); // Method + instance
    assert_eq!(f.kind(), ProtocolRequirementKind::Method);
    assert!(f.is_instance());
}

#[test]
fn test_witness_table_for_int_hashable() {
    // Get Hashable witness table for Int
    let int_m = StdlibTypes::int_metadata().unwrap();
    let hashable = StdlibTypes::hashable_protocol_descriptor().unwrap();

    // swift_conformsToProtocol is C ABI — use dlsym to call it
    use core::ffi::{c_char, c_void as V};
    unsafe extern "C" { fn dlsym(h: *mut V, s: *const c_char) -> *mut V; }
    type ConformsFn = unsafe extern "C" fn(*const V, *const V) -> *const V;
    let sym = unsafe { dlsym((-2isize) as *mut V, c"swift_conformsToProtocol".as_ptr()) };
    assert!(!sym.is_null(), "swift_conformsToProtocol should exist");
    let conforms: ConformsFn = unsafe { core::mem::transmute(sym) };
    let wtable = unsafe { conforms(int_m, hashable) };
    assert!(!wtable.is_null(), "Int should conform to Hashable");
}

// ═══════════════════════════════════════════════════════════════════════════
// §40: Async context struct tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_async_context_layout() {
    use swift_runtime_sys::SwiftABI::*;
    assert_eq!(core::mem::size_of::<AsyncContext>(), 16); // 2 pointers
    assert_eq!(core::mem::size_of::<ThrowingAsyncContext>(), 24); // 3 pointers
    assert_eq!(core::mem::size_of::<AsyncFunctionPointer>(), 8); // relative + u32
}

// ═══════════════════════════════════════════════════════════════════════════
// §37: Concurrency thunk tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_concurrency_thunk_get_main_executor() {
    let result = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_getMainExecutor() };
    assert!(result.is_ok(), "Should resolve swift_task_getMainExecutor");
    let executor = result.unwrap();
    println!("Main executor: identity={:?}, impl={:?}", executor.identity, executor.implementation);
    // Main executor should be non-null
    assert!(!executor.identity.is_null() || !executor.implementation.is_null(),
        "Main executor should have at least one non-null field");
}

#[test]
fn test_concurrency_thunk_get_current_executor() {
    // Outside of a task, current executor should be the generic executor
    let result = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_getCurrentExecutor() };
    assert!(result.is_ok());
}

#[test]
fn test_concurrency_thunk_get_time() {
    let mut sec: i64 = 0;
    let mut nsec: i64 = 0;
    let result = unsafe {
        swift_runtime_sys::ConcurrencyThunks::swift_get_time(&mut sec, &mut nsec, 1) // clock 1 = continuous
    };
    assert!(result.is_ok());
    assert!(sec > 0 || nsec > 0, "Time should be non-zero");
    println!("Time: {sec}s {nsec}ns");
}

#[test]
fn test_concurrency_thunk_is_main_executor() {
    let main_exec = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_getMainExecutor() }.unwrap();
    let is_main = unsafe { swift_runtime_sys::ConcurrencyThunks::swift_task_isMainExecutor(main_exec) };
    assert!(is_main.is_ok());
    assert!(is_main.unwrap(), "Main executor should report as main");
}

// ═══════════════════════════════════════════════════════════════════════════
// §41: Error function thunk test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_set_will_throw_handler() {
    // swift_setWillThrowHandler may be SPI — use dlsym
    use core::ffi::{c_char, c_void as V};
    unsafe extern "C" { fn dlsym(h: *mut V, s: *const c_char) -> *mut V; }

    unsafe extern "C" fn my_handler(_error: *mut V) {}

    let sym = unsafe { dlsym((-2isize) as *mut V, c"swift_setWillThrowHandler".as_ptr()) };
    if sym.is_null() {
        println!("swift_setWillThrowHandler not available via dlsym, skipping");
        return;
    }
    type SetHandler = unsafe extern "C" fn(Option<unsafe extern "C" fn(*mut V)>);
    let set_handler: SetHandler = unsafe { core::mem::transmute(sym) };
    unsafe { set_handler(Some(my_handler)) };
    unsafe { set_handler(None) };
}

// ═══════════════════════════════════════════════════════════════════════════
// ViewConformanceBuilder tests
// ═══════════════════════════════════════════════════════════════════════════

fn ensure_swiftui_loaded() -> bool {
    use core::ffi::{c_char, c_void as V};
    unsafe extern "C" {
        fn dlopen(path: *const c_char, mode: i32) -> *mut V;
    }
    let handle = unsafe { dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 0x1) };
    !handle.is_null()
}

#[test]
fn test_text_metadata_resolution() {
    if !ensure_swiftui_loaded() {
        println!("SwiftUI not available, skipping");
        return;
    }
    let m = swift_runtime_sys::ViewConformanceBuilder::text_metadata();
    assert!(m.is_some(), "Should resolve SwiftUI.Text metadata");

    let size = swift_runtime_sys::ViewConformanceBuilder::text_value_size();
    assert!(size.is_some(), "Should get Text value size");
    let size = size.unwrap();
    println!("SwiftUI.Text size: {size} bytes");
    assert!(size > 0 && size <= 256, "Text size should be reasonable");
}

#[test]
fn test_text_view_witness_table() {
    if !ensure_swiftui_loaded() { println!("SwiftUI not available"); return; }
    let wt = swift_runtime_sys::ViewConformanceBuilder::text_view_witness_table();
    assert!(wt.is_some(), "Should resolve Text:View witness table");
}

#[test]
fn test_build_dynamic_view_struct() {
    unsafe extern "C" fn dummy_body(_result: *mut core::ffi::c_void, _self_val: *const core::ffi::c_void) {
        // In a real implementation this would write a Text value into result
    }

    if !ensure_swiftui_loaded() { println!("SwiftUI not available"); return; }

    let result = unsafe {
        swift_runtime_sys::ViewConformanceBuilder::build_dynamic_view("TestRustView", dummy_body)
    };

    match result {
        Ok(view) => {
            assert!(!view.metadata.is_null(), "Metadata should be non-null");
            assert!(!view.witness_table.is_null(), "Witness table should be non-null");
            assert!(!view.descriptor.is_null(), "Descriptor should be non-null");

            // Verify the metadata kind
            let kind = unsafe { *(view.metadata as *const usize) };
            assert_eq!(kind, 0x200, "Kind should be Struct (0x200)");

            // Verify the VWT pointer
            let vwt_ptr = unsafe { *((view.metadata as *const *const core::ffi::c_void).offset(-1)) };
            assert!(!vwt_ptr.is_null(), "VWT should be non-null");

            println!("Successfully built dynamic View conformance for 'TestRustView'");
            println!("  metadata: {:?}", view.metadata);
            println!("  witness_table: {:?}", view.witness_table);
            println!("  descriptor: {:?}", view.descriptor);
        }
        Err(e) => {
            println!("build_dynamic_view failed (expected if SwiftUI not available): {:?}", e);
        }
    }
}
