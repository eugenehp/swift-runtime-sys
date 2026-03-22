#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime path query functions.

use core::ffi::c_char;

unsafe extern "C" {
    /// Get the root path of the Swift installation.
    pub fn swift_getRootPath() -> *const c_char;

    /// Get the path to the Swift runtime library.
    pub fn swift_getRuntimeLibraryPath() -> *const c_char;

    /// Copy the path to an auxiliary executable.
    pub fn swift_copyAuxiliaryExecutablePath(
        name: *const c_char,
    ) -> *mut c_char;
}
