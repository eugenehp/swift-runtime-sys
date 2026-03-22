//! Auto-discovery and loading of the Swift helper dylib.
//!
//! The helper is found by searching:
//! 1. `SWIFTUI_HELPER` environment variable
//! 2. `swift_helper/libSwiftUIHelper.dylib` (relative to cwd)
//! 3. `../../swift_helper/libSwiftUIHelper.dylib` (from examples/tests)
//! 4. Next to the current executable
//!
//! Usage: just call `app()` or `App::new()` — loading is automatic.

use core::ffi::{c_char, c_void};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
}

static HELPER_PATH: OnceLock<PathBuf> = OnceLock::new();

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

    // 2. Common relative paths
    let candidates = [
        "swift_helper/libSwiftUIHelper.dylib",
        "../../swift_helper/libSwiftUIHelper.dylib",
        "../swift_helper/libSwiftUIHelper.dylib",
        "libSwiftUIHelper.dylib",
    ];
    for c in candidates {
        if Path::new(c).exists() {
            return Some(PathBuf::from(c));
        }
    }

    // 3. Next to executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("libSwiftUIHelper.dylib");
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
                     Or set SWIFTUI_HELPER=/path/to/libSwiftUIHelper.dylib"
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
