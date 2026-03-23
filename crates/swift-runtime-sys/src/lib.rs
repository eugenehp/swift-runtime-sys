#![allow(
    non_snake_case,
    rustdoc::bare_urls,
    rustdoc::invalid_html_tags,
    rustdoc::broken_intra_doc_links,
)]

// ═══════════════════════════════════════════════════════════════════════════
// Auto-generated bindgen modules (from Swift runtime C/C++ headers)
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// ABI layout & calling convention
// ═══════════════════════════════════════════════════════════════════════════

/// ABI struct layouts: `HeapObject`, `ValueWitnessTable`, `Metadata`, etc.
pub mod SwiftABI;
/// Calling-convention documentation and safety notes.
pub mod SwiftCallingConvention;
/// Safe thunks for Swift-CC functions (arm64 inline assembly).
pub mod SwiftCCThunks;
/// Arm64 asm thunks for concurrency + error Swift-CC/swiftasync-CC functions.
pub mod ConcurrencyThunks;

// ═══════════════════════════════════════════════════════════════════════════
// Core runtime bindings (P0 — critical path)
// ═══════════════════════════════════════════════════════════════════════════

/// Dynamic casting (`as?`, `as!`, `is`).
pub mod DynamicCast;
/// Error handling (alloc, inspect, throw).
pub mod ErrorHandling;
/// Metadata introspection (types, functions, tuples, existentials).
pub mod MetadataIntrospection;
/// Concurrency runtime (tasks, actors, executors, groups, async let).
pub mod ConcurrencyRuntime;
/// Concurrency ABI types and layouts.
pub mod ConcurrencyAbi;
/// Concurrency executor hooks (global function pointers for custom executors).
pub mod ConcurrencyHooks;

// ═══════════════════════════════════════════════════════════════════════════
// Runtime bindings (P1 — important)
// ═══════════════════════════════════════════════════════════════════════════

/// Enum tag manipulation and metadata init.
pub mod EnumOps;
/// Box and existential allocation.
pub mod BoxExistential;
/// Unowned reference operations.
pub mod UnownedRef;
/// Unknown-object (ObjC-bridged) retain/release.
pub mod UnknownObject;
/// In-process reflection mirror.
pub mod ReflectionMirrorInProcess;
/// Witness tables and protocol conformance.
pub mod WitnessTable;
/// Bridge-object retain/release.
pub mod BridgeObject;
/// Nonatomic reference counting variants.
pub mod NonatomicRefCounting;

// ═══════════════════════════════════════════════════════════════════════════
// Runtime bindings (P2 — supplementary)
// ═══════════════════════════════════════════════════════════════════════════

/// Debugging and diagnostics hooks.
pub mod DebugHooks;
/// Debug variables.
pub mod DebugVars;
/// KeyPath runtime.
pub mod KeyPathRuntime;
/// Function replacement and dynamic dispatch.
pub mod FunctionReplacementExt;
/// Class and struct metadata initialization.
pub mod ClassMetadataInit;
/// Memory and allocation primitives.
pub mod MemoryAlloc;
/// Array value-witness operations.
pub mod ArrayValueWitness;
/// Generic value-witness operations.
pub mod GenericValueWitness;
/// POD operations.
pub mod PodOps;
/// Numeric and string conversion.
pub mod NumericConversion;
/// Stdlib utilities.
pub mod StdlibUtils;
/// ObjC bridge utilities.
pub mod ObjCBridge;
/// AutoDiff runtime.
pub mod AutoDiff;
/// Opaque type runtime.
pub mod OpaqueTypes;
/// Coroutine support.
pub mod Coroutine;
/// Runtime path queries.
pub mod RuntimePaths;
/// Instrumentation.
pub mod InstrumentationExt;

// ═══════════════════════════════════════════════════════════════════════════
// Higher-level runtime helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Remote mirror (out-of-process reflection).
pub mod RemoteMirror;
/// Runtime contract helpers.
pub mod RuntimeContract;
/// Runtime factory (convenience constructors).
pub mod RuntimeFactory;
/// Raw runtime function pointers.
pub mod RuntimeRaw;
/// Rust ↔ Swift executor interop.
pub mod RustExecutorInterop;
/// Swift symbol demangler.
pub mod SymbolDemangler;
/// Swift stdlib type metadata helpers.
pub mod StdlibTypes;
/// Runtime dlsym resolution of SPI symbols.
pub mod DlsymStdlib;

// ═══════════════════════════════════════════════════════════════════════════
// SwiftUI bridge (experimental)
// ═══════════════════════════════════════════════════════════════════════════

/// Dynamic SwiftUI `View` conformance builder (approach 3 — experimental).
pub mod ViewConformanceBuilder;
/// SwiftUI bridge — construct and display SwiftUI views from Rust.
pub mod SwiftUIBridge;

// ═══════════════════════════════════════════════════════════════════════════
// Platform
// ═══════════════════════════════════════════════════════════════════════════

/// Platform detection, OS constants, and SDK path helpers.
pub mod platform;

/// Win32 compatibility shims.
#[cfg(target_os = "windows")]
pub mod Win32;
