//! SwiftUI window from pure Rust.
//!
//! Step 1: Swift.String created via arm64 inline asm (pure Rust)
//! Step 2: String → `any View` existential via single C-ABI helper call
//! Step 3: Window displayed via helper
//!
//! The helper is 40 lines of Swift — just CC bridging, no logic.
//!
//! Prerequisite:
//!   xcrun swiftc -emit-library /tmp/swiftui_analysis/cc_probe2.swift \
//!     -o /tmp/swiftui_analysis/libCCProbe.dylib \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)

use std::ffi::{c_char, c_void};

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" { fn NSApplicationLoad() -> bool; }
unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn main() {
    unsafe {
        NSApplicationLoad();
        dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1);
        let h = dlopen(c"/tmp/swiftui_analysis/libCCProbe.dylib".as_ptr(), 10);
        assert!(!h.is_null(), "Compile the helper first — see doc comment");

        println!("=== Pure Rust → SwiftUI ===\n");

        // ── Step 1: Create Swift.String in pure Rust (inline asm) ──
        let msg = "Hi Rust";
        let string_init = dlsym((-2isize) as *mut c_void,
            c"$sSS21_builtinStringLiteral17utf8CodeUnitCount7isASCIISSBp_BwBi1_tcfC".as_ptr());
        let string_meta = dlsym((-2isize) as *mut c_void, c"$sSSN".as_ptr());

        let s0: u64;
        let s1: u64;
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
        println!("1. Swift.String created in Rust ✓");

        // ── Step 2: String → any View existential (C-ABI helper) ──
        let mut string_bytes = [0u8; 16];
        string_bytes[..8].copy_from_slice(&s0.to_le_bytes());
        string_bytes[8..].copy_from_slice(&s1.to_le_bytes());
        
        let to_ex_ptr = dlsym(h, c"rust_string_to_existential".as_ptr());
        println!("  to_existential fn: {:?}", to_ex_ptr);
        assert!(!to_ex_ptr.is_null(), "rust_string_to_existential not found");
        type ToExistentialFn = unsafe extern "C" fn(*const c_void, *mut c_void) -> usize;
        let to_existential: ToExistentialFn = core::mem::transmute(to_ex_ptr);
        println!("  calling with string_bytes at {:?}", string_bytes.as_ptr());
        let mut existential = [0u8; 48];
        let ex_size = to_existential(string_bytes.as_ptr() as _, existential.as_mut_ptr() as _);
        println!("2. Existential created ({ex_size} bytes) ✓");

        // Verify metadata (RUST)
        let ex_meta = u64::from_le_bytes(existential[24..32].try_into().unwrap());
        let ex_wt = u64::from_le_bytes(existential[32..40].try_into().unwrap());
        let text_meta = dlsym((-2isize) as *mut c_void, c"$s7SwiftUI4TextVN".as_ptr()) as u64;
        let view_proto = dlsym((-2isize) as *mut c_void, c"$s7SwiftUI4ViewMp".as_ptr());
        type ConformsFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
        let conforms: ConformsFn = core::mem::transmute(
            dlsym((-2isize) as *mut c_void, c"swift_conformsToProtocol".as_ptr()));
        let wt = conforms(text_meta as *const c_void, view_proto as _) as u64;
        println!("3. Metadata verified: meta={}, wt={} ✓", ex_meta == text_meta, ex_wt == wt);

        // ── Step 3: Show in window ──
        type ShowFn = unsafe extern "C" fn(*const c_void);
        let show: ShowFn = core::mem::transmute(
            dlsym(h, c"rust_show_existential".as_ptr()));
        println!("\n4. Opening SwiftUI window with: '{msg}'");
        show(existential.as_ptr() as _);
    }
}
