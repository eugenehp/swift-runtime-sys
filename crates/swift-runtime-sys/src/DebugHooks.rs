#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime debugging and diagnostics hooks.

use core::ffi::{c_char, c_void};

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;

unsafe extern "C" {
    /// Report to the debugger.
    pub fn swift_reportToDebugger(flags: u32, message: *const c_char, details: *const c_void);

    /// Report a runtime error.
    pub fn swift_reportError(flags: u32, message: *const c_char);

    /// Report a runtime warning.
    pub fn swift_reportWarning(flags: u32, message: *const c_char);

    /// Set whether fatal errors should be reported to the debugger.
    pub fn swift_reportFatalErrorsToDebugger(should_report: bool);

    /// Check whether fatal errors should be reported to the debugger.
    pub fn swift_shouldReportFatalErrorsToDebugger() -> bool;

    /// Hook for all runtime reports. Set via function pointer.
    pub fn swift_runtime_on_report(flags: u32, message: *const c_char, details: *const c_void);

    /// Demangle a Swift symbol name.
    pub fn swift_demangle(
        mangled_name: *const c_char,
        mangled_name_length: usize,
        output_buffer: *mut c_char,
        output_buffer_size: *mut usize,
        flags: u32,
    ) -> *mut c_char;

    /// Find an accessible function by name.
    pub fn swift_findAccessibleFunction(name: *const c_char) -> *const c_void;

    /// Thread-safe once initialization.
    pub fn swift_once(
        token: *mut usize,
        function: unsafe extern "C" fn(*mut c_void),
        context: *mut c_void,
    );

    /// Disable exclusivity checking.
    pub fn swift_disableExclusivityChecking();

    /// Begin a dynamic access (exclusivity enforcement).
    pub fn swift_beginAccess(
        pointer: *mut c_void,
        buffer: *mut c_void,
        flags: u32,
        pc: *const c_void,
    );

    /// End a dynamic access.
    pub fn swift_endAccess(buffer: *mut c_void);

    /// Check if a closure is escaping at a given file location.
    pub fn swift_isEscapingClosureAtFileLocation(
        object: *const c_void,
        filename: *const u8,
        filename_length: i32,
        line: i32,
        column: i32,
        escape_type: u32,
    ) -> bool;
}
