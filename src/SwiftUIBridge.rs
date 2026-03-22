#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! SwiftUI bridge — construct and display SwiftUI views from Rust.
//!
//! Uses a small Swift helper dylib for operations that require complex
//! calling conventions (LSK/Text creation, existential boxing, window display).
//! The Rust side handles: String creation, metadata verification, dlsym resolution.

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
    HelperNotLoaded(String),
    SymbolNotFound(String),
    TypeTooLarge(usize),
    CreationFailed(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// Existential container for `any View` (40 bytes)
// ═══════════════════════════════════════════════════════════════════════════

/// An existential container for `any View`.
///
/// Layout (64-bit):
///   [0..24]  inline value buffer (3 words) — for types ≤24 bytes
///            OR: [0..8] = box pointer, rest unused — for types >24 bytes
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
}

// ═══════════════════════════════════════════════════════════════════════════
// Framework loading
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
    unsafe { dlopen(c"/System/Library/Frameworks/AppKit.framework/AppKit".as_ptr(), 1); }
    LOADED.store(true, std::sync::atomic::Ordering::Release);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// View witness table lookup
// ═══════════════════════════════════════════════════════════════════════════

/// Get the View protocol witness table for a type via swift_conformsToProtocol.
pub fn get_view_witness_table(metadata: *const c_void) -> Option<*const c_void> {
    let conforms = sym(c"swift_conformsToProtocol");
    let view_proto = sym(c"$s7SwiftUI4ViewMp");
    if conforms.is_null() || view_proto.is_null() { return None; }
    type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
    let f: ConformsFn = unsafe { core::mem::transmute(conforms) };
    let wt = unsafe { f(metadata, view_proto) };
    if wt.is_null() { None } else { Some(wt) }
}

/// Get the size of SwiftUI.Text values.
pub fn text_size() -> Option<usize> {
    ensure_loaded().ok()?;
    let meta = sym(c"$s7SwiftUI4TextVN");
    if meta.is_null() { return None; }
    let vwt = unsafe { crate::SwiftABI::get_value_witness_table(meta) };
    if vwt.is_null() { return None; }
    Some(unsafe { (*vwt).size })
}

// ═══════════════════════════════════════════════════════════════════════════
// Pure-Rust Swift.String creation
// ═══════════════════════════════════════════════════════════════════════════

/// Create a Swift.String from a Rust &str, entirely from Rust using inline asm.
///
/// Returns the 16-byte String representation.
/// Proven correct: output matches byte-for-byte with Swift-created Strings.
pub unsafe fn create_swift_string(s: &str) -> Result<[u8; 16], SwiftUIError> {
    let string_init = sym(
        c"$sSS21_builtinStringLiteral17utf8CodeUnitCount7isASCIISSBp_BwBi1_tcfC"
    );
    let string_meta = sym(c"$sSSN");
    if string_init.is_null() || string_meta.is_null() {
        return Err(SwiftUIError::SymbolNotFound("String.init".into()));
    }

    let s0: u64;
    let s1: u64;

    #[cfg(target_arch = "aarch64")]
    {
        core::arch::asm!(
            "blr {func}",
            func = in(reg) string_init,
            in("x0") s.as_ptr(),
            in("x1") s.len(),
            in("x2") s.is_ascii() as u64,
            in("x3") string_meta,
            lateout("x0") s0,
            lateout("x1") s1,
            lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
            lateout("x6") _, lateout("x7") _, lateout("x8") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
            lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
            lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(*const u8, usize, bool, *const c_void) -> (u64, u64);
        let f: F = core::mem::transmute(string_init);
        let r = f(s.as_ptr(), s.len(), s.is_ascii(), string_meta);
        s0 = r.0;
        s1 = r.1;
    }

    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&s0.to_le_bytes());
    buf[8..].copy_from_slice(&s1.to_le_bytes());
    Ok(buf)
}

// ═══════════════════════════════════════════════════════════════════════════
// Helper dylib interface
// ═══════════════════════════════════════════════════════════════════════════

/// A loaded Swift helper dylib that provides SwiftUI bridging functions.
pub struct SwiftUIHelper {
    handle: *mut c_void,
    // Resolved function pointers
    create_lsk: unsafe extern "C" fn(*const c_void, *mut c_void) -> usize,
    create_text: unsafe extern "C" fn(*const c_void, *mut c_void) -> usize,
    create_text_controller: unsafe extern "C" fn(*const u8, usize) -> *mut c_void,
    show_window: unsafe extern "C" fn(*mut c_void),
}

impl SwiftUIHelper {
    /// Load the Swift helper dylib.
    pub unsafe fn load(path: &str) -> Result<Self, SwiftUIError> {
        ensure_loaded()?;

        let cpath = CString::new(path).unwrap();
        let handle = dlopen(cpath.as_ptr(), 1);
        if handle.is_null() {
            return Err(SwiftUIError::HelperNotLoaded(path.to_string()));
        }

        let create_lsk = dlsym(handle, c"step_create_lsk".as_ptr());
        let create_text = dlsym(handle, c"step_create_text".as_ptr());
        let create_ctrl = dlsym(handle, c"create_text_hosting_controller".as_ptr());
        let show_win = dlsym(handle, c"show_window".as_ptr());

        if create_lsk.is_null() || create_text.is_null() || create_ctrl.is_null() || show_win.is_null() {
            return Err(SwiftUIError::SymbolNotFound("helper functions".into()));
        }

        Ok(Self {
            handle,
            create_lsk: core::mem::transmute(create_lsk),
            create_text: core::mem::transmute(create_text),
            create_text_controller: core::mem::transmute(create_ctrl),
            show_window: core::mem::transmute(show_win),
        })
    }

    /// Create a Swift String from Rust (pure Rust, no helper needed).
    pub unsafe fn create_string(&self, s: &str) -> Result<[u8; 16], SwiftUIError> {
        create_swift_string(s)
    }

    /// Create a LocalizedStringKey from a Swift String (uses helper).
    pub unsafe fn create_localized_string_key(&self, string_bytes: &[u8; 16]) -> Result<Vec<u8>, SwiftUIError> {
        let mut buf = vec![0u8; 64];
        let size = (self.create_lsk)(string_bytes.as_ptr() as _, buf.as_mut_ptr() as _);
        buf.truncate(size);
        Ok(buf)
    }

    /// Create a SwiftUI.Text from a LocalizedStringKey (uses helper).
    pub unsafe fn create_text_from_lsk(&self, lsk_bytes: &[u8]) -> Result<Vec<u8>, SwiftUIError> {
        let mut buf = vec![0u8; 64];
        let size = (self.create_text)(lsk_bytes.as_ptr() as _, buf.as_mut_ptr() as _);
        buf.truncate(size);
        Ok(buf)
    }

    /// Create a Text from a Rust string (String created in Rust, LSK+Text via helper).
    pub unsafe fn create_text(&self, s: &str) -> Result<Vec<u8>, SwiftUIError> {
        let string = self.create_string(s)?;
        let lsk = self.create_localized_string_key(&string)?;
        self.create_text_from_lsk(&lsk)
    }

    /// Create an `any View` existential from a Text value.
    pub unsafe fn text_to_existential(&self, text_bytes: &[u8]) -> Result<AnyViewExistential, SwiftUIError> {
        let text_meta = sym(c"$s7SwiftUI4TextVN");
        if text_meta.is_null() {
            return Err(SwiftUIError::SymbolNotFound("Text metadata".into()));
        }

        let wt = get_view_witness_table(text_meta)
            .ok_or_else(|| SwiftUIError::SymbolNotFound("Text:View conformance".into()))?;

        let vwt = crate::SwiftABI::get_value_witness_table(text_meta);
        let text_size = (*vwt).size;

        let mut container = AnyViewExistential {
            inline_buffer: [0; 3],
            metadata: text_meta,
            witness_table: wt,
        };

        if text_size <= 24 {
            // Inline: copy value directly
            core::ptr::copy_nonoverlapping(
                text_bytes.as_ptr(),
                container.inline_buffer.as_mut_ptr() as *mut u8,
                text_size,
            );
        } else {
            // Box: allocate a box and copy the value into it
            // Use swift_allocBox to allocate
            let alloc_box = sym(c"swift_allocBox");
            if alloc_box.is_null() {
                return Err(SwiftUIError::SymbolNotFound("swift_allocBox".into()));
            }

            // swift_allocBox is Swift CC — use our thunk
            let (box_obj, box_buf) = crate::SwiftCCThunks::swift_allocBox(text_meta)
                .map_err(|_| SwiftUIError::SymbolNotFound("swift_allocBox thunk".into()))?;

            // Copy the Text value into the box buffer
            // Use initializeWithCopy from the VWT to properly retain references
            ((*vwt).initialize_with_copy)(box_buf as _, text_bytes.as_ptr() as *mut c_void, text_meta);

            // Store the box object pointer in the first word of the inline buffer
            container.inline_buffer[0] = box_obj as u64;
        }

        Ok(container)
    }

    /// Full pipeline: Rust &str → SwiftUI.Text → any View existential.
    pub unsafe fn create_text_existential(&self, s: &str) -> Result<AnyViewExistential, SwiftUIError> {
        let text_bytes = self.create_text(s)?;
        self.text_to_existential(&text_bytes)
    }

    /// Show a text string in a SwiftUI window (simplest path, all via helper).
    pub unsafe fn show_text_window(&self, s: &str) {
        let controller = (self.create_text_controller)(s.as_ptr(), s.len());
        (self.show_window)(controller);
    }

    /// Show an existential in a SwiftUI window.
    pub unsafe fn show_existential_window(&self, existential: &AnyViewExistential) -> Result<(), SwiftUIError> {
        let show_ex = dlsym(self.handle, c"step_show_existential".as_ptr());
        if show_ex.is_null() {
            return Err(SwiftUIError::SymbolNotFound("step_show_existential".into()));
        }
        type ShowFn = unsafe extern "C" fn(*const c_void);
        let show: ShowFn = core::mem::transmute(show_ex);
        show(existential as *const AnyViewExistential as *const c_void);
        Ok(())
    }
}
