#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime witness table and protocol conformance operations.

use core::ffi::c_void;

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a witness table.
pub type WitnessTableRef = *const c_void;
/// Opaque pointer to a relative witness table.
pub type RelativeWitnessTableRef = *const c_void;
/// Opaque pointer to a protocol conformance descriptor.
pub type ProtocolConformanceDescriptorRef = *const c_void;
/// Opaque pointer to a protocol requirement.
pub type ProtocolRequirementRef = *const c_void;

/// Return type for metadata requests (MetadataResponse).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetadataResponse {
    pub metadata: MetadataRef,
    pub state: usize,
}

/// A metadata request (desired state).
pub type MetadataRequest = usize;

unsafe extern "C" {
    /// Retrieve a witness table based on a conformance descriptor.
    pub fn swift_getWitnessTable(
        conformance: ProtocolConformanceDescriptorRef,
        metadata: MetadataRef,
        instantiation_args: *const *const c_void,
    ) -> WitnessTableRef;

    /// Retrieve a relative witness table based on a conformance descriptor.
    pub fn swift_getWitnessTableRelative(
        conformance: ProtocolConformanceDescriptorRef,
        metadata: MetadataRef,
        instantiation_args: *const *const c_void,
    ) -> RelativeWitnessTableRef;

    /// Retrieve an associated type witness from a witness table.
    pub fn swift_getAssociatedTypeWitness(
        request: MetadataRequest,
        wtable: WitnessTableRef,
        conforming_type: MetadataRef,
        req_base: ProtocolRequirementRef,
        assoc_type: ProtocolRequirementRef,
    ) -> MetadataResponse;

    /// Retrieve an associated type witness from a relative witness table.
    pub fn swift_getAssociatedTypeWitnessRelative(
        request: MetadataRequest,
        wtable: RelativeWitnessTableRef,
        conforming_type: MetadataRef,
        req_base: ProtocolRequirementRef,
        assoc_type: ProtocolRequirementRef,
    ) -> MetadataResponse;

    /// Retrieve an associated conformance witness from a witness table.
    pub fn swift_getAssociatedConformanceWitness(
        wtable: WitnessTableRef,
        conforming_type: MetadataRef,
        assoc_type: MetadataRef,
        req_base: ProtocolRequirementRef,
        assoc_conformance: ProtocolRequirementRef,
    ) -> WitnessTableRef;

    /// Retrieve an associated conformance witness from a relative witness table.
    pub fn swift_getAssociatedConformanceWitnessRelative(
        wtable: RelativeWitnessTableRef,
        conforming_type: MetadataRef,
        assoc_type: MetadataRef,
        req_base: ProtocolRequirementRef,
        assoc_conformance: ProtocolRequirementRef,
    ) -> WitnessTableRef;

    /// Register protocol conformance records.
    pub fn swift_registerProtocolConformances(
        begin: *const c_void,
        end: *const c_void,
    );

    /// Register protocol records.
    pub fn swift_registerProtocols(
        begin: *const c_void,
        end: *const c_void,
    );

    /// Register type metadata records.
    pub fn swift_registerTypeMetadataRecords(
        begin: *const c_void,
        end: *const c_void,
    );

    /// Compare two protocol conformance descriptors.
    pub fn swift_compareProtocolConformanceDescriptors(
        lhs: ProtocolConformanceDescriptorRef,
        rhs: ProtocolConformanceDescriptorRef,
    ) -> bool;

    /// Compare two type context descriptors.
    pub fn swift_compareTypeContextDescriptors(
        lhs: *const c_void,
        rhs: *const c_void,
    ) -> bool;

    /// Compare two witness tables.
    pub fn swift_compareWitnessTables(
        lhs: WitnessTableRef,
        rhs: WitnessTableRef,
    ) -> bool;

    /// Check protocol conformance (v2).
    pub fn swift_conformsToProtocol2(
        metadata: MetadataRef,
        protocol: *const c_void,
    ) -> WitnessTableRef;

    /// Check protocol conformance (common).
    pub fn swift_conformsToProtocolCommon(
        metadata: MetadataRef,
        protocol: *const c_void,
    ) -> WitnessTableRef;

    /// Check protocol conformance with execution context.
    pub fn swift_conformsToProtocolWithExecutionContext(
        metadata: MetadataRef,
        protocol: *const c_void,
        context: *mut c_void,
    ) -> WitnessTableRef;

    /// Check if in a conformance execution context.
    pub fn swift_isInConformanceExecutionContext() -> bool;

    /// Size of the conformance execution context.
    pub static swift_ConformanceExecutionContextSize: usize;
}
