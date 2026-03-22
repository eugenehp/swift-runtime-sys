//! Apple Translation framework — on-device text translation from Rust.
//!
//! Wraps Apple's Translation framework for privacy-preserving,
//! on-device translation. Available on macOS 15+ and iOS 18+.
//!
//! ```ignore
//! use translation::*;
//!
//! assert!(is_available());
//! ```
//!
//! Note: Full translation requires async session management which
//! is bridged through the SwiftUI `.translationPresentation()` modifier.

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// Check if the Translation framework is available.
pub fn is_available() -> bool {
    let f = sym(c"translation_available");
    if f.is_null() {
        return false;
    }
    type F = unsafe extern "C" fn() -> bool;
    unsafe { (std::mem::transmute::<_, F>(f))() }
}
