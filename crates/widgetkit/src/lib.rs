//! Apple WidgetKit — reload widget timelines from Rust.
//!
//! **Platform support:** iOS 14+, macOS 11+, watchOS 9+ (not available on tvOS or visionOS).
//!
//! ```ignore
//! widgetkit::reload_all();           // reload all widget timelines
//! widgetkit::reload("MyWidget");     // reload specific widget kind
//! ```

// ── Real implementation (macOS, iOS, watchOS) ──
#[cfg(not(any(target_os = "tvos", target_os = "xros")))]
mod real {
    /// Reload all widget timelines.
    pub fn reload_all() {
        let f = apple_sys_helpers::sym(c"widgetkit_reload_all");
        if !f.is_null() {
            type F = unsafe extern "C" fn();
            unsafe { (std::mem::transmute::<_, F>(f))() };
        }
    }

    /// Reload timelines for a specific widget kind.
    pub fn reload(kind: &str) {
        let f = apple_sys_helpers::sym(c"widgetkit_reload_kind");
        if !f.is_null() {
            type F = unsafe extern "C" fn(*const u8, usize);
            unsafe { (std::mem::transmute::<_, F>(f))(kind.as_ptr(), kind.len()) };
        }
    }
}

#[cfg(not(any(target_os = "tvos", target_os = "xros")))]
pub use real::*;

// ── Stub for unsupported platforms ──
#[cfg(any(target_os = "tvos", target_os = "xros"))]
mod stub {
    /// WidgetKit is not available on this platform. No-op.
    pub fn reload_all() {}
    /// WidgetKit is not available on this platform. No-op.
    pub fn reload(_kind: &str) {}
}

#[cfg(any(target_os = "tvos", target_os = "xros"))]
pub use stub::*;
