//! Open a SwiftUI Text window from Rust.
//!
//! Step 1 (String creation) is pure Rust via inline asm.
//! Steps 2-3 (LSK, Text) use a thin Swift helper for CC correctness.
//! Step 4 (existential assembly) is Rust.
//! Step 5 (window display) uses the Swift helper.
//!
//! Prerequisites — compile the Swift helpers:
//!   cd /tmp/swiftui_analysis
//!   xcrun swiftc -emit-library sizes_fix.swift -o libSizes.dylib \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)
//!   xcrun swiftc -emit-library probe_wt.swift -o libProbeWT.dylib \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)

use std::ffi::{c_char, c_void};

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    fn NSApplicationLoad() -> bool;
}

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn main() {
    unsafe {
        // 0. Initialize AppKit + load SwiftUI
        NSApplicationLoad();
        dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1);

        let sizes = dlopen(c"/tmp/swiftui_analysis/libSizes.dylib".as_ptr(), 1);
        let probe = dlopen(c"/tmp/swiftui_analysis/libProbeWT.dylib".as_ptr(), 1);
        assert!(!sizes.is_null(), "libSizes.dylib not found");
        assert!(!probe.is_null(), "libProbeWT.dylib not found");

        println!("=== SwiftUI from Rust ===\n");

        // 1. Create Swift.String (PURE RUST — inline asm)
        let msg = "Hello from Rust! 🦀";
        let string_init = dlsym((-2isize) as *mut c_void,
            c"$sSS21_builtinStringLiteral17utf8CodeUnitCount7isASCIISSBp_BwBi1_tcfC".as_ptr());
        let string_meta = dlsym((-2isize) as *mut c_void, c"$sSSN".as_ptr());
        assert!(!string_init.is_null() && !string_meta.is_null());

        let s0: u64; let s1: u64;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) string_init,
            in("x0") msg.as_ptr(),
            in("x1") msg.len(),
            in("x2") msg.is_ascii() as u64,
            in("x3") string_meta,
            lateout("x0") s0, lateout("x1") s1,
            lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
            lateout("x6") _, lateout("x7") _, lateout("x8") _,
            lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
            lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
            lateout("x17") _, lateout("lr") _, clobber_abi("C"),
        );
        let mut swift_string = [0u8; 16];
        swift_string[..8].copy_from_slice(&s0.to_le_bytes());
        swift_string[8..].copy_from_slice(&s1.to_le_bytes());
        println!("Step 1 — String (Rust asm): [{:016x}, {:016x}]", s0, s1);

        // 2. Create LocalizedStringKey (Swift helper for CC)
        type CreateLSK = unsafe extern "C" fn(*const c_void, *mut c_void) -> usize;
        let create_lsk: CreateLSK = core::mem::transmute(dlsym(sizes, c"step_create_lsk".as_ptr()));
        let mut lsk = [0u8; 64];
        let lsk_size = create_lsk(swift_string.as_ptr() as _, lsk.as_mut_ptr() as _);
        println!("Step 2 — LSK ({lsk_size} bytes)");

        // 3. Create Text (Swift helper for CC)
        type CreateText = unsafe extern "C" fn(*const c_void, *mut c_void) -> usize;
        let create_text: CreateText = core::mem::transmute(dlsym(sizes, c"step_create_text".as_ptr()));
        let mut text = [0u8; 64];
        let text_size = create_text(lsk.as_ptr() as _, text.as_mut_ptr() as _);
        println!("Step 3 — Text ({text_size} bytes)");

        // 4. Verify metadata (RUST)
        let text_meta = dlsym((-2isize) as *mut c_void, c"$s7SwiftUI4TextVN".as_ptr());
        let view_proto = dlsym((-2isize) as *mut c_void, c"$s7SwiftUI4ViewMp".as_ptr());
        type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
        let conforms: ConformsFn = core::mem::transmute(
            dlsym((-2isize) as *mut c_void, c"swift_conformsToProtocol".as_ptr())
        );
        let wt = conforms(text_meta as _, view_proto as _);
        println!("Step 4 — metadata={:?}, witness_table={:?}", text_meta, wt);

        // 5. Create hosting controller + show window (Swift helper)
        type CreateCtrl = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
        type ShowWin = unsafe extern "C" fn(*mut c_void);
        let create_ctrl: CreateCtrl = core::mem::transmute(
            dlsym(probe, c"create_text_hosting_controller".as_ptr()));
        let show_win: ShowWin = core::mem::transmute(
            dlsym(probe, c"show_window".as_ptr()));

        println!("\nStep 5 — Opening SwiftUI window with: '{msg}'");
        let ctrl = create_ctrl(msg.as_ptr(), msg.len());
        show_win(ctrl);
    }
}
