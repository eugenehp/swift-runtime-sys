//! Auto-discovery and loading of the Swift helper dylib.
//!
//! The helper is found by searching:
//! 1. `SWIFTUI_HELPER` environment variable
//! 2. Platform-specific library name in common locations
//! 3. Next to the current executable
//!
//! **Platform notes:**
//! - macOS: loads `libSwiftUIHelper.dylib`
//! - iOS/tvOS/visionOS: loads `SwiftUIHelper.framework/SwiftUIHelper` or
//!   `libSwiftUIHelper.dylib` (embedded in app bundle)
//!
//! Usage: just call `app()` or `App::new()` — loading is automatic.

use core::ffi::{c_char, c_void};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
}

static HELPER_PATH: OnceLock<PathBuf> = OnceLock::new();

/// The library file name for the current platform.
#[cfg(target_os = "macos")]
const LIB_NAME: &str = "libSwiftUIHelper.dylib";

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "xros"))]
const LIB_NAME: &str = "libSwiftUIHelper.dylib";

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "xros"))]
const FRAMEWORK_NAME: &str = "SwiftUIHelper.framework/SwiftUIHelper";

/// Find the Swift helper dylib, searching multiple locations.
pub fn find_helper() -> Option<PathBuf> {
    // 1. Compile-time env from build.rs
    if let Some(p) = option_env!("SWIFTUI_HELPER") {
        if Path::new(p).exists() {
            return Some(PathBuf::from(p));
        }
    }

    // 2. Runtime environment variable
    if let Ok(p) = std::env::var("SWIFTUI_HELPER") {
        if Path::new(&p).exists() {
            return Some(PathBuf::from(p));
        }
    }

    // 3. Common relative paths (dylib)
    let dylib_candidates = [
        &format!("swift_helper/{LIB_NAME}"),
        &format!("../../swift_helper/{LIB_NAME}"),
        &format!("../swift_helper/{LIB_NAME}"),
        LIB_NAME,
    ];
    for c in &dylib_candidates {
        if Path::new(c).exists() {
            return Some(PathBuf::from(c));
        }
    }

    // 4. Framework bundle paths (iOS/tvOS/visionOS)
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "xros"))]
    {
        let framework_candidates = [
            &format!("Frameworks/{FRAMEWORK_NAME}"),
            &format!("../Frameworks/{FRAMEWORK_NAME}"),
            FRAMEWORK_NAME,
        ];
        for c in &framework_candidates {
            if Path::new(c).exists() {
                return Some(PathBuf::from(c));
            }
        }

        // Inside app bundle: look for Frameworks/ relative to the executable
        if let Ok(exe) = std::env::current_exe() {
            if let Some(bundle_dir) = exe.parent() {
                let fw = bundle_dir.join("Frameworks").join(FRAMEWORK_NAME);
                if fw.exists() {
                    return Some(fw);
                }
            }
        }
    }

    // 5. Next to executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join(LIB_NAME);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

/// Get the cached helper path, finding it on first call.
pub fn helper_path() -> &'static Path {
    HELPER_PATH
        .get_or_init(|| {
            find_helper().unwrap_or_else(|| {
                panic!(
                    "Swift helper not found. Build it:\n  \
                     cd swift_helper && ./build.sh\n\n\
                     Or set SWIFTUI_HELPER=/path/to/{LIB_NAME}"
                )
            })
        })
        .as_path()
}

/// Load the helper dylib (idempotent).
pub fn ensure_loaded() {
    let path = helper_path();
    let cpath = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        dlopen(cpath.as_ptr(), 2);
    }
}
