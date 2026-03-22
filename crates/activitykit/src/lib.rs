//! Apple ActivityKit — Live Activities and Dynamic Island from Rust.
//!
//! ```ignore
//! assert!(activitykit::is_available());
//! ```
//!
//! Note: Creating Live Activities requires defining ActivityAttributes
//! via the @available Swift protocol, which needs compiler macro support.
//! This crate provides availability checking. Full Live Activity support
//! requires a Swift extension target in the app bundle.

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// Check if Live Activities are enabled on this device.
pub fn is_available() -> bool {
    let f = sym(c"activitykit_available");
    if f.is_null() {
        return false;
    }
    type F = unsafe extern "C" fn() -> bool;
    unsafe { (std::mem::transmute::<_, F>(f))() }
}
