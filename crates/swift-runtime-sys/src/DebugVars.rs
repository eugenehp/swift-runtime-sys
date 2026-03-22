#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime debug variables.
//! These are global variables used by debugging tools. Access via dlsym.

use core::ffi::c_void;

unsafe extern "C" {
    /// Pointer to the metadata allocation pool.
    pub static swift_debug_allocationPoolPointer: *const c_void;

    /// Pointer to the metadata allocation backtrace list.
    pub static swift_debug_metadataAllocationBacktraceList: *const c_void;

    /// Whether metadata allocation iteration is enabled.
    pub static swift_debug_metadataAllocationIterationEnabled: bool;

    /// The multi-payload enum pointer spare bits mask.
    pub static swift_debug_multiPayloadEnumPointerSpareBitsMask: usize;

    /// Pointer to the protocol conformance state.
    pub static swift_debug_protocolConformanceStatePointer: *const c_void;
}
