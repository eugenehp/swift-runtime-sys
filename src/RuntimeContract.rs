use core::ffi::{c_char, c_void};
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::process::Command;
use std::time::Instant;

use crate::RuntimeFactory::{OpaqueSwiftRef, RuntimeFactory, RuntimeFactoryError};
use crate::SymbolDemangler::SymbolDemangler;

#[derive(Debug, Clone, Copy)]
pub enum ContractOwnership {
    SwiftRetained,
}

#[derive(Debug, Clone, Copy)]
pub enum ContractArgValue {
    I32(i32),
    OpaqueRef(OpaqueSwiftRef),
}

#[derive(Debug, Clone)]
pub struct ContractArgBlob {
    bytes: Vec<u8>,
}

impl ContractArgBlob {
    pub fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn from_i32s(values: &[i32]) -> Self {
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(values));
        for value in values {
            bytes.extend_from_slice(&value.to_ne_bytes());
        }
        Self { bytes }
    }

    pub fn from_values(values: &[ContractArgValue]) -> Self {
        let mut bytes = Vec::new();
        for value in values {
            match value {
                ContractArgValue::I32(raw) => bytes.extend_from_slice(&raw.to_ne_bytes()),
                ContractArgValue::OpaqueRef(raw) => {
                    bytes.extend_from_slice(&(*raw as usize).to_ne_bytes())
                }
            }
        }
        Self { bytes }
    }

    pub fn as_ptr(&self) -> *const c_void {
        if self.bytes.is_empty() {
            std::ptr::null()
        } else {
            self.bytes.as_ptr() as *const c_void
        }
    }

    pub fn len(&self) -> i32 {
        self.bytes.len() as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ContractObject {
    pub type_id: i32,
    pub object: OpaqueSwiftRef,
    pub ownership: ContractOwnership,
}

#[derive(Debug)]
pub enum ContractResultValue<'a> {
    Void,
    I32(i32),
    OwnedObject(OwnedContractObject<'a>),
}

#[derive(Debug)]
pub enum RuntimeContractError {
    Factory(RuntimeFactoryError),
    DescriptorParse(String),
    UnsupportedFeature {
        feature: &'static str,
        status: CompilerFeatureStatus,
        reason: String,
    },
    MetadataLookupFailed {
        metadata_id: i32,
    },
    ProtocolNotSupported {
        type_id: i32,
        protocol_id: i32,
    },
    ProtocolInvokeFailed {
        type_id: i32,
        protocol_id: i32,
        method_id: i32,
    },
    NullConstruct {
        type_id: i32,
    },
    InvalidInvoke {
        type_id: i32,
        method_id: i32,
    },
    ReleaseFailed {
        type_id: i32,
    },
    MetadataPointerNotRegistered {
        pointer: usize,
    },
    WitnessTableResolutionFailed {
        type_name: String,
        protocol_name: String,
    },
    ConformanceNotFound {
        type_name: String,
        protocol_name: String,
    },
    VersionDetectionFailed {
        reason: String,
    },
    AdapterSelectionFailed {
        version: String,
        reason: String,
    },
}

impl From<RuntimeFactoryError> for RuntimeContractError {
    fn from(value: RuntimeFactoryError) -> Self {
        Self::Factory(value)
    }
}

type ContractConstruct = unsafe extern "C" fn(i32, *const c_void, i32) -> *mut c_void;
type ContractInvokeI32 = unsafe extern "C" fn(i32, i32, *mut c_void, *const c_void, i32) -> i32;
type ContractInvokeVoid = unsafe extern "C" fn(i32, i32, *mut c_void, *const c_void, i32) -> i32;
type ContractRelease = unsafe extern "C" fn(i32, *mut c_void) -> i32;
type ContractJSON = unsafe extern "C" fn() -> *const c_char;
type ContractLookupMetadata = unsafe extern "C" fn(i32) -> *const c_void;
type ContractProtocolHasConformance = unsafe extern "C" fn(i32, i32) -> i32;
type ContractProtocolInvokeI32 = unsafe extern "C" fn(i32, i32, i32, *mut c_void) -> i32;
type ContractConstructString = unsafe extern "C" fn(*const c_void, i32) -> *mut c_void;
type ContractStringLen = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractStringBytes = unsafe extern "C" fn(*mut c_void, *mut c_void, i32) -> i32;
type ContractArrayMake = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractArrayLen = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractArrayGet = unsafe extern "C" fn(*mut c_void, i32) -> i32;
type ContractArraySet = unsafe extern "C" fn(*mut c_void, i32, i32) -> i32;
type ContractArrayAppend = unsafe extern "C" fn(*mut c_void, i32) -> i32;
type ContractArrayData = unsafe extern "C" fn(*mut c_void) -> *const c_void;
type ContractArrayRefMake = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractArrayRefLen = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractArrayRefGet = unsafe extern "C" fn(*mut c_void, i32) -> *mut c_void;
type ContractArrayRefSet = unsafe extern "C" fn(*mut c_void, i32, *mut c_void) -> i32;
type ContractArrayRefAppend = unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32;
type ContractDictI32Make = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractDictI32Len = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractDictI32Get = unsafe extern "C" fn(*mut c_void, i32, *mut i32) -> i32;
type ContractDictI32Set = unsafe extern "C" fn(*mut c_void, i32, i32) -> i32;
type ContractDictI32Remove = unsafe extern "C" fn(*mut c_void, i32, *mut i32) -> i32;
type ContractDictI32Contains = unsafe extern "C" fn(*mut c_void, i32) -> i32;
type ContractDictRefMake = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractDictRefLen = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractDictRefGet = unsafe extern "C" fn(*mut c_void, i32, *mut *mut c_void) -> i32;
type ContractDictRefSet = unsafe extern "C" fn(*mut c_void, i32, *mut c_void) -> i32;
type ContractDictRefRemove = unsafe extern "C" fn(*mut c_void, i32) -> *mut c_void;
type ContractDictRefContains = unsafe extern "C" fn(*mut c_void, i32) -> i32;
type ContractAnyWrap = unsafe extern "C" fn(i32, *mut c_void) -> *mut c_void;
type ContractAnyTypeId = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractDynamicCast = unsafe extern "C" fn(*mut c_void, i32) -> *mut c_void;
type ContractDirectionMake = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractDirectionCase = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractShapeCircle = unsafe extern "C" fn(f32) -> *mut c_void;
type ContractShapeRect = unsafe extern "C" fn(f32, f32) -> *mut c_void;
type ContractShapeGetCase = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractShapeCircleRadius = unsafe extern "C" fn(*mut c_void) -> f32;
type ContractShapeRectDims = unsafe extern "C" fn(*mut c_void, *mut f32, *mut f32) -> i32;
type ContractStructLayoutSize = unsafe extern "C" fn() -> i32;
type ContractStructLayoutStride = unsafe extern "C" fn() -> i32;
type ContractStructLayoutAlignment = unsafe extern "C" fn() -> i32;
type ContractStructLayoutFieldOffset = unsafe extern "C" fn() -> i32;
type ContractStructConstruct = unsafe extern "C" fn(*const c_void, i32) -> *mut c_void;
type ContractStructFieldGetI32 = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractStructFieldGetI64 = unsafe extern "C" fn(*mut c_void) -> i64;
type ContractTuplePairConstruct = unsafe extern "C" fn(i32, i32) -> *mut c_void;
type ContractTuplePairGetFirst = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractTuplePairGetSecond = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractTupleTripleConstruct = unsafe extern "C" fn(i32, i32, i32) -> *mut c_void;
type ContractTupleTripleGetFirst = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractTupleTripleGetSecond = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractTupleTripleGetThird = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractClosureAdderConstruct = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractClosureAdderInvoke = unsafe extern "C" fn(*mut c_void, i32) -> i32;
type ContractClosureAdderGetCapture = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractClosureMultiConstruct = unsafe extern "C" fn(i32, i32) -> *mut c_void;
type ContractClosureMultiInvoke = unsafe extern "C" fn(*mut c_void, i32, i32) -> i32;
type ContractClosureMultiGetFactor = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractClosureMultiGetOffset = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractErrorMakeValidation = unsafe extern "C" fn(i32) -> i32;
type ContractErrorMakeIO = unsafe extern "C" fn(i32) -> i32;
type ContractErrorGetDescription = unsafe extern "C" fn() -> *mut c_char;
type ContractErrorGetCode = unsafe extern "C" fn() -> i32;
type ContractErrorIsValidation = unsafe extern "C" fn() -> i32;
type ContractErrorIsIO = unsafe extern "C" fn() -> i32;
type ContractErrorClear = unsafe extern "C" fn();
type ContractErrorMakeOutOfRange = unsafe extern "C" fn(i32, i32) -> i32;
type ContractErrorContextMakeValidation = unsafe extern "C" fn(i32, i32) -> i32;
type ContractErrorContextMakeIO = unsafe extern "C" fn(i32) -> i32;
type ContractErrorContextGetJSON = unsafe extern "C" fn() -> *mut c_char;
type ContractErrorContextGetString = unsafe extern "C" fn() -> *mut c_char;
type ContractErrorContextSetJSON = unsafe extern "C" fn(*const c_char) -> i32;
type ContractErrorContextClear = unsafe extern "C" fn();
type ContractTaskSpawnSum = unsafe extern "C" fn(i32, i32) -> i32;
type ContractTaskSpawnChain = unsafe extern "C" fn(i32, i32) -> i32;
type ContractContinuationReset = unsafe extern "C" fn();
type ContractContinuationResumeCount = unsafe extern "C" fn() -> i32;
type ContractContinuationRoundtrip = unsafe extern "C" fn(i32) -> i32;
type ContractContinuationValidateResumeOnce = unsafe extern "C" fn() -> i32;
type ContractActorMake = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractActorCurrent = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractActorAdd = unsafe extern "C" fn(*mut c_void, i32) -> i32;
type ContractActorValidateIsolation = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractStreamMake = unsafe extern "C" fn(i32, i32) -> *mut c_void;
type ContractStreamNext = unsafe extern "C" fn(*mut c_void, *mut i32) -> i32;
type ContractStreamCollectSum = unsafe extern "C" fn(i32, i32) -> i32;
type ContractTaskLocalGetDefault = unsafe extern "C" fn() -> i32;
type ContractTaskLocalRunWith = unsafe extern "C" fn(i32, i32) -> i32;
type ContractTaskLocalIsolationCheck = unsafe extern "C" fn(i32) -> i32;
type ContractGenericValidateSubstitution = unsafe extern "C" fn(*const c_char) -> i32;
type ContractGenericBoxI32Make = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractGenericBoxI32Get = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractGenericArrayI32Sum = unsafe extern "C" fn(i32, i32) -> i32;
type ContractGenericDictStringI32Sum = unsafe extern "C" fn(i32) -> i32;
type ContractGenericProtocolArrayI32SequenceSupported = unsafe extern "C" fn() -> i32;
type ContractGenericProtocolArrayI32Subscript = unsafe extern "C" fn(i32) -> i32;
type ContractGenericProtocolArrayI32WitnessToken = unsafe extern "C" fn() -> u64;
type ContractGenericProtocolDictStringI32Supported = unsafe extern "C" fn() -> i32;
type ContractGenericProtocolDictStringI32Lookup = unsafe extern "C" fn(*const c_char) -> i32;
// Track H.3 – Constrained Generic Bounds
type ContractConstrainedEquatableEqual = unsafe extern "C" fn(i32, i32) -> i32;
type ContractConstrainedComparableCmp = unsafe extern "C" fn(i32, i32) -> i32;
type ContractConstrainedHashableDistinctCount = unsafe extern "C" fn(i32, i32, i32) -> i32;
type ContractConstrainedAdditiveSum = unsafe extern "C" fn(i32, i32) -> i32;
type ContractConstrainedCodableRoundtrip = unsafe extern "C" fn(i32) -> i32;
type ContractConstrainedMultiMin = unsafe extern "C" fn(i32, i32) -> i32;
// Track I.1 – Foundation Date/Time
type ContractDatetimeFormatUnix = unsafe extern "C" fn(f64) -> *mut c_char;
type ContractDatetimeParseIso8601 = unsafe extern "C" fn(*const c_char) -> f64;
type ContractDatetimeYearUtc = unsafe extern "C" fn(f64) -> i32;
type ContractDatetimeMonthUtc = unsafe extern "C" fn(f64) -> i32;
type ContractDatetimeUtcOffsetSeconds = unsafe extern "C" fn() -> i32;
// Track I.2 – Foundation Data / UUID / CharacterSet
type ContractDataFromBytesChecksum = unsafe extern "C" fn(*const u8, i32) -> u32;
type ContractUuidNewString = unsafe extern "C" fn() -> *mut c_char;
type ContractUuidParseValidate = unsafe extern "C" fn(*const c_char) -> i32;
type ContractUuidRoundtrip = unsafe extern "C" fn() -> i32;
type ContractCharsetIsLetter = unsafe extern "C" fn(i32) -> i32;
// Track I.3 – Foundation URL / URLComponents
type ContractUrlParseValid = unsafe extern "C" fn(*const c_char) -> i32;
type ContractUrlScheme = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractUrlHost = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractUrlPath = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractUrlBuildFromComponents =
    unsafe extern "C" fn(*const c_char, *const c_char, *const c_char) -> *mut c_char;
// Track I.4 – NSCoding / NSCopying
type ContractNscodingIntegerRoundtrip = unsafe extern "C" fn(i32) -> i32;
type ContractNscodingStringRoundtrip = unsafe extern "C" fn(*const c_char) -> i32;
type ContractNscopyingArrayIndependence = unsafe extern "C" fn() -> i32;
// Track J.1 – KeyPath Runtime
type ContractKeypathGetAge = unsafe extern "C" fn(i32) -> i32;
type ContractKeypathGetNestedScore = unsafe extern "C" fn(i32) -> i32;
type ContractKeypathAnyMatches = unsafe extern "C" fn() -> i32;
// Track J.2 – Property Wrapper Metadata
type ContractWrapperInitClamped = unsafe extern "C" fn(i32) -> i32;
type ContractWrapperSetClamped = unsafe extern "C" fn(i32, i32) -> i32;
type ContractWrapperProjectedValue = unsafe extern "C" fn(i32) -> i32;
// Track J.3 – Opaque Type Bridging
type ContractOpaqueNamedGetName = unsafe extern "C" fn(i32) -> *mut c_char;
type ContractOpaqueNamedNameLen = unsafe extern "C" fn(i32) -> i32;
// Track J.4 – Result Builder DSL
type ContractBuilderSum2 = unsafe extern "C" fn(i32, i32) -> i32;
type ContractBuilderConditional = unsafe extern "C" fn(i32) -> i32;
type ContractBuilderLoopSum = unsafe extern "C" fn(i32) -> i32;
// Track K.1 – Weak/Unowned/Cycle tracking
type ContractK1WeakLifecycle = unsafe extern "C" fn() -> i32;
type ContractK1UnownedDanglingDetected = unsafe extern "C" fn() -> i32;
type ContractK1CycleDetectStrongPair = unsafe extern "C" fn() -> i32;
type ContractK1CycleDetectAcyclicPair = unsafe extern "C" fn() -> i32;
// Track K.2 – Retain count / graph probes
type ContractK2RetainDelta = unsafe extern "C" fn() -> i32;
type ContractK2ReferenceTypeInfer = unsafe extern "C" fn(i32) -> i32;
type ContractK2ReferenceGraphDot = unsafe extern "C" fn() -> *mut c_char;
// Track K.3 – Leak tracking probes
type ContractK3TrackerReset = unsafe extern "C" fn();
type ContractK3Alloc = unsafe extern "C" fn(i32) -> *mut c_void;
type ContractK3Release = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractK3SweepUnreleasedCount = unsafe extern "C" fn() -> i32;
type ContractK3LiveCountForSite = unsafe extern "C" fn(i32) -> i32;
type ContractK3RootCauseSite = unsafe extern "C" fn() -> i32;
// Track L.1 – User-defined type registration
type ContractL1RegistryReset = unsafe extern "C" fn();
type ContractL1RegisterType = unsafe extern "C" fn(*const c_char) -> i32;
type ContractL1LookupTypeId = unsafe extern "C" fn(*const c_char) -> i32;
type ContractL1BumpTypeVersion = unsafe extern "C" fn(i32) -> i32;
type ContractL1UpdateCompat = unsafe extern "C" fn(i32, i32) -> i32;
// Track L.2 – Cross-version compatibility
type ContractL2ContractDiffBreakingCount = unsafe extern "C" fn(i32, i32) -> i32;
type ContractL2BinaryVersionCompatible = unsafe extern "C" fn(i32, i32) -> i32;
type ContractL2ResilienceMarker = unsafe extern "C" fn(*const c_char) -> i32;
// Track L.3 – Contract derivation from source
type ContractL3DeriveContractFromSource = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractL3ValidateDerivedContract = unsafe extern "C" fn(*const c_char, *const c_char) -> i32;
type ContractL3ExporterMacroSim = unsafe extern "C" fn(*const c_char) -> *mut c_char;
// Track M.1 – Instruments integration
type ContractM1Reset = unsafe extern "C" fn();
type ContractM1OsLogEvent = unsafe extern "C" fn(*const c_char) -> i32;
type ContractM1PoiBegin = unsafe extern "C" fn(i32) -> i32;
type ContractM1PoiEnd = unsafe extern "C" fn(i32) -> i32;
type ContractM1EventCount = unsafe extern "C" fn() -> i32;
type ContractM1LastDurationNanos = unsafe extern "C" fn() -> u64;
type ContractM1ProfileIterations = unsafe extern "C" fn(i32) -> u64;
// Track M.2 – DWARF debug info access
type ContractM2Reset = unsafe extern "C" fn();
type ContractM2CacheBinary = unsafe extern "C" fn(*const c_char) -> i32;
type ContractM2CacheSize = unsafe extern "C" fn() -> i32;
type ContractM2LookupSource = unsafe extern "C" fn(u64) -> *mut c_char;
type ContractM2LookupVariable = unsafe extern "C" fn(*const c_char) -> *mut c_char;
// Track M.3 – Memory profiling / malloc tagging
type ContractM3Reset = unsafe extern "C" fn();
type ContractM3TagAlloc = unsafe extern "C" fn(*const c_char, i64) -> *mut c_void;
type ContractM3ReleaseAlloc = unsafe extern "C" fn(*mut c_void) -> i32;
type ContractM3UsageForSubsystem = unsafe extern "C" fn(*const c_char) -> i64;
type ContractM3HealthReport = unsafe extern "C" fn() -> *mut c_char;
// Track M.4 – Performance regression testing
type ContractM4RunBenchmark = unsafe extern "C" fn(*const c_char, i32) -> u64;
type ContractM4SetBaseline = unsafe extern "C" fn(*const c_char, u64) -> i32;
type ContractM4RegressionAlarm = unsafe extern "C" fn(*const c_char, u64, i32) -> i32;
type ContractM4BaselineGet = unsafe extern "C" fn(*const c_char) -> u64;
// Track N.1 – Universal Runtime Metadata Graph
type ContractN1MetadataKind = unsafe extern "C" fn(i32) -> i32;
type ContractN1MetadataFieldCount = unsafe extern "C" fn(i32) -> i32;
type ContractN1MetadataFieldOffset = unsafe extern "C" fn(i32, i32) -> i32;
type ContractN1MetadataGraphTraverseCount = unsafe extern "C" fn() -> i32;
type ContractN1MetadataSnapshotJson = unsafe extern "C" fn() -> *mut c_char;
type ContractN1MetadataKindByName = unsafe extern "C" fn(*const c_char) -> i32;
type ContractN1MetadataFieldCountByName = unsafe extern "C" fn(*const c_char) -> i32;
type ContractN1MetadataDiscoverTypesJson = unsafe extern "C" fn() -> *mut c_char;
type ContractN1MetadataGraphTraverseDiscoveredCount = unsafe extern "C" fn() -> i32;
// Track N.1 – Runtime-Wide Enumeration (exit criterion path)
type ContractN1EnumerateAllTypesJson = unsafe extern "C" fn() -> *mut c_char;
type ContractN1TypeInfoJson = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractN1ImageCount = unsafe extern "C" fn() -> i32;
type ContractN1ImageTypesJson = unsafe extern "C" fn(i32) -> *mut c_char;
// Track N.2 – Universal Call Lowering & Invocation
type ContractN2CapabilityMask = unsafe extern "C" fn() -> u32;
type ContractN2InvokeI32 =
    unsafe extern "C" fn(*const c_char, i32, i32, *mut i32, *mut i32, *mut i32) -> i32;
type ContractN2InvokeSymbolI32 = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    i32,
    i32,
    *mut i32,
    *mut i32,
    *mut i32,
) -> i32;
type ContractN2LoweringStrategyJson = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractN2SymbolDescribe = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractN2InvokeAuto =
    unsafe extern "C" fn(*const c_char, i32, i32, *mut i32, *mut i32, *mut i32) -> i32;
// Track N.3 – Arbitrary Generic/Witness Instantiation
type ContractN3BuildContextJson = unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_char;
type ContractN3ResolveWitnessJson =
    unsafe extern "C" fn(*const c_char, *const c_char, *const c_char) -> *mut c_char;
type ContractN3ValidateRequirementsJson =
    unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_char;
type ContractN3InvokeGenericI32 =
    unsafe extern "C" fn(*const c_char, *const c_char, *const c_char, i32, i32, *mut i32) -> i32;
// Track N.4 – Unsafe Runtime Ops Isolation & Recovery
type ContractN4SafePing = unsafe extern "C" fn(i32) -> i32;
type ContractN4TriggerAbort = unsafe extern "C" fn();
// Track N.5 – Cross-Version ABI Adaptation Layer
type ContractN5AdapterTableJson = unsafe extern "C" fn() -> *mut c_char;
type ContractN5FeatureProbeJson = unsafe extern "C" fn() -> *mut c_char;
type ContractN5SelectAdapterJson = unsafe extern "C" fn() -> *mut c_char;
// Track N.6 – Differential Fuzzing & Semantic Oracle
type ContractN6GenerateProgramJson = unsafe extern "C" fn(i64, i32) -> *mut c_char;
type ContractN6ExecuteProgramJson = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractBacktraceCapture = unsafe extern "C" fn() -> *mut c_char;
type ContractBacktraceAnchorAddress = unsafe extern "C" fn() -> u64;

// Phase B.2 – Witness Table Dynamic Resolver
type ContractB2ScanConformancesJson = unsafe extern "C" fn() -> *mut c_char;
type ContractB2ResolveWitnessTable = unsafe extern "C" fn(*const c_char, *const c_char) -> *const c_void;
type ContractB2DescribeConformance = unsafe extern "C" fn(*const c_void) -> *mut c_char;

// Phase B.3 – Cross-Version ABI Compatibility Shim
type ContractB3RuntimeVersionJson = unsafe extern "C" fn() -> *mut c_char;
type ContractB3GetAdapterTableJson = unsafe extern "C" fn(*const c_char) -> *mut c_char;
type ContractB3SelectAdapterProfile = unsafe extern "C" fn(*const c_char) -> i32;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractTypeDescriptor {
    pub type_id: i32,
    pub name: String,
    #[serde(default)]
    pub kind: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractMetadataEntry {
    pub metadata_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractMetadataRegistry {
    #[serde(default)]
    pub entries: Vec<ContractMetadataEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractProtocolMethodDescriptor {
    pub method_id: i32,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractProtocolEntry {
    pub protocol_id: i32,
    #[serde(default)]
    pub name: String,
    pub type_id: i32,
    #[serde(default)]
    pub methods: Vec<ContractProtocolMethodDescriptor>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContractProtocolRegistry {
    #[serde(default)]
    pub entries: Vec<ContractProtocolEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContractDescriptorJson {
    pub contract_version: i32,
    pub bridge: String,
    pub cooperation_boundary: CooperationBoundary,
    pub compiler_features: CompilerFeatureCapabilities,
    #[serde(default)]
    pub types: Vec<ContractTypeDescriptor>,
    #[serde(default)]
    pub metadata_registry: ContractMetadataRegistry,
    #[serde(default)]
    pub protocol_registry: ContractProtocolRegistry,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CooperationBoundary {
    pub swift_side: Vec<String>,
    pub rust_side: Vec<String>,
    pub research_only: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CompilerFeatureStatus {
    Supported,
    Fallback,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompilerFeatureCapability {
    pub status: CompilerFeatureStatus,
    pub reason: String,
    pub provider: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompilerFeatureCapabilities {
    pub resilient_dispatch: CompilerFeatureCapability,
    pub generic_metadata_registry: CompilerFeatureCapability,
    pub protocol_witness_registry: CompilerFeatureCapability,
    pub raw_runtime_research_mode: CompilerFeatureCapability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractErrorContextPayload {
    pub domain: String,
    pub code: i32,
    pub message: String,
    pub chain: Vec<String>,
    #[serde(rename = "user_info")]
    pub user_info: std::collections::HashMap<String, String>,
    #[serde(rename = "recovery_hints")]
    pub recovery_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N5FeatureFlags {
    pub swift5_types_scan: bool,
    pub objc_class_scan: bool,
    pub dynamic_symbol_lowering: bool,
    pub recursive_generic_solver: bool,
    pub broker_isolation: bool,
    pub private_type_kind_fallback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N5FeatureProbe {
    pub compiler_family: String,
    pub platform: String,
    pub architecture: String,
    pub os_major: i32,
    pub os_minor: i32,
    pub os_patch: i32,
    pub optimization_mode: String,
    pub features: N5FeatureFlags,
}

#[derive(Debug, Clone, Deserialize)]
pub struct N1TypeInfoJson {
    pub name: String,
    pub kind: String,
    pub kind_id: i32,
    pub field_count: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataLayout {
    pub type_name: String,
    pub metadata_id: Option<i32>,
    pub kind: String,
    pub kind_id: i32,
    pub field_count: i32,
    pub field_offsets: Vec<i32>,
    pub witness_count: i32,
    pub generic_param_count: i32,
    pub size: Option<i32>,
}

// Phase B.2 – Witness Table Dynamic Resolver Structures
#[derive(Debug, Clone, Deserialize)]
pub struct ConformanceDescriptor {
    pub type_name: String,
    pub protocol_name: String,
    pub witness_table: Option<u64>,
    pub conditional_requirements: i32,
    pub generic_parameters: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WitnessTableResolver {
    pub cached_conformances: std::collections::HashMap<String, ConformanceDescriptor>,
    pub enabled: bool,
}

impl WitnessTableResolver {
    pub fn new() -> Self {
        Self {
            cached_conformances: std::collections::HashMap::new(),
            enabled: true,
        }
    }

    pub fn cache_key(type_name: &str, protocol_name: &str) -> String {
        format!("{}:{}", type_name, protocol_name)
    }

    pub fn get_cached(&self, type_name: &str, protocol_name: &str) -> Option<ConformanceDescriptor> {
        self.cached_conformances.get(&Self::cache_key(type_name, protocol_name)).cloned()
    }

    pub fn cache_conformance(&mut self, conformance: ConformanceDescriptor) {
        self.cached_conformances.insert(
            Self::cache_key(&conformance.type_name, &conformance.protocol_name),
            conformance,
        );
    }

    pub fn clear_cache(&mut self) {
        self.cached_conformances.clear();
    }
}

// Phase B.3 – Cross-Version ABI Compatibility Shim Structures
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuntimeVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub version_string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdapterFieldOffset {
    pub field_name: String,
    pub offset: i32,
    pub size: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdapterTypeLayout {
    pub type_name: String,
    pub size: i32,
    pub alignment: i32,
    pub fields: Vec<AdapterFieldOffset>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdapterProfile {
    pub profile_id: String,
    pub swift_version: RuntimeVersion,
    pub compatibility_range: (i32, i32), // (min_major, min_minor)
    pub type_layouts: Vec<AdapterTypeLayout>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionAdapterTable {
    pub current_version: RuntimeVersion,
    pub profiles: Vec<AdapterProfile>,
    pub selected_profile: Option<String>,
}

impl RuntimeVersion {
    pub fn from_parts(major: i32, minor: i32, patch: i32) -> Self {
        Self {
            major,
            minor,
            patch,
            version_string: format!("{}.{}.{}", major, minor, patch),
        }
    }

    pub fn is_compatible_with(&self, min_major: i32, min_minor: i32) -> bool {
        self.major > min_major || (self.major == min_major && self.minor >= min_minor)
    }

    pub fn matches_major(&self, major: i32) -> bool {
        self.major == major
    }
}

impl VersionAdapterTable {
    pub fn new(version: RuntimeVersion) -> Self {
        Self {
            current_version: version,
            profiles: Vec::new(),
            selected_profile: None,
        }
    }

    pub fn add_profile(&mut self, profile: AdapterProfile) {
        self.profiles.push(profile);
    }

    pub fn select_profile(&mut self, profile_id: &str) -> bool {
        if self.profiles.iter().any(|p| p.profile_id == profile_id) {
            self.selected_profile = Some(profile_id.to_string());
            return true;
        }
        false
    }

    pub fn get_selected_profile(&self) -> Option<&AdapterProfile> {
        self.selected_profile.as_ref()
            .and_then(|id| self.profiles.iter().find(|p| &p.profile_id == id))
    }

    pub fn find_compatible_profile(&self) -> Option<&AdapterProfile> {
        for profile in &self.profiles {
            let (min_major, min_minor) = profile.compatibility_range;
            if self.current_version.is_compatible_with(min_major, min_minor) {
                return Some(profile);
            }
        }
        None
    }
}

impl N5FeatureProbe {
    pub fn feature_enabled(&self, feature: &str) -> bool {
        match feature {
            "swift5_types_scan" => self.features.swift5_types_scan,
            "objc_class_scan" => self.features.objc_class_scan,
            "dynamic_symbol_lowering" => self.features.dynamic_symbol_lowering,
            "recursive_generic_solver" => self.features.recursive_generic_solver,
            "broker_isolation" => self.features.broker_isolation,
            "private_type_kind_fallback" => self.features.private_type_kind_fallback,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N5AdapterProfile {
    pub profile_id: String,
    pub compiler_family: String,
    pub platforms: Vec<String>,
    pub architectures: Vec<String>,
    pub supported_optimization_modes: Vec<String>,
    pub required_features: Vec<String>,
    pub symbol_aliases: std::collections::HashMap<String, Vec<String>>,
    pub layout_rules: std::collections::HashMap<String, String>,
    pub witness_rules: std::collections::HashMap<String, String>,
    pub adaptation_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N5AdapterTable {
    pub profiles: Vec<N5AdapterProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N5SelectedAdapter {
    pub profile_id: String,
    pub compatible: bool,
    pub reason: String,
    pub compiler_family: String,
    pub optimization_mode: String,
    pub selected_symbols: std::collections::HashMap<String, String>,
    pub missing_features: Vec<String>,
    pub adaptation_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N5DriftReport {
    pub mode: String,
    pub drift_detected: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct N6Fragment {
    pub id: i32,
    pub kind: String,
    pub a: i32,
    pub b: i32,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct N6Program {
    pub seed: i64,
    pub fragments: Vec<N6Fragment>,
    pub swift_source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct N6Result {
    pub id: i32,
    pub kind: String,
    pub status: String,
    pub value: Option<i32>,
    pub error: Option<String>,
    pub side_effect: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct N6Execution {
    pub seed: i64,
    pub result_count: i32,
    pub results: Vec<N6Result>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N7DerivedCallable {
    pub symbol: String,
    pub demangled: String,
    pub observed_runtime: bool,
    pub shape: Option<String>,
    pub confidence: f64,
    pub fallback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N7DerivedType {
    pub name: String,
    pub kind: String,
    pub field_count: i32,
    pub observed_runtime: bool,
    pub confidence: f64,
    pub fallback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N7DerivedContract {
    pub binary_path: String,
    pub module_hint: String,
    pub callables: Vec<N7DerivedCallable>,
    pub types: Vec<N7DerivedType>,
    pub confidence: f64,
    pub low_confidence_regions: Vec<String>,
    pub fallback_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N7ValidationReport {
    pub callable_coverage: f64,
    pub type_coverage: f64,
    pub validated_callables: i32,
    pub validated_types: i32,
    pub confidence: f64,
    pub low_confidence_regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8SloBudget {
    pub operation: String,
    pub p50_latency_ms_budget: f64,
    pub p95_latency_ms_budget: f64,
    pub min_throughput_ops_per_sec: f64,
    pub max_rss_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8BenchmarkSample {
    pub operation: String,
    pub iterations: i32,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub throughput_ops_per_sec: f64,
    pub rss_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8BudgetGate {
    pub operation: String,
    pub passed: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8OperationalReport {
    pub budgets: Vec<N8SloBudget>,
    pub samples: Vec<N8BenchmarkSample>,
    pub gates: Vec<N8BudgetGate>,
    pub alerts: Vec<String>,
    pub degraded_mode_runbook: String,
}

pub struct RuntimeContract<'a> {
    factory: &'a RuntimeFactory,
}

pub struct OwnedContractObject<'a> {
    contract: &'a RuntimeContract<'a>,
    object: ContractObject,
    released: bool,
}

impl<'a> RuntimeContract<'a> {
    pub fn new(factory: &'a RuntimeFactory) -> Self {
        Self { factory }
    }

    fn resolve<T: Copy>(&self, symbol: &str) -> Result<T, RuntimeContractError> {
        let ptr = self.factory.symbol_address(symbol)?;
        Ok(unsafe { std::mem::transmute_copy(&ptr) })
    }

    pub fn contract_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractJSON = self.resolve("swift_runtime_contract_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::Factory(
                RuntimeFactoryError::ContractValidation(
                    "swift_runtime_contract_json returned null".to_string(),
                ),
            ));
        }
        Ok(unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned())
    }

    pub fn descriptor(&self) -> Result<ContractDescriptorJson, RuntimeContractError> {
        let json = self.contract_json()?;
        serde_json::from_str(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    pub fn cooperation_boundary(&self) -> Result<CooperationBoundary, RuntimeContractError> {
        Ok(self.descriptor()?.cooperation_boundary)
    }

    pub fn compiler_feature_capabilities(
        &self,
    ) -> Result<CompilerFeatureCapabilities, RuntimeContractError> {
        Ok(self.descriptor()?.compiler_features)
    }

    pub fn metadata_registry_entries(
        &self,
    ) -> Result<Vec<ContractMetadataEntry>, RuntimeContractError> {
        Ok(self.descriptor()?.metadata_registry.entries)
    }

    fn ensure_feature(
        &self,
        feature: &'static str,
        capability: &CompilerFeatureCapability,
    ) -> Result<(), RuntimeContractError> {
        if capability.status == CompilerFeatureStatus::Unsupported {
            return Err(RuntimeContractError::UnsupportedFeature {
                feature,
                status: capability.status,
                reason: capability.reason.clone(),
            });
        }
        Ok(())
    }

    pub fn lookup_metadata(&self, metadata_id: i32) -> Result<*const c_void, RuntimeContractError> {
        let features = self.compiler_feature_capabilities()?;
        self.ensure_feature(
            "generic_metadata_registry",
            &features.generic_metadata_registry,
        )?;

        let func: ContractLookupMetadata = self.resolve("swift_contract_lookup_metadata")?;
        let metadata = unsafe { func(metadata_id) };
        if metadata.is_null() {
            return Err(RuntimeContractError::MetadataLookupFailed { metadata_id });
        }
        Ok(metadata)
    }

    fn n1_type_info(&self, name: &str) -> Result<N1TypeInfoJson, RuntimeContractError> {
        let json = self.n1_type_info_json(name)?;
        serde_json::from_str(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    fn generic_argument_count(type_name: &str) -> i32 {
        let Some(start) = type_name.find('<') else {
            return 0;
        };
        let Some(end) = type_name.rfind('>') else {
            return 0;
        };
        if end <= start + 1 {
            return 0;
        }

        let mut count = 1;
        let mut depth = 0;
        for ch in type_name[start + 1..end].chars() {
            match ch {
                '<' => depth += 1,
                '>' => depth -= 1,
                ',' if depth == 0 => count += 1,
                _ => {}
            }
        }
        count
    }

    fn synthetic_n1_type_id(type_name: &str) -> Option<i32> {
        match type_name {
            "N1LayoutClass" => Some(1),
            "N1LayoutStruct" => Some(2),
            "Direction" | "RustBridge.Direction" => Some(3),
            "Array<Int32>" | "Swift.Array<Swift.Int32>" => Some(8),
            _ => None,
        }
    }

    fn type_name_candidates(type_name: &str) -> Vec<String> {
        let mut names = vec![type_name.to_string()];
        match type_name {
            "String" => names.push("Swift.String".to_string()),
            "Array<Int32>" => names.push("Swift.Array<Swift.Int32>".to_string()),
            "Dictionary<Int32, Int32>" => {
                names.push("Swift.Dictionary<Swift.Int32, Swift.Int32>".to_string())
            }
            _ => {
                if !type_name.contains('.') && !type_name.starts_with("Swift.") {
                    names.push(format!("RustBridge.{type_name}"));
                }
            }
        }
        names
    }

    fn resolve_type_info(&self, type_name: &str) -> Result<N1TypeInfoJson, RuntimeContractError> {
        let mut fallback = None;
        for candidate in Self::type_name_candidates(type_name) {
            let info = self.n1_type_info(&candidate)?;
            if fallback.is_none() {
                fallback = Some(info.clone());
            }
            if info.kind != "unknown" && info.kind_id >= 0 {
                return Ok(info);
            }
        }
        fallback.ok_or_else(|| {
            RuntimeContractError::DescriptorParse(format!(
                "no type info candidates for {type_name}"
            ))
        })
    }

    fn resolve_metadata_entry_by_pointer(
        &self,
        metadata: *const c_void,
    ) -> Result<ContractMetadataEntry, RuntimeContractError> {
        for entry in self.metadata_registry_entries()? {
            let candidate = match self.lookup_metadata(entry.metadata_id) {
                Ok(ptr) => ptr,
                Err(_) => continue,
            };
            if candidate == metadata {
                return Ok(entry);
            }
        }
        Err(RuntimeContractError::MetadataPointerNotRegistered {
            pointer: metadata as usize,
        })
    }

    pub fn scan_metadata_header_by_name(
        &self,
        type_name: &str,
    ) -> Result<MetadataLayout, RuntimeContractError> {
        let descriptor = self.descriptor()?;
        let metadata_id = descriptor
            .metadata_registry
            .entries
            .iter()
            .find(|entry| entry.name == type_name)
            .map(|entry| entry.metadata_id);
        let info = self.resolve_type_info(type_name)?;

        let mut field_offsets = Vec::new();
        if let Some(type_id) =
            Self::synthetic_n1_type_id(type_name).or_else(|| Self::synthetic_n1_type_id(&info.name))
        {
            for field_index in 0..info.field_count {
                if let Ok(offset) = self.n1_metadata_field_offset(type_id, field_index) {
                    field_offsets.push(offset);
                }
            }
        }

        let witness_count = metadata_id
            .map(|id| {
                descriptor
                    .protocol_registry
                    .entries
                    .iter()
                    .filter(|entry| entry.type_id == id)
                    .count() as i32
            })
            .unwrap_or(0);

        Ok(MetadataLayout {
            type_name: type_name.to_string(),
            metadata_id,
            kind: info.kind,
            kind_id: info.kind_id,
            field_count: info.field_count,
            field_offsets,
            witness_count,
            generic_param_count: Self::generic_argument_count(type_name),
            size: None,
        })
    }

    pub fn scan_metadata_header(
        &self,
        metadata: *const c_void,
    ) -> Result<MetadataLayout, RuntimeContractError> {
        let entry = self.resolve_metadata_entry_by_pointer(metadata)?;
        let mut layout = self.scan_metadata_header_by_name(&entry.name)?;
        layout.metadata_id = Some(entry.metadata_id);
        Ok(layout)
    }

    pub fn protocol_has_conformance(
        &self,
        type_id: i32,
        protocol_id: i32,
    ) -> Result<bool, RuntimeContractError> {
        let features = self.compiler_feature_capabilities()?;
        self.ensure_feature(
            "protocol_witness_registry",
            &features.protocol_witness_registry,
        )?;

        let func: ContractProtocolHasConformance =
            self.resolve("swift_contract_protocol_has_conformance")?;
        Ok(unsafe { func(type_id, protocol_id) } == 1)
    }

    pub fn protocol_invoke_i32(
        &self,
        object: ContractObject,
        protocol_id: i32,
        method_id: i32,
    ) -> Result<i32, RuntimeContractError> {
        let features = self.compiler_feature_capabilities()?;
        self.ensure_feature(
            "protocol_witness_registry",
            &features.protocol_witness_registry,
        )?;

        if !self.protocol_has_conformance(object.type_id, protocol_id)? {
            return Err(RuntimeContractError::ProtocolNotSupported {
                type_id: object.type_id,
                protocol_id,
            });
        }

        let func: ContractProtocolInvokeI32 = self.resolve("swift_contract_protocol_invoke_i32")?;
        let result = unsafe { func(object.type_id, protocol_id, method_id, object.object) };
        if result == i32::MIN {
            return Err(RuntimeContractError::ProtocolInvokeFailed {
                type_id: object.type_id,
                protocol_id,
                method_id,
            });
        }
        Ok(result)
    }

    pub fn construct(
        &self,
        type_id: i32,
        args: &ContractArgBlob,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractConstruct = self.resolve("swift_contract_construct")?;
        let object = unsafe { func(type_id, args.as_ptr(), args.len()) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id });
        }
        Ok(ContractObject {
            type_id,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    pub fn construct_owned(
        &'a self,
        type_id: i32,
        args: &[ContractArgValue],
    ) -> Result<OwnedContractObject<'a>, RuntimeContractError> {
        let object = self.construct(type_id, &ContractArgBlob::from_values(args))?;
        Ok(OwnedContractObject {
            contract: self,
            object,
            released: false,
        })
    }

    pub fn invoke_i32(
        &self,
        object: ContractObject,
        method_id: i32,
        args: &ContractArgBlob,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractInvokeI32 = self.resolve("swift_contract_invoke_i32")?;
        let result = unsafe {
            func(
                object.type_id,
                method_id,
                object.object,
                args.as_ptr(),
                args.len(),
            )
        };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: object.type_id,
                method_id,
            });
        }
        Ok(result)
    }

    pub fn invoke_i32_args(
        &self,
        object: ContractObject,
        method_id: i32,
        args: &[ContractArgValue],
    ) -> Result<i32, RuntimeContractError> {
        self.invoke_i32(object, method_id, &ContractArgBlob::from_values(args))
    }

    pub fn invoke_void(
        &self,
        object: ContractObject,
        method_id: i32,
        args: &ContractArgBlob,
    ) -> Result<(), RuntimeContractError> {
        let func: ContractInvokeVoid = self.resolve("swift_contract_invoke_void")?;
        let ok = unsafe {
            func(
                object.type_id,
                method_id,
                object.object,
                args.as_ptr(),
                args.len(),
            )
        };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: object.type_id,
                method_id,
            });
        }
        Ok(())
    }

    pub fn invoke_void_args(
        &self,
        object: ContractObject,
        method_id: i32,
        args: &[ContractArgValue],
    ) -> Result<(), RuntimeContractError> {
        self.invoke_void(object, method_id, &ContractArgBlob::from_values(args))
    }

    pub fn construct_boxed(
        &'a self,
        type_id: i32,
        args: &[ContractArgValue],
    ) -> Result<ContractResultValue<'a>, RuntimeContractError> {
        Ok(ContractResultValue::OwnedObject(
            self.construct_owned(type_id, args)?,
        ))
    }

    pub fn invoke_i32_boxed(
        &self,
        object: ContractObject,
        method_id: i32,
        args: &[ContractArgValue],
    ) -> Result<ContractResultValue<'a>, RuntimeContractError> {
        Ok(ContractResultValue::I32(
            self.invoke_i32_args(object, method_id, args)?,
        ))
    }

    pub fn invoke_void_boxed(
        &self,
        object: ContractObject,
        method_id: i32,
        args: &[ContractArgValue],
    ) -> Result<ContractResultValue<'a>, RuntimeContractError> {
        self.invoke_void_args(object, method_id, args)?;
        Ok(ContractResultValue::Void)
    }

    // MARK: - String Support (Track C.1)

    /// Constructs a String from UTF-8 bytes. Returns bytes as a direct ContractObject.
    pub fn construct_string(&self, bytes: &[u8]) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractConstructString = self.resolve("swift_contract_construct_string")?;
        let ptr = unsafe { func(bytes.as_ptr() as *const c_void, bytes.len() as i32) };
        if ptr.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 3 });
        }
        Ok(ContractObject {
            type_id: 3,
            object: ptr,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Constructs a String from UTF-8 bytes and returns an owned reference.
    pub fn construct_string_owned(
        &'a self,
        bytes: &[u8],
    ) -> Result<OwnedContractObject<'a>, RuntimeContractError> {
        let object = self.construct_string(bytes)?;
        Ok(OwnedContractObject {
            contract: self,
            object,
            released: false,
        })
    }

    /// Returns the length of a String in UTF-8 bytes.
    pub fn string_len(&self, object: OpaqueSwiftRef) -> Result<i32, RuntimeContractError> {
        let func: ContractStringLen = self.resolve("swift_contract_string_len")?;
        let len = unsafe { func(object) };
        if len < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 3,
                method_id: 1,
            });
        }
        Ok(len)
    }

    /// Copies String bytes into a Rust buffer. Returns the actual UTF-8 byte count.
    pub fn string_bytes(
        &self,
        object: OpaqueSwiftRef,
        buffer: &mut [u8],
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractStringBytes = self.resolve("swift_contract_string_bytes")?;
        let actual_count = unsafe {
            func(
                object,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len() as i32,
            )
        };
        if actual_count < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 3,
                method_id: 2,
            });
        }
        Ok(actual_count)
    }

    /// Creates a new array with optional preallocated capacity.
    pub fn construct_array(&self, capacity: i32) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractArrayMake = self.resolve("swift_contract_array_make")?;
        let object = unsafe { func(capacity) };
        if object.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 4,
                method_id: 0,
            });
        }
        Ok(ContractObject {
            type_id: 4,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Returns the number of elements in the array.
    pub fn array_len(&self, object: OpaqueSwiftRef) -> Result<i32, RuntimeContractError> {
        let func: ContractArrayLen = self.resolve("swift_contract_array_len")?;
        let len = unsafe { func(object) };
        if len < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 4,
                method_id: 1,
            });
        }
        Ok(len)
    }

    /// Gets the element at the specified index.
    pub fn array_get(
        &self,
        object: OpaqueSwiftRef,
        index: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractArrayGet = self.resolve("swift_contract_array_get")?;
        let value = unsafe { func(object, index) };
        if value == -1 && index < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 4,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Sets the element at the specified index.
    pub fn array_set(
        &self,
        object: OpaqueSwiftRef,
        index: i32,
        value: i32,
    ) -> Result<(), RuntimeContractError> {
        let func: ContractArraySet = self.resolve("swift_contract_array_set")?;
        let result = unsafe { func(object, index, value) };
        if result < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 4,
                method_id: 3,
            });
        }
        Ok(())
    }

    /// Appends an element to the array and returns the new count.
    pub fn array_append(
        &self,
        object: OpaqueSwiftRef,
        value: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractArrayAppend = self.resolve("swift_contract_array_append")?;
        let new_count = unsafe { func(object, value) };
        if new_count < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 4,
                method_id: 4,
            });
        }
        Ok(new_count)
    }

    /// Returns a read-only pointer to the backing contiguous storage for pointer-based iteration.
    pub fn array_data(&self, object: OpaqueSwiftRef) -> Result<*const i32, RuntimeContractError> {
        let func: ContractArrayData = self.resolve("swift_contract_array_data")?;
        let ptr = unsafe { func(object) };
        Ok(ptr as *const i32)
    }

    /// Reads array elements through the exported contiguous storage pointer.
    pub fn array_elements_via_pointer(
        &self,
        object: OpaqueSwiftRef,
    ) -> Result<Vec<i32>, RuntimeContractError> {
        let len = self.array_len(object)?;
        if len == 0 {
            return Ok(Vec::new());
        }

        let ptr = self.array_data(object)?;
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 4,
                method_id: 5,
            });
        }

        let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
        Ok(slice.to_vec())
    }

    /// Creates a new Array<OpaqueRef> with optional preallocated capacity.
    pub fn construct_array_ref(
        &self,
        capacity: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractArrayRefMake = self.resolve("swift_contract_array_ref_make")?;
        let object = unsafe { func(capacity) };
        if object.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 5,
                method_id: 0,
            });
        }
        Ok(ContractObject {
            type_id: 5,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Returns the number of elements in Array<OpaqueRef>.
    pub fn array_ref_len(&self, object: OpaqueSwiftRef) -> Result<i32, RuntimeContractError> {
        let func: ContractArrayRefLen = self.resolve("swift_contract_array_ref_len")?;
        let len = unsafe { func(object) };
        if len < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 5,
                method_id: 1,
            });
        }
        Ok(len)
    }

    /// Gets an opaque element pointer at the provided index.
    pub fn array_ref_get(
        &self,
        object: OpaqueSwiftRef,
        index: i32,
    ) -> Result<OpaqueSwiftRef, RuntimeContractError> {
        let func: ContractArrayRefGet = self.resolve("swift_contract_array_ref_get")?;
        let value = unsafe { func(object, index) };
        if value.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 5,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Sets an opaque element pointer at the provided index.
    pub fn array_ref_set(
        &self,
        object: OpaqueSwiftRef,
        index: i32,
        value: OpaqueSwiftRef,
    ) -> Result<(), RuntimeContractError> {
        let func: ContractArrayRefSet = self.resolve("swift_contract_array_ref_set")?;
        let result = unsafe { func(object, index, value) };
        if result < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 5,
                method_id: 3,
            });
        }
        Ok(())
    }

    /// Appends an opaque element pointer and returns the new count.
    pub fn array_ref_append(
        &self,
        object: OpaqueSwiftRef,
        value: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractArrayRefAppend = self.resolve("swift_contract_array_ref_append")?;
        let new_count = unsafe { func(object, value) };
        if new_count < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 5,
                method_id: 4,
            });
        }
        Ok(new_count)
    }

    /// Creates a new Dictionary<Int32, Int32> with optional preallocated capacity.
    pub fn construct_dict_i32_i32(
        &self,
        capacity: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractDictI32Make = self.resolve("swift_contract_dict_i32_make")?;
        let object = unsafe { func(capacity) };
        if object.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 6,
                method_id: 0,
            });
        }
        Ok(ContractObject {
            type_id: 6,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Returns the number of key-value pairs in the dictionary.
    pub fn dict_i32_i32_len(&self, object: OpaqueSwiftRef) -> Result<i32, RuntimeContractError> {
        let func: ContractDictI32Len = self.resolve("swift_contract_dict_i32_len")?;
        let len = unsafe { func(object) };
        if len < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 6,
                method_id: 1,
            });
        }
        Ok(len)
    }

    /// Gets a value by key. Returns Ok(Some(value)) when found, Ok(None) when absent.
    pub fn dict_i32_i32_get(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
    ) -> Result<Option<i32>, RuntimeContractError> {
        let func: ContractDictI32Get = self.resolve("swift_contract_dict_i32_get")?;
        let mut out_value = 0i32;
        let status = unsafe { func(object, key, &mut out_value as *mut i32) };
        match status {
            1 => Ok(Some(out_value)),
            0 => Ok(None),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 6,
                method_id: 2,
            }),
        }
    }

    /// Inserts or updates a key and returns the resulting count.
    pub fn dict_i32_i32_set(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
        value: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractDictI32Set = self.resolve("swift_contract_dict_i32_set")?;
        let count = unsafe { func(object, key, value) };
        if count < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 6,
                method_id: 3,
            });
        }
        Ok(count)
    }

    /// Removes a key and returns the removed value when present.
    pub fn dict_i32_i32_remove(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
    ) -> Result<Option<i32>, RuntimeContractError> {
        let func: ContractDictI32Remove = self.resolve("swift_contract_dict_i32_remove")?;
        let mut out_value = 0i32;
        let status = unsafe { func(object, key, &mut out_value as *mut i32) };
        match status {
            1 => Ok(Some(out_value)),
            0 => Ok(None),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 6,
                method_id: 4,
            }),
        }
    }

    /// Checks whether the dictionary contains a key.
    pub fn dict_i32_i32_contains(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractDictI32Contains = self.resolve("swift_contract_dict_i32_contains")?;
        let status = unsafe { func(object, key) };
        match status {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 6,
                method_id: 5,
            }),
        }
    }

    // MARK: - Dictionary<Int32, OpaqueRef> (Track C.3 generic allocator, type_id 7)

    /// Creates a new Dictionary<Int32, OpaqueRef> with optional preallocated capacity.
    pub fn construct_dict_ref(
        &self,
        capacity: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractDictRefMake = self.resolve("swift_contract_dict_ref_make")?;
        let object = unsafe { func(capacity) };
        if object.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 7,
                method_id: 0,
            });
        }
        Ok(ContractObject {
            type_id: 7,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Returns the number of key-value pairs in Dictionary<Int32, OpaqueRef>.
    pub fn dict_ref_len(&self, object: OpaqueSwiftRef) -> Result<i32, RuntimeContractError> {
        let func: ContractDictRefLen = self.resolve("swift_contract_dict_ref_len")?;
        let len = unsafe { func(object) };
        if len < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 7,
                method_id: 1,
            });
        }
        Ok(len)
    }

    /// Gets an opaque value by Int32 key. Returns Ok(Some(ptr)) when found, Ok(None) when absent.
    pub fn dict_ref_get(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
    ) -> Result<Option<OpaqueSwiftRef>, RuntimeContractError> {
        let func: ContractDictRefGet = self.resolve("swift_contract_dict_ref_get")?;
        let mut out_ptr: *mut c_void = std::ptr::null_mut();
        let status = unsafe { func(object, key, &mut out_ptr as *mut *mut c_void) };
        match status {
            1 => Ok(Some(out_ptr)),
            0 => Ok(None),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 7,
                method_id: 2,
            }),
        }
    }

    /// Inserts or updates a key with an opaque ref value; returns resulting count.
    pub fn dict_ref_set(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
        value: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractDictRefSet = self.resolve("swift_contract_dict_ref_set")?;
        let count = unsafe { func(object, key, value) };
        if count < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 7,
                method_id: 3,
            });
        }
        Ok(count)
    }

    /// Removes a key and returns the removed opaque ref value, or None if absent.
    pub fn dict_ref_remove(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
    ) -> Result<Option<OpaqueSwiftRef>, RuntimeContractError> {
        let func: ContractDictRefRemove = self.resolve("swift_contract_dict_ref_remove")?;
        let removed = unsafe { func(object, key) };
        if removed.is_null() {
            Ok(None)
        } else {
            Ok(Some(removed))
        }
    }

    /// Checks whether Dictionary<Int32, OpaqueRef> contains a key.
    pub fn dict_ref_contains(
        &self,
        object: OpaqueSwiftRef,
        key: i32,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractDictRefContains = self.resolve("swift_contract_dict_ref_contains")?;
        let status = unsafe { func(object, key) };
        match status {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 7,
                method_id: 5,
            }),
        }
    }

    /// Wraps a contract object in a type-erased ContractAnyBox (type_id = 8).
    /// The inner object's ownership is not transferred; release the box separately.
    pub fn wrap_any_object(
        &self,
        type_id: i32,
        object: OpaqueSwiftRef,
    ) -> Result<OpaqueSwiftRef, RuntimeContractError> {
        let func: ContractAnyWrap = self.resolve("swift_contract_any_wrap")?;
        let ptr = unsafe { func(type_id, object) };
        if ptr.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 8 });
        }
        Ok(ptr)
    }

    /// Returns the contract type_id stored inside a ContractAnyBox (metatype identity check).
    pub fn any_object_type_id(&self, any_box: OpaqueSwiftRef) -> Result<i32, RuntimeContractError> {
        let func: ContractAnyTypeId = self.resolve("swift_contract_any_type_id")?;
        let id = unsafe { func(any_box) };
        if id < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 8,
                method_id: 0,
            });
        }
        Ok(id)
    }

    /// Attempts a dynamic narrowing cast. Returns `Some(inner_ptr)` when the
    /// ContractAnyBox holds an object of `target_type_id`, or `None` on cast failure.
    /// Mirrors Swift's `as?` semantics across the FFI boundary.
    pub fn dynamic_cast(
        &self,
        any_box: OpaqueSwiftRef,
        target_type_id: i32,
    ) -> Result<Option<OpaqueSwiftRef>, RuntimeContractError> {
        let func: ContractDynamicCast = self.resolve("swift_contract_dynamic_cast")?;
        let ptr = unsafe { func(any_box, target_type_id) };
        Ok(if ptr.is_null() { None } else { Some(ptr) })
    }

    /// Construct a Direction enum case from a discriminant (0=north, 1=south, 2=east, 3=west).
    pub fn construct_direction(
        &self,
        case_id: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractDirectionMake = self.resolve("swift_contract_direction_make")?;
        let object = unsafe { func(case_id) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 9 });
        }
        Ok(ContractObject {
            type_id: 9,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Extract the case discriminant from a Direction enum (0=north, 1=south, 2=east, 3=west).
    pub fn direction_case(&self, dir: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractDirectionCase = self.resolve("swift_contract_direction_case")?;
        let case_id = unsafe { func(dir.object) };
        if case_id < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 9,
                method_id: 1,
            });
        }
        Ok(case_id)
    }

    /// Construct a Shape enum case: circle (radius as Float parameter).
    pub fn construct_shape_circle(
        &self,
        radius: f32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractShapeCircle = self.resolve("swift_contract_shape_circle")?;
        let object = unsafe { func(radius) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 10 });
        }
        Ok(ContractObject {
            type_id: 10,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Construct a Shape enum case: rectangle (width and height as Float parameters).
    pub fn construct_shape_rect(
        &self,
        width: f32,
        height: f32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractShapeRect = self.resolve("swift_contract_shape_rect")?;
        let object = unsafe { func(width, height) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 10 });
        }
        Ok(ContractObject {
            type_id: 10,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Extract the case discriminant from a Shape enum (0=circle, 1=rectangle).
    pub fn shape_get_case(&self, shape: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractShapeGetCase = self.resolve("swift_contract_shape_get_case")?;
        let case_id = unsafe { func(shape.object) };
        if case_id < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 10,
                method_id: 1,
            });
        }
        Ok(case_id)
    }

    /// Extract radius from a Shape.circle case. Returns error if shape is not circle.
    pub fn shape_circle_radius(&self, shape: ContractObject) -> Result<f32, RuntimeContractError> {
        let func: ContractShapeCircleRadius = self.resolve("swift_contract_shape_circle_radius")?;
        let radius = unsafe { func(shape.object) };
        // Negative radius indicates failure (not a circle case)
        if radius < 0.0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 10,
                method_id: 2,
            });
        }
        Ok(radius)
    }

    /// Extract width and height from a Shape.rectangle case.
    /// Returns (width, height) on success, or error if shape is not rectangle.
    pub fn shape_rect_dims(
        &self,
        shape: ContractObject,
    ) -> Result<(f32, f32), RuntimeContractError> {
        let func: ContractShapeRectDims = self.resolve("swift_contract_shape_rect_dims")?;
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        let ok = unsafe { func(shape.object, &mut width, &mut height) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 10,
                method_id: 3,
            });
        }
        Ok((width, height))
    }

    // MARK: - Struct Layout & Construction (Track F.1)

    /// Get the total size in bytes of TestPayload struct.
    pub fn struct_testpayload_size(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractStructLayoutSize = self.resolve("swift_struct_testpayload_size")?;
        Ok(unsafe { func() })
    }

    /// Get the stride (size + padding) of TestPayload struct.
    pub fn struct_testpayload_stride(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractStructLayoutStride = self.resolve("swift_struct_testpayload_stride")?;
        Ok(unsafe { func() })
    }

    /// Get the alignment requirement of TestPayload struct.
    pub fn struct_testpayload_alignment(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractStructLayoutAlignment =
            self.resolve("swift_struct_testpayload_alignment")?;
        Ok(unsafe { func() })
    }

    /// Get the byte offset of field_a in TestPayload.
    pub fn struct_testpayload_offset_field_a(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractStructLayoutFieldOffset =
            self.resolve("swift_struct_testpayload_offset_a")?;
        Ok(unsafe { func() })
    }

    /// Get the byte offset of field_b in TestPayload.
    pub fn struct_testpayload_offset_field_b(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractStructLayoutFieldOffset =
            self.resolve("swift_struct_testpayload_offset_b")?;
        Ok(unsafe { func() })
    }

    /// Get the byte offset of field_c in TestPayload.
    pub fn struct_testpayload_offset_field_c(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractStructLayoutFieldOffset =
            self.resolve("swift_struct_testpayload_offset_c")?;
        Ok(unsafe { func() })
    }

    /// Construct a TestPayload from raw bytes (field_a, field_b, field_c in order).
    pub fn construct_struct_testpayload(
        &self,
        field_a: i32,
        field_b: i64,
        field_c: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        // Pack the fields into a contiguous byte buffer: [field_a (4 bytes), field_b (8 bytes), field_c (4 bytes)]
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&field_a.to_ne_bytes());
        bytes.extend_from_slice(&[0u8; 4]); // Padding between field_a and field_b
        bytes.extend_from_slice(&field_b.to_ne_bytes());
        bytes.extend_from_slice(&field_c.to_ne_bytes());

        let func: ContractStructConstruct =
            self.resolve("swift_contract_struct_testpayload_construct")?;
        let object = unsafe { func(bytes.as_ptr() as *const c_void, bytes.len() as i32) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 11 });
        }
        Ok(ContractObject {
            type_id: 11,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Extract field_a (Int32) from a TestPayload.
    pub fn struct_testpayload_get_field_a(
        &self,
        obj: ContractObject,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractStructFieldGetI32 =
            self.resolve("swift_contract_struct_testpayload_get_field_a")?;
        let value = unsafe { func(obj.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 11,
                method_id: 1,
            });
        }
        Ok(value)
    }

    /// Extract field_b (Int64) from a TestPayload.
    pub fn struct_testpayload_get_field_b(
        &self,
        obj: ContractObject,
    ) -> Result<i64, RuntimeContractError> {
        let func: ContractStructFieldGetI64 =
            self.resolve("swift_contract_struct_testpayload_get_field_b")?;
        let value = unsafe { func(obj.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 11,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Extract field_c (Int32) from a TestPayload.
    pub fn struct_testpayload_get_field_c(
        &self,
        obj: ContractObject,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractStructFieldGetI32 =
            self.resolve("swift_contract_struct_testpayload_get_field_c")?;
        let value = unsafe { func(obj.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 11,
                method_id: 3,
            });
        }
        Ok(value)
    }

    // MARK: - Tuple Construction & Unpacking (Track F.2)

    /// Construct a Pair (2-element tuple) from two Int32 values.
    pub fn construct_tuple_pair(
        &self,
        first: i32,
        second: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractTuplePairConstruct =
            self.resolve("swift_contract_tuple_pair_construct")?;
        let object = unsafe { func(first, second) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 12 });
        }
        Ok(ContractObject {
            type_id: 12,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Extract the first element from a Pair.
    pub fn tuple_pair_get_first(&self, pair: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractTuplePairGetFirst =
            self.resolve("swift_contract_tuple_pair_get_first")?;
        let value = unsafe { func(pair.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 12,
                method_id: 1,
            });
        }
        Ok(value)
    }

    /// Extract the second element from a Pair.
    pub fn tuple_pair_get_second(&self, pair: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractTuplePairGetSecond =
            self.resolve("swift_contract_tuple_pair_get_second")?;
        let value = unsafe { func(pair.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 12,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Construct a Triple (3-element tuple) from three Int32 values.
    pub fn construct_tuple_triple(
        &self,
        first: i32,
        second: i32,
        third: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractTupleTripleConstruct =
            self.resolve("swift_contract_tuple_triple_construct")?;
        let object = unsafe { func(first, second, third) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 13 });
        }
        Ok(ContractObject {
            type_id: 13,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Extract the first element from a Triple.
    pub fn tuple_triple_get_first(
        &self,
        triple: ContractObject,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractTupleTripleGetFirst =
            self.resolve("swift_contract_tuple_triple_get_first")?;
        let value = unsafe { func(triple.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 13,
                method_id: 1,
            });
        }
        Ok(value)
    }

    /// Extract the second element from a Triple.
    pub fn tuple_triple_get_second(
        &self,
        triple: ContractObject,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractTupleTripleGetSecond =
            self.resolve("swift_contract_tuple_triple_get_second")?;
        let value = unsafe { func(triple.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 13,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Extract the third element from a Triple.
    pub fn tuple_triple_get_third(
        &self,
        triple: ContractObject,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractTupleTripleGetThird =
            self.resolve("swift_contract_tuple_triple_get_third")?;
        let value = unsafe { func(triple.object) };
        if value == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 13,
                method_id: 3,
            });
        }
        Ok(value)
    }

    // MARK: - Closure/Function Pointer Bridging (Track F.3)

    /// Construct a closure with a captured delta (single-arg adder).
    pub fn construct_closure_adder(
        &self,
        delta: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractClosureAdderConstruct =
            self.resolve("swift_contract_closure_make_adder")?;
        let object = unsafe { func(delta) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 14 });
        }
        Ok(ContractObject {
            type_id: 14,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Invoke a closure with a single argument.
    pub fn closure_invoke_adder(
        &self,
        closure: ContractObject,
        arg: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractClosureAdderInvoke =
            self.resolve("swift_contract_closure_invoke_adder")?;
        let result = unsafe { func(closure.object, arg) };
        if result == -999 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 14,
                method_id: 1,
            });
        }
        Ok(result)
    }

    /// Extract the captured delta from a closure.
    pub fn closure_get_capture(
        &self,
        closure: ContractObject,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractClosureAdderGetCapture =
            self.resolve("swift_contract_closure_get_capture")?;
        let capture = unsafe { func(closure.object) };
        if capture == -999 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 14,
                method_id: 2,
            });
        }
        Ok(capture)
    }

    /// Construct a multi-argument closure with factor and offset captures.
    pub fn construct_closure_multi(
        &self,
        factor: i32,
        offset: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractClosureMultiConstruct =
            self.resolve("swift_contract_closure_make_multi")?;
        let object = unsafe { func(factor, offset) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 15 });
        }
        Ok(ContractObject {
            type_id: 15,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Invoke a multi-argument closure with two Int32 arguments.
    pub fn closure_invoke_multi(
        &self,
        closure: ContractObject,
        arg_a: i32,
        arg_b: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractClosureMultiInvoke =
            self.resolve("swift_contract_closure_invoke_multi")?;
        let result = unsafe { func(closure.object, arg_a, arg_b) };
        if result == -999 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 15,
                method_id: 1,
            });
        }
        Ok(result)
    }

    /// Extract the factor from a multi-argument closure.
    pub fn closure_get_factor(&self, closure: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractClosureMultiGetFactor =
            self.resolve("swift_contract_closure_get_factor")?;
        let factor = unsafe { func(closure.object) };
        if factor == -999 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 15,
                method_id: 2,
            });
        }
        Ok(factor)
    }

    /// Extract the offset from a multi-argument closure.
    pub fn closure_get_offset(&self, closure: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractClosureMultiGetOffset =
            self.resolve("swift_contract_closure_get_offset")?;
        let offset = unsafe { func(closure.object) };
        if offset == -999 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 15,
                method_id: 3,
            });
        }
        Ok(offset)
    }

    // MARK: - Error Handling & Introspection (Track E.1)

    /// Construct a ValidationError with a code and store it for introspection.
    pub fn error_make_validation(&self, code: i32) -> Result<(), RuntimeContractError> {
        let func: ContractErrorMakeValidation =
            self.resolve("swift_contract_error_make_validation")?;
        let ok = unsafe { func(code) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 16,
                method_id: 1,
            });
        }
        Ok(())
    }

    /// Construct an IOError with a code and store it for introspection.
    pub fn error_make_io(&self, code: i32) -> Result<(), RuntimeContractError> {
        let func: ContractErrorMakeIO = self.resolve("swift_contract_error_make_io")?;
        let ok = unsafe { func(code) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 16,
                method_id: 2,
            });
        }
        Ok(())
    }

    /// Extract the error description as a Rust String. Must free the C string returned by Swift.
    pub fn error_get_description(&self) -> Result<String, RuntimeContractError> {
        let func: ContractErrorGetDescription =
            self.resolve("swift_contract_error_get_description")?;
        let c_str_ptr = unsafe { func() };
        if c_str_ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 16,
                method_id: 3,
            });
        }

        let c_str = unsafe { std::ffi::CStr::from_ptr(c_str_ptr) };
        let rust_string = c_str.to_string_lossy().into_owned();

        // Free the C string allocated by Swift (via strdup)
        unsafe { libc::free(c_str_ptr as *mut std::ffi::c_void) };

        Ok(rust_string)
    }

    /// Extract the error code from the stored error.
    pub fn error_get_code(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractErrorGetCode = self.resolve("swift_contract_error_get_code")?;
        let code = unsafe { func() };
        if code == -1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 16,
                method_id: 4,
            });
        }
        Ok(code)
    }

    /// Check if the stored error is a ValidationError.
    pub fn error_is_validation(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractErrorIsValidation = self.resolve("swift_contract_error_is_validation")?;
        let is_validation = unsafe { func() };
        Ok(is_validation == 1)
    }

    /// Check if the stored error is an IOError.
    pub fn error_is_io(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractErrorIsIO = self.resolve("swift_contract_error_is_io")?;
        let is_io = unsafe { func() };
        Ok(is_io == 1)
    }

    /// Clear the stored error.
    pub fn error_clear(&self) -> Result<(), RuntimeContractError> {
        let func: ContractErrorClear = self.resolve("swift_contract_error_clear")?;
        unsafe { func() };
        Ok(())
    }

    /// Construct an OutOfRange ValidationError with code and limit.
    pub fn error_make_out_of_range(
        &self,
        code: i32,
        limit: i32,
    ) -> Result<(), RuntimeContractError> {
        let func: ContractErrorMakeOutOfRange =
            self.resolve("swift_contract_error_make_out_of_range")?;
        let ok = unsafe { func(code, limit) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 16,
                method_id: 7,
            });
        }
        Ok(())
    }

    // MARK: - Structured Error Propagation (Track E.3)

    /// Construct a deterministic validation error context with a linked cause code.
    pub fn error_context_make_validation(
        &self,
        code: i32,
        cause_code: i32,
    ) -> Result<(), RuntimeContractError> {
        let func: ContractErrorContextMakeValidation =
            self.resolve("swift_contract_error_context_make_validation")?;
        let ok = unsafe { func(code, cause_code) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 18,
                method_id: 1,
            });
        }
        Ok(())
    }

    /// Construct a deterministic IO error context.
    pub fn error_context_make_io(&self, code: i32) -> Result<(), RuntimeContractError> {
        let func: ContractErrorContextMakeIO =
            self.resolve("swift_contract_error_context_make_io")?;
        let ok = unsafe { func(code) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 18,
                method_id: 2,
            });
        }
        Ok(())
    }

    /// Get the structured error context as JSON.
    pub fn error_context_get_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractErrorContextGetJSON =
            self.resolve("swift_contract_error_context_get_json")?;
        let c_str_ptr = unsafe { func() };
        if c_str_ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 18,
                method_id: 3,
            });
        }

        let c_str = unsafe { CStr::from_ptr(c_str_ptr) };
        let json = c_str.to_string_lossy().into_owned();
        unsafe { libc::free(c_str_ptr as *mut c_void) };
        Ok(json)
    }

    /// Get the structured error context as a compact logging string.
    pub fn error_context_get_string(&self) -> Result<String, RuntimeContractError> {
        let func: ContractErrorContextGetString =
            self.resolve("swift_contract_error_context_get_string")?;
        let c_str_ptr = unsafe { func() };
        if c_str_ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 18,
                method_id: 4,
            });
        }

        let c_str = unsafe { CStr::from_ptr(c_str_ptr) };
        let text = c_str.to_string_lossy().into_owned();
        unsafe { libc::free(c_str_ptr as *mut c_void) };
        Ok(text)
    }

    /// Replace stored structured error context from JSON.
    pub fn error_context_set_json(&self, json: &str) -> Result<(), RuntimeContractError> {
        let func: ContractErrorContextSetJSON =
            self.resolve("swift_contract_error_context_set_json")?;
        let json_c = CString::new(json).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 18,
            method_id: 5,
        })?;
        let ok = unsafe { func(json_c.as_ptr()) };
        if ok != 1 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 18,
                method_id: 5,
            });
        }
        Ok(())
    }

    /// Clear the stored structured error context.
    pub fn error_context_clear(&self) -> Result<(), RuntimeContractError> {
        let func: ContractErrorContextClear = self.resolve("swift_contract_error_context_clear")?;
        unsafe { func() };
        Ok(())
    }

    /// Parse the structured error context into a typed Rust payload.
    pub fn error_context_parse(&self) -> Result<ContractErrorContextPayload, RuntimeContractError> {
        let json = self.error_context_get_json()?;
        serde_json::from_str::<ContractErrorContextPayload>(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    // MARK: - Task Creation & Continuation (Track G.1)

    /// Spawn a Swift Task and return sum result.
    pub fn task_spawn_sum(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractTaskSpawnSum = self.resolve("swift_contract_task_spawn_sum")?;
        let result = unsafe { func(a, b) };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 19,
                method_id: 1,
            });
        }
        Ok(result)
    }

    /// Spawn a Swift Task with deterministic yield chain.
    pub fn task_spawn_chain(&self, base: i32, steps: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractTaskSpawnChain = self.resolve("swift_contract_task_spawn_chain")?;
        let result = unsafe { func(base, steps) };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 19,
                method_id: 2,
            });
        }
        Ok(result)
    }

    /// Reset continuation resume counter in Swift bridge.
    pub fn continuation_reset(&self) -> Result<(), RuntimeContractError> {
        let func: ContractContinuationReset = self.resolve("swift_contract_continuation_reset")?;
        unsafe { func() };
        Ok(())
    }

    /// Return continuation resume counter value.
    pub fn continuation_resume_count(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractContinuationResumeCount =
            self.resolve("swift_contract_continuation_resume_count")?;
        Ok(unsafe { func() })
    }

    /// Run continuation round-trip and return resumed value.
    pub fn continuation_roundtrip(&self, value: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractContinuationRoundtrip =
            self.resolve("swift_contract_continuation_roundtrip")?;
        let result = unsafe { func(value) };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 19,
                method_id: 3,
            });
        }
        Ok(result)
    }

    /// Validate continuation resume-once safety semantics.
    pub fn continuation_validate_resume_once(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractContinuationValidateResumeOnce =
            self.resolve("swift_contract_continuation_validate_resume_once")?;
        let ok = unsafe { func() };
        match ok {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 19,
                method_id: 4,
            }),
        }
    }

    // MARK: - Actor Isolation & Isolation Domains (Track G.2)

    /// Construct a probe actor object with initial state.
    pub fn construct_actor(&self, start: i32) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractActorMake = self.resolve("swift_contract_actor_make")?;
        let object = unsafe { func(start) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 20 });
        }
        Ok(ContractObject {
            type_id: 20,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Read current actor state through isolated method invocation.
    pub fn actor_current(&self, actor: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractActorCurrent = self.resolve("swift_contract_actor_current")?;
        let value = unsafe { func(actor.object) };
        if value == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 20,
                method_id: 1,
            });
        }
        Ok(value)
    }

    /// Invoke isolated actor mutation and return updated value.
    pub fn actor_add(
        &self,
        actor: ContractObject,
        delta: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractActorAdd = self.resolve("swift_contract_actor_add")?;
        let value = unsafe { func(actor.object, delta) };
        if value == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 20,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Validate actor isolation under concurrent task access.
    pub fn actor_validate_isolation(
        &self,
        actor: ContractObject,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractActorValidateIsolation =
            self.resolve("swift_contract_actor_validate_isolation")?;
        let ok = unsafe { func(actor.object) };
        match ok {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 20,
                method_id: 3,
            }),
        }
    }

    // MARK: - Async Streams & AsyncSequence (Track G.3)

    /// Construct an async stream iterator object.
    pub fn construct_stream(
        &self,
        start: i32,
        count: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractStreamMake = self.resolve("swift_contract_stream_make")?;
        let object = unsafe { func(start, count) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 21 });
        }
        Ok(ContractObject {
            type_id: 21,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Pull next value from async iterator. Returns None when sequence is exhausted.
    pub fn stream_next(&self, stream: ContractObject) -> Result<Option<i32>, RuntimeContractError> {
        let func: ContractStreamNext = self.resolve("swift_contract_stream_next")?;
        let mut value: i32 = 0;
        let status = unsafe { func(stream.object, &mut value as *mut i32) };
        match status {
            1 => Ok(Some(value)),
            0 => Ok(None),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 21,
                method_id: 1,
            }),
        }
    }

    /// Collect stream sum entirely on Swift side via AsyncSequence iteration.
    pub fn stream_collect_sum(&self, start: i32, count: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractStreamCollectSum = self.resolve("swift_contract_stream_collect_sum")?;
        let sum = unsafe { func(start, count) };
        if sum == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 21,
                method_id: 2,
            });
        }
        Ok(sum)
    }

    // MARK: - Task-Local Values (Track G.4)

    /// Read default task-local value (outside injected scope).
    pub fn task_local_get_default(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractTaskLocalGetDefault =
            self.resolve("swift_contract_task_local_get_default")?;
        Ok(unsafe { func() })
    }

    /// Run a task-local scope and return computed value from inherited child task.
    pub fn task_local_run_with(&self, value: i32, delta: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractTaskLocalRunWith = self.resolve("swift_contract_task_local_run_with")?;
        let result = unsafe { func(value, delta) };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 22,
                method_id: 1,
            });
        }
        Ok(result)
    }

    /// Validate that inherited child task sees parent value while detached task sees default.
    pub fn task_local_isolation_check(
        &self,
        parent_value: i32,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractTaskLocalIsolationCheck =
            self.resolve("swift_contract_task_local_isolation_check")?;
        let ok = unsafe { func(parent_value) };
        match ok {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(RuntimeContractError::InvalidInvoke {
                type_id: 22,
                method_id: 2,
            }),
        }
    }

    // MARK: - Generic Metadata Accessor Chains (Track H.1)

    /// Validate that a concrete substitution string is supported by generic wrappers.
    pub fn generic_validate_substitution(
        &self,
        type_name: &str,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractGenericValidateSubstitution =
            self.resolve("swift_contract_generic_validate_substitution")?;
        let c_name = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 23,
            method_id: 1,
        })?;
        Ok(unsafe { func(c_name.as_ptr()) } == 1)
    }

    /// Construct a generic ContractGenericBox<Int32> object.
    pub fn construct_generic_box_i32(
        &self,
        value: i32,
    ) -> Result<ContractObject, RuntimeContractError> {
        let func: ContractGenericBoxI32Make =
            self.resolve("swift_contract_generic_box_i32_make")?;
        let object = unsafe { func(value) };
        if object.is_null() {
            return Err(RuntimeContractError::NullConstruct { type_id: 23 });
        }
        Ok(ContractObject {
            type_id: 23,
            object,
            ownership: ContractOwnership::SwiftRetained,
        })
    }

    /// Read value from generic ContractGenericBox<Int32>.
    pub fn generic_box_i32_get(&self, object: ContractObject) -> Result<i32, RuntimeContractError> {
        let func: ContractGenericBoxI32Get = self.resolve("swift_contract_generic_box_i32_get")?;
        let value = unsafe { func(object.object) };
        if value == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 23,
                method_id: 2,
            });
        }
        Ok(value)
    }

    /// Instantiate Array<Int32> generic specialization and return deterministic sum.
    pub fn generic_array_i32_sum(
        &self,
        start: i32,
        count: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractGenericArrayI32Sum =
            self.resolve("swift_contract_generic_array_i32_sum")?;
        Ok(unsafe { func(start, count) })
    }

    /// Instantiate Dictionary<String, Int32> generic specialization and return deterministic sum.
    pub fn generic_dict_string_i32_sum(&self, base: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractGenericDictStringI32Sum =
            self.resolve("swift_contract_generic_dict_string_i32_sum")?;
        Ok(unsafe { func(base) })
    }

    // MARK: - Generic Protocol Witness Lookup (Track H.2)

    /// Validate generic protocol conformance for Array<Int32>: Sequence.
    pub fn generic_protocol_array_i32_sequence_supported(
        &self,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractGenericProtocolArrayI32SequenceSupported =
            self.resolve("swift_contract_generic_protocol_array_i32_sequence_supported")?;
        Ok(unsafe { func() } == 1)
    }

    /// Resolve generic subscript behavior for Array<Int32> specialization.
    pub fn generic_protocol_array_i32_subscript(
        &self,
        index: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractGenericProtocolArrayI32Subscript =
            self.resolve("swift_contract_generic_protocol_array_i32_subscript")?;
        let value = unsafe { func(index) };
        if value == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 24,
                method_id: 1,
            });
        }
        Ok(value)
    }

    /// Return a stable non-zero token representing generic witness-resolution path.
    pub fn generic_protocol_array_i32_witness_token(&self) -> Result<u64, RuntimeContractError> {
        let func: ContractGenericProtocolArrayI32WitnessToken =
            self.resolve("swift_contract_generic_protocol_array_i32_witness_token")?;
        let token = unsafe { func() };
        if token == 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 24,
                method_id: 2,
            });
        }
        Ok(token)
    }

    /// Validate generic protocol support for Dictionary<String, Int32>.
    pub fn generic_protocol_dict_string_i32_supported(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractGenericProtocolDictStringI32Supported =
            self.resolve("swift_contract_generic_protocol_dict_string_i32_supported")?;
        Ok(unsafe { func() } == 1)
    }

    /// Resolve generic dictionary lookup for Dictionary<String, Int32> specialization.
    pub fn generic_protocol_dict_string_i32_lookup(
        &self,
        key: &str,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractGenericProtocolDictStringI32Lookup =
            self.resolve("swift_contract_generic_protocol_dict_string_i32_lookup")?;
        let c_key = CString::new(key).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 24,
            method_id: 3,
        })?;
        let value = unsafe { func(c_key.as_ptr()) };
        if value == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 24,
                method_id: 3,
            });
        }
        Ok(value)
    }

    // MARK: - Constrained Generic Bounds (Track H.3)

    /// `a == b` via `ContractEquatableBox<Int32>` — returns 1 if equal, 0 if not.
    pub fn constrained_equatable_equal(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractConstrainedEquatableEqual =
            self.resolve("swift_contract_constrained_equatable_equal")?;
        Ok(unsafe { func(a, b) })
    }

    /// Comparison result (-1 / 0 / 1) via `ContractComparableBox<Int32>`.
    pub fn constrained_comparable_cmp(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractConstrainedComparableCmp =
            self.resolve("swift_contract_constrained_comparable_cmp")?;
        Ok(unsafe { func(a, b) })
    }

    /// Count of distinct values in (a, b, c) using `Set<T>` constrained by `T: Hashable`.
    pub fn constrained_hashable_distinct_count(
        &self,
        a: i32,
        b: i32,
        c: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractConstrainedHashableDistinctCount =
            self.resolve("swift_contract_constrained_hashable_distinct_count")?;
        Ok(unsafe { func(a, b, c) })
    }

    /// `a + b` via a generic function constrained by `T: AdditiveArithmetic`.
    pub fn constrained_additive_sum(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractConstrainedAdditiveSum =
            self.resolve("swift_contract_constrained_additive_sum")?;
        Ok(unsafe { func(a, b) })
    }

    /// JSON-encode then decode `v` — exercises `T: Codable` generic constraint.
    pub fn constrained_codable_roundtrip(&self, v: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractConstrainedCodableRoundtrip =
            self.resolve("swift_contract_constrained_codable_roundtrip")?;
        let result = unsafe { func(v) };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 25,
                method_id: 5,
            });
        }
        Ok(result)
    }

    /// `min(a, b)` via a double-constrained generic (`T: Comparable & Hashable`).
    pub fn constrained_multi_min(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractConstrainedMultiMin =
            self.resolve("swift_contract_constrained_multi_min")?;
        Ok(unsafe { func(a, b) })
    }

    // MARK: - Arbitrary Generic/Witness Instantiation (Track N.3)

    /// Build a runtime generic context from a type name and optional semicolon-separated constraints.
    pub fn n3_build_context_json(
        &self,
        type_name: &str,
        constraints: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractN3BuildContextJson =
            self.resolve("swift_contract_n3_build_context_json")?;
        let c_type = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 72,
            method_id: 1,
        })?;
        let c_constraints =
            CString::new(constraints).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 1,
            })?;
        let ptr = unsafe { func(c_type.as_ptr(), c_constraints.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 1,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Resolve a protocol witness path for a type plus optional semicolon-separated requirements.
    pub fn n3_resolve_witness_json(
        &self,
        type_name: &str,
        protocol_name: &str,
        requirements: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractN3ResolveWitnessJson =
            self.resolve("swift_contract_n3_resolve_witness_json")?;
        let c_type = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 72,
            method_id: 2,
        })?;
        let c_protocol =
            CString::new(protocol_name).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 2,
            })?;
        let c_requirements =
            CString::new(requirements).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 2,
            })?;
        let ptr = unsafe {
            func(
                c_type.as_ptr(),
                c_protocol.as_ptr(),
                c_requirements.as_ptr(),
            )
        };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 2,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Validate a generic requirement set and return machine-readable JSON diagnostics.
    pub fn n3_validate_requirements_json(
        &self,
        type_name: &str,
        requirements: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractN3ValidateRequirementsJson =
            self.resolve("swift_contract_n3_validate_requirements_json")?;
        let c_type = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 72,
            method_id: 3,
        })?;
        let c_requirements =
            CString::new(requirements).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 3,
            })?;
        let ptr = unsafe { func(c_type.as_ptr(), c_requirements.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Dispatch a generic/protocol-bound operation using only type and requirement strings.
    pub fn n3_invoke_generic_i32(
        &self,
        type_name: &str,
        requirements: &str,
        operation: &str,
        a: i32,
        b: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractN3InvokeGenericI32 =
            self.resolve("swift_contract_n3_invoke_generic_i32")?;
        let c_type = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 72,
            method_id: 4,
        })?;
        let c_requirements =
            CString::new(requirements).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 4,
            })?;
        let c_operation =
            CString::new(operation).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: 4,
            })?;
        let mut error = 0i32;
        let out = unsafe {
            func(
                c_type.as_ptr(),
                c_requirements.as_ptr(),
                c_operation.as_ptr(),
                a,
                b,
                &mut error,
            )
        };
        if error != 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 72,
                method_id: match error {
                    -531 => 5,
                    -532 => 6,
                    _ => 4,
                },
            });
        }
        Ok(out)
    }

    // MARK: - Unsafe Runtime Ops Isolation & Recovery (Track N.4)

    /// Deterministic safe op for subprocess/broker validation.
    pub fn n4_safe_ping(&self, value: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractN4SafePing = self.resolve("swift_contract_n4_safe_ping")?;
        Ok(unsafe { func(value) })
    }

    /// Deliberately abort the current process. Must only be used from the broker subprocess.
    pub fn n4_trigger_abort(&self) -> Result<(), RuntimeContractError> {
        let func: ContractN4TriggerAbort = self.resolve("swift_contract_n4_trigger_abort")?;
        unsafe { func() };
        Err(RuntimeContractError::InvalidInvoke {
            type_id: 73,
            method_id: 2,
        })
    }

    // MARK: - Cross-Version ABI Adaptation Layer (Track N.5)

    /// Return the exported adapter table describing supported Swift toolchain families.
    pub fn n5_adapter_table_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractN5AdapterTableJson =
            self.resolve("swift_contract_n5_adapter_table_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 74,
                method_id: 1,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Parse the exported adapter table for use in Rust-side selection and validation.
    pub fn n5_adapter_table(&self) -> Result<N5AdapterTable, RuntimeContractError> {
        let json = self.n5_adapter_table_json()?;
        serde_json::from_str(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    /// Return the current runtime feature probe snapshot.
    pub fn n5_feature_probe_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractN5FeatureProbeJson =
            self.resolve("swift_contract_n5_feature_probe_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 74,
                method_id: 2,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Parse the current runtime feature probe snapshot.
    pub fn n5_feature_probe(&self) -> Result<N5FeatureProbe, RuntimeContractError> {
        let json = self.n5_feature_probe_json()?;
        serde_json::from_str(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    /// Return the adapter profile selected by the Swift bridge for the current runtime.
    pub fn n5_select_adapter_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractN5SelectAdapterJson =
            self.resolve("swift_contract_n5_select_adapter_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 74,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Parse the adapter profile selected by the Swift bridge for the current runtime.
    pub fn n5_select_adapter(&self) -> Result<N5SelectedAdapter, RuntimeContractError> {
        let json = self.n5_select_adapter_json()?;
        serde_json::from_str(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    /// Select the best adapter profile for a given feature probe using the exported adapter table.
    pub fn n5_select_profile_from_table(
        table: &N5AdapterTable,
        probe: &N5FeatureProbe,
    ) -> Option<N5SelectedAdapter> {
        table.profiles.iter().find_map(|profile| {
            if profile.compiler_family != probe.compiler_family
                || !profile.platforms.iter().any(|platform| platform == &probe.platform)
                || !profile
                    .architectures
                    .iter()
                    .any(|architecture| architecture == &probe.architecture)
                || !profile
                    .supported_optimization_modes
                    .iter()
                    .any(|mode| mode == &probe.optimization_mode)
            {
                return None;
            }

            let missing_features: Vec<String> = profile
                .required_features
                .iter()
                .filter(|feature| !probe.feature_enabled(feature))
                .cloned()
                .collect();
            if !missing_features.is_empty() {
                return None;
            }

            let selected_symbols = profile
                .symbol_aliases
                .iter()
                .filter_map(|(name, aliases)| {
                    aliases
                        .first()
                        .map(|alias| (name.clone(), alias.clone()))
                })
                .collect();

            Some(N5SelectedAdapter {
                profile_id: profile.profile_id.clone(),
                compatible: true,
                reason: "selected from exported adapter table using compiler family, platform, architecture, optimization mode, and feature probes".to_string(),
                compiler_family: probe.compiler_family.clone(),
                optimization_mode: probe.optimization_mode.clone(),
                selected_symbols,
                missing_features,
                adaptation_notes: profile.adaptation_notes.clone(),
            })
        })
    }

    /// Compare an expected and observed adapter selection and report any behavior drift.
    pub fn n5_regression_report(
        expected: &N5SelectedAdapter,
        observed: &N5SelectedAdapter,
        mode: &str,
    ) -> N5DriftReport {
        let mut issues = Vec::new();

        if expected.profile_id != observed.profile_id {
            issues.push(format!(
                "profile drift: expected {} but observed {}",
                expected.profile_id, observed.profile_id
            ));
        }
        if expected.compiler_family != observed.compiler_family {
            issues.push(format!(
                "compiler-family drift: expected {} but observed {}",
                expected.compiler_family, observed.compiler_family
            ));
        }
        if observed.optimization_mode != mode {
            issues.push(format!(
                "optimization-mode drift: expected {} but observed {}",
                mode, observed.optimization_mode
            ));
        }
        if !observed.missing_features.is_empty() {
            issues.push(format!(
                "missing required features: {}",
                observed.missing_features.join(",")
            ));
        }
        for (name, expected_symbol) in &expected.selected_symbols {
            match observed.selected_symbols.get(name) {
                Some(observed_symbol) if observed_symbol == expected_symbol => {}
                Some(observed_symbol) => issues.push(format!(
                    "symbol drift for {}: expected {} but observed {}",
                    name, expected_symbol, observed_symbol
                )),
                None => issues.push(format!("symbol missing for {}", name)),
            }
        }

        N5DriftReport {
            mode: mode.to_string(),
            drift_detected: !issues.is_empty(),
            issues,
        }
    }

    // MARK: - Differential Fuzzing & Semantic Oracle (Track N.6)

    /// Generate a deterministic Swift-side random program corpus for the given seed.
    pub fn n6_generate_program_json(
        &self,
        seed: i64,
        fragment_count: i32,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractN6GenerateProgramJson =
            self.resolve("swift_contract_n6_generate_program_json")?;
        let ptr = unsafe { func(seed, fragment_count) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 75,
                method_id: 1,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Parse a deterministic Swift-side random program corpus for the given seed.
    pub fn n6_generate_program(
        &self,
        seed: i64,
        fragment_count: i32,
    ) -> Result<N6Program, RuntimeContractError> {
        let json = self.n6_generate_program_json(seed, fragment_count)?;
        serde_json::from_str(&json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    /// Execute a generated program natively on the Swift side and return the semantic trace JSON.
    pub fn n6_execute_program_json(
        &self,
        program_json: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractN6ExecuteProgramJson =
            self.resolve("swift_contract_n6_execute_program_json")?;
        let json_c =
            CString::new(program_json).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 75,
                method_id: 2,
            })?;
        let ptr = unsafe { func(json_c.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 75,
                method_id: 2,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Execute a generated program natively on the Swift side and parse the semantic trace.
    pub fn n6_execute_program(
        &self,
        program: &N6Program,
    ) -> Result<N6Execution, RuntimeContractError> {
        let json = serde_json::to_string(program)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))?;
        let output = self.n6_execute_program_json(&json)?;
        serde_json::from_str(&output)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))
    }

    // MARK: - Binary-Driven Contract Derivation (Track N.7)

    fn n7_binary_symbols(&self, binary_path: &str) -> Result<Vec<String>, RuntimeContractError> {
        let output = Command::new("nm")
            .args(["-gj", binary_path])
            .output()
            .map_err(|error| {
                RuntimeContractError::DescriptorParse(format!("nm invocation failed: {error}"))
            })?;
        if !output.status.success() {
            return Err(RuntimeContractError::DescriptorParse(format!(
                "nm failed for {}",
                binary_path
            )));
        }

        let text = String::from_utf8_lossy(&output.stdout);
        let mut symbols = Vec::new();
        for line in text.lines() {
            let symbol = line.trim();
            if symbol.is_empty() {
                continue;
            }
            if symbol.starts_with("__") && !symbol.starts_with("__$") {
                continue;
            }
            let normalized = symbol.strip_prefix('_').unwrap_or(symbol).to_string();
            symbols.push(normalized);
        }
        symbols.sort();
        symbols.dedup();
        Ok(symbols)
    }

    fn n7_demangle(symbol: &str, demangler: &mut Option<SymbolDemangler>) -> String {
        if symbol.starts_with("$s") || symbol.starts_with("_$s") {
            if let Some(demangler) = demangler {
                return demangler.demangle(symbol);
            }
        }
        symbol.to_string()
    }

    fn n7_confidence_label(confidence: f64) -> &'static str {
        if confidence >= 0.85 {
            "high"
        } else if confidence >= 0.6 {
            "medium"
        } else {
            "low"
        }
    }

    fn n7_parse_type_entries(types_json: &str) -> Result<Vec<String>, RuntimeContractError> {
        let value: serde_json::Value = serde_json::from_str(types_json)
            .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))?;
        let types = value
            .get("types")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| {
                RuntimeContractError::DescriptorParse("missing types array".to_string())
            })?;

        let mut names = Vec::new();
        for entry in types {
            if let Some(name) = entry.get("name").and_then(serde_json::Value::as_str) {
                names.push(name.to_string());
            }
        }
        Ok(names)
    }

    /// Derive callable/type contract surfaces directly from a compiled module binary.
    pub fn n7_derive_contract_from_binary(
        &self,
        binary_path: &str,
        module_hint: &str,
    ) -> Result<N7DerivedContract, RuntimeContractError> {
        let symbols = self.n7_binary_symbols(binary_path)?;
        let mut demangler = SymbolDemangler::new().ok();
        let mut fallback_paths = Vec::new();
        if demangler.is_none() {
            fallback_paths.push(
                "swift-demangle unavailable; using symbol-name-only reconstruction".to_string(),
            );
        }

        let mut callables = Vec::new();
        let mut mangled_budget = 0usize;
        for symbol in &symbols {
            let candidate = symbol.starts_with("swift_")
                || symbol.starts_with("swift_contract_")
                || symbol.starts_with("runtime_thunk_")
                || symbol.starts_with("$s")
                || symbol.starts_with("_$s");
            if !candidate {
                continue;
            }

            let demangled = Self::n7_demangle(symbol, &mut demangler);
            if !demangled.contains(module_hint)
                && !symbol.contains("swift_contract_")
                && !symbol.contains("runtime_thunk_")
            {
                continue;
            }

            let observed_runtime = self.factory.symbol_address(symbol).is_ok();
            let (shape, shape_supported) = if symbol.starts_with("swift_contract_n2_unknown_") {
                match self.n2_symbol_describe(symbol) {
                    Ok((shape, supported)) => (Some(shape), supported),
                    Err(_) => {
                        fallback_paths.push(format!(
                            "shape discovery failed for {}; fallback to symbol-address invoke",
                            symbol
                        ));
                        (None, false)
                    }
                }
            } else {
                (None, false)
            };

            let mut confidence = 0.0;
            if symbol.starts_with("swift_contract_") || symbol.starts_with("runtime_thunk_") {
                confidence += 0.35;
            }
            if demangled != *symbol {
                confidence += 0.2;
            }
            if observed_runtime {
                confidence += 0.25;
            }
            if shape_supported {
                confidence += 0.2;
            }
            if confidence > 1.0 {
                confidence = 1.0;
            }

            let fallback = if observed_runtime {
                "runtime symbol-address resolution".to_string()
            } else {
                "defer to N.2 dynamic symbol invocation with explicit shape negotiation".to_string()
            };

            let keep = if symbol.starts_with("swift_contract_")
                || symbol.starts_with("runtime_thunk_")
                || symbol.starts_with("swift_")
            {
                true
            } else if observed_runtime && demangled.contains('(') {
                true
            } else if (symbol.starts_with("$s") || symbol.starts_with("_$s"))
                && demangled.contains(module_hint)
                && demangled.contains('(')
                && mangled_budget < 64
            {
                mangled_budget += 1;
                true
            } else {
                false
            };
            if !keep {
                continue;
            }

            callables.push(N7DerivedCallable {
                symbol: symbol.clone(),
                demangled,
                observed_runtime,
                shape,
                confidence,
                fallback,
            });
        }

        let all_types_json = self.n1_enumerate_all_types_json()?;
        let type_names = Self::n7_parse_type_entries(&all_types_json)?;
        let known_type_seeds = [
            "Person",
            "Counter",
            "Direction",
            "Shape",
            "ContractGenericBox<Int32>",
            "ContractGenericBox<String>",
            "N1LayoutStruct",
        ];
        let mut types = Vec::new();
        for name in type_names {
            let name_seeded = known_type_seeds.iter().any(|seed| name.contains(seed));
            if !name.contains(module_hint)
                && !name_seeded
                && !name.starts_with("Array<")
                && !name.starts_with("Dictionary<")
                && !name.starts_with("Set<")
            {
                continue;
            }

            let info_json = self.n1_type_info_json(&name)?;
            let info: serde_json::Value = serde_json::from_str(&info_json)
                .map_err(|error| RuntimeContractError::DescriptorParse(error.to_string()))?;
            let kind = info
                .get("kind")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown")
                .to_string();
            let field_count = info
                .get("field_count")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(-1) as i32;
            let observed_runtime = kind != "unknown";
            if !observed_runtime && !name_seeded {
                continue;
            }
            let symbol_hint = callables
                .iter()
                .any(|callable| callable.demangled.contains(&name));

            let mut confidence = 0.4;
            if observed_runtime {
                confidence += 0.3;
            }
            if field_count >= 0 {
                confidence += 0.2;
            }
            if symbol_hint {
                confidence += 0.1;
            }
            if confidence > 1.0 {
                confidence = 1.0;
            }

            types.push(N7DerivedType {
                name,
                kind,
                field_count,
                observed_runtime,
                confidence,
                fallback: "use N.1 metadata graph traversal as runtime source-of-truth".to_string(),
            });
        }

        callables.sort_by(|left, right| right.confidence.total_cmp(&left.confidence));
        types.sort_by(|left, right| right.confidence.total_cmp(&left.confidence));
        callables.truncate(256);
        types.truncate(128);

        if callables.is_empty() {
            fallback_paths.push(
                "no callable candidates discovered from nm output; fallback to runtime registry/metadata APIs"
                    .to_string(),
            );
        }
        if types.is_empty() {
            fallback_paths.push(
                "no type candidates discovered from binary stitching; fallback to n1_enumerate_all_types_json"
                    .to_string(),
            );
        }

        let mut confidence_sum = 0.0;
        let mut confidence_count = 0usize;
        for callable in &callables {
            confidence_sum += callable.confidence;
            confidence_count += 1;
        }
        for entry in &types {
            confidence_sum += entry.confidence;
            confidence_count += 1;
        }
        let confidence = if confidence_count == 0 {
            0.0
        } else {
            confidence_sum / confidence_count as f64
        };

        let mut low_confidence_regions = Vec::new();
        for callable in &callables {
            if callable.confidence < 0.6 {
                low_confidence_regions.push(format!(
                    "callable:{}:{}",
                    callable.symbol,
                    Self::n7_confidence_label(callable.confidence)
                ));
            }
        }
        for entry in &types {
            if entry.confidence < 0.6 {
                low_confidence_regions.push(format!(
                    "type:{}:{}",
                    entry.name,
                    Self::n7_confidence_label(entry.confidence)
                ));
            }
        }

        Ok(N7DerivedContract {
            binary_path: binary_path.to_string(),
            module_hint: module_hint.to_string(),
            callables,
            types,
            confidence,
            low_confidence_regions,
            fallback_paths,
        })
    }

    /// Validate a binary-derived contract against live runtime observations.
    pub fn n7_validate_derived_contract(
        &self,
        contract: &N7DerivedContract,
    ) -> Result<N7ValidationReport, RuntimeContractError> {
        let mut callable_ok = 0usize;
        for callable in &contract.callables {
            let validated = if callable.symbol.starts_with("swift_contract_n2_unknown_") {
                self.n2_symbol_describe(&callable.symbol)
                    .map(|(_, supported)| supported)
                    .unwrap_or(false)
            } else {
                self.factory.symbol_address(&callable.symbol).is_ok()
            };
            if validated {
                callable_ok += 1;
            }
        }

        let mut type_ok = 0usize;
        for entry in &contract.types {
            let validated = self
                .n1_type_info_json(&entry.name)
                .ok()
                .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
                .and_then(|value| {
                    value
                        .get("kind")
                        .and_then(serde_json::Value::as_str)
                        .map(|kind| kind != "unknown")
                })
                .unwrap_or(false);
            if validated {
                type_ok += 1;
            }
        }

        let callable_coverage = if contract.callables.is_empty() {
            0.0
        } else {
            callable_ok as f64 / contract.callables.len() as f64
        };
        let type_coverage = if contract.types.is_empty() {
            0.0
        } else {
            type_ok as f64 / contract.types.len() as f64
        };

        let confidence =
            (contract.confidence * 0.6) + (callable_coverage * 0.2) + (type_coverage * 0.2);

        Ok(N7ValidationReport {
            callable_coverage,
            type_coverage,
            validated_callables: callable_ok as i32,
            validated_types: type_ok as i32,
            confidence,
            low_confidence_regions: contract.low_confidence_regions.clone(),
        })
    }

    // MARK: - Operational Guarantees & SLOs (Track N.8)

    fn n8_percentile(samples: &[f64], ratio: f64) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        let mut sorted = samples.to_vec();
        sorted.sort_by(|left, right| left.total_cmp(right));
        let index = ((sorted.len() - 1) as f64 * ratio).round() as usize;
        sorted[index]
    }

    fn n8_process_rss_bytes() -> u64 {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
        let ok = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } == 0;
        if !ok {
            return 0;
        }
        let usage = unsafe { usage.assume_init() };
        #[cfg(target_os = "macos")]
        {
            if usage.ru_maxrss < 0 {
                0
            } else {
                usage.ru_maxrss as u64
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            if usage.ru_maxrss < 0 {
                0
            } else {
                (usage.ru_maxrss as u64) * 1024
            }
        }
    }

    pub fn n8_default_slos(&self) -> Vec<N8SloBudget> {
        vec![
            N8SloBudget {
                operation: "dynamic_invoke".to_string(),
                p50_latency_ms_budget: 0.40,
                p95_latency_ms_budget: 1.20,
                min_throughput_ops_per_sec: 3_000.0,
                max_rss_bytes: 512 * 1024 * 1024,
            },
            N8SloBudget {
                operation: "metadata_traversal".to_string(),
                p50_latency_ms_budget: 1.20,
                p95_latency_ms_budget: 5.00,
                min_throughput_ops_per_sec: 200.0,
                max_rss_bytes: 768 * 1024 * 1024,
            },
            N8SloBudget {
                operation: "graph_operations".to_string(),
                p50_latency_ms_budget: 2.00,
                p95_latency_ms_budget: 8.00,
                min_throughput_ops_per_sec: 100.0,
                max_rss_bytes: 1024 * 1024 * 1024,
            },
        ]
    }

    pub fn n8_measure_dynamic_invoke(
        &self,
        iterations: i32,
    ) -> Result<N8BenchmarkSample, RuntimeContractError> {
        if iterations <= 0 {
            return Err(RuntimeContractError::DescriptorParse(
                "iterations must be > 0".to_string(),
            ));
        }
        let mut latencies_ms = Vec::with_capacity(iterations as usize);
        let rss_before = Self::n8_process_rss_bytes();
        let started = Instant::now();

        for _ in 0..iterations {
            let one_start = Instant::now();
            let _ = self.n2_invoke_auto("swift_contract_n2_unknown_add_offset", 10, 5)?;
            latencies_ms.push(one_start.elapsed().as_secs_f64() * 1000.0);
        }

        let elapsed = started.elapsed().as_secs_f64();
        let throughput = if elapsed <= 0.0 {
            iterations as f64
        } else {
            iterations as f64 / elapsed
        };
        let rss_after = Self::n8_process_rss_bytes();

        Ok(N8BenchmarkSample {
            operation: "dynamic_invoke".to_string(),
            iterations,
            p50_latency_ms: Self::n8_percentile(&latencies_ms, 0.50),
            p95_latency_ms: Self::n8_percentile(&latencies_ms, 0.95),
            throughput_ops_per_sec: throughput,
            rss_bytes: rss_before.max(rss_after),
        })
    }

    pub fn n8_measure_metadata_traversal(
        &self,
        iterations: i32,
    ) -> Result<N8BenchmarkSample, RuntimeContractError> {
        if iterations <= 0 {
            return Err(RuntimeContractError::DescriptorParse(
                "iterations must be > 0".to_string(),
            ));
        }
        let mut latencies_ms = Vec::with_capacity(iterations as usize);
        let rss_before = Self::n8_process_rss_bytes();
        let started = Instant::now();

        for _ in 0..iterations {
            let one_start = Instant::now();
            let traversed = self.n1_metadata_graph_traverse_discovered_count()?;
            if traversed <= 0 {
                return Err(RuntimeContractError::DescriptorParse(
                    "n1 traversal count unexpectedly <= 0".to_string(),
                ));
            }
            latencies_ms.push(one_start.elapsed().as_secs_f64() * 1000.0);
        }

        let elapsed = started.elapsed().as_secs_f64();
        let throughput = if elapsed <= 0.0 {
            iterations as f64
        } else {
            iterations as f64 / elapsed
        };
        let rss_after = Self::n8_process_rss_bytes();

        Ok(N8BenchmarkSample {
            operation: "metadata_traversal".to_string(),
            iterations,
            p50_latency_ms: Self::n8_percentile(&latencies_ms, 0.50),
            p95_latency_ms: Self::n8_percentile(&latencies_ms, 0.95),
            throughput_ops_per_sec: throughput,
            rss_bytes: rss_before.max(rss_after),
        })
    }

    pub fn n8_measure_graph_operations(
        &self,
        iterations: i32,
    ) -> Result<N8BenchmarkSample, RuntimeContractError> {
        if iterations <= 0 {
            return Err(RuntimeContractError::DescriptorParse(
                "iterations must be > 0".to_string(),
            ));
        }
        let mut latencies_ms = Vec::with_capacity(iterations as usize);
        let rss_before = Self::n8_process_rss_bytes();
        let started = Instant::now();

        for _ in 0..iterations {
            let one_start = Instant::now();
            let snapshot = self.n1_metadata_snapshot_json()?;
            if snapshot.len() < 16 {
                return Err(RuntimeContractError::DescriptorParse(
                    "metadata snapshot unexpectedly tiny".to_string(),
                ));
            }
            latencies_ms.push(one_start.elapsed().as_secs_f64() * 1000.0);
        }

        let elapsed = started.elapsed().as_secs_f64();
        let throughput = if elapsed <= 0.0 {
            iterations as f64
        } else {
            iterations as f64 / elapsed
        };
        let rss_after = Self::n8_process_rss_bytes();

        Ok(N8BenchmarkSample {
            operation: "graph_operations".to_string(),
            iterations,
            p50_latency_ms: Self::n8_percentile(&latencies_ms, 0.50),
            p95_latency_ms: Self::n8_percentile(&latencies_ms, 0.95),
            throughput_ops_per_sec: throughput,
            rss_bytes: rss_before.max(rss_after),
        })
    }

    pub fn n8_run_benchmarks(
        &self,
        dynamic_iterations: i32,
        metadata_iterations: i32,
        graph_iterations: i32,
    ) -> Result<Vec<N8BenchmarkSample>, RuntimeContractError> {
        Ok(vec![
            self.n8_measure_dynamic_invoke(dynamic_iterations)?,
            self.n8_measure_metadata_traversal(metadata_iterations)?,
            self.n8_measure_graph_operations(graph_iterations)?,
        ])
    }

    pub fn n8_evaluate_budget_gates(
        &self,
        samples: &[N8BenchmarkSample],
        budgets: &[N8SloBudget],
    ) -> Vec<N8BudgetGate> {
        let mut gates = Vec::new();
        for budget in budgets {
            let sample = samples
                .iter()
                .find(|sample| sample.operation == budget.operation);
            let mut reasons = Vec::new();

            match sample {
                Some(sample) => {
                    if sample.p50_latency_ms > budget.p50_latency_ms_budget {
                        reasons.push(format!(
                            "p50 latency {:.3}ms exceeded budget {:.3}ms",
                            sample.p50_latency_ms, budget.p50_latency_ms_budget
                        ));
                    }
                    if sample.p95_latency_ms > budget.p95_latency_ms_budget {
                        reasons.push(format!(
                            "p95 latency {:.3}ms exceeded budget {:.3}ms",
                            sample.p95_latency_ms, budget.p95_latency_ms_budget
                        ));
                    }
                    if sample.throughput_ops_per_sec < budget.min_throughput_ops_per_sec {
                        reasons.push(format!(
                            "throughput {:.1} ops/sec below budget {:.1} ops/sec",
                            sample.throughput_ops_per_sec, budget.min_throughput_ops_per_sec
                        ));
                    }
                    if sample.rss_bytes > budget.max_rss_bytes {
                        reasons.push(format!(
                            "rss {} bytes exceeded budget {} bytes",
                            sample.rss_bytes, budget.max_rss_bytes
                        ));
                    }
                }
                None => reasons.push("benchmark sample missing".to_string()),
            }

            gates.push(N8BudgetGate {
                operation: budget.operation.clone(),
                passed: reasons.is_empty(),
                reasons,
            });
        }
        gates
    }

    pub fn n8_ci_budget_alerts(&self, gates: &[N8BudgetGate]) -> Vec<String> {
        let mut alerts = Vec::new();
        for gate in gates {
            if gate.passed {
                continue;
            }
            let detail = if gate.reasons.is_empty() {
                "budget gate failed with no reason".to_string()
            } else {
                gate.reasons.join("; ")
            };
            alerts.push(format!("ALERT[N8][{}]: {}", gate.operation, detail,));
        }
        alerts
    }

    pub fn n8_degraded_mode_runbook(&self) -> Result<String, RuntimeContractError> {
        let descriptor = self.descriptor()?;
        let probe = self.n5_feature_probe()?;
        let mut lines = vec![
            "# N.8 Degraded Mode Runbook".to_string(),
            "".to_string(),
            "When capability probes fail, switch to conservative paths to preserve correctness."
                .to_string(),
            "".to_string(),
            "## Capability Snapshot".to_string(),
            format!("- compiler_family: {}", probe.compiler_family),
            format!("- platform: {} {}", probe.platform, probe.architecture),
            format!("- optimization_mode: {}", probe.optimization_mode),
            "".to_string(),
            "## Degraded-Mode Steps".to_string(),
        ];

        let capabilities = descriptor.compiler_features;
        let feature_rows = [
            ("resilient_dispatch", capabilities.resilient_dispatch),
            (
                "generic_metadata_registry",
                capabilities.generic_metadata_registry,
            ),
            (
                "protocol_witness_registry",
                capabilities.protocol_witness_registry,
            ),
            (
                "raw_runtime_research_mode",
                capabilities.raw_runtime_research_mode,
            ),
        ];

        for (name, capability) in feature_rows {
            let status = match capability.status {
                CompilerFeatureStatus::Supported => "supported",
                CompilerFeatureStatus::Fallback => "fallback",
                CompilerFeatureStatus::Unsupported => "unsupported",
            };
            lines.push(format!(
                "- {}: {} (provider={}, reason={})",
                name, status, capability.provider, capability.reason
            ));
            if capability.status != CompilerFeatureStatus::Supported {
                lines.push(format!(
                    "  action: route {} calls through safe contract wrappers and disable direct fast-path dispatch",
                    name
                ));
            }
        }

        lines.push("".to_string());
        lines.push("## Incident Response".to_string());
        lines.push(
            "- Re-run capability probe and refresh adapter/profile selection (Track N.5)."
                .to_string(),
        );
        lines.push("- Enable additional telemetry for N.6 differential fuzzing and N.7 validation coverage.".to_string());
        lines.push("- If two consecutive CI runs fail budget gates, freeze rollout and investigate runtime drift.".to_string());

        Ok(lines.join("\n"))
    }

    pub fn n8_operational_report(
        &self,
        dynamic_iterations: i32,
        metadata_iterations: i32,
        graph_iterations: i32,
    ) -> Result<N8OperationalReport, RuntimeContractError> {
        let budgets = self.n8_default_slos();
        let samples =
            self.n8_run_benchmarks(dynamic_iterations, metadata_iterations, graph_iterations)?;
        let gates = self.n8_evaluate_budget_gates(&samples, &budgets);
        let alerts = self.n8_ci_budget_alerts(&gates);
        let degraded_mode_runbook = self.n8_degraded_mode_runbook()?;
        Ok(N8OperationalReport {
            budgets,
            samples,
            gates,
            alerts,
            degraded_mode_runbook,
        })
    }

    // MARK: - Foundation Date/Time (Track I.1)

    /// Format a Unix timestamp as ISO 8601 (UTC). Returns owned String.
    pub fn datetime_format_unix(&self, ts: f64) -> Result<String, RuntimeContractError> {
        let func: ContractDatetimeFormatUnix =
            self.resolve("swift_contract_datetime_format_unix")?;
        let ptr = unsafe { func(ts) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 30,
                method_id: 1,
            });
        }
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(s)
    }

    /// Parse an ISO 8601 string to a Unix timestamp (seconds since epoch).
    pub fn datetime_parse_iso8601(&self, s: &str) -> Result<f64, RuntimeContractError> {
        let func: ContractDatetimeParseIso8601 =
            self.resolve("swift_contract_datetime_parse_iso8601")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 30,
            method_id: 2,
        })?;
        let ts = unsafe { func(cs.as_ptr()) };
        if ts.is_nan() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 30,
                method_id: 2,
            });
        }
        Ok(ts)
    }

    /// Calendar year (UTC) for a Unix timestamp.
    pub fn datetime_year_utc(&self, ts: f64) -> Result<i32, RuntimeContractError> {
        let func: ContractDatetimeYearUtc = self.resolve("swift_contract_datetime_year_utc")?;
        Ok(unsafe { func(ts) })
    }

    /// Calendar month (1-12, UTC) for a Unix timestamp.
    pub fn datetime_month_utc(&self, ts: f64) -> Result<i32, RuntimeContractError> {
        let func: ContractDatetimeMonthUtc = self.resolve("swift_contract_datetime_month_utc")?;
        Ok(unsafe { func(ts) })
    }

    /// UTC timezone offset in seconds (always 0 for UTC zone).
    pub fn datetime_utc_offset_seconds(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractDatetimeUtcOffsetSeconds =
            self.resolve("swift_contract_datetime_utc_offset_seconds")?;
        Ok(unsafe { func() })
    }

    // MARK: - Foundation Data / UUID / CharacterSet (Track I.2)

    /// Byte-sum (wrapping UInt32) of a byte slice via Foundation Data.
    pub fn data_from_bytes_checksum(&self, bytes: &[u8]) -> Result<u32, RuntimeContractError> {
        let func: ContractDataFromBytesChecksum =
            self.resolve("swift_contract_data_from_bytes_checksum")?;
        let ptr = if bytes.is_empty() {
            std::ptr::null()
        } else {
            bytes.as_ptr()
        };
        Ok(unsafe { func(ptr, bytes.len() as i32) })
    }

    /// Generate a new UUID and return its string representation.
    pub fn uuid_new_string(&self) -> Result<String, RuntimeContractError> {
        let func: ContractUuidNewString = self.resolve("swift_contract_uuid_new_string")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 31,
                method_id: 2,
            });
        }
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(s)
    }

    /// Parse a UUID string; returns true if valid.
    pub fn uuid_parse_validate(&self, s: &str) -> Result<bool, RuntimeContractError> {
        let func: ContractUuidParseValidate = self.resolve("swift_contract_uuid_parse_validate")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 31,
            method_id: 3,
        })?;
        Ok(unsafe { func(cs.as_ptr()) } == 1)
    }

    /// Generate a UUID, render to string, and parse back — returns true on success.
    pub fn uuid_roundtrip(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractUuidRoundtrip = self.resolve("swift_contract_uuid_roundtrip")?;
        Ok(unsafe { func() } == 1)
    }

    /// Returns true if the Unicode codepoint is a letter.
    pub fn charset_is_letter(&self, codepoint: i32) -> Result<bool, RuntimeContractError> {
        let func: ContractCharsetIsLetter = self.resolve("swift_contract_charset_is_letter")?;
        Ok(unsafe { func(codepoint) } == 1)
    }

    // MARK: - Foundation URL / URLComponents (Track I.3)

    /// Returns true if the string is a valid absolute URL.
    pub fn url_parse_valid(&self, s: &str) -> Result<bool, RuntimeContractError> {
        let func: ContractUrlParseValid = self.resolve("swift_contract_url_parse_valid")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 1,
        })?;
        Ok(unsafe { func(cs.as_ptr()) } == 1)
    }

    /// Extract the URL scheme. Returns owned String.
    pub fn url_scheme(&self, s: &str) -> Result<String, RuntimeContractError> {
        let func: ContractUrlScheme = self.resolve("swift_contract_url_scheme")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 2,
        })?;
        let ptr = unsafe { func(cs.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 32,
                method_id: 2,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Extract the URL host. Returns owned String.
    pub fn url_host(&self, s: &str) -> Result<String, RuntimeContractError> {
        let func: ContractUrlHost = self.resolve("swift_contract_url_host")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 3,
        })?;
        let ptr = unsafe { func(cs.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 32,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Extract the URL path. Returns owned String.
    pub fn url_path(&self, s: &str) -> Result<String, RuntimeContractError> {
        let func: ContractUrlPath = self.resolve("swift_contract_url_path")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 4,
        })?;
        let ptr = unsafe { func(cs.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 32,
                method_id: 4,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Build a URL from scheme + host + path components. Returns owned String.
    pub fn url_build_from_components(
        &self,
        scheme: &str,
        host: &str,
        path: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractUrlBuildFromComponents =
            self.resolve("swift_contract_url_build_from_components")?;
        let cs = CString::new(scheme).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 5,
        })?;
        let ch = CString::new(host).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 5,
        })?;
        let cp = CString::new(path).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 32,
            method_id: 5,
        })?;
        let ptr = unsafe { func(cs.as_ptr(), ch.as_ptr(), cp.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 32,
                method_id: 5,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Foundation NSCoding / NSCopying (Track I.4)

    /// Archive then unarchive an Int32 via NSKeyedArchiver. Returns decoded value.
    pub fn nscoding_integer_roundtrip(&self, v: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractNscodingIntegerRoundtrip =
            self.resolve("swift_contract_nscoding_integer_roundtrip")?;
        let result = unsafe { func(v) };
        if result == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 33,
                method_id: 1,
            });
        }
        Ok(result)
    }

    /// Archive then unarchive a string via NSKeyedArchiver. Returns decoded length.
    pub fn nscoding_string_roundtrip(&self, s: &str) -> Result<i32, RuntimeContractError> {
        let func: ContractNscodingStringRoundtrip =
            self.resolve("swift_contract_nscoding_string_roundtrip")?;
        let cs = CString::new(s).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 33,
            method_id: 2,
        })?;
        let result = unsafe { func(cs.as_ptr()) };
        if result < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 33,
                method_id: 2,
            });
        }
        Ok(result)
    }

    /// NSCopying: mutate a copy; verify original is unchanged. Returns true if independent.
    pub fn nscopying_array_independence(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractNscopyingArrayIndependence =
            self.resolve("swift_contract_nscopying_array_independence")?;
        Ok(unsafe { func() } == 1)
    }

    // MARK: - Key Path Runtime Support (Track J.1)

    /// Read `age` through a typed key path path.
    pub fn keypath_get_age(&self, age: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractKeypathGetAge = self.resolve("swift_contract_keypath_get_age")?;
        Ok(unsafe { func(age) })
    }

    /// Read nested `stats.score` through a composed key path path.
    pub fn keypath_get_nested_score(&self, score: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractKeypathGetNestedScore =
            self.resolve("swift_contract_keypath_get_nested_score")?;
        Ok(unsafe { func(score) })
    }

    /// Validate AnyKeyPath matching for the probe model.
    pub fn keypath_any_matches(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractKeypathAnyMatches = self.resolve("swift_contract_keypath_any_matches")?;
        Ok(unsafe { func() } == 1)
    }

    // MARK: - Property Wrapper Metadata (Track J.2)

    /// Construct wrapper-backed storage and return clamped value.
    pub fn wrapper_init_clamped(&self, v: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractWrapperInitClamped =
            self.resolve("swift_contract_wrapper_init_clamped")?;
        Ok(unsafe { func(v) })
    }

    /// Mutate wrapper-backed storage and return post-clamp value.
    pub fn wrapper_set_clamped(
        &self,
        initial: i32,
        new_value: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractWrapperSetClamped = self.resolve("swift_contract_wrapper_set_clamped")?;
        Ok(unsafe { func(initial, new_value) })
    }

    /// Return projected value (`$value`) from wrapper-backed storage.
    pub fn wrapper_projected_value(&self, v: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractWrapperProjectedValue =
            self.resolve("swift_contract_wrapper_projected_value")?;
        Ok(unsafe { func(v) })
    }

    // MARK: - Opaque Type Bridging (Track J.3)

    /// Get opaque `some ProbeNamed` name as owned String.
    pub fn opaque_named_get_name(&self, tag: i32) -> Result<String, RuntimeContractError> {
        let func: ContractOpaqueNamedGetName =
            self.resolve("swift_contract_opaque_named_get_name")?;
        let ptr = unsafe { func(tag) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 36,
                method_id: 1,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Get opaque `some ProbeNamed` name length.
    pub fn opaque_named_name_len(&self, tag: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractOpaqueNamedNameLen =
            self.resolve("swift_contract_opaque_named_name_len")?;
        Ok(unsafe { func(tag) })
    }

    // MARK: - Result Builder & DSL Support (Track J.4)

    /// Build DSL sum from two values.
    pub fn builder_sum2(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractBuilderSum2 = self.resolve("swift_contract_builder_sum2")?;
        Ok(unsafe { func(a, b) })
    }

    /// Build conditional DSL sum.
    pub fn builder_conditional(&self, flag: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractBuilderConditional =
            self.resolve("swift_contract_builder_conditional")?;
        Ok(unsafe { func(flag) })
    }

    /// Build loop-based DSL sum over 1...n.
    pub fn builder_loop_sum(&self, n: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractBuilderLoopSum = self.resolve("swift_contract_builder_loop_sum")?;
        Ok(unsafe { func(n) })
    }

    // MARK: - Reference Cycle & Memory Safety (Track K.1)

    /// Validate weak lifecycle clears after strong drop.
    pub fn k1_weak_lifecycle(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractK1WeakLifecycle = self.resolve("swift_contract_k1_weak_lifecycle")?;
        Ok(unsafe { func() } == 1)
    }

    /// Validate unowned(unsafe) dangling condition is detected safely.
    pub fn k1_unowned_dangling_detected(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractK1UnownedDanglingDetected =
            self.resolve("swift_contract_k1_unowned_dangling_detected")?;
        Ok(unsafe { func() } == 1)
    }

    /// Validate strong pair cycle detection path.
    pub fn k1_cycle_detect_strong_pair(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractK1CycleDetectStrongPair =
            self.resolve("swift_contract_k1_cycle_detect_strong_pair")?;
        Ok(unsafe { func() } == 1)
    }

    /// Validate acyclic pair does not survive after release.
    pub fn k1_cycle_detect_acyclic_pair(&self) -> Result<bool, RuntimeContractError> {
        let func: ContractK1CycleDetectAcyclicPair =
            self.resolve("swift_contract_k1_cycle_detect_acyclic_pair")?;
        Ok(unsafe { func() } == 1)
    }

    // MARK: - Retain Count Inspection & Graph (Track K.2)

    /// Returns retain-count delta from a retain/release pair.
    pub fn k2_retain_delta(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractK2RetainDelta = self.resolve("swift_contract_k2_retain_delta")?;
        let delta = unsafe { func() };
        if delta == i32::MIN {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 40,
                method_id: 1,
            });
        }
        Ok(delta)
    }

    /// Reference-type inference probe (1=class, 2=value, 3=metatype).
    pub fn k2_reference_type_infer(&self, mode: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractK2ReferenceTypeInfer =
            self.resolve("swift_contract_k2_reference_type_infer")?;
        Ok(unsafe { func(mode) })
    }

    /// Return deterministic DOT reference graph output.
    pub fn k2_reference_graph_dot(&self) -> Result<String, RuntimeContractError> {
        let func: ContractK2ReferenceGraphDot =
            self.resolve("swift_contract_k2_reference_graph_dot")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 40,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Leak Tracking & Root Cause (Track K.3)

    /// Reset allocation tracker state.
    pub fn k3_tracker_reset(&self) -> Result<(), RuntimeContractError> {
        let func: ContractK3TrackerReset = self.resolve("swift_contract_k3_tracker_reset")?;
        unsafe { func() };
        Ok(())
    }

    /// Allocate tracked token for `site` and return opaque token pointer.
    pub fn k3_alloc(&self, site: i32) -> Result<*mut c_void, RuntimeContractError> {
        let func: ContractK3Alloc = self.resolve("swift_contract_k3_alloc")?;
        let ptr = unsafe { func(site) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 41,
                method_id: 2,
            });
        }
        Ok(ptr)
    }

    /// Release tracked token; true if released.
    pub fn k3_release(&self, token: *mut c_void) -> Result<bool, RuntimeContractError> {
        let func: ContractK3Release = self.resolve("swift_contract_k3_release")?;
        Ok(unsafe { func(token) } == 1)
    }

    /// Sweep for unreleased token count.
    pub fn k3_sweep_unreleased_count(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractK3SweepUnreleasedCount =
            self.resolve("swift_contract_k3_sweep_unreleased_count")?;
        Ok(unsafe { func() })
    }

    /// Live count for a site id.
    pub fn k3_live_count_for_site(&self, site: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractK3LiveCountForSite =
            self.resolve("swift_contract_k3_live_count_for_site")?;
        Ok(unsafe { func(site) })
    }

    /// Root-cause site id with max live allocations.
    pub fn k3_root_cause_site(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractK3RootCauseSite = self.resolve("swift_contract_k3_root_cause_site")?;
        Ok(unsafe { func() })
    }

    // MARK: - ABI Stability v2+ & User-Defined Types (Track L.1)

    /// Reset user-type registry state.
    pub fn l1_registry_reset(&self) -> Result<(), RuntimeContractError> {
        let func: ContractL1RegistryReset = self.resolve("swift_contract_l1_registry_reset")?;
        unsafe { func() };
        Ok(())
    }

    /// Register a user type name and return a stable type ID.
    pub fn l1_register_type(&self, name: &str) -> Result<i32, RuntimeContractError> {
        let func: ContractL1RegisterType = self.resolve("swift_contract_l1_register_type")?;
        let c_name = CString::new(name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 50,
            method_id: 1,
        })?;
        let id = unsafe { func(c_name.as_ptr()) };
        if id < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 50,
                method_id: 1,
            });
        }
        Ok(id)
    }

    /// Lookup registered type ID by name.
    pub fn l1_lookup_type_id(&self, name: &str) -> Result<i32, RuntimeContractError> {
        let func: ContractL1LookupTypeId = self.resolve("swift_contract_l1_lookup_type_id")?;
        let c_name = CString::new(name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 50,
            method_id: 2,
        })?;
        let id = unsafe { func(c_name.as_ptr()) };
        if id < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 50,
                method_id: 2,
            });
        }
        Ok(id)
    }

    /// Bump user-type version number.
    pub fn l1_bump_type_version(&self, type_id: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractL1BumpTypeVersion =
            self.resolve("swift_contract_l1_bump_type_version")?;
        let version = unsafe { func(type_id) };
        if version < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 50,
                method_id: 3,
            });
        }
        Ok(version)
    }

    /// Check update compatibility (major equal, monotonic minor).
    pub fn l1_update_compat(
        &self,
        old_version: i32,
        new_version: i32,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractL1UpdateCompat = self.resolve("swift_contract_l1_update_compat")?;
        Ok(unsafe { func(old_version, new_version) } == 1)
    }

    // MARK: - Cross-Version Binary Compatibility (Track L.2)

    /// Count breaking removals by comparing old/new exported type counts.
    pub fn l2_contract_diff_breaking_count(
        &self,
        old_type_count: i32,
        new_type_count: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractL2ContractDiffBreakingCount =
            self.resolve("swift_contract_l2_contract_diff_breaking_count")?;
        Ok(unsafe { func(old_type_count, new_type_count) })
    }

    /// Binary compatibility checker for encoded versions (major*1000 + minor).
    pub fn l2_binary_version_compatible(
        &self,
        runtime_version: i32,
        contract_version: i32,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractL2BinaryVersionCompatible =
            self.resolve("swift_contract_l2_binary_version_compatible")?;
        Ok(unsafe { func(runtime_version, contract_version) } == 1)
    }

    /// Resolve resilience marker bit value by marker name.
    pub fn l2_resilience_marker(&self, marker: &str) -> Result<i32, RuntimeContractError> {
        let func: ContractL2ResilienceMarker =
            self.resolve("swift_contract_l2_resilience_marker")?;
        let c_marker = CString::new(marker).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 51,
            method_id: 3,
        })?;
        Ok(unsafe { func(c_marker.as_ptr()) })
    }

    // MARK: - Contract Derivation from Swift Source (Track L.3)

    /// Derive contract JSON from Swift source declaration.
    pub fn l3_derive_contract_from_source(
        &self,
        source: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractL3DeriveContractFromSource =
            self.resolve("swift_contract_l3_derive_contract_from_source")?;
        let c_source = CString::new(source).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 52,
            method_id: 1,
        })?;
        let ptr = unsafe { func(c_source.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 52,
                method_id: 1,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Validate derived JSON against handwritten JSON.
    pub fn l3_validate_derived_contract(
        &self,
        derived: &str,
        handwritten: &str,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractL3ValidateDerivedContract =
            self.resolve("swift_contract_l3_validate_derived_contract")?;
        let c_derived = CString::new(derived).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 52,
            method_id: 2,
        })?;
        let c_hand =
            CString::new(handwritten).map_err(|_| RuntimeContractError::InvalidInvoke {
                type_id: 52,
                method_id: 2,
            })?;
        Ok(unsafe { func(c_derived.as_ptr(), c_hand.as_ptr()) } == 1)
    }

    /// Return exporter macro simulation string for a type name.
    pub fn l3_exporter_macro_sim(&self, name: &str) -> Result<String, RuntimeContractError> {
        let func: ContractL3ExporterMacroSim =
            self.resolve("swift_contract_l3_exporter_macro_sim")?;
        let c_name = CString::new(name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 52,
            method_id: 3,
        })?;
        let ptr = unsafe { func(c_name.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 52,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Instruments Integration (Track M.1)

    pub fn m1_reset(&self) -> Result<(), RuntimeContractError> {
        let func: ContractM1Reset = self.resolve("swift_contract_m1_reset")?;
        unsafe { func() };
        Ok(())
    }

    pub fn m1_os_log_event(&self, name: &str) -> Result<bool, RuntimeContractError> {
        let func: ContractM1OsLogEvent = self.resolve("swift_contract_m1_os_log_event")?;
        let c_name = CString::new(name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 60,
            method_id: 1,
        })?;
        Ok(unsafe { func(c_name.as_ptr()) } == 1)
    }

    pub fn m1_poi_begin(&self, token: i32) -> Result<bool, RuntimeContractError> {
        let func: ContractM1PoiBegin = self.resolve("swift_contract_m1_poi_begin")?;
        Ok(unsafe { func(token) } == 1)
    }

    pub fn m1_poi_end(&self, token: i32) -> Result<bool, RuntimeContractError> {
        let func: ContractM1PoiEnd = self.resolve("swift_contract_m1_poi_end")?;
        Ok(unsafe { func(token) } == 1)
    }

    pub fn m1_event_count(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractM1EventCount = self.resolve("swift_contract_m1_event_count")?;
        Ok(unsafe { func() })
    }

    pub fn m1_last_duration_nanos(&self) -> Result<u64, RuntimeContractError> {
        let func: ContractM1LastDurationNanos =
            self.resolve("swift_contract_m1_last_duration_nanos")?;
        Ok(unsafe { func() })
    }

    pub fn m1_profile_iterations(&self, iterations: i32) -> Result<u64, RuntimeContractError> {
        let func: ContractM1ProfileIterations =
            self.resolve("swift_contract_m1_profile_iterations")?;
        Ok(unsafe { func(iterations) })
    }

    // MARK: - DWARF Debug Info Access (Track M.2)

    pub fn m2_reset(&self) -> Result<(), RuntimeContractError> {
        let func: ContractM2Reset = self.resolve("swift_contract_m2_reset")?;
        unsafe { func() };
        Ok(())
    }

    pub fn m2_cache_binary(&self, path: &str) -> Result<bool, RuntimeContractError> {
        let func: ContractM2CacheBinary = self.resolve("swift_contract_m2_cache_binary")?;
        let c_path = CString::new(path).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 61,
            method_id: 1,
        })?;
        Ok(unsafe { func(c_path.as_ptr()) } == 1)
    }

    pub fn m2_cache_size(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractM2CacheSize = self.resolve("swift_contract_m2_cache_size")?;
        Ok(unsafe { func() })
    }

    pub fn m2_lookup_source(&self, address: u64) -> Result<String, RuntimeContractError> {
        let func: ContractM2LookupSource = self.resolve("swift_contract_m2_lookup_source")?;
        let ptr = unsafe { func(address) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 61,
                method_id: 2,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    pub fn m2_lookup_variable(&self, name: &str) -> Result<String, RuntimeContractError> {
        let func: ContractM2LookupVariable = self.resolve("swift_contract_m2_lookup_variable")?;
        let c_name = CString::new(name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 61,
            method_id: 3,
        })?;
        let ptr = unsafe { func(c_name.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 61,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Memory Profiling & Malloc Tagging (Track M.3)

    pub fn m3_reset(&self) -> Result<(), RuntimeContractError> {
        let func: ContractM3Reset = self.resolve("swift_contract_m3_reset")?;
        unsafe { func() };
        Ok(())
    }

    pub fn m3_tag_alloc(
        &self,
        subsystem: &str,
        bytes: i64,
    ) -> Result<*mut c_void, RuntimeContractError> {
        let func: ContractM3TagAlloc = self.resolve("swift_contract_m3_tag_alloc")?;
        let c_sub = CString::new(subsystem).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 62,
            method_id: 1,
        })?;
        let ptr = unsafe { func(c_sub.as_ptr(), bytes) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 62,
                method_id: 1,
            });
        }
        Ok(ptr)
    }

    pub fn m3_release_alloc(&self, token: *mut c_void) -> Result<bool, RuntimeContractError> {
        let func: ContractM3ReleaseAlloc = self.resolve("swift_contract_m3_release_alloc")?;
        Ok(unsafe { func(token) } == 1)
    }

    pub fn m3_usage_for_subsystem(&self, subsystem: &str) -> Result<i64, RuntimeContractError> {
        let func: ContractM3UsageForSubsystem =
            self.resolve("swift_contract_m3_usage_for_subsystem")?;
        let c_sub = CString::new(subsystem).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 62,
            method_id: 2,
        })?;
        Ok(unsafe { func(c_sub.as_ptr()) })
    }

    pub fn m3_health_report(&self) -> Result<String, RuntimeContractError> {
        let func: ContractM3HealthReport = self.resolve("swift_contract_m3_health_report")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 62,
                method_id: 3,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Performance Regression Testing (Track M.4)

    pub fn m4_run_benchmark(&self, op: &str, iterations: i32) -> Result<u64, RuntimeContractError> {
        let func: ContractM4RunBenchmark = self.resolve("swift_contract_m4_run_benchmark")?;
        let c_op = CString::new(op).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 63,
            method_id: 1,
        })?;
        Ok(unsafe { func(c_op.as_ptr(), iterations) })
    }

    pub fn m4_set_baseline(&self, op: &str, nanos: u64) -> Result<bool, RuntimeContractError> {
        let func: ContractM4SetBaseline = self.resolve("swift_contract_m4_set_baseline")?;
        let c_op = CString::new(op).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 63,
            method_id: 2,
        })?;
        Ok(unsafe { func(c_op.as_ptr(), nanos) } == 1)
    }

    pub fn m4_regression_alarm(
        &self,
        op: &str,
        current_nanos: u64,
        threshold_percent: i32,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractM4RegressionAlarm = self.resolve("swift_contract_m4_regression_alarm")?;
        let c_op = CString::new(op).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 63,
            method_id: 3,
        })?;
        Ok(unsafe { func(c_op.as_ptr(), current_nanos, threshold_percent) } == 1)
    }

    pub fn m4_baseline_get(&self, op: &str) -> Result<u64, RuntimeContractError> {
        let func: ContractM4BaselineGet = self.resolve("swift_contract_m4_baseline_get")?;
        let c_op = CString::new(op).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 63,
            method_id: 4,
        })?;
        Ok(unsafe { func(c_op.as_ptr()) })
    }

    // MARK: - Universal Runtime Metadata Graph (Track N.1)

    /// Metadata kind id for a synthetic graph node/type id.
    pub fn n1_metadata_kind(&self, type_id: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataKind = self.resolve("swift_contract_n1_metadata_kind")?;
        let out = unsafe { func(type_id) };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 1,
            });
        }
        Ok(out)
    }

    /// Metadata field count for a synthetic graph node/type id.
    pub fn n1_metadata_field_count(&self, type_id: i32) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataFieldCount =
            self.resolve("swift_contract_n1_metadata_field_count")?;
        let out = unsafe { func(type_id) };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 2,
            });
        }
        Ok(out)
    }

    /// Field byte offset for (type_id, field_index).
    pub fn n1_metadata_field_offset(
        &self,
        type_id: i32,
        field_index: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataFieldOffset =
            self.resolve("swift_contract_n1_metadata_field_offset")?;
        let out = unsafe { func(type_id, field_index) };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 3,
            });
        }
        Ok(out)
    }

    /// Cycle-safe graph traversal count.
    pub fn n1_metadata_graph_traverse_count(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataGraphTraverseCount =
            self.resolve("swift_contract_n1_metadata_graph_traverse_count")?;
        let out = unsafe { func() };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 4,
            });
        }
        Ok(out)
    }

    /// Deterministic metadata graph snapshot over mixed type set.
    pub fn n1_metadata_snapshot_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractN1MetadataSnapshotJson =
            self.resolve("swift_contract_n1_metadata_snapshot_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 5,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Metadata kind for runtime-discovered type by name.
    pub fn n1_metadata_kind_by_name(&self, type_name: &str) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataKindByName =
            self.resolve("swift_contract_n1_metadata_kind_by_name")?;
        let c_name = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 70,
            method_id: 6,
        })?;
        let out = unsafe { func(c_name.as_ptr()) };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 6,
            });
        }
        Ok(out)
    }

    /// Field count for runtime-discovered type by name.
    pub fn n1_metadata_field_count_by_name(
        &self,
        type_name: &str,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataFieldCountByName =
            self.resolve("swift_contract_n1_metadata_field_count_by_name")?;
        let c_name = CString::new(type_name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 70,
            method_id: 7,
        })?;
        let out = unsafe { func(c_name.as_ptr()) };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 7,
            });
        }
        Ok(out)
    }

    /// JSON list of discovered runtime type names.
    pub fn n1_metadata_discover_types_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractN1MetadataDiscoverTypesJson =
            self.resolve("swift_contract_n1_metadata_discover_types_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 8,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Traversal cardinality over discovered runtime type graph roots.
    pub fn n1_metadata_graph_traverse_discovered_count(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractN1MetadataGraphTraverseDiscoveredCount =
            self.resolve("swift_contract_n1_metadata_graph_traverse_discovered_count")?;
        let out = unsafe { func() };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 9,
            });
        }
        Ok(out)
    }

    /// Enumerate ALL Swift nominal types from loaded dyld images without pre-registered seeds.
    /// JSON: {"types":[{"name":"..."},...], "count": N}
    pub fn n1_enumerate_all_types_json(&self) -> Result<String, RuntimeContractError> {
        let func: ContractN1EnumerateAllTypesJson =
            self.resolve("swift_contract_n1_enumerate_all_types_json")?;
        let ptr = unsafe { func() };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 10,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// JSON type info (kind, kind_id, field_count) for any runtime-discoverable name.
    pub fn n1_type_info_json(&self, name: &str) -> Result<String, RuntimeContractError> {
        let func: ContractN1TypeInfoJson = self.resolve("swift_contract_n1_type_info_json")?;
        let c_name = CString::new(name).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 70,
            method_id: 11,
        })?;
        let ptr = unsafe { func(c_name.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 11,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    /// Number of loaded dyld images (for incremental per-image traversal).
    pub fn n1_image_count(&self) -> Result<i32, RuntimeContractError> {
        let func: ContractN1ImageCount = self.resolve("swift_contract_n1_image_count")?;
        let out = unsafe { func() };
        if out < 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 12,
            });
        }
        Ok(out)
    }

    /// Swift nominal types in a specific dyld image by 0-based index.
    /// JSON: {"image":"...", "types":[{"name":"..."},...], "count": N}
    pub fn n1_image_types_json(&self, index: i32) -> Result<String, RuntimeContractError> {
        let func: ContractN1ImageTypesJson = self.resolve("swift_contract_n1_image_types_json")?;
        let ptr = unsafe { func(index) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 70,
                method_id: 13,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Universal Call Lowering & Invocation (Track N.2)

    /// Bitmask of supported invocation-lowering capabilities for N.2.
    pub fn n2_capability_mask(&self) -> Result<u32, RuntimeContractError> {
        let func: ContractN2CapabilityMask = self.resolve("swift_contract_n2_capability_mask")?;
        Ok(unsafe { func() })
    }

    fn n2_invoke_raw(
        &self,
        signature: &str,
        a: i32,
        b: i32,
        inout_value: Option<&mut i32>,
        out_value: *mut i32,
        error_code: Option<&mut i32>,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractN2InvokeI32 = self.resolve("swift_contract_n2_invoke_i32")?;
        let c_sig = CString::new(signature).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 2,
        })?;
        let inout_ptr = inout_value
            .map(|v| v as *mut i32)
            .unwrap_or(std::ptr::null_mut());
        let error_ptr = error_code
            .map(|v| v as *mut i32)
            .unwrap_or(std::ptr::null_mut());
        let ok = unsafe { func(c_sig.as_ptr(), a, b, inout_ptr, out_value, error_ptr) } == 1;
        Ok(ok)
    }

    pub fn n2_direct_add(&self, a: i32, b: i32) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        if !self.n2_invoke_raw(
            "direct.add.i32_i32_to_i32",
            a,
            b,
            None,
            &mut out as *mut i32,
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 3,
            });
        }
        Ok(out)
    }

    pub fn n2_inout_add_assign(
        &self,
        initial: i32,
        delta: i32,
    ) -> Result<(i32, i32), RuntimeContractError> {
        let mut inout_value = initial;
        let mut out = 0i32;
        if !self.n2_invoke_raw(
            "inout.add_assign.i32ptr_i32_to_i32",
            0,
            delta,
            Some(&mut inout_value),
            &mut out as *mut i32,
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 4,
            });
        }
        Ok((inout_value, out))
    }

    pub fn n2_throwing_require_non_negative(
        &self,
        value: i32,
    ) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        let mut error = 0i32;
        if !self.n2_invoke_raw(
            "throwing.require_non_negative.i32_to_i32",
            value,
            0,
            None,
            &mut out as *mut i32,
            Some(&mut error),
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: if error == -100 { 5 } else { 6 },
            });
        }
        Ok(out)
    }

    pub fn n2_async_double(&self, value: i32) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        if !self.n2_invoke_raw(
            "async.double.i32_to_i32",
            value,
            0,
            None,
            &mut out as *mut i32,
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 7,
            });
        }
        Ok(out)
    }

    pub fn n2_indirect_pair_sum_diff(
        &self,
        a: i32,
        b: i32,
    ) -> Result<(i32, i32), RuntimeContractError> {
        let mut out = [0i32; 2];
        if !self.n2_invoke_raw(
            "indirect_ret.pair_sum_diff.i32_i32_to_pair",
            a,
            b,
            None,
            out.as_mut_ptr(),
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 8,
            });
        }
        Ok((out[0], out[1]))
    }

    pub fn n2_resilient_counter_addpair(
        &self,
        a: i32,
        b: i32,
    ) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        if !self.n2_invoke_raw(
            "resilient.counter_addpair.i32_i32_to_i32",
            a,
            b,
            None,
            &mut out as *mut i32,
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 9,
            });
        }
        Ok(out)
    }

    fn n2_invoke_symbol_raw(
        &self,
        symbol: &str,
        shape: &str,
        a: i32,
        b: i32,
        inout_value: Option<&mut i32>,
        out_value: *mut i32,
        error_code: Option<&mut i32>,
    ) -> Result<bool, RuntimeContractError> {
        let func: ContractN2InvokeSymbolI32 =
            self.resolve("swift_contract_n2_invoke_symbol_i32")?;
        let c_symbol = CString::new(symbol).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 10,
        })?;
        let c_shape = CString::new(shape).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 10,
        })?;
        let inout_ptr = inout_value
            .map(|v| v as *mut i32)
            .unwrap_or(std::ptr::null_mut());
        let error_ptr = error_code
            .map(|v| v as *mut i32)
            .unwrap_or(std::ptr::null_mut());

        let ok = unsafe {
            func(
                c_symbol.as_ptr(),
                c_shape.as_ptr(),
                a,
                b,
                inout_ptr,
                out_value,
                error_ptr,
            )
        } == 1;
        Ok(ok)
    }

    pub fn n2_dynamic_symbol_i32(
        &self,
        symbol: &str,
        a: i32,
        b: i32,
    ) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        if !self.n2_invoke_symbol_raw(
            symbol,
            "i32_i32_to_i32",
            a,
            b,
            None,
            &mut out as *mut i32,
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 11,
            });
        }
        Ok(out)
    }

    pub fn n2_dynamic_symbol_inout(
        &self,
        symbol: &str,
        initial: i32,
        delta: i32,
    ) -> Result<(i32, i32), RuntimeContractError> {
        let mut inout_value = initial;
        let mut out = 0i32;
        if !self.n2_invoke_symbol_raw(
            symbol,
            "i32ptr_i32_to_i32",
            0,
            delta,
            Some(&mut inout_value),
            &mut out as *mut i32,
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 12,
            });
        }
        Ok((inout_value, out))
    }

    pub fn n2_dynamic_symbol_pair(
        &self,
        symbol: &str,
        a: i32,
        b: i32,
    ) -> Result<(i32, i32), RuntimeContractError> {
        let mut out = [0i32; 2];
        if !self.n2_invoke_symbol_raw(
            symbol,
            "i32_i32_to_pair",
            a,
            b,
            None,
            out.as_mut_ptr(),
            None,
        )? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 13,
            });
        }
        Ok((out[0], out[1]))
    }

    pub fn n2_dynamic_symbol_rejects_unknown_shape(
        &self,
        symbol: &str,
    ) -> Result<bool, RuntimeContractError> {
        let mut out = 0i32;
        let mut error = 0i32;
        let ok = self.n2_invoke_symbol_raw(
            symbol,
            "unknown.shape",
            1,
            2,
            None,
            &mut out as *mut i32,
            Some(&mut error),
        )?;
        Ok(!ok && error != 0)
    }

    pub fn n2_dynamic_symbol_single(
        &self,
        symbol: &str,
        a: i32,
    ) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        if !self.n2_invoke_symbol_raw(symbol, "i32_to_i32", a, 0, None, &mut out, None)? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 15,
            });
        }
        Ok(out)
    }

    pub fn n2_dynamic_symbol_const(&self, symbol: &str) -> Result<i32, RuntimeContractError> {
        let mut out = 0i32;
        if !self.n2_invoke_symbol_raw(symbol, "void_to_i32", 0, 0, None, &mut out, None)? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 16,
            });
        }
        Ok(out)
    }

    pub fn n2_symbol_describe(&self, symbol: &str) -> Result<(String, bool), RuntimeContractError> {
        let func: ContractN2SymbolDescribe = self.resolve("swift_contract_n2_symbol_describe")?;
        let c_sym = CString::new(symbol).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 17,
        })?;
        let ptr = unsafe { func(c_sym.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 17,
            });
        }
        let json = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        let supported = json.contains("\"supported\":true");
        let shape = json
            .find("\"shape\":\"")
            .and_then(|i| {
                let rest = &json[i + 9..];
                rest.find('"').map(|e| rest[..e].to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());
        Ok((shape, supported))
    }

    /// Single-call auto-dispatch: Swift side resolves shape from its registry and invokes atomically.
    /// Optimization variant of describe-then-invoke with one fewer FFI round-trip.
    pub fn n2_invoke_auto(
        &self,
        symbol: &str,
        a: i32,
        b: i32,
    ) -> Result<i32, RuntimeContractError> {
        let func: ContractN2InvokeAuto = self.resolve("swift_contract_n2_invoke_auto")?;
        let c_sym = CString::new(symbol).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 19,
        })?;
        let mut out = 0i32;
        let mut error = 0i32;
        let ok = unsafe {
            func(
                c_sym.as_ptr(),
                a,
                b,
                std::ptr::null_mut(),
                &mut out,
                &mut error,
            )
        } == 1;
        if !ok {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: if error == -450 { 20 } else { 19 },
            });
        }
        Ok(out)
    }

    /// Describe-then-invoke: discovers shape at runtime and dispatches.
    /// Demonstrates the N.2 exit criterion — Rust needs only the symbol name.
    pub fn n2_describe_and_invoke(
        &self,
        symbol: &str,
        a: i32,
        b: i32,
    ) -> Result<i32, RuntimeContractError> {
        let (shape, supported) = self.n2_symbol_describe(symbol)?;
        if !supported {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 18,
            });
        }
        let mut out = 0i32;
        if !self.n2_invoke_symbol_raw(symbol, &shape, a, b, None, &mut out, None)? {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 18,
            });
        }
        Ok(out)
    }

    pub fn n2_lowering_strategy_json(
        &self,
        signature: &str,
    ) -> Result<String, RuntimeContractError> {
        let func: ContractN2LoweringStrategyJson =
            self.resolve("swift_contract_n2_lowering_strategy_json")?;
        let c_sig = CString::new(signature).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 71,
            method_id: 14,
        })?;
        let ptr = unsafe { func(c_sig.as_ptr()) };
        if ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 71,
                method_id: 14,
            });
        }
        let out = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(out)
    }

    // MARK: - Backtrace & Crash Symbolication (Track E.2)

    /// Capture a Swift backtrace as newline-delimited text.
    pub fn backtrace_capture(&self) -> Result<String, RuntimeContractError> {
        let func: ContractBacktraceCapture = self.resolve("swift_contract_backtrace_capture")?;
        let c_str_ptr = unsafe { func() };
        if c_str_ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 17,
                method_id: 1,
            });
        }

        let c_str = unsafe { CStr::from_ptr(c_str_ptr) };
        let backtrace = c_str.to_string_lossy().into_owned();
        unsafe { libc::free(c_str_ptr as *mut c_void) };
        Ok(backtrace)
    }

    /// Return the runtime address of the Swift anchor symbol used in debug probes.
    pub fn backtrace_anchor_address(&self) -> Result<u64, RuntimeContractError> {
        let func: ContractBacktraceAnchorAddress =
            self.resolve("swift_contract_backtrace_anchor_address")?;
        let addr = unsafe { func() };
        if addr == 0 {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 17,
                method_id: 2,
            });
        }
        Ok(addr)
    }

    /// Demangle a symbol using Swift's runtime backtrace demangler.
    pub fn backtrace_demangle_symbol(&self, mangled: &str) -> Result<String, RuntimeContractError> {
        let input = CString::new(mangled).map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 17,
            method_id: 3,
        })?;
        let mut out_size: usize = 0;
        let demangled_ptr = unsafe {
            crate::Backtrace::swift_runtime_backtrace__swift_backtrace_demangle(
                input.as_ptr(),
                mangled.len(),
                std::ptr::null_mut(),
                &mut out_size as *mut usize,
            )
        };

        if demangled_ptr.is_null() {
            return Err(RuntimeContractError::InvalidInvoke {
                type_id: 17,
                method_id: 3,
            });
        }

        let demangled = unsafe { CStr::from_ptr(demangled_ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(demangled_ptr as *mut c_void) };
        Ok(demangled)
    }

    // MARK: - Phase B.2: Witness Table Dynamic Resolver

    /// Scan all protocol conformances from the Swift binary without pre-seeded lookups.
    pub fn b2_scan_all_conformances(&self) -> Result<Vec<ConformanceDescriptor>, RuntimeContractError> {
        let func: ContractB2ScanConformancesJson = self.resolve("swift_contract_b2_scan_conformances_json")?;
        let json_ptr = unsafe { func() };
        if json_ptr.is_null() {
            return Ok(Vec::new());
        }

        let c_str = unsafe { CStr::from_ptr(json_ptr) };
        let json_str = c_str.to_string_lossy();
        let conformances: Result<Vec<ConformanceDescriptor>, _> = serde_json::from_str(&json_str)
            .map_err(|e| RuntimeContractError::DescriptorParse(format!("Failed to parse conformances: {}", e)));

        unsafe { libc::free(json_ptr as *mut c_void) };
        conformances
    }

    /// Resolve a witness table pointer for a given type name and protocol name.
    /// Returns null if the type does not conform to the protocol.
    pub fn b2_resolve_witness_table(&self, type_name: &str, protocol_name: &str) -> Result<*const c_void, RuntimeContractError> {
        let type_c = CString::new(type_name)
            .map_err(|_| RuntimeContractError::ConformanceNotFound {
                type_name: type_name.to_string(),
                protocol_name: protocol_name.to_string(),
            })?;
        let protocol_c = CString::new(protocol_name)
            .map_err(|_| RuntimeContractError::ConformanceNotFound {
                type_name: type_name.to_string(),
                protocol_name: protocol_name.to_string(),
            })?;

        let func: ContractB2ResolveWitnessTable = self.resolve("swift_contract_b2_resolve_witness_table")?;
        let witness_table = unsafe { func(type_c.as_ptr(), protocol_c.as_ptr()) };

        if witness_table.is_null() {
            return Err(RuntimeContractError::ConformanceNotFound {
                type_name: type_name.to_string(),
                protocol_name: protocol_name.to_string(),
            });
        }

        Ok(witness_table)
    }

    /// Attempt to resolve a witness table but return Ok(null) if not found instead of an error.
    pub fn b2_try_resolve_witness_table(&self, type_name: &str, protocol_name: &str) -> Result<*const c_void, RuntimeContractError> {
        let type_c = CString::new(type_name)
            .map_err(|_| RuntimeContractError::WitnessTableResolutionFailed {
                type_name: type_name.to_string(),
                protocol_name: protocol_name.to_string(),
            })?;
        let protocol_c = CString::new(protocol_name)
            .map_err(|_| RuntimeContractError::WitnessTableResolutionFailed {
                type_name: type_name.to_string(),
                protocol_name: protocol_name.to_string(),
            })?;

        let func: ContractB2ResolveWitnessTable = self.resolve("swift_contract_b2_resolve_witness_table")?;
        let witness_table = unsafe { func(type_c.as_ptr(), protocol_c.as_ptr()) };
        Ok(witness_table)
    }

    /// Get a description of a conformance descriptor (for debugging/inspection).
    pub fn b2_describe_conformance(&self, witness_ptr: *const c_void) -> Result<String, RuntimeContractError> {
        if witness_ptr.is_null() {
            return Err(RuntimeContractError::WitnessTableResolutionFailed {
                type_name: "null".to_string(),
                protocol_name: "unknown".to_string(),
            });
        }

        let func: ContractB2DescribeConformance = self.resolve("swift_contract_b2_describe_conformance")?;
        let desc_ptr = unsafe { func(witness_ptr) };
        if desc_ptr.is_null() {
            return Err(RuntimeContractError::WitnessTableResolutionFailed {
                type_name: "unknown".to_string(),
                protocol_name: "unknown".to_string(),
            });
        }

        let c_str = unsafe { CStr::from_ptr(desc_ptr) };
        let description = c_str.to_string_lossy().into_owned();
        unsafe { libc::free(desc_ptr as *mut c_void) };
        Ok(description)
    }

    /// Resolve witness table for a standard protocol conformance with caching.
    pub fn b2_resolve_standard_conformance(&self, type_name: &str, protocol_name: &str) -> Result<*const c_void, RuntimeContractError> {
        // Try common type name aliases first
        let type_candidates = self.b2_type_name_candidates(type_name);
        for candidate in type_candidates {
            match self.b2_try_resolve_witness_table(&candidate, protocol_name) {
                Ok(ptr) if !ptr.is_null() => return Ok(ptr),
                _ => continue,
            }
        }

        Err(RuntimeContractError::ConformanceNotFound {
            type_name: type_name.to_string(),
            protocol_name: protocol_name.to_string(),
        })
    }

    /// Generate type name candidates for fallback resolution (e.g., "String" -> ["String", "Swift.String"])
    fn b2_type_name_candidates(&self, type_name: &str) -> Vec<String> {
        let mut candidates = vec![type_name.to_string()];
        if !type_name.contains('.') {
            candidates.push(format!("Swift.{}", type_name));
            candidates.push(format!("RustBridge.{}", type_name));
        }
        candidates
    }

    // MARK: - Phase B.3: Cross-Version ABI Compatibility Shim

    /// Get the current Swift runtime version detected at the host.
    pub fn b3_detect_runtime_version(&self) -> Result<RuntimeVersion, RuntimeContractError> {
        let func: ContractB3RuntimeVersionJson = self.resolve("swift_contract_b3_runtime_version_json")?;
        let json_ptr = unsafe { func() };
        if json_ptr.is_null() {
            return Err(RuntimeContractError::VersionDetectionFailed {
                reason: "null response from version detection".to_string(),
            });
        }

        let c_str = unsafe { CStr::from_ptr(json_ptr) };
        let json_str = c_str.to_string_lossy();
        let version: Result<RuntimeVersion, _> = serde_json::from_str(&json_str)
            .map_err(|e| RuntimeContractError::VersionDetectionFailed {
                reason: format!("Failed to parse runtime version: {}", e),
            });

        unsafe { libc::free(json_ptr as *mut c_void) };
        version
    }

    /// Get adapter table for the specified version profile.
    pub fn b3_get_adapter_table(&self, profile_id: &str) -> Result<Vec<AdapterTypeLayout>, RuntimeContractError> {
        let profile_c = CString::new(profile_id)
            .map_err(|_| RuntimeContractError::AdapterSelectionFailed {
                version: profile_id.to_string(),
                reason: "invalid profile ID".to_string(),
            })?;

        let func: ContractB3GetAdapterTableJson = self.resolve("swift_contract_b3_get_adapter_table_json")?;
        let json_ptr = unsafe { func(profile_c.as_ptr()) };
        if json_ptr.is_null() {
            return Ok(Vec::new());
        }

        let c_str = unsafe { CStr::from_ptr(json_ptr) };
        let json_str = c_str.to_string_lossy();
        let layouts: Result<Vec<AdapterTypeLayout>, _> = serde_json::from_str(&json_str)
            .map_err(|e| RuntimeContractError::AdapterSelectionFailed {
                version: profile_id.to_string(),
                reason: format!("Failed to parse adapter table: {}", e),
            });

        unsafe { libc::free(json_ptr as *mut c_void) };
        layouts
    }

    /// Select a version adapter profile and activate it.
    pub fn b3_select_adapter_profile(&self, profile_id: &str) -> Result<bool, RuntimeContractError> {
        let profile_c = CString::new(profile_id)
            .map_err(|_| RuntimeContractError::AdapterSelectionFailed {
                version: profile_id.to_string(),
                reason: "invalid profile ID".to_string(),
            })?;

        let func: ContractB3SelectAdapterProfile = self.resolve("swift_contract_b3_select_adapter_profile")?;
        let result = unsafe { func(profile_c.as_ptr()) };
        
        if result == 0 {
            return Err(RuntimeContractError::AdapterSelectionFailed {
                version: profile_id.to_string(),
                reason: "profile selection failed at runtime".to_string(),
            });
        }

        Ok(result != 0)
    }

    /// Auto-select the most appropriate adapter profile for the current runtime version.
    pub fn b3_auto_select_profile(&self) -> Result<String, RuntimeContractError> {
        let version = self.b3_detect_runtime_version()?;

        // Build a profile_id based on the detected version
        let profile_id = format!("swift_{}_{}_arm64_macos", version.major, version.minor);

        // Try to select it
        match self.b3_select_adapter_profile(&profile_id) {
            Ok(true) => Ok(profile_id),
            Ok(false) => {
                // Fallback to detecting closest compatible version
                let fallback_id = format!("swift_6_2_arm64_macos"); // Fallback to 6.2
                self.b3_select_adapter_profile(&fallback_id)?;
                Ok(fallback_id)
            }
            Err(_) => {
                // Last resort: try the default profile
                let default_id = "swift_6_2_arm64_macos".to_string();
                self.b3_select_adapter_profile(&default_id)?;
                Ok(default_id)
            }
        }
    }

    /// Get the offset of a field in a given type within the selected adapter profile.
    pub fn b3_get_field_offset(&self, type_name: &str, field_name: &str) -> Result<i32, RuntimeContractError> {
        let version = self.b3_detect_runtime_version()?;
        
        // For now, use default profiles based on detected version
        let profile_id = format!("swift_{}_{}_arm64_macos", version.major, version.minor);
        let layouts = self.b3_get_adapter_table(&profile_id)?;

        for layout in layouts {
            if layout.type_name == type_name {
                for field in layout.fields {
                    if field.field_name == field_name {
                        return Ok(field.offset);
                    }
                }
                return Err(RuntimeContractError::AdapterSelectionFailed {
                    version: type_name.to_string(),
                    reason: format!("field {} not found", field_name),
                });
            }
        }

        Err(RuntimeContractError::AdapterSelectionFailed {
            version: type_name.to_string(),
            reason: "type not found in adapter table".to_string(),
        })
    }

    pub fn release(&self, object: ContractObject) -> Result<(), RuntimeContractError> {
        let func: ContractRelease = self.resolve("swift_contract_release")?;
        let ok = unsafe { func(object.type_id, object.object) };
        if ok != 1 {
            return Err(RuntimeContractError::ReleaseFailed {
                type_id: object.type_id,
            });
        }
        Ok(())
    }
}

impl<'a> OwnedContractObject<'a> {
    pub fn as_object(&self) -> ContractObject {
        self.object
    }

    pub fn ownership(&self) -> ContractOwnership {
        self.object.ownership
    }

    pub fn release(mut self) -> Result<(), RuntimeContractError> {
        self.contract.release(self.object)?;
        self.released = true;
        Ok(())
    }
}

impl std::fmt::Debug for OwnedContractObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedContractObject")
            .field("type_id", &self.object.type_id)
            .field("ownership", &self.object.ownership)
            .field("released", &self.released)
            .finish()
    }
}

impl Drop for OwnedContractObject<'_> {
    fn drop(&mut self) {
        if self.released {
            return;
        }
        let _ = self.contract.release(self.object);
        self.released = true;
    }
}
