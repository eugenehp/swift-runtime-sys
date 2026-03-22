#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! C-ABI thunk wrappers for Swift-CC functions.
//!
//! On arm64, functions declared with `SWIFT_CC(swift)` use a different calling
//! convention than C. This module provides safe wrappers that use dlsym to
//! resolve the functions and call them through architecture-appropriate thunks.
//!
//! On x86_64, Swift CC is identical to C CC, so these wrappers work directly.
//! On arm64, MetadataResponse-returning functions return (x0, x1) in Swift CC
//! but would need an x8 return pointer in C CC. We handle this by using
//! inline assembly or by splitting the return value.

use core::ffi::{c_char, c_void, CStr};

const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// Error type for thunk resolution failures.
#[derive(Debug)]
pub enum ThunkError {
    SymbolNotFound(String),
}

/// Resolved metadata response (always safe to use from Rust).
#[derive(Debug, Clone, Copy)]
pub struct MetadataResponse {
    pub metadata: *const c_void,
    pub state: usize,
}

fn resolve(name: &CStr) -> Result<*const c_void, ThunkError> {
    let ptr = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) };
    if ptr.is_null() {
        Err(ThunkError::SymbolNotFound(
            name.to_string_lossy().into_owned(),
        ))
    } else {
        Ok(ptr as *const c_void)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MetadataResponse-returning thunks
// ═══════════════════════════════════════════════════════════════════════════

/// On arm64 with Swift CC, MetadataResponse is returned in (x0, x1).
/// We use a 2-register return type to match this.
#[cfg(target_arch = "aarch64")]
mod arch {
    use super::*;

    /// Call a Swift-CC function that returns MetadataResponse with 2 args.
    /// Signature: (MetadataRequest, *const c_void) -> MetadataResponse
    pub unsafe fn call_metadata_response_2(
        func: *const c_void,
        arg0: usize,
        arg1: *const c_void,
    ) -> MetadataResponse {
        let metadata: *const c_void;
        let state: usize;
        // Swift CC on arm64: args in x0, x1; return in x0, x1
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") arg0,
            in("x1") arg1,
            lateout("x0") metadata,
            lateout("x1") state,
            // Clobber all caller-saved registers
            lateout("x2") _,
            lateout("x3") _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        MetadataResponse { metadata, state }
    }

    /// Call a Swift-CC function that returns MetadataResponse with 3 args.
    pub unsafe fn call_metadata_response_3(
        func: *const c_void,
        arg0: usize,
        arg1: *const c_void,
        arg2: *const c_void,
    ) -> MetadataResponse {
        let metadata: *const c_void;
        let state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") arg0,
            in("x1") arg1,
            in("x2") arg2,
            lateout("x0") metadata,
            lateout("x1") state,
            lateout("x3") _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        MetadataResponse { metadata, state }
    }

    /// Call a Swift-CC function that returns MetadataResponse with 4 args.
    pub unsafe fn call_metadata_response_4(
        func: *const c_void,
        arg0: usize,
        arg1: *const c_void,
        arg2: *const c_void,
        arg3: *const c_void,
    ) -> MetadataResponse {
        let metadata: *const c_void;
        let state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") arg0,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            lateout("x0") metadata,
            lateout("x1") state,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        MetadataResponse { metadata, state }
    }

    /// Call swift_getTypeName (Swift CC, returns two-word TypeNamePair in x0,x1).
    pub unsafe fn call_type_name_pair(
        func: *const c_void,
        metadata: *const c_void,
        qualified: bool,
    ) -> (*const u8, usize) {
        let data: *const u8;
        let length: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") metadata,
            in("x1") qualified as usize,
            lateout("x0") data,
            lateout("x1") length,
            lateout("x2") _,
            lateout("x3") _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        (data, length)
    }
}

#[cfg(target_arch = "x86_64")]
mod arch {
    use super::*;

    // On x86_64, Swift CC == C CC, so we can use transmute.

    pub unsafe fn call_metadata_response_2(
        func: *const c_void,
        arg0: usize,
        arg1: *const c_void,
    ) -> MetadataResponse {
        type F = unsafe extern "C" fn(usize, *const c_void) -> MetadataResponse;
        let f: F = core::mem::transmute(func);
        f(arg0, arg1)
    }

    pub unsafe fn call_metadata_response_3(
        func: *const c_void,
        arg0: usize,
        arg1: *const c_void,
        arg2: *const c_void,
    ) -> MetadataResponse {
        type F = unsafe extern "C" fn(usize, *const c_void, *const c_void) -> MetadataResponse;
        let f: F = core::mem::transmute(func);
        f(arg0, arg1, arg2)
    }

    pub unsafe fn call_metadata_response_4(
        func: *const c_void,
        arg0: usize,
        arg1: *const c_void,
        arg2: *const c_void,
        arg3: *const c_void,
    ) -> MetadataResponse {
        type F = unsafe extern "C" fn(
            usize,
            *const c_void,
            *const c_void,
            *const c_void,
        ) -> MetadataResponse;
        let f: F = core::mem::transmute(func);
        f(arg0, arg1, arg2, arg3)
    }

    pub unsafe fn call_type_name_pair(
        func: *const c_void,
        metadata: *const c_void,
        qualified: bool,
    ) -> (*const u8, usize) {
        #[repr(C)]
        struct Pair(*const u8, usize);
        type F = unsafe extern "C" fn(*const c_void, bool) -> Pair;
        let f: F = core::mem::transmute(func);
        let p = f(metadata, qualified);
        (p.0, p.1)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Public safe thunk API
// ═══════════════════════════════════════════════════════════════════════════

/// Safely call `swift_getSingletonMetadata` (Swift CC).
pub unsafe fn swift_getSingletonMetadata(
    request: usize,
    description: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getSingletonMetadata")?;
    Ok(arch::call_metadata_response_2(func, request, description))
}

/// Safely call `swift_getGenericMetadata` (Swift CC).
pub unsafe fn swift_getGenericMetadata(
    request: usize,
    arguments: *const *const c_void,
    description: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getGenericMetadata")?;
    Ok(arch::call_metadata_response_3(
        func,
        request,
        arguments as _,
        description,
    ))
}

/// Safely call `swift_checkMetadataState` (Swift CC).
pub unsafe fn swift_checkMetadataState(
    request: usize,
    metadata: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_checkMetadataState")?;
    Ok(arch::call_metadata_response_2(func, request, metadata))
}

/// Safely call `swift_getForeignTypeMetadata` (Swift CC).
pub unsafe fn swift_getForeignTypeMetadata(
    request: usize,
    non_unique: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getForeignTypeMetadata")?;
    Ok(arch::call_metadata_response_2(func, request, non_unique))
}

/// Safely call `swift_getFixedArrayTypeMetadata` (Swift CC).
pub unsafe fn swift_getFixedArrayTypeMetadata(
    request: usize,
    count: usize,
    element: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getFixedArrayTypeMetadata")?;
    Ok(arch::call_metadata_response_3(
        func, request, count as _, element,
    ))
}

/// Safely call `swift_getTupleTypeMetadata2` (Swift CC).
pub unsafe fn swift_getTupleTypeMetadata2(
    request: usize,
    elt0: *const c_void,
    elt1: *const c_void,
    labels: *const c_char,
    proposed: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getTupleTypeMetadata2")?;
    // This has 5 args; we need a 5-arg variant. On arm64, we use asm directly.
    #[cfg(target_arch = "aarch64")]
    {
        let metadata: *const c_void;
        let state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") request,
            in("x1") elt0,
            in("x2") elt1,
            in("x3") labels,
            in("x4") proposed,
            lateout("x0") metadata,
            lateout("x1") state,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        Ok(MetadataResponse { metadata, state })
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            usize,
            *const c_void,
            *const c_void,
            *const c_char,
            *const c_void,
        ) -> MetadataResponse;
        let f: F = core::mem::transmute(func);
        Ok(f(request, elt0, elt1, labels, proposed))
    }
}

/// Safely call `swift_getTypeName` (Swift CC, returns TypeNamePair).
pub unsafe fn swift_getTypeName(
    metadata: *const c_void,
    qualified: bool,
) -> Result<(&'static str, usize), ThunkError> {
    let func = resolve(c"swift_getTypeName")?;
    let (data, length) = arch::call_type_name_pair(func, metadata, qualified);
    if data.is_null() || length == 0 {
        Ok(("", 0))
    } else {
        let slice = core::slice::from_raw_parts(data, length);
        let s = core::str::from_utf8_unchecked(slice);
        Ok((s, length))
    }
}

/// Safely call `swift_getMangledTypeName` (Swift CC, returns TypeNamePair).
pub unsafe fn swift_getMangledTypeName(
    metadata: *const c_void,
) -> Result<(*const u8, usize), ThunkError> {
    let func = resolve(c"swift_getMangledTypeName")?;
    Ok(arch::call_type_name_pair(func, metadata, false))
}

/// Safely call `swift_getTypeContextDescriptor` (Swift CC).
pub unsafe fn swift_getTypeContextDescriptor(
    metadata: *const c_void,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_getTypeContextDescriptor")?;
    // This returns a single pointer, which is the same in both CCs.
    type F = unsafe extern "C" fn(*const c_void) -> *const c_void;
    let f: F = core::mem::transmute(func);
    Ok(f(metadata))
}

/// Safely call `swift_getCanonicalSpecializedMetadata` (Swift CC).
pub unsafe fn swift_getCanonicalSpecializedMetadata(
    request: usize,
    candidate: *const c_void,
    cache: *mut *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getCanonicalSpecializedMetadata")?;
    Ok(arch::call_metadata_response_3(
        func, request, candidate, cache as _,
    ))
}

/// Safely call `swift_getCanonicalPrespecializedGenericMetadata` (Swift CC).
pub unsafe fn swift_getCanonicalPrespecializedGenericMetadata(
    request: usize,
    arguments: *const *const c_void,
    description: *const c_void,
    token: *mut usize,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getCanonicalPrespecializedGenericMetadata")?;
    Ok(arch::call_metadata_response_4(
        func,
        request,
        arguments as _,
        description,
        token as _,
    ))
}

/// Safely call `swift_getAssociatedTypeWitness` (Swift CC, 5 args).
pub unsafe fn swift_getAssociatedTypeWitness(
    request: usize,
    wtable: *const c_void,
    conforming_type: *const c_void,
    req_base: *const c_void,
    assoc_type: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getAssociatedTypeWitness")?;
    #[cfg(target_arch = "aarch64")]
    {
        let metadata: *const c_void;
        let state: usize;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") request,
            in("x1") wtable,
            in("x2") conforming_type,
            in("x3") req_base,
            in("x4") assoc_type,
            lateout("x0") metadata,
            lateout("x1") state,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        Ok(MetadataResponse { metadata, state })
    }
    #[cfg(target_arch = "x86_64")]
    {
        type F = unsafe extern "C" fn(
            usize,
            *const c_void,
            *const c_void,
            *const c_void,
            *const c_void,
        ) -> MetadataResponse;
        let f: F = core::mem::transmute(func);
        Ok(f(request, wtable, conforming_type, req_base, assoc_type))
    }
}

/// Safely call `swift_allocBox` (Swift CC, returns BoxPair in x0,x1).
pub unsafe fn swift_allocBox(
    metadata: *const c_void,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    let func = resolve(c"swift_allocBox")?;
    #[cfg(target_arch = "aarch64")]
    {
        let object: *mut c_void;
        let buffer: *mut c_void;
        core::arch::asm!(
            "blr {func}",
            func = in(reg) func,
            in("x0") metadata,
            lateout("x0") object,
            lateout("x1") buffer,
            lateout("x2") _,
            lateout("x3") _,
            lateout("x4") _,
            lateout("x5") _,
            lateout("x6") _,
            lateout("x7") _,
            lateout("x8") _,
            lateout("x9") _,
            lateout("x10") _,
            lateout("x11") _,
            lateout("x12") _,
            lateout("x13") _,
            lateout("x14") _,
            lateout("x15") _,
            lateout("x16") _,
            lateout("x17") _,
            lateout("lr") _,
            clobber_abi("C"),
        );
        Ok((object, buffer))
    }
    #[cfg(target_arch = "x86_64")]
    {
        #[repr(C)]
        struct BoxPair(*mut c_void, *mut c_void);
        type F = unsafe extern "C" fn(*const c_void) -> BoxPair;
        let f: F = core::mem::transmute(func);
        let p = f(metadata);
        Ok((p.0, p.1))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Additional Swift-CC thunks (remaining from plan)
// ═══════════════════════════════════════════════════════════════════════════

/// Safely call `swift_getAssociatedTypeWitnessRelative` (Swift CC, 5 args).
pub unsafe fn swift_getAssociatedTypeWitnessRelative(
    request: usize,
    wtable: *const c_void,
    conforming_type: *const c_void,
    req_base: *const c_void,
    assoc_type: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getAssociatedTypeWitnessRelative")?;
    Ok(call_swift_cc_5_to_mr(
        func,
        request,
        wtable,
        conforming_type,
        req_base,
        assoc_type,
    ))
}

/// Safely call `swift_getAssociatedConformanceWitness` (Swift CC, 5 args -> ptr).
pub unsafe fn swift_getAssociatedConformanceWitness(
    wtable: *const c_void,
    conforming_type: *const c_void,
    assoc_type: *const c_void,
    req_base: *const c_void,
    assoc_conformance: *const c_void,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_getAssociatedConformanceWitness")?;
    Ok(call_swift_cc_5_to_ptr(
        func,
        wtable,
        conforming_type,
        assoc_type,
        req_base,
        assoc_conformance,
    ))
}

/// Safely call `swift_getAssociatedConformanceWitnessRelative`.
pub unsafe fn swift_getAssociatedConformanceWitnessRelative(
    wtable: *const c_void,
    conforming_type: *const c_void,
    assoc_type: *const c_void,
    req_base: *const c_void,
    assoc_conformance: *const c_void,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_getAssociatedConformanceWitnessRelative")?;
    Ok(call_swift_cc_5_to_ptr(
        func,
        wtable,
        conforming_type,
        assoc_type,
        req_base,
        assoc_conformance,
    ))
}

/// Safely call `swift_compareTypeContextDescriptors` (Swift CC, bool return).
pub unsafe fn swift_compareTypeContextDescriptors(
    lhs: *const c_void,
    rhs: *const c_void,
) -> Result<bool, ThunkError> {
    let func = resolve(c"swift_compareTypeContextDescriptors")?;
    Ok(call_swift_cc_2_to_bool(func, lhs, rhs))
}

/// Safely call `swift_compareWitnessTables` (Swift CC, bool return).
pub unsafe fn swift_compareWitnessTables(
    lhs: *const c_void,
    rhs: *const c_void,
) -> Result<bool, ThunkError> {
    let func = resolve(c"swift_compareWitnessTables")?;
    Ok(call_swift_cc_2_to_bool(func, lhs, rhs))
}

/// Safely call `swift_compareProtocolConformanceDescriptors` (Swift CC, bool return).
pub unsafe fn swift_compareProtocolConformanceDescriptors(
    lhs: *const c_void,
    rhs: *const c_void,
) -> Result<bool, ThunkError> {
    let func = resolve(c"swift_compareProtocolConformanceDescriptors")?;
    Ok(call_swift_cc_2_to_bool(func, lhs, rhs))
}

/// Safely call `swift_allocateMetadataPack` (Swift CC).
pub unsafe fn swift_allocateMetadataPack(
    elements: *const *const c_void,
    count: usize,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_allocateMetadataPack")?;
    Ok(call_swift_cc_2_to_ptr(func, elements as _, count as _))
}

/// Safely call `swift_allocateWitnessTablePack` (Swift CC).
pub unsafe fn swift_allocateWitnessTablePack(
    tables: *const *const c_void,
    count: usize,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_allocateWitnessTablePack")?;
    Ok(call_swift_cc_2_to_ptr(func, tables as _, count as _))
}

/// Safely call `swift_getTupleTypeMetadata` (Swift CC, 5 args).
pub unsafe fn swift_getTupleTypeMetadata(
    request: usize,
    flags: usize,
    elements: *const *const c_void,
    labels: *const c_char,
    proposed: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getTupleTypeMetadata")?;
    Ok(call_swift_cc_5_to_mr(
        func,
        request,
        flags as _,
        elements as _,
        labels as _,
        proposed,
    ))
}

/// Safely call `swift_getTupleTypeMetadata3` (Swift CC, 6 args).
pub unsafe fn swift_getTupleTypeMetadata3(
    request: usize,
    elt0: *const c_void,
    elt1: *const c_void,
    elt2: *const c_void,
    labels: *const c_char,
    proposed: *const c_void,
) -> Result<MetadataResponse, ThunkError> {
    let func = resolve(c"swift_getTupleTypeMetadata3")?;
    Ok(call_swift_cc_6_to_mr(
        func,
        request,
        elt0,
        elt1,
        elt2,
        labels as _,
        proposed,
    ))
}

/// Safely call `swift_initClassMetadata2` (Swift CC, 5 args, void return).
pub unsafe fn swift_initClassMetadata2(
    metadata: *const c_void,
    flags: usize,
    num_fields: usize,
    field_types: *const *const c_void,
    field_offsets: *mut usize,
) -> Result<(), ThunkError> {
    let func = resolve(c"swift_initClassMetadata2")?;
    call_swift_cc_5_void(
        func,
        metadata,
        flags as _,
        num_fields as _,
        field_types as _,
        field_offsets as _,
    );
    Ok(())
}

/// Safely call `swift_updateClassMetadata2` (Swift CC, 5 args, void return).
pub unsafe fn swift_updateClassMetadata2(
    metadata: *const c_void,
    flags: usize,
    num_fields: usize,
    field_types: *const *const c_void,
    field_offsets: *mut usize,
) -> Result<(), ThunkError> {
    let func = resolve(c"swift_updateClassMetadata2")?;
    call_swift_cc_5_void(
        func,
        metadata,
        flags as _,
        num_fields as _,
        field_types as _,
        field_offsets as _,
    );
    Ok(())
}

/// Safely call `swift_conformsToProtocolCommon` (Swift CC, returns WitnessTable*).
pub unsafe fn swift_conformsToProtocolCommon(
    metadata: *const c_void,
    protocol: *const c_void,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_conformsToProtocolCommon")?;
    Ok(call_swift_cc_2_to_ptr(func, metadata, protocol))
}

/// Safely call `swift_conformsToProtocolWithExecutionContext` (Swift CC, 3 args).
pub unsafe fn swift_conformsToProtocolWithExecutionContext(
    metadata: *const c_void,
    protocol: *const c_void,
    context: *mut c_void,
) -> Result<*const c_void, ThunkError> {
    let func = resolve(c"swift_conformsToProtocolWithExecutionContext")?;
    Ok(call_swift_cc_3_to_ptr(func, metadata, protocol, context))
}

/// Safely call `swift_allocError` (Swift CC, returns BoxPair).
pub unsafe fn swift_allocError(
    error_type: *const c_void,
    conformance: *const c_void,
    value: *mut c_void,
    is_take: bool,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    let func = resolve(c"swift_allocError")?;
    call_swift_cc_4_to_pair(func, error_type, conformance, value, is_take as usize as _)
}

/// Safely call `swift_makeBoxUnique` (Swift CC, returns BoxPair).
pub unsafe fn swift_makeBoxUnique(
    buffer: *mut c_void,
    metadata: *const c_void,
    align_mask: usize,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    let func = resolve(c"swift_makeBoxUnique")?;
    call_swift_cc_3_to_pair(func, buffer, metadata, align_mask as _)
}

/// Safely call `swift_getEnumTagSinglePayloadGeneric` (Swift CC, 4 args -> u32).
pub unsafe fn swift_getEnumTagSinglePayloadGeneric(
    value: *const c_void,
    empty_cases: u32,
    payload_type: *const c_void,
    get_tag: *const c_void,
) -> Result<u32, ThunkError> {
    let func = resolve(c"swift_getEnumTagSinglePayloadGeneric")?;
    Ok(call_swift_cc_4_to_u32(
        func,
        value,
        empty_cases as usize as _,
        payload_type,
        get_tag,
    ))
}

/// Safely call `swift_storeEnumTagSinglePayloadGeneric` (Swift CC, 5 args, void).
pub unsafe fn swift_storeEnumTagSinglePayloadGeneric(
    value: *mut c_void,
    which_case: u32,
    empty_cases: u32,
    payload_type: *const c_void,
    store_tag: *const c_void,
) -> Result<(), ThunkError> {
    let func = resolve(c"swift_storeEnumTagSinglePayloadGeneric")?;
    call_swift_cc_5_void(
        func,
        value,
        which_case as usize as _,
        empty_cases as usize as _,
        payload_type,
        store_tag,
    );
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Generic call helpers
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_5_to_mr(
    func: *const c_void,
    a0: usize,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
) -> MetadataResponse {
    let m: *const c_void;
    let s: usize;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3, in("x4") a4,
        lateout("x0") m, lateout("x1") s,
        lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    MetadataResponse {
        metadata: m,
        state: s,
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_5_to_mr(
    func: *const c_void,
    a0: usize,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
) -> MetadataResponse {
    type F = unsafe extern "C" fn(
        usize,
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
    ) -> MetadataResponse;
    let f: F = core::mem::transmute(func);
    f(a0, a1, a2, a3, a4)
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_6_to_mr(
    func: *const c_void,
    a0: usize,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
    a5: *const c_void,
) -> MetadataResponse {
    let m: *const c_void;
    let s: usize;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3, in("x4") a4, in("x5") a5,
        lateout("x0") m, lateout("x1") s,
        lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    MetadataResponse {
        metadata: m,
        state: s,
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_6_to_mr(
    func: *const c_void,
    a0: usize,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
    a5: *const c_void,
) -> MetadataResponse {
    type F = unsafe extern "C" fn(
        usize,
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
    ) -> MetadataResponse;
    let f: F = core::mem::transmute(func);
    f(a0, a1, a2, a3, a4, a5)
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_5_to_ptr(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
) -> *const c_void {
    let r: *const c_void;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3, in("x4") a4,
        lateout("x0") r,
        lateout("x1") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_5_to_ptr(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
) -> *const c_void {
    type F = unsafe extern "C" fn(
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
    ) -> *const c_void;
    (core::mem::transmute::<_, F>(func))(a0, a1, a2, a3, a4)
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_2_to_bool(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
) -> bool {
    let r: usize;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1,
        lateout("x0") r,
        lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
        lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r != 0
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_2_to_bool(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
) -> bool {
    type F = unsafe extern "C" fn(*const c_void, *const c_void) -> bool;
    (core::mem::transmute::<_, F>(func))(a0, a1)
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_2_to_ptr(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
) -> *const c_void {
    let r: *const c_void;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1,
        lateout("x0") r,
        lateout("x2") _, lateout("x3") _, lateout("x4") _, lateout("x5") _,
        lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_2_to_ptr(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
) -> *const c_void {
    type F = unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void;
    (core::mem::transmute::<_, F>(func))(a0, a1)
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_3_to_ptr(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
) -> *const c_void {
    let r: *const c_void;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2,
        lateout("x0") r,
        lateout("x3") _, lateout("x4") _, lateout("x5") _,
        lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_3_to_ptr(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
) -> *const c_void {
    type F = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void) -> *const c_void;
    (core::mem::transmute::<_, F>(func))(a0, a1, a2)
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_5_void(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
) {
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3, in("x4") a4,
        lateout("x0") _, lateout("x1") _,
        lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_5_void(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
    a4: *const c_void,
) {
    type F = unsafe extern "C" fn(
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
        *const c_void,
    );
    (core::mem::transmute::<_, F>(func))(a0, a1, a2, a3, a4);
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_4_to_pair(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    let r0: *mut c_void;
    let r1: *mut c_void;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3,
        lateout("x0") r0, lateout("x1") r1,
        lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    Ok((r0, r1))
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_4_to_pair(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    #[repr(C)]
    struct P(*mut c_void, *mut c_void);
    type F = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void, *const c_void) -> P;
    let p = (core::mem::transmute::<_, F>(func))(a0, a1, a2, a3);
    Ok((p.0, p.1))
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_3_to_pair(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    let r0: *mut c_void;
    let r1: *mut c_void;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2,
        lateout("x0") r0, lateout("x1") r1,
        lateout("x3") _, lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    Ok((r0, r1))
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_3_to_pair(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
) -> Result<(*mut c_void, *mut c_void), ThunkError> {
    #[repr(C)]
    struct P(*mut c_void, *mut c_void);
    type F = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void) -> P;
    let p = (core::mem::transmute::<_, F>(func))(a0, a1, a2);
    Ok((p.0, p.1))
}

#[cfg(target_arch = "aarch64")]
unsafe fn call_swift_cc_4_to_u32(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
) -> u32 {
    let r: usize;
    core::arch::asm!("blr {f}", f = in(reg) func,
        in("x0") a0, in("x1") a1, in("x2") a2, in("x3") a3,
        lateout("x0") r,
        lateout("x1") _, lateout("x4") _, lateout("x5") _, lateout("x6") _, lateout("x7") _, lateout("x8") _,
        lateout("x9") _, lateout("x10") _, lateout("x11") _, lateout("x12") _,
        lateout("x13") _, lateout("x14") _, lateout("x15") _, lateout("x16") _,
        lateout("x17") _, lateout("lr") _, clobber_abi("C"),
    );
    r as u32
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_swift_cc_4_to_u32(
    func: *const c_void,
    a0: *const c_void,
    a1: *const c_void,
    a2: *const c_void,
    a3: *const c_void,
) -> u32 {
    type F =
        unsafe extern "C" fn(*const c_void, *const c_void, *const c_void, *const c_void) -> u32;
    (core::mem::transmute::<_, F>(func))(a0, a1, a2, a3)
}
