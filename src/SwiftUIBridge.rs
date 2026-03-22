#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! SwiftUI bridge — construct and display SwiftUI views from Rust.
//!
//! This works by:
//! 1. Calling SwiftUI type initializers (Text.init, VStack.init, etc.) via dlsym
//! 2. Getting the View witness table via swift_conformsToProtocol
//! 3. Constructing an existential container (`any View` = 40 bytes)
//! 4. Passing it to NSHostingController/NSHostingView
//!
//! No fake metadata or witness tables needed — we use the real ones from SwiftUI.

use core::ffi::{c_char, c_void};
use std::ffi::CString;

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;
unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) as *const c_void }
}

#[derive(Debug)]
pub enum SwiftUIError {
    FrameworkNotLoaded,
    SymbolNotFound(String),
    TypeTooLarge(usize),
}

// ═══════════════════════════════════════════════════════════════════════════
// Existential container for `any View` (40 bytes)
// ═══════════════════════════════════════════════════════════════════════════

/// An existential container for `any View`.
///
/// Layout (64-bit):
///   [0..24]  inline value buffer (3 words)
///   [24..32] type metadata pointer
///   [32..40] View witness table pointer
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AnyViewExistential {
    pub inline_buffer: [u64; 3],
    pub metadata: *const c_void,
    pub witness_table: *const c_void,
}

impl AnyViewExistential {
    pub const SIZE: usize = 40;

    /// Create an existential container from a value, its metadata, and witness table.
    ///
    /// # Safety
    /// - `value` must point to a valid value of the type described by `metadata`
    /// - `value_size` must match the type's actual size
    /// - `metadata` must be valid type metadata
    /// - `witness_table` must be the View witness table for this type
    pub unsafe fn new(
        value: *const c_void,
        value_size: usize,
        metadata: *const c_void,
        witness_table: *const c_void,
    ) -> Result<Self, SwiftUIError> {
        if value_size > 24 {
            return Err(SwiftUIError::TypeTooLarge(value_size));
        }
        let mut container = AnyViewExistential {
            inline_buffer: [0; 3],
            metadata,
            witness_table,
        };
        core::ptr::copy_nonoverlapping(
            value as *const u8,
            container.inline_buffer.as_mut_ptr() as *mut u8,
            value_size,
        );
        Ok(container)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SwiftUI framework loader
// ═══════════════════════════════════════════════════════════════════════════

static LOADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Ensure SwiftUI.framework is loaded.
pub fn ensure_loaded() -> Result<(), SwiftUIError> {
    if LOADED.load(std::sync::atomic::Ordering::Acquire) {
        return Ok(());
    }
    let handle = unsafe {
        dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1)
    };
    if handle.is_null() {
        return Err(SwiftUIError::FrameworkNotLoaded);
    }
    // Also load AppKit
    unsafe {
        dlopen(c"/System/Library/Frameworks/AppKit.framework/AppKit".as_ptr(), 1);
    }
    LOADED.store(true, std::sync::atomic::Ordering::Release);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Get View witness table for any SwiftUI type
// ═══════════════════════════════════════════════════════════════════════════

/// Get the View protocol witness table for a type.
///
/// Uses `swift_conformsToProtocol(metadata, ViewProtocol)`.
pub fn get_view_witness_table(metadata: *const c_void) -> Option<*const c_void> {
    let conforms = sym(c"swift_conformsToProtocol");
    let view_proto = sym(c"$s7SwiftUI4ViewMp");
    if conforms.is_null() || view_proto.is_null() {
        return None;
    }
    type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
    let f: ConformsFn = unsafe { core::mem::transmute(conforms) };
    let wt = unsafe { f(metadata, view_proto) };
    if wt.is_null() { None } else { Some(wt) }
}

// ═══════════════════════════════════════════════════════════════════════════
// SwiftUI.Text construction
// ═══════════════════════════════════════════════════════════════════════════

/// Get the size of SwiftUI.Text values.
pub fn text_size() -> Option<usize> {
    ensure_loaded().ok()?;
    let meta = sym(c"$s7SwiftUI4TextVN");
    if meta.is_null() { return None; }
    let vwt = unsafe { crate::SwiftABI::get_value_witness_table(meta) };
    if vwt.is_null() { return None; }
    Some(unsafe { (*vwt).size })
}

/// Create a `SwiftUI.Text` value from a Rust string.
///
/// Returns the raw bytes of the Text value and its metadata.
///
/// # Safety
/// The returned bytes are a valid Swift Text value that must be properly
/// destroyed when no longer needed (via the VWT destroy function).
pub unsafe fn create_text(s: &str) -> Result<(Vec<u8>, *const c_void, *const c_void), SwiftUIError> {
    ensure_loaded()?;

    let text_meta = sym(c"$s7SwiftUI4TextVN");
    if text_meta.is_null() {
        return Err(SwiftUIError::SymbolNotFound("SwiftUI.Text metadata".into()));
    }

    let vwt = crate::SwiftABI::get_value_witness_table(text_meta);
    let size = (*vwt).size;

    let wt = get_view_witness_table(text_meta)
        .ok_or_else(|| SwiftUIError::SymbolNotFound("Text:View conformance".into()))?;

    // Resolve Text.init(_:tableName:bundle:comment:)
    let text_init = sym(
        c"$s7SwiftUI4TextV_9tableName6bundle7commentAcA18LocalizedStringKeyV_SSSgSo8NSBundleCSgs06StaticI0VSgtcfC"
    );
    let lsk_init = sym(c"$s7SwiftUI18LocalizedStringKeyV13stringLiteralACSS_tcfC");
    let string_init = sym(c"$sSS21_builtinStringLiteral17utf8CodeUnitCount7isASCIISSBp_BwBi1_tcfC");
    let string_meta = sym(c"$sSSN");
    let lsk_meta = sym(c"$s7SwiftUI18LocalizedStringKeyVN");
    let text_type_meta = sym(c"$s7SwiftUI4TextVN");

    if text_init.is_null() || lsk_init.is_null() || string_init.is_null()
        || string_meta.is_null() || lsk_meta.is_null() || text_type_meta.is_null()
    {
        return Err(SwiftUIError::SymbolNotFound("Text init chain".into()));
    }

    // Step 1: Create Swift.String
    // String.init(_builtinStringLiteral:utf8CodeUnitCount:isASCII:)
    // Returns String (16 bytes) via C ABI
    type StringInitFn = unsafe extern "C" fn(*const u8, usize, bool, *const c_void) -> [u8; 16];
    let make_string: StringInitFn = core::mem::transmute(string_init);
    let swift_string = make_string(s.as_ptr(), s.len(), s.is_ascii(), string_meta);

    // Step 2: Create LocalizedStringKey
    // LocalizedStringKey.init(stringLiteral:) 
    // Takes ownership of the String, returns LSK
    // LSK size can vary — let's get it from metadata
    let lsk_vwt = crate::SwiftABI::get_value_witness_table(lsk_meta);
    let lsk_size = (*lsk_vwt).size;

    // Allocate buffer for LSK
    let mut lsk_buf = vec![0u8; lsk_size];

    // Call LSK init — this is a Swift CC function that takes String and returns LSK
    // On arm64, for struct returns > 16 bytes, the return is via an indirect pointer (x8)
    // But for many SwiftUI types, the return fits in registers.
    // Let's use the asm approach:
    #[cfg(target_arch = "aarch64")]
    {
        let lsk_ptr = lsk_buf.as_mut_ptr() as *mut c_void;
        // LSK.init(stringLiteral:) takes String (16 bytes in x0,x1) + metatype
        // Returns LSK which may be indirect via x8
        core::arch::asm!(
            "mov x8, {result}",  // indirect return pointer
            "blr {func}",
            func = in(reg) lsk_init,
            result = in(reg) lsk_ptr,
            in("x0") u64::from_le_bytes(swift_string[0..8].try_into().unwrap()),
            in("x1") u64::from_le_bytes(swift_string[8..16].try_into().unwrap()),
            in("x2") lsk_meta, // metatype
            lateout("x0") _,
            lateout("x1") _,
            lateout("x3") _, lateout("x4") _, lateout("x5") _,
            lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        // On x86_64, large struct returns use a hidden first parameter
        type LskInitFn = unsafe extern "C" fn(*mut c_void, [u8; 16], *const c_void);
        let f: LskInitFn = core::mem::transmute(lsk_init);
        f(lsk_buf.as_mut_ptr() as _, swift_string, lsk_meta);
    }

    // Step 3: Create Text from LSK
    // Text.init(_:tableName:bundle:comment:)
    // Args: LSK, Optional<String>=nil, Optional<Bundle>=nil, Optional<StaticString>=nil, Text.Type
    let mut text_buf = vec![0u8; size];
    
    #[cfg(target_arch = "aarch64")]
    {
        let text_ptr = text_buf.as_mut_ptr() as *mut c_void;
        // This function takes LSK (passed indirectly), nil, nil, nil, metatype
        // and returns Text (indirectly via x8)
        // LSK is passed in x0 (pointer to value for large structs)
        core::arch::asm!(
            "mov x8, {result}",
            "blr {func}",
            func = in(reg) text_init,
            result = in(reg) text_ptr,
            in("x0") lsk_buf.as_ptr(),  // LSK pointer (passed indirectly)
            in("x1") 0u64,              // tableName = nil  
            in("x2") 0u64,              // bundle = nil
            in("x3") 0u64,              // comment = nil (Optional<StaticString>)
            in("x4") text_type_meta,    // Text.Type metatype
            lateout("x0") _,
            lateout("x1") _,
            lateout("x5") _, lateout("x6") _, lateout("x7") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _,
            lateout("x12") _, lateout("x13") _, lateout("x14") _, lateout("x15") _,
            lateout("x16") _, lateout("x17") _, lateout("lr") _,
            clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type TextInitFn = unsafe extern "C" fn(*mut c_void, *const c_void, u64, u64, u64, *const c_void);
        let f: TextInitFn = core::mem::transmute(text_init);
        f(text_buf.as_mut_ptr() as _, lsk_buf.as_ptr() as _, 0, 0, 0, text_type_meta);
    }

    // Destroy the LSK (it was consumed by Text.init, but let's be safe)
    // Actually, Swift init methods take ownership, so LSK is consumed. Don't destroy.

    Ok((text_buf, text_meta, wt))
}

/// Create an `any View` existential containing a `Text` with the given string.
pub unsafe fn create_text_existential(s: &str) -> Result<AnyViewExistential, SwiftUIError> {
    let (text_bytes, metadata, wt) = create_text(s)?;
    AnyViewExistential::new(
        text_bytes.as_ptr() as *const c_void,
        text_bytes.len(),
        metadata,
        wt,
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Window display
// ═══════════════════════════════════════════════════════════════════════════

/// Show a SwiftUI view in a window.
///
/// This requires a compiled Swift helper dylib that provides `show_existential_in_window`.
/// The helper creates NSHostingController and NSWindow from the existential.
///
/// If no helper is available, this returns an error.
pub unsafe fn show_in_window(
    existential: &AnyViewExistential,
    helper_path: &str,
) -> Result<(), SwiftUIError> {
    let path = CString::new(helper_path).unwrap();
    let handle = dlopen(path.as_ptr(), 1);
    if handle.is_null() {
        return Err(SwiftUIError::SymbolNotFound(format!("Could not load helper: {helper_path}")));
    }

    let show_fn = dlsym(handle, c"show_existential_in_window".as_ptr());
    if show_fn.is_null() {
        return Err(SwiftUIError::SymbolNotFound("show_existential_in_window".into()));
    }

    type ShowFn = unsafe extern "C" fn(*const c_void);
    let show: ShowFn = core::mem::transmute(show_fn);
    show(existential as *const AnyViewExistential as *const c_void);

    Ok(())
}

/// Show a Text view in a window using the probe helper.
///
/// This is the simplest end-to-end path: Rust string → SwiftUI window.
pub unsafe fn show_text(s: &str, helper_path: &str) -> Result<(), SwiftUIError> {
    let existential = create_text_existential(s)?;
    show_in_window(&existential, helper_path)
}
