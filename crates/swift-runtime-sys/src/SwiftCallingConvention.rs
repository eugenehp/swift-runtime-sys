#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift calling convention compatibility layer.
//!
//! # CRITICAL: Calling Convention Issues
//!
//! Many Swift runtime functions use `SWIFT_CC(swift)`, NOT the C calling
//! convention. On arm64 macOS, the Swift CC differs from C in important ways:
//!
//! - `self` is passed in x20 (not x0)
//! - error results are passed in x21
//! - the context pointer uses x20
//! - multi-word returns (like MetadataResponse) are returned in (x0, x1)
//!   as separate registers, NOT as a struct pointer
//!
//! ## Functions that use SWIFT_CC(swift) (DANGER — our extern "C" declarations are wrong):
//!
//! From Metadata.h:
//!   - swift_getSingletonMetadata
//!   - swift_getGenericMetadata
//!   - swift_getCanonicalSpecializedMetadata
//!   - swift_getCanonicalPrespecializedGenericMetadata
//!   - swift_checkMetadataState
//!   - swift_getAssociatedTypeWitness / Relative
//!   - swift_getAssociatedConformanceWitness / Relative
//!   - swift_compareTypeContextDescriptors
//!   - swift_compareWitnessTables
//!   - swift_compareProtocolConformanceDescriptors
//!   - swift_allocateMetadataPack
//!   - swift_allocateWitnessTablePack
//!   - swift_getForeignTypeMetadata
//!   - swift_getFixedArrayTypeMetadata
//!   - swift_getTupleTypeMetadata / 2 / 3
//!   - swift_initClassMetadata2
//!   - swift_updateClassMetadata2
//!   - swift_getTypeContextDescriptor
//!   - swift_conformsToProtocolCommon
//!   - swift_conformsToProtocolWithExecutionContext
//!
//! From Error.h:
//!   - swift_allocError
//!   - swift_willThrow
//!   - swift_willThrowTypedImpl
//!   - swift_errorInMain
//!   - swift_unexpectedError
//!
//! From HeapObject.h:
//!   - swift_allocBox
//!   - swift_makeBoxUnique
//!   - swift_getTypeName
//!   - swift_getMangledTypeName
//!
//! From Enum.h:
//!   - swift_getEnumTagSinglePayloadGeneric
//!   - swift_storeEnumTagSinglePayloadGeneric
//!
//! From Concurrency.h:
//!   - Almost ALL concurrency functions
//!
//! ## Workaround
//!
//! On **x86_64**, SWIFT_CC(swift) is identical to the C calling convention,
//! so all our `extern "C"` declarations work correctly.
//!
//! On **arm64**, there are three options:
//!
//! 1. **Use dlsym + thunks** (current approach via RuntimeFactory):
//!    Write a small C/Swift wrapper that uses the correct CC and exposes
//!    a C-ABI version. This is what RuntimeFactory already does.
//!
//! 2. **Use inline assembly** to call with the Swift CC:
//!    Set up registers manually before calling.
//!
//! 3. **Wait for Rust to support `extern "swift"`**:
//!    There is an RFC for this but it's not stabilized.
//!
//! For functions that return `MetadataResponse` (a 2-word struct),
//! the C ABI on arm64 passes a return pointer in x8, while Swift CC
//! returns in (x0, x1). This means our current declarations will
//! silently get garbage for the `state` field.
//!
//! ## Safe Functions (truly extern "C")
//!
//! These functions are confirmed to use the C calling convention and are
//! safe to call directly from Rust on all architectures:
//!
//!   - swift_allocObject
//!   - swift_deallocObject / swift_deallocClassInstance / swift_deallocUninitializedObject
//!   - swift_retain / swift_release / swift_retain_n / swift_release_n
//!   - swift_tryRetain / swift_retainCount
//!   - swift_weakInit / swift_weakLoadStrong / swift_weakDestroy / etc.
//!   - swift_unownedRetain / swift_unownedRelease / etc.
//!   - swift_unknownObjectRetain / swift_unknownObjectRelease / etc.
//!   - swift_bridgeObjectRetain / swift_bridgeObjectRelease / etc.
//!   - swift_nonatomic_retain / swift_nonatomic_release / etc.
//!   - swift_isDeallocating / swift_setDeallocating
//!   - swift_isUniquelyReferenced / etc.
//!   - swift_getWitnessTable / swift_getWitnessTableRelative
//!   - swift_conformsToProtocol
//!   - swift_dynamicCast and all dynamicCast variants
//!   - swift_getDynamicType
//!   - swift_getObjectType
//!   - swift_getMetadataKind
//!   - swift_getMetatypeMetadata
//!   - swift_getObjCClassFromMetadata / FromObject
//!   - swift_getObjCClassMetadata
//!   - swift_getInitializedObjCClass
//!   - swift_isClassType / swift_isOptionalType / swift_isClassOrObjCExistentialType
//!   - swift_class_isSubclass
//!   - swift_initStackObject / swift_initStaticObject
//!   - swift_verifyEndOfLifetime
//!   - swift_allocEmptyBox / swift_deallocBox / swift_projectBox
//!   - swift_errorRetain / swift_errorRelease
//!   - swift_deallocError
//!   - swift_getErrorValue
//!   - swift_demangle
//!   - swift_once
//!   - swift_beginAccess / swift_endAccess
//!   - swift_slowAlloc / swift_slowDealloc
//!   - swift_EnumCaseName
//!   - swift_getEnumCaseMultiPayload
//!   - swift_storeEnumTagMultiPayload
//!   - swift_initEnumMetadataSingleCase / SinglePayload / MultiPayload
//!   - swift_getFunctionTypeMetadata (all variants)
//!   - swift_getExistentialTypeMetadata / Metatype
//!   - swift_registerProtocolConformances / Protocols / TypeMetadataRecords
//!   - swift_getTypeByMangledNameInContext / InEnvironment
//!   - All swift_stdlib_* functions
//!   - All swift_tsan_* functions
//!   - All numeric conversion functions
//!   - All array value witness functions
//!   - All generic value witness functions
//!   - All POD functions

// This module is documentation-only. No executable code.
