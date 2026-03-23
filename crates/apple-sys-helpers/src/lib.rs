//! Shared helpers for Apple framework crates.
//!
//! Provides common `dlsym`-based dynamic symbol lookup used by all
//! framework wrapper crates, plus macros to eliminate boilerplate.
//!
//! # Quick start
//!
//! For a framework available on **all** Apple platforms:
//!
//! ```ignore
//! apple_sys_helpers::apple_framework!(c"storekit_available");
//! ```
//!
//! For a framework available on a **subset** of platforms:
//!
//! ```ignore
//! apple_sys_helpers::apple_framework!(c"arkit_available"; "ios", "xros");
//! ```
//!
//! Both generate a public `is_available() -> bool` that does the right thing
//! on every target: dynamic `dlsym` probe on supported platforms, compile-time
//! `false` on unsupported ones.

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// Look up a C symbol by name via `dlsym(RTLD_DEFAULT, name)`.
///
/// Returns a non-null pointer to the symbol, or null if not found.
#[inline]
pub fn sym(name: &core::ffi::CStr) -> *const c_void {
    // RTLD_DEFAULT = -2 on Apple platforms
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

// ── Macros ──────────────────────────────────────────────────────────────────

/// Declare `is_available()` for a framework available on **all** Apple platforms.
///
/// ```ignore
/// apple_sys_helpers::framework_availability!(c"storekit_available");
/// ```
#[macro_export]
macro_rules! framework_availability {
    ($probe_symbol:expr) => {
        /// Check if this framework is available at runtime.
        pub fn is_available() -> bool {
            let f = $crate::sym($probe_symbol);
            if f.is_null() {
                return false;
            }
            type F = unsafe extern "C" fn() -> bool;
            unsafe { (core::mem::transmute::<_, F>(f))() }
        }
    };
}

/// Declare a platform-aware `is_available()` for an Apple framework crate.
///
/// # All-platform form
///
/// ```ignore
/// apple_sys_helpers::apple_framework!(c"cloudkit_available");
/// ```
///
/// # Platform-restricted form
///
/// ```ignore
/// apple_sys_helpers::apple_framework!(c"arkit_available"; "ios", "xros");
/// ```
///
/// On listed platforms the function performs a runtime `dlsym` probe.
/// On all other platforms it returns `false` at compile time (zero cost).
#[macro_export]
macro_rules! apple_framework {
    // ── All platforms (no restriction) ──
    ($probe:expr) => {
        /// Check if this framework is available at runtime.
        pub fn is_available() -> bool {
            let f = $crate::sym($probe);
            if f.is_null() {
                return false;
            }
            type F = unsafe extern "C" fn() -> bool;
            unsafe { (core::mem::transmute::<_, F>(f))() }
        }
    };

    // ── Platform-restricted ──
    ($probe:expr; $($platform:literal),+ $(,)?) => {
        #[cfg(any($(target_os = $platform),+))]
        /// Check if this framework is available at runtime.
        pub fn is_available() -> bool {
            let f = $crate::sym($probe);
            if f.is_null() {
                return false;
            }
            type F = unsafe extern "C" fn() -> bool;
            unsafe { (core::mem::transmute::<_, F>(f))() }
        }

        #[cfg(not(any($(target_os = $platform),+)))]
        /// This framework is not available on this platform. Always returns `false`.
        pub fn is_available() -> bool {
            false
        }
    };
}

// ── Build-script helpers ────────────────────────────────────────────────────

/// Build-script helper: emit platform availability warnings.
///
/// Call from `build.rs`:
///
/// ```ignore
/// apple_sys_helpers::build_check_platform("StoreKit", &["macos", "ios", "tvos", "xros", "watchos"]);
/// ```
pub fn build_check_platform(framework_name: &str, supported_os: &[&str]) {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if supported_os.contains(&os.as_str()) {
        println!("cargo:warning={framework_name} framework available");
    } else {
        println!("cargo:warning={framework_name} framework not available on {os}");
    }
}
