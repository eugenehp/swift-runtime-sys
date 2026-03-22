#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Dynamic SwiftUI View conformance builder.
//!
//! Constructs the metadata artifacts needed to make a Rust-defined type conform
//! to the SwiftUI `View` protocol at runtime, without the Swift compiler.
//!
//! # Architecture
//!
//! A SwiftUI View conformance requires these heap-allocated structures:
//!
//! 1. **Nominal type descriptor** — identifies the type by name
//! 2. **Full type metadata** — VWT + Kind + descriptor pointer
//! 3. **Witness table** — function pointers for View protocol methods
//! 4. **Protocol conformance descriptor** — links type → protocol → witness table
//!
//! All relative pointers are computed as `(target_addr - field_addr)` as i32.

use core::ffi::{c_char, c_void};
use std::ffi::CString;

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;
unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) as *const c_void }
}

/// Write a relative pointer: i32 offset from `field_addr` to `target_addr`.
unsafe fn write_relative(field_addr: *mut i32, target: *const c_void) {
    let offset = (target as isize) - (field_addr as isize);
    *field_addr = offset as i32;
}

/// Write a relative pointer with the indirect flag (low bit set).
unsafe fn write_relative_indirect(field_addr: *mut i32, target: *const c_void) {
    let offset = (target as isize) - (field_addr as isize);
    *field_addr = (offset as i32) | 1;
}

// ═══════════════════════════════════════════════════════════════════════════
// External symbols we need from SwiftUI
// ═══════════════════════════════════════════════════════════════════════════

struct SwiftUISymbols {
    /// The View protocol descriptor: $s7SwiftUI4ViewMp
    view_protocol: *const c_void,
    /// Default _makeView: static (extension in SwiftUI):View._makeView(...)
    default_make_view: *const c_void,
    /// Default _makeViewList: static (extension in SwiftUI):View._makeViewList(...)
    default_make_view_list: *const c_void,
    /// Default _viewListCount: static (extension in SwiftUI):View._viewListCount(...)
    default_view_list_count: *const c_void,
    /// The empty tuple VWT ($sytWV) for zero-size types
    empty_vwt: *const c_void,
    /// SwiftUI.Text type metadata
    text_metadata: *const c_void,
    /// Text : View witness table
    text_view_witness_table: *const c_void,
}

impl SwiftUISymbols {
    fn resolve() -> Option<Self> {
        let view_protocol = sym(c"$s7SwiftUI4ViewMp");
        let default_make_view = sym(
            c"$s7SwiftUI4ViewPAAE05_makeC04view6inputsAA01_C7OutputsVAA11_GraphValueVyxG_AA01_C6InputsVtFZ"
        );
        let default_make_view_list = sym(
            c"$s7SwiftUI4ViewPAAE05_makeC4List4view6inputsAA01_cE7OutputsVAA11_GraphValueVyxG_AA01_cE6InputsVtFZ"
        );
        let default_view_list_count =
            sym(c"$s7SwiftUI4ViewPAAE14_viewListCount6inputsSiSgAA01_ceF6InputsV_tFZ");
        let empty_vwt = sym(c"$sytWV");
        let text_metadata = sym(c"$s7SwiftUI4TextVN");
        let text_view_witness_table = sym(c"$s7SwiftUI4TextVAA4ViewAAWP");

        if view_protocol.is_null()
            || default_make_view.is_null()
            || default_make_view_list.is_null()
            || default_view_list_count.is_null()
            || empty_vwt.is_null()
            || text_metadata.is_null()
        {
            return None;
        }

        Some(Self {
            view_protocol,
            default_make_view,
            default_make_view_list,
            default_view_list_count,
            empty_vwt,
            text_metadata,
            text_view_witness_table,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Memory layout structures (heap-allocated, never freed)
// ═══════════════════════════════════════════════════════════════════════════

/// Nominal type descriptor for a struct (StructDescriptor).
/// Layout: flags(4) + parent(4) + name(4) + access(4) + fields(4) + num_fields(4) + field_offset_vector(4) = 28 bytes
const STRUCT_DESCRIPTOR_SIZE: usize = 28;

/// Full type metadata: VWT_ptr(8) + kind(8) + descriptor_ptr(8) = 24 bytes
/// The metadata pointer points at the `kind` field (offset 8).
const FULL_METADATA_SIZE: usize = 24;

/// Witness table for View protocol.
/// The View protocol has these requirements:
///   [0] conformance descriptor (relative pointer)
///   [1] associated type Body metadata accessor
///   [2] associated conformance Body: View
///   [3] _makeView witness
///   [4] _makeViewList witness
///   [5] _viewListCount witness
///   [6] body.getter witness
///
/// Instantiated witness tables use absolute pointers.
const WITNESS_TABLE_SIZE: usize = 7 * 8; // 7 entries × 8 bytes

// ═══════════════════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════════════════

/// A body-getter function type.
/// Signature: (result_ptr: *mut c_void, self_ptr: *const c_void) -> ()
/// The function should write a SwiftUI.Text value into result_ptr.
pub type BodyGetterFn = unsafe extern "C" fn(result: *mut c_void, self_val: *const c_void);

/// Wrapper around the body getter that matches the witness_method convention.
/// Signature: (@out Body, @in_guaranteed Self) -> ()
pub type WitnessBodyGetterFn = unsafe extern "C" fn(result: *mut c_void, self_val: *const c_void);

/// Result of building a dynamic View conformance.
#[derive(Debug)]
pub struct DynamicView {
    /// The type metadata pointer (points at the Kind field).
    pub metadata: *const c_void,
    /// The witness table for View conformance.
    pub witness_table: *const c_void,
    /// The nominal type descriptor.
    pub descriptor: *const c_void,
    /// The type name (kept alive).
    _name: CString,
    /// Raw allocations (kept alive to prevent deallocation).
    _allocs: Vec<*mut c_void>,
}

/// Error type.
#[derive(Debug)]
pub enum ViewBuilderError {
    SymbolsNotFound(String),
    AllocationFailed,
}

/// Build a dynamic SwiftUI View conformance for a zero-size struct
/// whose body returns `SwiftUI.Text`.
///
/// # Arguments
/// * `name` — The type name (e.g., "MyRustView")
/// * `body_getter` — Function that writes a `SwiftUI.Text` into the result pointer.
///   Called as `body_getter(result_ptr, self_ptr)` where self_ptr points to the
///   (empty) struct value.
///
/// # Safety
/// The `body_getter` must write exactly `sizeof(SwiftUI.Text)` bytes into `result_ptr`.
/// The returned `DynamicView` must be kept alive for as long as the view is used.
///
/// # Example
/// ```ignore
/// unsafe extern "C" fn my_body(result: *mut c_void, _self: *const c_void) {
///     // Call Text.init to create a Text value and write it to result
///     let text = create_swift_text("Hello from Rust");
///     std::ptr::copy_nonoverlapping(&text as *const _ as *const u8, result as *mut u8, size_of_text);
/// }
///
/// let view = build_dynamic_view("MyRustView", my_body).unwrap();
/// // view.metadata and view.witness_table can be used to present the view
/// ```
pub unsafe fn build_dynamic_view(
    name: &str,
    body_getter: BodyGetterFn,
) -> Result<DynamicView, ViewBuilderError> {
    let syms = SwiftUISymbols::resolve().ok_or_else(|| {
        ViewBuilderError::SymbolsNotFound(
            "Could not resolve SwiftUI symbols (View protocol, default impls, Text metadata)"
                .into(),
        )
    })?;

    let mut allocs: Vec<*mut c_void> = Vec::new();

    // ── 1. Allocate and fill the nominal type descriptor ──
    let name_cstr = CString::new(name).unwrap();
    let desc = malloc(STRUCT_DESCRIPTOR_SIZE);
    if desc.is_null() {
        return Err(ViewBuilderError::AllocationFailed);
    }
    allocs.push(desc);
    core::ptr::write_bytes(desc as *mut u8, 0, STRUCT_DESCRIPTOR_SIZE);

    let desc_fields = desc as *mut i32;
    // [0] flags: ContextDescriptorKind::Struct (17) | unique (1<<6) = 17 | 64 = 81
    *desc_fields.add(0) = 81i32;
    // [1] parent: null (relative pointer = 0, meaning no parent module context)
    *desc_fields.add(1) = 0i32;
    // [2] name: relative pointer to the name C string
    write_relative(desc_fields.add(2), name_cstr.as_ptr() as *const c_void);
    // [3] access function: null (0)
    *desc_fields.add(3) = 0i32;
    // [4] fields descriptor: null (0 = no field descriptor)
    *desc_fields.add(4) = 0i32;
    // [5] num_fields: 0
    *desc_fields.add(5) = 0i32;
    // [6] field_offset_vector_offset: 0
    *desc_fields.add(6) = 0i32;

    // ── 2. Allocate and fill full type metadata ──
    let meta = malloc(FULL_METADATA_SIZE);
    if meta.is_null() {
        return Err(ViewBuilderError::AllocationFailed);
    }
    allocs.push(meta);
    core::ptr::write_bytes(meta as *mut u8, 0, FULL_METADATA_SIZE);

    let meta_ptrs = meta as *mut *const c_void;
    // [0] VWT pointer — use empty tuple VWT for zero-size struct
    *meta_ptrs.add(0) = syms.empty_vwt;

    let meta_words = meta as *mut usize;
    // [1] Kind = 512 (MetadataKind::Struct = 0x200)
    *meta_words.add(1) = 0x200;

    // [2] Descriptor pointer
    *meta_ptrs.add(2) = desc as *const c_void;

    // The actual metadata pointer points at offset 8 (the Kind field)
    let metadata_ptr = (meta as *const u8).add(8) as *const c_void;

    // ── 3. Build the witness table ──
    // For a simple approach, we build an absolute-pointer witness table.
    // The runtime may also accept relative witness tables, but absolute is simpler.
    //
    // The witness table layout for View:
    //   [0] = protocol conformance descriptor (we'll set to null for now)
    //   [1] = associated type Body metadata accessor function
    //   [2] = associated conformance Body: View accessor function
    //   [3] = _makeView witness (calls default impl)
    //   [4] = _makeViewList witness (calls default impl)
    //   [5] = _viewListCount witness (calls default impl)
    //   [6] = body.getter witness (OUR function)

    let wt = malloc(WITNESS_TABLE_SIZE);
    if wt.is_null() {
        return Err(ViewBuilderError::AllocationFailed);
    }
    allocs.push(wt);
    core::ptr::write_bytes(wt as *mut u8, 0, WITNESS_TABLE_SIZE);

    let wt_ptrs = wt as *mut *const c_void;

    // [0] conformance descriptor — null for dynamically created
    *wt_ptrs.add(0) = core::ptr::null();

    // [1] associated type Body metadata accessor
    // This should return the metadata for the body type (SwiftUI.Text).
    // We create a simple function that returns Text metadata.
    let body_type_accessor = create_body_type_accessor(syms.text_metadata);
    *wt_ptrs.add(1) = body_type_accessor;

    // [2] associated conformance Body: View
    // Returns the witness table for Text: View
    let body_conformance_accessor = if !syms.text_view_witness_table.is_null() {
        create_body_conformance_accessor(syms.text_view_witness_table)
    } else {
        core::ptr::null()
    };
    *wt_ptrs.add(2) = body_conformance_accessor;

    // [3] _makeView — use SwiftUI's default implementation
    *wt_ptrs.add(3) = syms.default_make_view;

    // [4] _makeViewList — use SwiftUI's default implementation
    *wt_ptrs.add(4) = syms.default_make_view_list;

    // [5] _viewListCount — use SwiftUI's default implementation
    *wt_ptrs.add(5) = syms.default_view_list_count;

    // [6] body.getter — OUR function
    *wt_ptrs.add(6) = body_getter as *const c_void;

    Ok(DynamicView {
        metadata: metadata_ptr,
        witness_table: wt as *const c_void,
        descriptor: desc as *const c_void,
        _name: name_cstr,
        _allocs: allocs,
    })
}

/// Create a function that returns Text metadata when called.
/// The associated type accessor has signature:
///   (MetadataRequest, *const WitnessTable) -> MetadataResponse
unsafe fn create_body_type_accessor(text_metadata: *const c_void) -> *const c_void {
    // We use a trampoline approach: store the metadata pointer in a static
    // and return it. For a real implementation you'd generate code or use
    // a closure. For simplicity, we store in a global.
    BODY_TYPE_METADATA.store(
        text_metadata as usize,
        core::sync::atomic::Ordering::Release,
    );
    body_type_accessor_trampoline as *const c_void
}

static BODY_TYPE_METADATA: core::sync::atomic::AtomicUsize =
    core::sync::atomic::AtomicUsize::new(0);

/// Trampoline that returns the stored body type metadata.
unsafe extern "C" fn body_type_accessor_trampoline(
    _request: usize,
    _wtable: *const c_void,
) -> *const c_void {
    BODY_TYPE_METADATA.load(core::sync::atomic::Ordering::Acquire) as *const c_void
}

/// Create a function that returns Text:View witness table.
unsafe fn create_body_conformance_accessor(text_view_wt: *const c_void) -> *const c_void {
    BODY_CONFORMANCE_WT.store(text_view_wt as usize, core::sync::atomic::Ordering::Release);
    body_conformance_accessor_trampoline as *const c_void
}

static BODY_CONFORMANCE_WT: core::sync::atomic::AtomicUsize =
    core::sync::atomic::AtomicUsize::new(0);

unsafe extern "C" fn body_conformance_accessor_trampoline(
    _assoc_type: *const c_void,
    _conforming_type: *const c_void,
    _wtable: *const c_void,
) -> *const c_void {
    BODY_CONFORMANCE_WT.load(core::sync::atomic::Ordering::Acquire) as *const c_void
}

// ═══════════════════════════════════════════════════════════════════════════
// Helper: create a SwiftUI.Text value
// ═══════════════════════════════════════════════════════════════════════════

/// Create a SwiftUI.Text from a string literal by calling into SwiftUI.
///
/// Returns the raw bytes of the Text value (caller must provide a buffer of sufficient size).
///
/// # Safety
/// `result` must point to a buffer of at least `text_value_size()` bytes.
pub unsafe fn create_text(_result: *mut c_void, _string: &str) -> bool {
    // Resolve Text.init(_:tableName:bundle:comment:)
    let text_init = sym(
        c"$s7SwiftUI4TextV_9tableName6bundle7commentAcA18LocalizedStringKeyV_SSSgSo8NSBundleCSgs06StaticI0VSgtcfC"
    );
    let lsk_init = sym(c"$s7SwiftUI18LocalizedStringKeyV13stringLiteralACSS_tcfC");

    if text_init.is_null() || lsk_init.is_null() {
        return false;
    }

    // Step 1: Create a Swift.String
    let string_init = sym(c"$sSS21_builtinStringLiteral17utf8CodeUnitCount7isASCIISSBp_BwBi1_tcfC");
    if string_init.is_null() {
        return false;
    }

    type StringInitFn = unsafe extern "C" fn(*const u8, usize, bool, *const c_void) -> [u8; 16];
    let make_string: StringInitFn = core::mem::transmute(string_init);
    let string_metatype = sym(c"$sSSN");
    let swift_string = make_string(
        _string.as_ptr(),
        _string.len(),
        _string.is_ascii(),
        string_metatype,
    );

    // Step 2: Create LocalizedStringKey from string
    type LskInitFn = unsafe extern "C" fn([u8; 16], *const c_void) -> [u8; 24]; // LSK is ~24 bytes
    let make_lsk: LskInitFn = core::mem::transmute(lsk_init);
    let lsk_metatype = sym(c"$s7SwiftUI18LocalizedStringKeyVN");
    if lsk_metatype.is_null() {
        return false;
    }
    let _lsk = make_lsk(swift_string, lsk_metatype);

    // Step 3: Create Text from LSK
    // Text.init(_:tableName:bundle:comment:)
    // This is a complex initializer. The exact calling convention depends on
    // Text's size and the platform. For now, we write the Text value through
    // an indirect return pointer.
    //
    // Note: The actual Text type is opaque and its size varies.
    // A safe approach is to use the VWT size from Text metadata.

    // For now, return false to indicate this is a stub.
    // The real implementation would call through the resolved function pointers
    // with correct argument passing.
    false
}

/// Get the size of a SwiftUI.Text value.
pub fn text_value_size() -> Option<usize> {
    let text_meta = sym(c"$s7SwiftUI4TextVN");
    if text_meta.is_null() {
        return None;
    }
    let vwt = unsafe { crate::SwiftABI::get_value_witness_table(text_meta) };
    if vwt.is_null() {
        return None;
    }
    Some(unsafe { (*vwt).size })
}

/// Get the metadata for SwiftUI.Text.
pub fn text_metadata() -> Option<*const c_void> {
    let m = sym(c"$s7SwiftUI4TextVN");
    if m.is_null() {
        None
    } else {
        Some(m)
    }
}

/// Get the View protocol witness table for SwiftUI.Text.
pub fn text_view_witness_table() -> Option<*const c_void> {
    let m = sym(c"$s7SwiftUI4TextVAA4ViewAAWP");
    if m.is_null() {
        None
    } else {
        Some(m)
    }
}
