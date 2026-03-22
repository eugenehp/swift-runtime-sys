// ── Auto-generated bindgen modules (from Swift runtime headers) ──
pub mod Atomic;
pub mod Backtrace;
pub mod Config;
pub mod CrashInfo;
pub mod CustomRRABI;
pub mod Exception;
pub mod Exclusivity;
pub mod FoundationSupport;
pub mod FunctionReplacement;
pub mod Heap;
pub mod InstrumentsSupport;
pub mod Paths;
pub mod Portability;
pub mod PrebuiltStringMap;
pub mod SwiftDtoa;
pub mod TracingCommon;
pub mod VoucherShims;

// ── Hand-written higher-level modules ──
#[allow(non_snake_case)]
pub mod ConcurrencyAbi;
#[allow(non_snake_case)]
pub mod RemoteMirror;
#[allow(non_snake_case)]
pub mod RuntimeContract;
#[allow(non_snake_case)]
pub mod RuntimeFactory;
#[allow(non_snake_case)]
pub mod RuntimeRaw;
#[allow(non_snake_case)]
pub mod RustExecutorInterop;
#[allow(non_snake_case)]
pub mod SymbolDemangler;

// ── New comprehensive runtime bindings ──

/// P0: Dynamic casting (`as?`, `as!`, `is`)
#[allow(non_snake_case)]
pub mod DynamicCast;

/// P0: Error handling (alloc, inspect, throw)
#[allow(non_snake_case)]
pub mod ErrorHandling;

/// P0: Metadata introspection (types, functions, tuples, existentials)
#[allow(non_snake_case)]
pub mod MetadataIntrospection;

/// P0: Concurrency runtime (tasks, actors, executors, groups, async let)
#[allow(non_snake_case)]
pub mod ConcurrencyRuntime;

/// P1: Enum operations (tag manipulation, metadata init)
#[allow(non_snake_case)]
pub mod EnumOps;

/// P1: Box and existential allocation
#[allow(non_snake_case)]
pub mod BoxExistential;

/// P1: Unowned reference operations
#[allow(non_snake_case)]
pub mod UnownedRef;

/// P1: Unknown object (ObjC-bridged) retain/release
#[allow(non_snake_case)]
pub mod UnknownObject;

/// P1: In-process reflection mirror
#[allow(non_snake_case)]
pub mod ReflectionMirrorInProcess;

/// P1: Witness tables and protocol conformance
#[allow(non_snake_case)]
pub mod WitnessTable;

/// P1: Bridge object retain/release
#[allow(non_snake_case)]
pub mod BridgeObject;

/// P1: Nonatomic reference counting variants
#[allow(non_snake_case)]
pub mod NonatomicRefCounting;

/// P2: Debugging and diagnostics hooks
#[allow(non_snake_case)]
pub mod DebugHooks;

/// P2: KeyPath runtime
#[allow(non_snake_case)]
pub mod KeyPathRuntime;

/// P2: Function replacement and dynamic dispatch
#[allow(non_snake_case)]
pub mod FunctionReplacementExt;

/// P2: Class and struct metadata initialization
#[allow(non_snake_case)]
pub mod ClassMetadataInit;

/// P2: Memory and allocation primitives
#[allow(non_snake_case)]
pub mod MemoryAlloc;

/// P2: Array value-witness operations
#[allow(non_snake_case)]
pub mod ArrayValueWitness;

/// P2: Generic value-witness operations
#[allow(non_snake_case)]
pub mod GenericValueWitness;

/// P2: POD operations
#[allow(non_snake_case)]
pub mod PodOps;

/// P2: Numeric and string conversion
#[allow(non_snake_case)]
pub mod NumericConversion;

/// P2: Stdlib utilities
#[allow(non_snake_case)]
pub mod StdlibUtils;

/// P2: ObjC bridge utilities
#[allow(non_snake_case)]
pub mod ObjCBridge;

/// P2: AutoDiff runtime
#[allow(non_snake_case)]
pub mod AutoDiff;

/// P2: Opaque type runtime
#[allow(non_snake_case)]
pub mod OpaqueTypes;

/// P2: Coroutine support
#[allow(non_snake_case)]
pub mod Coroutine;

/// P2: Runtime path queries
#[allow(non_snake_case)]
pub mod RuntimePaths;

/// P2: Debug variables
#[allow(non_snake_case)]
pub mod DebugVars;

/// P2: Instrumentation
#[allow(non_snake_case)]
pub mod InstrumentationExt;

/// ABI struct layouts (HeapObject, ValueWitnessTable, Metadata, etc.)
#[allow(non_snake_case)]
pub mod SwiftABI;

/// Concurrency executor hooks (global function pointers for custom executors)
#[allow(non_snake_case)]
pub mod ConcurrencyHooks;

/// Calling convention documentation and safety notes
#[allow(non_snake_case)]
pub mod SwiftCallingConvention;

/// Safe thunks for Swift-CC functions (arm64 inline assembly)
#[allow(non_snake_case)]
pub mod SwiftCCThunks;

/// Swift stdlib type metadata helpers
#[allow(non_snake_case)]
pub mod StdlibTypes;

/// Runtime dlsym resolution of SPI symbols
#[allow(non_snake_case)]
pub mod DlsymStdlib;

/// Arm64 asm thunks for all concurrency + error Swift-CC/swiftasync-CC functions
#[allow(non_snake_case)]
pub mod ConcurrencyThunks;

/// Dynamic SwiftUI View conformance builder (approach 3)
#[allow(non_snake_case)]
pub mod ViewConformanceBuilder;

// ── Platform-specific ──
#[cfg(target_os = "windows")]
pub mod Win32;
