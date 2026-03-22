//! Apple WidgetKit — reload widget timelines from Rust.
//!
//! ```ignore
//! widgetkit::reload_all();           // reload all widget timelines
//! widgetkit::reload("MyWidget");     // reload specific widget kind
//! ```

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// Reload all widget timelines.
pub fn reload_all() {
    let f = sym(c"widgetkit_reload_all");
    if !f.is_null() {
        type F = unsafe extern "C" fn();
        unsafe { (std::mem::transmute::<_, F>(f))() };
    }
}

/// Reload timelines for a specific widget kind.
pub fn reload(kind: &str) {
    let f = sym(c"widgetkit_reload_kind");
    if !f.is_null() {
        type F = unsafe extern "C" fn(*const u8, usize);
        unsafe { (std::mem::transmute::<_, F>(f))(kind.as_ptr(), kind.len()) };
    }
}
