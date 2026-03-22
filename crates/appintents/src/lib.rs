//! Apple AppIntents — Siri shortcuts and Spotlight integration from Rust.
//!
//! ```ignore
//! assert!(appintents::is_available());
//! ```
//!
//! Note: Defining actual App Intents requires the Swift @AppIntent protocol
//! which needs compiler macro support. This crate provides availability
//! checking and will be extended as the bridge generator supports protocols.

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// Check if AppIntents framework is available.
pub fn is_available() -> bool {
    let f = sym(c"appintents_available");
    if f.is_null() {
        return false;
    }
    type F = unsafe extern "C" fn() -> bool;
    unsafe { (std::mem::transmute::<_, F>(f))() }
}
