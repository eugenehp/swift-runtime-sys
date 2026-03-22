#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime opaque type support.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a witness table.
pub type WitnessTableRef = *const c_void;

/// Return type for metadata requests.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetadataResponse {
    pub metadata: MetadataRef,
    pub state: usize,
}

pub type MetadataRequest = usize;

unsafe extern "C" {
    /// Get opaque type metadata.
    pub fn swift_getOpaqueTypeMetadata(
        request: MetadataRequest,
        arguments: *const *const c_void,
        descriptor: *const c_void,
        index: usize,
    ) -> MetadataResponse;

    /// Get opaque type metadata (v2).
    pub fn swift_getOpaqueTypeMetadata2(
        request: MetadataRequest,
        arguments: *const *const c_void,
        descriptor: *const c_void,
        index: usize,
    ) -> MetadataResponse;

    /// Get opaque type conformance.
    pub fn swift_getOpaqueTypeConformance(
        arguments: *const *const c_void,
        descriptor: *const c_void,
        index: usize,
    ) -> WitnessTableRef;

    /// Get opaque type conformance (v2).
    pub fn swift_getOpaqueTypeConformance2(
        arguments: *const *const c_void,
        descriptor: *const c_void,
        index: usize,
    ) -> WitnessTableRef;
}
