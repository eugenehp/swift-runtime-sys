#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime error handling functions.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift opaque value.
pub type OpaqueValueRef = *mut c_void;
/// Opaque pointer to a Swift error object.
pub type SwiftErrorRef = *mut c_void;
/// Opaque pointer to a witness table.
pub type WitnessTableRef = *const c_void;

/// Result of extracting value from an error object.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ErrorValueResult {
    pub value: *const c_void,
    pub error_type: MetadataRef,
    pub error_conformance: WitnessTableRef,
}

/// Return type for swift_allocError (BoxPair).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoxPair {
    pub object: *mut c_void,
    pub buffer: OpaqueValueRef,
}

unsafe extern "C" {
    /// Allocate a catchable error object.
    pub fn swift_allocError(
        error_type: MetadataRef,
        error_conformance: WitnessTableRef,
        value: OpaqueValueRef,
        is_take: bool,
    ) -> BoxPair;

    /// Deallocate an error object whose contained object has already been destroyed.
    pub fn swift_deallocError(error: SwiftErrorRef, error_type: MetadataRef);

    /// Extract a pointer to the value, the type metadata, and the Error
    /// protocol witness from an error object.
    pub fn swift_getErrorValue(
        error_object: SwiftErrorRef,
        scratch: *mut *mut c_void,
        out: *mut ErrorValueResult,
    );

    /// Retain an error box.
    pub fn swift_errorRetain(object: SwiftErrorRef) -> SwiftErrorRef;

    /// Release an error box.
    pub fn swift_errorRelease(object: SwiftErrorRef);

    /// Called when throwing an error. Serves as a breakpoint hook for debuggers.
    pub fn swift_willThrow(unused: *mut c_void, error: *mut SwiftErrorRef);

    /// Called when throwing a typed error. Serves as a breakpoint hook.
    pub fn swift_willThrowTypedImpl(
        value: OpaqueValueRef,
        error_type: MetadataRef,
        error_conformance: WitnessTableRef,
    );

    /// Called when an error is thrown out of the top level of a script.
    pub fn swift_errorInMain(object: SwiftErrorRef) -> !;

    /// Called when try! fails.
    pub fn swift_unexpectedError(
        object: SwiftErrorRef,
        filename_start: OpaqueValueRef,
        filename_length: isize,
        is_ascii: bool,
        line: usize,
    ) -> !;

    /// Set a handler to be called when an error is about to be thrown.
    pub fn swift_setWillThrowHandler(handler: Option<unsafe extern "C" fn(SwiftErrorRef)>);
}
