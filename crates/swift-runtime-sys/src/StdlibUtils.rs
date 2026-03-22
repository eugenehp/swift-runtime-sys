#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift stdlib utility functions.

use core::ffi::{c_char, c_void};

/// Operating system version tuple.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OSVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

unsafe extern "C" {
    /// Generate random bytes.
    pub fn swift_stdlib_random(buf: *mut c_void, nbytes: usize);

    /// Read a line from stdin.
    pub fn swift_stdlib_readLine_stdin(line_ptr: *mut *mut u8, line_length: *mut usize) -> bool;

    /// Get the hardware concurrency (number of CPUs).
    pub fn swift_stdlib_getHardwareConcurrency() -> usize;

    /// Get the current stack bounds.
    pub fn swift_stdlib_getCurrentStackBounds(begin: *mut *mut c_void, end: *mut *mut c_void);

    /// Check if stack allocation is safe.
    pub fn swift_stdlib_isStackAllocationSafe(size: usize, alignment: usize) -> bool;

    /// Get the operating system version.
    pub fn swift_stdlib_operatingSystemVersion() -> OSVersion;

    /// Immortalize an object.
    pub fn swift_stdlib_immortalize(object: *mut c_void);

    /// Get a description of a value.
    pub fn swift_stdlib_getDescription(
        value: *const c_void,
        metadata: *const c_void,
    ) -> *mut c_void;

    /// Get the default error code.
    pub fn swift_stdlib_getDefaultErrorCode(error: *const c_void) -> isize;

    /// Report a fatal error.
    pub fn swift_stdlib_reportFatalError(
        prefix: *const c_char,
        prefix_length: usize,
        message: *const c_char,
        message_length: usize,
        flags: u32,
    ) -> !;

    /// Report a fatal error with file info.
    pub fn swift_stdlib_reportFatalErrorInFile(
        prefix: *const c_char,
        prefix_length: usize,
        message: *const c_char,
        message_length: usize,
        file: *const c_char,
        file_length: usize,
        line: u32,
        flags: u32,
    ) -> !;

    /// Report an unimplemented initializer.
    pub fn swift_stdlib_reportUnimplementedInitializer(
        class_name: *const c_char,
        class_name_length: usize,
        init_name: *const c_char,
        init_name_length: usize,
        file: *const c_char,
        file_length: usize,
        line: u32,
        column: u32,
    ) -> !;

    /// Report an unimplemented initializer in file.
    pub fn swift_stdlib_reportUnimplementedInitializerInFile(
        class_name: *const c_char,
        class_name_length: usize,
        init_name: *const c_char,
        init_name_length: usize,
        file: *const c_char,
        file_length: usize,
        line: u32,
        column: u32,
    ) -> !;

    /// Write a character to stderr.
    pub fn swift_stdlib_putc_stderr(c: u32);

    /// Lock stdout.
    pub fn swift_stdlib_flockfile_stdout();

    /// Unlock stdout.
    pub fn swift_stdlib_funlockfile_stdout();

    /// Override unsafe argv/argc.
    pub fn swift_stdlib_overrideUnsafeArgvArgc(argv: *const *const c_char, argc: i32);

    /// Parse a double from a C string using the C locale.
    pub fn swift_stdlib_strtod_clocale(str: *const c_char, end: *mut *mut c_char) -> f64;

    /// Parse a float from a C string using the C locale.
    pub fn swift_stdlib_strtof_clocale(str: *const c_char, end: *mut *mut c_char) -> f32;

    /// Parse a float16 from a C string using the C locale.
    pub fn swift_stdlib_strtof16_clocale(str: *const c_char, end: *mut *mut c_char) -> u16;

    /// Parse a long double from a C string using the C locale.
    pub fn swift_stdlib_strtold_clocale(str: *const c_char, end: *mut *mut c_char) -> f64; // long double varies by platform

    /// Hashing parameters.
    pub static swift_stdlib_Hashing_parameters: [u64; 2];

    /// Check if a value is an NSString.
    pub fn swift_stdlib_isNSString(object: *const c_void) -> bool;

    /// Connect NSBase classes.
    pub fn swift_stdlib_connectNSBaseClasses();
}
