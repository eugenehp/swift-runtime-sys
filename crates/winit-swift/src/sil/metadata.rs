//! Swift type metadata structures constructed in Rust.
//!
//! These match the exact binary layout that `swiftc -emit-ir` produces
//! for a zero-sized struct conforming to SwiftUI.App.

use core::ffi::c_void;
use std::sync::OnceLock;

// ═══════════════════════════════════════════════════════════════════════════
// Symbol resolution
// ═══════════════════════════════════════════════════════════════════════════

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const core::ffi::c_char) -> *mut c_void;
}

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

fn sym(name: &core::ffi::CStr) -> *mut c_void {
    unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) }
}

/// Resolved SwiftUI symbols needed for App.main().
// SAFETY: These are function/data pointers resolved once at init, never mutated.
unsafe impl Send for SwiftUISymbols {}
unsafe impl Sync for SwiftUISymbols {}
pub(crate) struct SwiftUISymbols {
    /// `SwiftUI.App.main<T>()` — the entry point
    pub app_main: *mut c_void,
    /// `SwiftUI.App` protocol descriptor
    pub app_protocol: *mut c_void,
    /// Empty tuple value witness table (`Void` / `()`)
    pub empty_vwt: *mut c_void,
    /// `SwiftUI.WindowGroup.init(id:title:lazyContent:)`
    pub window_group_init: *mut c_void,
    /// `SwiftUI.Text.init(_:tableName:bundle:comment:)`
    pub text_init: *mut c_void,
    /// `SwiftUI.Text` metadata
    pub text_metadata: *mut c_void,
    /// `SwiftUI.Text: View` witness table
    pub text_view_wt: *mut c_void,
    /// `SwiftUI.LocalizedStringKey.init(stringLiteral:)`
    pub lsk_init: *mut c_void,
    /// `SwiftUI.WindowGroup<Text>: Scene` witness table accessor
    pub windowgroup_scene_mc: *mut c_void,
    /// `SwiftUI.WindowGroup<Text>` metadata nominal descriptor
    pub windowgroup_nominal: *mut c_void,
    /// `swift_getWitnessTable`
    pub get_witness_table: *mut c_void,
    /// `SwiftUI.App` protocol requirement: body.getter
    pub app_body_getter_req: *mut c_void,
    /// `SwiftUI.App` protocol requirement: init
    pub app_init_req: *mut c_void,
    /// `SwiftUI.App.Body` associated type requirement
    pub app_body_assoc: *mut c_void,
    /// `SwiftUI.App.Body: Scene` associated conformance requirement
    pub app_body_scene_assoc: *mut c_void,
    /// `SwiftUI.Scene` protocol descriptor
    pub scene_protocol: *mut c_void,
}

static SYMBOLS: OnceLock<SwiftUISymbols> = OnceLock::new();

pub(crate) fn resolve() -> &'static SwiftUISymbols {
    SYMBOLS.get_or_init(|| {
        // Ensure SwiftUI is loaded
        unsafe {
            dlsym(
                dlsym(RTLD_DEFAULT, c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr()) as _,
                std::ptr::null(),
            );
            // Actually load it properly
            extern "C" { fn dlopen(path: *const core::ffi::c_char, mode: i32) -> *mut c_void; }
            dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1);
        }

        let s = SwiftUISymbols {
            app_main:             sym(c"$s7SwiftUI3AppPAAE4mainyyFZ"),
            app_protocol:         sym(c"$s7SwiftUI3AppMp"),
            empty_vwt:            sym(c"$sytWV"),
            window_group_init:    sym(c"$s7SwiftUI11WindowGroupV2id5title11lazyContentACyxGSSSg_AA4TextVSgxyctcfC"),
            text_init:            sym(c"$s7SwiftUI4TextV_9tableName6bundle7commentAcA18LocalizedStringKeyV_SSSgSo8NSBundleCSgs06StaticI0VSgtcfC"),
            text_metadata:        sym(c"$s7SwiftUI4TextVN"),
            text_view_wt:         sym(c"$s7SwiftUI4TextVAA4ViewAAWP"),
            lsk_init:             sym(c"$s7SwiftUI18LocalizedStringKeyV13stringLiteralACSS_tcfC"),
            windowgroup_scene_mc: sym(c"$s7SwiftUI11WindowGroupVyxGAA5SceneAAMc"),
            windowgroup_nominal:  sym(c"$s7SwiftUI11WindowGroupVMn"),
            get_witness_table:    sym(c"swift_getWitnessTable"),
            app_body_getter_req:  sym(c"$s7SwiftUI3AppP4body4BodyQzvgTq"),
            app_init_req:         sym(c"$s7SwiftUI3AppPxycfCTq"),
            app_body_assoc:       sym(c"$s4Body7SwiftUI3AppPTl"),
            app_body_scene_assoc: sym(c"$s7SwiftUI3AppP4BodyAC_AA5SceneTn"),
            scene_protocol:       sym(c"$s7SwiftUI5SceneMp"),
        };

        // Verify critical symbols
        let critical = [
            ("App.main", s.app_main),
            ("App protocol", s.app_protocol),
            ("empty VWT", s.empty_vwt),
            ("WindowGroup.init", s.window_group_init),
            ("Text metadata", s.text_metadata),
            ("Text:View WT", s.text_view_wt),
            ("swift_getWitnessTable", s.get_witness_table),
        ];
        for (name, ptr) in critical {
            assert!(!ptr.is_null(), "Failed to resolve SwiftUI symbol: {name}");
        }

        s
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// Type metadata for our Rust "App" struct
// ═══════════════════════════════════════════════════════════════════════════

/// Full type metadata layout for a zero-sized Swift struct.
///
/// ```text
/// offset -16: superclass pointer (null for structs)
/// offset  -8: value witness table pointer
/// offset   0: kind (0x200 = struct)     ← THIS is the "metadata pointer"
/// offset   8: nominal type descriptor
/// ```
#[repr(C)]
pub(crate) struct FullTypeMetadata {
    pub superclass: *const c_void,
    pub vwt: *const c_void,
    pub kind: usize,
    pub descriptor: *const c_void,
}

/// Module descriptor (parent of the type).
#[repr(C)]
pub(crate) struct ModuleDescriptor {
    pub flags: u32,
    pub parent: u32,   // relative pointer (0 = no parent)
    pub name: i32,     // relative pointer to module name string
}

/// Nominal type descriptor for a struct.
#[repr(C)]
pub(crate) struct NominalTypeDescriptor {
    pub flags: u32,           // 0x51 = struct + has import info
    pub parent: i32,          // relative pointer to module descriptor
    pub name: i32,            // relative pointer to type name string
    pub accessor: i32,        // relative pointer to metadata accessor
    pub field_descriptor: i32,// relative pointer to fields (0 for none)
    pub num_fields: u32,
    pub field_offset_vector_offset: u32,
}

// ═══════════════════════════════════════════════════════════════════════════
// Static metadata instances
// ═══════════════════════════════════════════════════════════════════════════

/// Module name.
static MODULE_NAME: &[u8] = b"RustApp\0";

/// Type name.
static TYPE_NAME: &[u8] = b"RustSwiftUIApp\0";

/// The metadata accessor function — returns the metadata pointer.
///
/// Swift CC: `(MetadataRequest) -> MetadataResponse`
/// We return the pre-allocated metadata with state = Complete (0).
#[no_mangle]
unsafe extern "C" fn rust_app_metadata_accessor(request: usize) -> (*const c_void, usize) {
    let meta = metadata_ptr();
    (meta, 0) // (metadata, state=Complete)
}

/// Get the metadata pointer (points at kind field, offset +16 from start).
pub(crate) fn metadata_ptr() -> *const c_void {
    unsafe {
        let full = &METADATA as *const FullTypeMetadata;
        (full as *const u8).add(16) as *const c_void
    }
}

// These are initialized at first use
static mut METADATA: FullTypeMetadata = FullTypeMetadata {
    superclass: std::ptr::null(),
    vwt: std::ptr::null(),
    kind: 0x200, // MetadataKind::Struct = 0x200
    descriptor: std::ptr::null(),
};

static mut MODULE_DESC: ModuleDescriptor = ModuleDescriptor {
    flags: 0,
    parent: 0,
    name: 0,
};

static mut TYPE_DESC: NominalTypeDescriptor = NominalTypeDescriptor {
    flags: 0x51,
    parent: 0,
    name: 0,
    accessor: 0,
    field_descriptor: 0,
    num_fields: 0,
    field_offset_vector_offset: 2,
};

/// Initialize all metadata with correct relative pointers.
pub(crate) unsafe fn init_metadata() {
    let syms = resolve();

    // Module descriptor
    MODULE_DESC.name = relative_ptr(
        &MODULE_DESC.name as *const _ as *const u8,
        MODULE_NAME.as_ptr(),
    );

    // Type descriptor
    TYPE_DESC.parent = relative_ptr(
        &TYPE_DESC.parent as *const _ as *const u8,
        &MODULE_DESC as *const _ as *const u8,
    );
    TYPE_DESC.name = relative_ptr(
        &TYPE_DESC.name as *const _ as *const u8,
        TYPE_NAME.as_ptr(),
    );
    TYPE_DESC.accessor = relative_ptr(
        &TYPE_DESC.accessor as *const _ as *const u8,
        rust_app_metadata_accessor as *const u8,
    );

    // Full metadata
    METADATA.vwt = syms.empty_vwt;
    METADATA.descriptor = &TYPE_DESC as *const _ as *const c_void;
}

/// Compute a relative pointer (target - source) as i32.
fn relative_ptr(from: *const u8, to: *const u8) -> i32 {
    let diff = (to as isize) - (from as isize);
    diff as i32
}
