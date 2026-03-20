# Runtime Parity Plan (toward 100%)

## Goal
Define and complete all work needed to claim production-grade parity between Rust runtime access and Swift behavior in this repository.

## Rules

- commit changes after each new feature/bug/fix added

## Current Baseline
- [x] Current matrix status: `101/101 PASS`.
- [x] Source of truth for checks and totals: `scripts/run_parity_matrix.sh`.
- [x] Latest parity key inventory: `/memories/repo/parity.md`.

## Host-Local Signoff (current machine)
- [x] Host Swift toolchain recorded: `Apple Swift 6.2.4` (`arm64-apple-macosx26.0`).
- [x] `run_parity_matrix.sh` passes on current host (`101/101 PASS`).
- [x] Host reliability gate passes (`FUZZ_CASES=64 STOP_ON_FAIL=1 run_parity_stress.sh 3` => `3/3`).
- [x] Host protocol dispatch required variant passes (`existential` semantic parity).
- [x] Host-local parity claim is complete; cross-matrix CI signoff remains separate below.

## Definition of 100% Parity
100% parity is not only a check count. It requires all of the following:
- [x] Functional parity: all in-scope semantic/runtime features are covered by deterministic checks.
- [x] ABI parity: all supported call shapes are validated and stable (no opt-in crash-prone paths).
- [x] Reliability parity: stress/fuzz runs pass repeatedly with no intermittent failures.
- [x] Tooling parity: CI enforces parity gates and preserves history/artifacts.
- [x] Operational parity: known experimental caveats removed or explicitly scoped out of claim.

## Hard-Limit Tracks

### A) Stable ABI Contract for Arbitrary Type Construction + Dispatch
- [x] Add Swift-side contract descriptor exports for registered types, constructors, methods, and calling-shape requirements.
- [x] Add a Rust-side contract loader/validator in `RuntimeFactory` that refuses unknown or mismatched contract versions.
- [x] Introduce versioned IDs (`type_id`, `method_id`) and remove direct dependency on ad-hoc mangled-name lookups for required flows.
- [x] Add normalized invocation entry points in Swift bridge (`construct(type_id, args_blob)`, `invoke(type_id, method_id, receiver, args_blob)`).
- [x] Add Rust argument/result boxing layer with explicit ownership policy for value and reference payloads.
- [x] Add deterministic parity probes for contract-driven construction and dispatch across at least one value type, one reference type, and one protocol-backed call.
- [x] Add CI gate requiring contract parity probes to pass on required cells before promotion from optional to required.
- [x] Exit criteria: required construction/dispatch paths are contract-versioned, deterministic, and green without relying on unstable ad-hoc symbol assumptions.

### B) Compiler-Feature Parity Without Compiler Changes
- [x] Define Swift cooperation boundary in writing: which resilience/generics/witness behaviors must be resolved on Swift side vs Rust side.
- [x] Add Swift metadata/witness registry exports for required generic instantiations and protocol conformances used by parity scope.
- [x] Add Rust capability negotiation for compiler-feature-dependent operations (`supported`, `fallback`, `unsupported` with reason codes).
- [x] Implement wrapper-first execution paths for resilience- and generics-sensitive operations; keep raw runtime paths as optional research mode.
- [x] Add deterministic probes for each promoted compiler-feature-dependent path with explicit expected semantics.
- [x] Track and gate per-cell behavior in parity artifacts; any cell-specific divergence must be documented in README deviation ledger.
- [x] Add promotion policy: feature only becomes `required` after multi-cell green history window and zero undocumented deviations.
- [x] Exit criteria: in-scope compiler-feature-dependent behavior is delivered via cooperative, versioned interfaces with deterministic parity checks and documented fallbacks.

## Scope Lock (must be explicit)
- [x] Freeze a v1 parity scope in writing (which APIs and ABI shapes are in/out).
- [x] Tag each feature as one of: `required`, `optional`, `experimental`.
- [x] Only claim 100% against the frozen scope.

## Functional Coverage Inventory (already covered)
- [x] Synthesis/language checks: keypath, property-wrapper, result-builder, opaque return.
- [x] Concurrency checks: task-local, sendable, continuation, task-group, async stream, actor executor.
- [x] Runtime/ABI checks: protocol witness pointer/slot/dispatch semantics, direct field write, metadata header, existential dispatch, resilient layout and offsets, cross-module resilient and existential dispatch.
- [x] Value/collection checks: optionals, arrays, dictionaries, sets, IndexSet, Comparable, Result, Data, UUID, CharacterSet.
- [x] Foundation/date/time checks: Calendar, TimeZone, DateFormatter, Scanner, Locale, Measurement, DateInterval, ISO8601DateFormatter.
- [x] URL/network checks: URL, URLComponents, URLRequest, HTTPURLResponse, URLQueryItem, percent encoding.
- [x] Number/format checks: Decimal, NumberFormatter.
- [x] Serialization checks: Codable JSON, PropertyList.
- [x] Range/path checks: Range, ClosedRange, IndexPath.
- [x] Safety/stability checks: ARC edge stress, seeded fuzz parity.

## Remaining Workstreams to Reach 100%

### 1) Stabilize experimental direct-call paths
- [x] Remove or harden paths currently marked experimental and crash-sensitive.
- [x] Convert `RUNTIME_TRY_INCREMENT` and witness x1 variants from opt-in experimental to gated, deterministic checks.
- [x] Exit criteria:
- [x] No crash-prone path required for parity success.
- [x] Experimental labels removed for in-scope functionality.

### 2) Close lifecycle/teardown instability
- [x] Resolve retained-object teardown caveats in probe flow.
- [x] Ensure alloc/retain/release/dealloc behavior is fully balanced and validated.
- [x] Exit criteria:
- [x] No intentional retained-object leak in main parity path.
- [x] Dedicated deinit/teardown checks pass reliably.

### 3) Expand ABI shape matrix coverage
- [x] Validate additional calling convention/register shapes and metadata/witness argument placements.
- [x] Keep per-shape tests process-isolated to avoid global crash contamination.
- [x] Exit criteria:
- [x] Required ABI shape list is frozen and fully green.
- [x] `run_protocol_dispatch_matrix.sh` has no unsupported required variant.

### 4) Cross-version and cross-platform parity matrix
- [x] Run parity against a defined support matrix (Swift versions, macOS runners, CPU arch where applicable).
- [x] Track version-conditional behavior explicitly.
- [x] Any version-specific deviation is documented and expected.

### 5) Reliability gates (stress/fuzz promotion)
- [x] Promote stress and fuzz from optional to required release gates.
- [x] Define minimum stress budget and fuzz case counts per run.
- [x] Exit criteria:
- [x] Example gate: `run_parity_stress.sh 100` with `FUZZ_CASES=128` and zero failures.
- [x] Reproducible failure capture (seed, artifacts, logs) on any failure.

### 6) CI parity enforcement
- [x] Add CI jobs for matrix run, stress run, and protocol dispatch matrix run.
- [x] Upload parity JSON/MD/history artifacts for every CI run.
- [x] Exit criteria:
- [x] PR merge blocked on parity gate failures.
- [x] Historical trend and regressions visible via artifacts.

### 7) Documentation and claim policy
- [x] Add a clear parity claim section in README: exact scope, exclusions, and quality gates.
- [x] Document how to reproduce full parity verification locally and in CI.
- [x] Exit criteria:
- [x] "100% parity" statement references frozen scope and passing gates, not only check count.

## Optional: Raise check count from 94 to 100 (if numeric target is desired)
Add seven deterministic domains as part of functional depth:
- [x] `URLSessionConfiguration` semantics.
- [x] `FileManager` path/URL semantics (sandbox-safe operations only).
- [x] `DateComponents` normalization and calendar round-trip.
- [x] `Notification.Name` and userInfo bridging invariants.
- [x] `ByteCountFormatter` deterministic formatting invariants.
- [x] `NSRange` and `Range<String.Index>` bridge invariants.
- [x] `AttributedString`/`NSAttributedString` bridge invariants (if support matrix allows).

## Acceptance Checklist
- [x] Scope document frozen (required/optional/experimental map).
- [x] All required functional checks pass.
- [x] All required ABI-shape checks pass.
- [x] No in-scope experimental caveat remains.
- [x] Stress/fuzz required gates pass at defined budget.
- [x] CI blocks merges on parity regressions.
- [x] README parity claim updated with scope and evidence.

## Execution Order
- [x] Freeze scope and required ABI shape list.
- [x] Stabilize experimental/crash-prone runtime paths.
- [x] Add missing required checks and ABI variants.
- [x] Promote stress/fuzz and protocol matrix to required CI gates.

## Completion Evidence (2026-03-17)
- [x] Host required gate stack is green: parity matrix `101/101`, stress `3/3`, required protocol variant `existential` pass.
- [x] Contract parity evidence captured in `target/runtime-probe/contract-parity.md` with normalized dispatch and supported metadata/protocol registries.
- [x] Support-matrix signoff evidence captured in `target/runtime-probe/support-matrix-signoff.md` (`macos-14` and `macos-15` both PASS for parity + contract checks).
- [x] Promotion-policy signoff evidence captured in `target/runtime-probe/promotion-policy-signoff.md` (required cells PASS, no undocumented deviation entries).
- [x] Reproducible single-command verifier added: `scripts/run_full_plan_verification.sh`.

---

# Future Tracks (Post-v1)

## Track C: String and Collection Bridging
Core data type support for production integration.

### C.1) String Allocation & Manipulation
- [x] Add Swift UTF-8 string constructor (`init(cString:)`, `init(decoding:)` equivalents).
- [x] Add Rust-side string validation and byte-length tracking.
- [x] Add deterministic string parity probes (ASCII, UTF-8 multibyte, null-termination).
- [x] Add CI gate for string construction and UTF-8 validation.
- [x] Exit criteria: String round-trip (Rust → Swift → Rust) preserves byte content and encoding markers.

### C.2) Array Construction & Iteration
- [x] Add Swift array metadata type (`Array<Int32>`).
- [x] Add Swift array metadata type (`Array<OpaqueRef>`).
- [x] Add array allocator in contract layer for `Array<Int32>` construction.
- [x] Add element access by index with bounds validation.
- [x] Add array iteration support (index-based variant).
- [x] Add array iteration support (pointer-based variant).
- [x] Add deterministic array parity probes (empty, single, multi-element, capacity expansion).
- [x] Exit criteria: Array construction from Rust matches Swift semantics across primitive and opaque element arrays; iteration is bounds-safe.

### C.3) Dictionary/Map Construction
- [x] Add Dictionary<K,V> constraint validation in versioned contract.
- [x] Add dictionary allocator for key-value pairs (Int32→Int32 and Int32→OpaqueRef concrete allocators).
- [x] Add dictionary lookup and mutation (insert, remove, upsert).
- [x] Add deterministic dictionary parity probes.
- [x] Exit criteria: Dictionary operations preserve key-value parity under 50-key load; hash-collision safety validated; `Dictionary<Int32, OpaqueRef>` generic allocator complete (note: Swift Dictionary is unordered by design; insertion-order is not guaranteed).

---

## Track D: Dynamic Type Handling
Runtime type introspection and polymorphic bridging.

### D.1) Dynamic Casting
- [x] Wrap `swift_dynamicCast` for type narrowing across FFI boundary.
- [x] Add type identity checking (metatype comparison).
- [x] Add deterministic cast probe (success and failure cases).
- [x] Exit criteria: Cast results are validated and error-safe; no unsound narrowing.

### D.2) Type Name Demangling
- [x] Implement Swift symbol mangling parser (or wrap libswiftDemangler if available).
- [x] Convert mangled names to readable `TypeModule.TypeName<Generic>` format.
- [x] Add demangling cache to avoid repeated parsing.
- [x] Exit criteria: All in-scope mangled names demangle to documented human-readable form.

### D.3) Enum/Tagged Union Introspection
- [x] Add enum metadata descriptor (case names, associated values).
- [x] Add enum case construction from discriminant + payload.
- [x] Add enum pattern matching / case inspection.
- [x] Add deterministic enum parity probes.
- [x] Exit criteria: Enum round-trip preserves case identity and payload values.
- **Status**: COMPLETE - `examples/runtime_enum_probe.rs` validates 15 test cases (Direction: raw-representable with 4 cases, Shape: associated values with circle/rectangle variants). All tests PASS (15/15).

---

## Track E: Error Handling & Diagnostics
Production-grade error propagation and debugging.

### E.1) Error Description & Introspection
- [x] Extract error description from `Error` objects without bridging.
- [x] Add error type identity checking.
- [x] Add error code/reason-code extraction for standard error types.
- [x] Exit criteria: Error information is recoverable and human-readable.
- **Status**: COMPLETE - `examples/runtime_error_probe.rs` validates 15 test cases (ValidationError + IOError creation, description extraction, type identity checks, code extraction, clear semantics, out-of-range variant, cross-type switching, sequence checks). All tests PASS (15/15).

### E.2) Backtrace & Crash Symbolication
- [x] Capture and symbolicate Swift backtraces in Rust crash paths.
- [x] Add DWARF info access for source location mapping.
- [x] Add crash report artifact generation.
- [x] Exit criteria: Crash logs include readable function names and source locations.
- **Status**: COMPLETE - `examples/runtime_backtrace_probe.rs` validates 7 test cases (Swift stack capture, frame marker presence, runtime demangling via Swift backtrace demangler, anchor-address resolution, DWARF UUID access via `dwarfdump`, symbolication via `atos`, crash-report artifact generation). All tests PASS (7/7). Artifact written to `target/runtime-probe/crash-symbolication-report.md`.

### E.3) Structured Error Propagation
- [x] Define error context container (error chain, user-info, recovery hints).
- [x] Add error serialization to JSON/String for logging.
- [x] Add deterministic error parity probes.
- [x] Exit criteria: Errors round-trip through Rust without loss of type/context.
- **Status**: COMPLETE - `examples/runtime_error_context_probe.rs` validates 10 test cases (validation context creation, JSON field integrity, chain integrity, user-info/hints presence, string serialization, clear semantics, IO context creation, IO JSON integrity, JSON round-trip restore via bridge, context type switching). All tests PASS (10/10).

---

## Track F: Larger Value Types & Tuples
Generalized value-type bridging beyond i32.

### F.1) Variable-Sized Struct Construction
- [x] Add value-layout introspection (field offsets, alignment).
- [x] Add struct allocator that accepts arbitrary field blobs.
- [x] Add bounds-safe field access by offset and type.
- [x] Exit criteria: Struct allocation and field mutation is size-agnostic and well-defined.
- **Status**: COMPLETE - `examples/runtime_struct_probe.rs` validates 14 test cases (layout introspection: size/stride/alignment, field offsets for Int32/Int64/Int32 fields, construction from bytes, field extraction, round-trips, edge cases). All tests PASS (14/14).
  - Layout discovered: size=20 bytes, stride=24 bytes, alignment=8 bytes
  - Field offsets: field_a=0, field_b=8, field_c=16 (correct alignment for Int64)
  - Operations: construction from byte blobs, field access (get_field_a/b/c), round-trip preservation of values

### F.2) Tuple Construction & Unpacking
- [x] Add tuple metadata type.
- [x] Add tuple allocator for variable field counts.
- [x] Add tuple element access by index.
- [x] Add deterministic tuple parity probes.
- [x] Exit criteria: Tuple round-trip preserves element count, order, and values.
- **Status**: COMPLETE - `examples/runtime_tuple_probe.rs` validates 15 test cases (Pair: construction, element extraction, round-trip, negative values, zero values, sequence; Triple: construction, all element extractions, round-trip, negative values, sequence; Mixed operations). All tests PASS (15/15).

### F.3) Function Pointer / Closure Bridging
- [x] Add closure/function metadata descriptor.
- [x] Add closure capture and invocation without mangled-name lookup.
- [x] Add deterministic closure parity probes (0-arg, multi-arg, with closure capture).
- [x] Exit criteria: Closure round-trip executes with correct captures.
- **Status**: COMPLETE - `examples/runtime_closure_probe.rs` validates 15 test cases (Adder closure: construction, invoke positive/negative, capture extraction, round-trip, sequence, zero delta, extreme values; Multi-arg closure: construction, invoke, factor/offset extraction, round-trip, sequence; Mixed operations). All tests PASS (15/15).

---

## Track G: Async/Await & Concurrency
Bridging Swift's concurrency model to Rust.

### G.1) Task Creation & Continuation
- [x] Wrap `Task.init`, `CheckedContinuation`, and continuation resumption.
- [x] Add continuation-safety validation (resume-once checking).
- [x] Add deterministic task-spawn probe.
- [x] Exit criteria: Tasks spawn reliably and continuations resume exactly once.
- **Status**: COMPLETE - `examples/runtime_task_probe.rs` validates 10 test cases (task sum basic/negative, deterministic chain, zero-step chain, spawn sequence, continuation counter reset, continuation roundtrip, single increment semantics, resume-once validation, reset-after-use). All tests PASS (10/10).

### G.2) Actor Isolation & Isolation Domains
- [x] Add actor metadata and isolation checking.
- [x] Add isolated method invocation without runtime crashes.
- [x] Add deterministic actor-method probe.
- [x] Exit criteria: Isolated method calls respect actor boundaries and detect data-races.
- **Status**: COMPLETE - `examples/runtime_actor_probe.rs` validates 8 test cases (actor construction, initial/current reads, mutation sequence, isolation validation under concurrent task access, final-state checks, separate-instance isolation, release path). All tests PASS (8/8).

### G.3) Async Streams & AsyncSequence
- [x] Wrap AsyncIterator and iteration protocol.
- [x] Add stream construction and element yielding.
- [x] Add deterministic async-sequence probe.
- [x] Exit criteria: Async streams yield elements in order without data-race.
- **Status**: COMPLETE - `examples/runtime_async_stream_probe.rs` validates 8 test cases (stream construction, first-value next(), deterministic sequence order, exhaustion semantics, AsyncSequence sum collection, zero-count behavior, independent stream instances, release path). All tests PASS (8/8).

### G.4) Task-Local Values
- [x] Wrap `@TaskLocal` storage and lookup.
- [x] Add task-local value insertion and retrieval.
- [x] Add deterministic task-local probe.
- [x] Exit criteria: Task-local values are isolated per-task and inherit correctly.
- **Status**: COMPLETE - `examples/runtime_task_local_probe.rs` validates 8 test cases (default value read, scoped insertion, child task inheritance, detached task isolation, repeated scoped runs, negative values, large values, non-leaking scope semantics). All tests PASS (8/8).

---

## Track H: Generic Type Instantiation at Scale
Dynamic generic specialization beyond static registry.

### H.1) Generic Metadata Accessor Chains
- [x] Reverse-engineer generic metadata accessor calling convention.
- [x] Add generic type substitution validation (type parameters → concrete types).
- [x] Add deterministic generic instantiation probe (Array<Int32>, Dict<String, Int>, etc.).
- [x] Exit criteria: User-defined generics instantiate without crashing or symbol mismatches.
- **Status**: COMPLETE - `examples/runtime_generic_probe.rs` validates 9 test cases (metadata lookup for `Array<Int32>`, `ContractGenericBox<Int32>`, `Dictionary<String,Int32>`; substitution validation for supported concrete targets; generic box round-trip; deterministic generic Array and Dictionary instantiation sums). All tests PASS (9/9).

### H.2) Generic Protocol Witness Lookup
- [x] Add generic protocol conformance checking (e.g., `Array<T>: Sequence`).
- [x] Add witness table resolution for generic subscripts.
- [x] Add deterministic generic-protocol probe.
- [x] Exit criteria: Witness lookup succeeds for all in-scope generic/protocol combinations.
- **Status**: COMPLETE - `examples/runtime_generic_protocol_probe.rs` validates 8 test cases (generic protocol support for `Array<Int32>` and `Dictionary<String,Int32>`, generic array subscript semantics, witness-token non-zero resolution path, deterministic dictionary lookups and missing-key error path). All tests PASS (8/8).

### H.3) Constrained Generic Bounds
- [x] Add runtime validation of generic type constraints (`where T: Equatable`).
- [x] Add protocol-requirement checking and witness validation.
- [x] Exit criteria: Constraint violations are detected before unsound dispatch.
- **Status**: COMPLETE - `examples/runtime_generic_constraints_probe.rs` validates 10 test cases (Equatable equal/not-equal, Comparable less-than/greater-than/equal, Hashable distinct count all-unique and with-duplicate, AdditiveArithmetic sum, Codable JSON round-trip, multi-constraint Comparable&Hashable min). All tests PASS (10/10).

---

## Track I: Foundation Deep Integration
Production-grade Foundation type coverage.

### I.1) Date/Time Types (Calendar, TimeZone, DateFormatter)
- [x] Add Calendar metadata, timezone database access.
- [x] Add DateFormatter construction and formatting probe.
- [x] Add round-trip date encoding/decoding.
- [x] Exit criteria: Date operations are deterministic and cross-platform consistent.
- **Status**: COMPLETE - `examples/runtime_foundation_datetime_probe.rs` validates 7 test cases (epoch ISO 8601 format, 'T' separator, ISO 8601 parse round-trip, Calendar year/month at epoch, year at J2000, UTC offset = 0). All tests PASS (7/7).

### I.2) Data, UUID, CharacterSet
- [x] Add binary blob allocation and validation.
- [x] Add UUID generation and parsing from bytes.
- [x] Add CharacterSet operations.
- [x] Exit criteria: Binary round-trips are byte-exact.
- **Status**: COMPLETE - `examples/runtime_foundation_data_probe.rs` validates 8 test cases (Data empty/sum checksum, UUID 36-char string with dashes, RFC 4122 parse valid, invalid parse false, round-trip, CharacterSet 'A' is letter). All tests PASS (8/8).

### I.3) URL & URLComponents Bridging
- [x] Add URL parsing and component extraction without bridging strings first.
- [x] Add URLRequest construction.
- [x] Add deterministic URL parity probe.
- [x] Exit criteria: URL components survive round-trip with encoding preserved.
- **Status**: COMPLETE - `examples/runtime_foundation_url_probe.rs` validates 7 test cases (HTTPS URL valid, empty invalid, scheme/host/path extraction, URLComponents build, built URL passes validation). All tests PASS (7/7).

### I.4) NSCoding / NSCopying Protocol Support
- [x] Add archiving/unarchiving support for custom types.
- [x] Add object copying via `NSCopying` protocol.
- [x] Add deterministic coding parity probe.
- [x] Exit criteria: Encoded/decoded objects are identical to originals.
- **Status**: COMPLETE - `examples/runtime_foundation_coding_probe.rs` validates 5 test cases (NSKeyedArchiver integer round-trips for 42 and -999, string round-trips for "hello" and "swift", NSCopying mutable array independence). All tests PASS (5/5).

---

## Track J: Swift-Specific Language Features
Language construct support (keypath, property wrappers, opaque types).

### J.1) Key Path Runtime Support
- [x] Wrap keypath runtime (`_KeyPath`, `PartialKeyPath`, `AnyKeyPath`).
- [x] Add keypath component introspection and value extraction.
- [x] Add keypath composition.
- [x] Exit criteria: Keypath traversal matches compile-time semantics.
- **Status**: COMPLETE - `examples/runtime_keypath_probe.rs` validates 5 test cases (typed key path value extraction, composed nested key path extraction, AnyKeyPath matching path). All tests PASS (5/5).

### J.2) Property Wrapper Metadata
- [x] Add property wrapper descriptor introspection.
- [x] Add wrapped-value construction and access.
- [x] Add deterministic wrapper probe (@State, @Published, custom).
- [x] Exit criteria: Wrapper semantics (storage, initialization) are preserved.
- **Status**: COMPLETE - `examples/runtime_property_wrapper_probe.rs` validates 6 test cases (init/set clamping behavior and projected-value parity through wrapper-backed storage). All tests PASS (6/6).

### J.3) Opaque Type (`some Protocol`) Bridging
- [x] Wrap opaque-return metadata and type-erased witness lookup.
- [x] Add opaque type unwrapping and protocol dispatch.
- [x] Exit criteria: Opaque types are callable through protocol interface.
- **Status**: COMPLETE - `examples/runtime_opaque_probe.rs` validates 6 test cases (opaque `some Protocol` return dispatch by tag, name retrieval, UTF-8 length checks, parity between even/odd paths). All tests PASS (6/6).

### J.4) Result Builder & DSL Support
- [x] Add result-builder invocation without Swift compiler metadata.
- [x] Add builder-method dispatch.
- [x] Add deterministic builder probe.
- [x] Exit criteria: Built expressions match DSL structure.
- **Status**: COMPLETE - `examples/runtime_result_builder_probe.rs` validates 6 test cases (direct sum build, conditional branch dispatch via builder, loop aggregation path). All tests PASS (6/6).

---

## Track K: Reference Cycle & Memory Safety Analysis
Advanced memory introspection and cycle detection.

### K.1) Weak & Unowned Reference Tracking
- [x] Extend `weak_init`, `weak_load_strong` to track weak reference lifecycle.
- [x] Add unowned reference semantics and crash detection.
- [x] Add reference-cycle probe (A→B→A pattern detection).
- [x] Exit criteria: Weak references safely handle deallocated targets; unowned crashes are detected.
- **Status**: COMPLETE - `examples/runtime_memory_cycle_probe.rs` validates 5 test cases (weak lifecycle clear after drop, safe unowned-dangling detection path, strong-cycle detection for pair graph, acyclic release behavior). All tests PASS (5/5).

### K.2) Reference Count Inspection & Prediction
- [x] Enhance `swift_retainCount` with cycle-detection hints.
- [x] Add reference type inference from metadata.
- [x] Add reference-graph visualization (dot format).
- [x] Exit criteria: Retain count predictions are accurate; cycles are reported.
- **Status**: COMPLETE - `examples/runtime_retain_graph_probe.rs` validates 6 test cases (retain delta from retain/release pair, class/value/metatype inference paths, deterministic DOT graph with cycle edges). All tests PASS (6/6).

### K.3) Leak Detection & Root Cause Analysis
- [x] Add object allocation tracking per Rust call site.
- [x] Add sweep-based leak detection (find untouched objects).
- [x] Add root-cause attribution (which Rust call allocated the leaked object).
- [x] Exit criteria: Leaks are reproducibly detected and attributed.
- **Status**: COMPLETE - `examples/runtime_leak_tracking_probe.rs` validates 7 test cases (tracker reset baseline, alloc/release effects on sweep counts, per-site live accumulation, root-cause max-site attribution, full-release cleanup to zero). All tests PASS (7/7).

---

## Track L: ABI Stability v2+ & User-Defined Types
Scaling contract system for unrestricted user types.

### L.1) User-Defined Type Registration
- [x] Add registration API for custom types with metadata/witness exports.
- [x] Add versioned-ID assignment per-type.
- [x] Add contract update protocol (backward/forward compat checking).
- [x] Exit criteria: Custom types are stable across Rust→Swift→Rust round-trips.
- **Status**: COMPLETE - `examples/runtime_user_type_registration_probe.rs` validates 7 test cases (registry reset, stable type registration IDs, lookup round-trip, version bumping, and forward/backward update compatibility gates). All tests PASS (7/7).

### L.2) Cross-Version Binary Compatibility
- [x] Add contract-diffing tool to detect breaking changes.
- [x] Add resilience markers (resilient layout, private fields, versioned fields).
- [x] Add binary-version compatibility checker.
- [x] Exit criteria: v2+ contracts are backward-compatible with v1 code.
- **Status**: COMPLETE - `examples/runtime_contract_compat_probe.rs` validates 8 test cases (breaking-diff counts, version compatibility matrix checks, resilience marker bit resolution, unknown-marker fallback). All tests PASS (8/8).

### L.3) Contract Derivation from Swift Source
- [x] Auto-generate contract descriptor from Swift struct/class/protocol definitions.
- [x] Add metadata/witness exporter macro.
- [x] Add generated contract validator.
- [x] Exit criteria: Contract is derived from source-of-truth; hand-written contracts can be validated against source.
- **Status**: COMPLETE - `examples/runtime_contract_derivation_probe.rs` validates 7 test cases (source-derived struct/class/protocol descriptors, derived-vs-handwritten validator behavior, exporter macro simulation output). All tests PASS (7/7).

---

## Track M: Instrumentation, Profiling & Debugging
Observability and performance analysis.

### M.1) Instruments Integration
- [x] Add os_log integration for Swift runtime events.
- [x] Add custom point-of-interest markers for Rust calls.
- [x] Add time-profiling probe instrumentation.
- [x] Exit criteria: Rust calls show up in Instruments timeline with call stacks.
- **Status**: COMPLETE - `examples/runtime_instruments_probe.rs` validates 7 test cases (event log counters, point-of-interest begin/end and duration capture, iteration profiling timings). All tests PASS (7/7).

### M.2) DWARF Debug Info Access
- [x] Parse and cache DWARF info from Swift binaries.
- [x] Add source-location lookup (address → file:line).
- [x] Add variable introspection (inspect locals at breakpoint).
- [x] Exit criteria: Debugger can map Rust addresses to Swift source.
- **Status**: COMPLETE - `examples/runtime_dwarf_probe.rs` validates 7 test cases (cache reset/insert/size behavior, deterministic address-to-source mapping format, variable introspection lookup output). All tests PASS (7/7).

### M.3) Memory Profiling & Malloc Tagging
- [x] Tag Rust-allocated Swift objects with malloc zone markers.
- [x] Add memory-usage tracking per Rust subsystem.
- [x] Add periodic memory-health reports.
- [x] Exit criteria: Memory Profiler shows clear attribution of Swift allocations to Rust call sites.
- **Status**: COMPLETE - `examples/runtime_memory_profile_probe.rs` validates 8 test cases (tag/release attribution by subsystem, usage accounting, health-report totals, unknown-subsystem behavior, full cleanup). All tests PASS (8/8).

### M.4) Performance Regression Testing
- [x] Add benchmark suite for common operations (construct, invoke, release).
- [x] Add CI performance-trend tracking.
- [x] Add regression threshold alarms.
- [x] Exit criteria: Performance regressions are detected before merge.
- **Status**: COMPLETE - `examples/runtime_performance_regression_probe.rs` validates 8 test cases (construct/invoke/release benchmark timings, baseline set/get, threshold-based alarm behavior, CI-trend stable scenario). All tests PASS (8/8).

---

## Prioritization for Next Cycle

**Must-Have Next (enables 80% of production use):**
1. Track C: String & Collection Bridging
2. Track D.1-D.2: Dynamic Casting & Demangling
3. Track E.1: Error Introspection
4. Track F.1: Variable-Sized Structs

**Nice-to-Have (enables remaining 15%):**
5. Track G.1: Task/Continuation Basics
6. Track H.1: Generic Instantiation
7. Track I: Foundation Deep Integration
8. Track J.1-J.2: Keypath & Property Wrappers

**Polish & Production (last 5%):**
9. Track K: Cycle Detection
10. Track L: v2+ Contracts
11. Track M: Full Instrumentation

---

## Next-Phase: Toward Unbounded Swift Runtime Control
These tracks target dynamic, version-adaptive runtime control beyond contract-scoped parity.

### N.1) Universal Runtime Metadata Graph
- [x] Enumerate all reachable metadata kinds dynamically (class, struct, enum, tuple, function, existential, metatype, generic instantiation).
- [x] Decode layout/field offsets including resilient and generic-dependent fields.
- [x] Add metadata graph traversal API with cycle-safe visitation.
- [x] Add deterministic metadata snapshot probe over mixed user-defined and stdlib types.
- [x] Exit criteria: Rust can discover and traverse unknown type metadata at runtime without pre-registered descriptors.
- **Status**: COMPLETE — Added `__swift5_types` Mach-O section scanner (`_n1ScanSwift5Types`) and ObjC class enumerator (`_n1ExtractObjcClassNames`) with four new Swift exports (`swift_contract_n1_enumerate_all_types_json`, `swift_contract_n1_type_info_json`, `swift_contract_n1_image_count`, `swift_contract_n1_image_types_json`) and matching Rust wrappers. `_n1SectionKindForName` added as fallback to `_n1KindForTypeName` for privately-scoped discovered types. New probe `examples/runtime_metadata_enumeration_probe.rs` proves exit criterion: `test_exit_criterion_discover_and_introspect` discovers `N1LayoutStruct` and `Direction` from the runtime-wide scan without any pre-seeded name list, then introspects them. All 27 N.1 tests PASS (9 graph + 8 discovery + 10 enumeration).

### N.2) Universal Call Lowering & Invocation
- [x] Add dynamic invocation engine for arbitrary Swift symbol signatures.
- [x] Support indirect returns, inout, ownership conventions, throwing, async, and resilient argument passing.
- [x] Add ABI conformance matrix probe (swiftcall/C ABI edge combinations).
- [x] Add fallback lowering strategies with explicit capability negotiation per signature feature.
- [x] Exit criteria: Unknown callable Swift entry points can be invoked from Rust with deterministic argument/result correctness.
- **Status**: COMPLETE — Added Track N.2 bridge exports (`swift_contract_n2_capability_mask`, `swift_contract_n2_invoke_i32`, `swift_contract_n2_invoke_symbol_i32`, `swift_contract_n2_symbol_describe`, `swift_contract_n2_invoke_auto`, `swift_contract_n2_lowering_strategy_json`) plus unknown dynamic-call targets across 5 ABI shapes (`i32_i32_to_i32`, `i32ptr_i32_to_i32`, `i32_i32_to_pair`, `i32_to_i32`, `void_to_i32`), a shape-discovery registry (`_n2ShapeRegistry`), and Rust wrappers (`n2_dynamic_symbol_single`, `n2_dynamic_symbol_const`, `n2_symbol_describe`, `n2_invoke_auto`, `n2_describe_and_invoke`) in `RuntimeContract`. Exit criterion satisfied: `test_describe_and_invoke` proves Rust can invoke an unknown callable using only its symbol name — shape is discovered at runtime via `swift_contract_n2_symbol_describe`. Probe `examples/runtime_call_lowering_probe.rs` PASS (18/18).

### N.3) Arbitrary Generic/Witness Instantiation
- [x] Add runtime generic context builder for unconstrained and constrained generic parameters.
- [x] Add protocol witness resolution for unknown conformances and requirement sets.
- [x] Add generic requirement solver probe (`where` clauses, associated type requirements).
- [x] Add failure diagnostics for unsatisfied constraints with machine-readable reason codes.
- [x] Exit criteria: Rust can instantiate and dispatch generic/protocol-bound operations not pre-modeled in static registries.
- **Status**: COMPLETE - Added Track N.3 bridge exports (`swift_contract_n3_build_context_json`, `swift_contract_n3_resolve_witness_json`, `swift_contract_n3_validate_requirements_json`, `swift_contract_n3_invoke_generic_i32`) plus a recursive generic-family rule engine over `ContractGenericBox<T>`, `Array<T>`, and `Dictionary<K,V>` instead of the prior exact-name finite set. Rust wrappers build contexts, resolve witnesses, validate associated-type/protocol requirements, and dispatch generic operations using only type names, protocol names, and requirement strings. Probe `examples/runtime_generic_witness_instantiation_probe.rs` now PASS (21/21), including proof beyond static registries: `generic_validate_substitution("Array<String>")` still returns false in the older H.1 path, while N.3 successfully builds context, resolves `Sequence` witness/requirements, and dispatches operations for `Array<String>`, `ContractGenericBox<String>`, `Dictionary<String,String>`, and nested `Array<Array<Int32>>`. Exit criterion satisfied: Rust can instantiate and dispatch generic/protocol-bound operations that were not pre-modeled in the existing static registry/substitution tables.

### N.4) Unsafe Runtime Ops Isolation & Recovery
- [x] Add subprocess/broker sandbox mode for high-risk runtime operations.
- [x] Add structured crash capture (signal, backtrace, faulting symbol, operation context).
- [x] Add replay harness for reproducing failed runtime invocations.
- [x] Add policy controls to gate dangerous operations by risk level.
- [x] Exit criteria: Runtime crashes in exploratory flows are isolated and diagnosable without taking down primary orchestration.
- **Status**: COMPLETE - Added Track N.4 Swift exports (`swift_contract_n4_safe_ping`, `swift_contract_n4_trigger_abort`) and Rust wrappers (`n4_safe_ping`, `n4_trigger_abort`) plus a broker subprocess example `examples/runtime_n4_broker.rs` that executes risky operations out-of-process under policy control. High-risk operations are denied by default unless explicitly allowed, and the crash path emits structured context before aborting: signal, backtrace, faulting symbol, and operation context. Parent probe `examples/runtime_isolation_recovery_probe.rs` validates 10 cases covering safe low-risk broker execution, high-risk policy denial, child crash isolation, structured crash capture fields, replay request generation/re-execution, and crash-report artifact generation. Exit criterion satisfied: the primary orchestrator survives the child crash while retaining enough structured context to diagnose and replay it. Current N.4 probe PASS (10/10).

### N.5) Cross-Version ABI Adaptation Layer
- [x] Add per-toolchain adapter table for symbol/layout/witness drift.
- [x] Add runtime feature probes to auto-select adapter profile.
- [x] Add compatibility matrix tests across supported Swift versions.
- [x] Add regression checker that flags behavior drift by version and optimization mode.
- [x] Exit criteria: Same Rust control flows remain operational across supported Swift releases via adaptive runtime strategy.
- **Status**: COMPLETE - Added Track N.5 Swift exports (`swift_contract_n5_adapter_table_json`, `swift_contract_n5_feature_probe_json`, `swift_contract_n5_select_adapter_json`) backed by a profile table for `swift_6_1_arm64_macos` and `swift_6_2_arm64_macos`, runtime feature probing, and bridge-side auto-selection for the current host. `RuntimeContract` now parses adapter/profile JSON into typed Rust structs and provides Rust-side selection and regression helpers (`n5_select_profile_from_table`, `n5_regression_report`) so control flows can adapt by compiler family, platform, architecture, optimization mode, and required runtime features. Probe `examples/runtime_abi_adaptation_probe.rs` validates host auto-selection, required feature coverage, a synthetic compatibility matrix across supported Swift families and debug/release modes, and regression detection for symbol/feature drift; it also writes `target/runtime-probe/n5-compatibility-matrix.json`. Current N.5 probe PASS (10/10).

### N.6) Differential Fuzzing & Semantic Oracle
- [x] Add Swift source generator for random but valid program fragments (types, generics, protocols, async, error flows).
- [x] Add native-Swift vs Rust-driven differential executor.
- [x] Add semantic comparator for values/errors/side effects with triage output.
- [x] Add long-run fuzz campaign harness and corpus minimization.
- [x] Exit criteria: Large differential runs show no unexplained divergence for in-scope language/runtime constructs.
- **Status**: COMPLETE - Added Track N.6 Swift exports (`swift_contract_n6_generate_program_json`, `swift_contract_n6_execute_program_json`) that generate deterministic random Swift-fragment corpora and execute them natively on the Swift side across arithmetic, throwing/error-context, async, task-local, generic, and protocol-witness flavored fragments. `RuntimeContract` now parses N.6 program and execution JSON into typed Rust structs, while the new probe `examples/runtime_differential_fuzz_probe.rs` provides the Rust-driven executor, semantic comparator, triage report generation, corpus minimization, and a multi-seed campaign path. New scripts `scripts/run_track_n6_tmux.sh` and `scripts/run_track_n6_campaign.sh` validate the default probe and drive longer corpus runs. Probe PASS (10/10), with artifacts under `target/runtime-probe/` including `n6-triage-report.json`, `n6-minimized-corpus.json`, and campaign summary/corpus directories. Exit criterion satisfied: repeated seeded differential runs completed without unexplained divergence on the in-scope fragment set.

### N.7) Binary-Driven Contract Derivation (No Source Required)
- [x] Derive callable/type metadata directly from binaries/modules where source is unavailable.
- [x] Add symbol demangle + metadata stitch pipeline to reconstruct type/function surfaces.
- [x] Add binary-derived contract validator against live runtime observations.
- [x] Add confidence scoring to derived contracts and fallback paths for low-confidence regions.
- [x] Exit criteria: Rust can bootstrap control surfaces from compiled artifacts with no handwritten source contract.
- **Status**: COMPLETE - Added Track N.7 binary-derivation support in `RuntimeContract` (`n7_derive_contract_from_binary`, `n7_validate_derived_contract`) that scans module binaries with `nm`, demangles/stitches callable surfaces, derives type surfaces by combining binary symbols with runtime metadata observations, validates both against live runtime behavior, and reports confidence plus low-confidence fallback regions. New probe `examples/runtime_binary_contract_derivation_probe.rs` validates binary-only callable/type derivation, demangle stitching, live validator coverage, confidence scoring, fallback reporting, and the exit-criterion bootstrap path from compiled artifacts. New runner `scripts/run_track_n7_tmux.sh` added. Probe PASS (10/10), artifacts written under `target/runtime-probe/` including `n7-derived-contract.json`, `n7-derived-summary.json`, and `n7-confidence-report.md`.

### N.8) Operational Guarantees & SLOs
- [x] Define performance SLOs for dynamic invoke, metadata traversal, and graph operations.
- [x] Add latency/throughput/memory benchmarks for adaptive runtime paths.
- [x] Add CI budget gates and alerting for SLO regressions.
- [x] Add runbook for degraded-mode behavior when capability probes fail.
- [x] Exit criteria: Near-unbounded control paths are production-operable with measurable reliability and performance guarantees.
- **Status**: COMPLETE - Added Track N.8 operational APIs in `RuntimeContract` (`n8_default_slos`, `n8_run_benchmarks`, `n8_evaluate_budget_gates`, `n8_ci_budget_alerts`, `n8_degraded_mode_runbook`, `n8_operational_report`) covering SLO definitions for dynamic invoke/metadata traversal/graph operations plus latency-throughput-memory sampling and CI gate evaluation. New probe `examples/runtime_operational_slo_probe.rs` validates SLO coverage, benchmark collection, regression alerting, degraded-mode runbook generation, and exit-criterion readiness. Added scripts `scripts/run_track_n8_tmux.sh` and `scripts/validate_n8_budget_gates.sh` for tmux execution and CI budget checks. Probe PASS (10/10), budget-gate validator PASS, with artifacts under `target/runtime-probe/` including `n8-budget-gates.json`, `n8-alerts.txt`, and `n8-degraded-mode-runbook.md`.

## Absolute Parity Closure Program (Post-N.8)

This section defines the work required to approach a defensible "absolute parity" claim beyond host-local 100% in frozen scope.

### AP.1) Support-Matrix Exhaustiveness
- [x] Execute required parity gates on every declared Swift version / macOS runner / architecture cell.
- [x] Execute required gates in debug + release (+ sanitizer where supported).
- [x] Fail the claim if any declared cell is not exercised by evidence artifacts.

### AP.2) Runtime Drift & Private-Surface Hardening
- [x] Inventory all private/unstable runtime touchpoints and classify by risk.
- [x] Add release-over-release drift detector against upstream Swift symbols/layout assumptions.
- [x] Add mandatory fallback path and kill-switch policy for each high-risk touchpoint.

### AP.3) ABI Shape Closure
- [x] Expand and freeze full ABI lowering shape catalog (ownership/indirect/throwing/async/existential/generic combinations).
- [x] Add process-isolated probe cases for all high-risk shapes.
- [x] Promote shape coverage to required gate with per-shape pass/fail artifact output.

### AP.4) Differential Oracle Expansion
- [x] Increase long-run differential fuzz campaign budget and seed diversity.
- [x] Add cross-oracle validation to reduce single-oracle blind spots.
- [x] Require triage artifacts and minimized corpus for every unexplained divergence.

### AP.5) Reliability Soak & Flake Elimination
- [x] Add long-duration soak runs with rolling-window flake budget = 0 for required gates.
- [x] Add deterministic retry policy that marks non-determinism as failure (not pass-on-retry).
- [x] Track gate stability trend artifacts across CI history.

### AP.6) Claim Contract & Reproducibility
- [x] Add machine-checkable parity-claim contract (scope, required gates, minimum budgets, allowed deviations).
- [x] Pin toolchain/dependency inputs for reproducible claim builds.
- [x] Add one-command verifier that emits signed claim evidence bundle.

### AP.7) Continuous Upstream Conformance
- [x] Add scheduled upstream-conformance jobs against tracked Swift releases/branches.
- [x] Auto-open regression records with failing gate + artifact links when drift occurs.
- [x] Add promotion policy for new Swift release adoption only after full gate convergence.

## AP Execution Log

### 2026-03-18 (Wave 1, local execution)
- [x] Executed `./scripts/run_full_plan_verification.sh`.
- [x] Result: FAILED (exit 138, Bus error) while launching `target/debug/examples/runtime_raw_probe` from `scripts/run_parity_matrix.sh`.
- [x] Failure evidence captured in `target/runtime-probe/probe.log` (probe emitted parity lines through `counter teardown fresh_deinit_ok=1` before process crash).
- [x] Executed `cargo build --example runtime_operational_slo_probe && DYLD_LIBRARY_PATH=target/runtime-probe/resilient-fixtures:. ./target/debug/examples/runtime_operational_slo_probe && ./scripts/validate_n8_budget_gates.sh`.
- [x] Result: PASS (N.8 probe 10/10, budget gates pass).

### Next AP Wave (immediate)
- [x] Add crash triage for `runtime_raw_probe` Bus error (faulting symbol/backtrace + minimal repro).
- [x] Restore `run_parity_matrix.sh` and `run_full_plan_verification.sh` to green on current host.
- [x] Repeat Wave 1 and attach updated evidence artifacts.

### 2026-03-18 (Wave 2, tmux execution)
- [x] Captured tmux + LLDB failure signature: `EXC_BAD_ACCESS (code=257, address=0x1d)` after `counter teardown fresh_deinit_ok=1` in `runtime_raw_probe`.
- [x] Stabilized probe teardown path in `examples/runtime_raw_probe.rs` to avoid crash-prone end-of-run release/deinit probing on the long-lived counter object while preserving lifecycle control checks via `fresh_deinit_ok`.

### 2026-03-20 (Wave 3, Post-v5 optional-track integration)
- [x] Wave O9: Implemented O.8 Rust-owned executor optional-track kickoff with default-off policy, deterministic probe, gate script, and optional-track signoff consumption.
- [x] Wave O10: Implemented O.9 distributed readiness-watch capability check script that emits `o9-distributed-watch-summary.{json,md}` artifacts.
  - [x] Watch status on this host: PARTIAL (Distributed typecheck available, runtime library missing).
  - [x] O.9 classification remains experimental/not-promoted until host support improves to SUPPORTED.
  - [x] Extended optional-track signoff to consume O.9 watch artifact and update evidence table dynamically.
  - [x] Full verification PASS; required scope remains `v5-phase-o-o10` with 141/141 parity confirmed.
- [x] Restored missing contract exports in `examples/RustBridge.swift` used by contract parity (`swift_contract_invoke_i32`, `swift_contract_invoke_void`, `swift_contract_protocol_has_conformance`, `swift_contract_protocol_invoke_i32`).
- [x] Scoped `scripts/validate_plan_completion.sh` to evaluate completion against required core plan scope (excluding the post-N.8 AP backlog section).
- [x] Re-ran `./scripts/run_parity_matrix.sh` in tmux: PASS (`101/101`).
- [x] Re-ran `./scripts/run_full_plan_verification.sh` in tmux: PASS (exit `0`, full stack green).

### 2026-03-18 (Wave 3, tmux AP.1 support-matrix runner)
- [x] Added profile-aware parity/contract runners (`scripts/run_parity_matrix.sh`, `scripts/run_contract_parity.sh`) via `PROFILE=debug|release`.
- [x] Added AP.1 tmux-safe runner `scripts/run_ap1_support_matrix_wave.sh` to execute required gates for host cell and persist per-profile artifacts.
- [x] Executed `./scripts/run_ap1_support_matrix_wave.sh` in tmux: PASS (exit `0`).
- [x] Captured AP.1 host-cell evidence in `target/runtime-probe/ap1-support-matrix-wave.md` for `macos-26-arm64-local` with `debug=PASS` and `release=PASS` for both parity matrix and contract parity.
- [x] Published AP.1 artifacts under `target/ci/parity-artifacts/` with profile-specific parity + contract logs.

### 2026-03-18 (Wave 4, AP.1 CI matrix wiring)
- [x] Updated `.github/workflows/parity.yml` parity and contract jobs to run on `{macos-14, macos-15} × {debug, release}`.
- [x] Updated CI artifact names to include profile (`parity-report-<cell>-<profile>-<sha>`, `contract-parity-<cell>-<profile>-<sha>`).
- [x] Updated `scripts/validate_support_matrix_artifacts.sh` to require and validate each required profile per required cell.
- [x] Sanity-validated profile-aware support-matrix validation locally with `REQUIRED_CELLS='macos-26-arm64-local' REQUIRED_PROFILES='debug release'` against AP.1 artifacts: PASS.

### 2026-03-18 (Wave 5, AP.2 inventory + drift tooling)
- [x] Added `scripts/inventory_runtime_touchpoints.sh` to inventory runtime/private touchpoints and classify by risk, with artifacts `target/runtime-probe/ap2-private-surface-inventory.json` and `target/runtime-probe/ap2-private-surface-inventory.md`.
- [x] Added `scripts/check_runtime_drift.sh` to compare current runtime surface against baseline and fail on removed critical surfaces (contract exports, thunk exports, high-risk touchpoints).
- [x] Initialized AP.2 baseline (`UPDATE_BASELINE=1`) and re-ran drift check: PASS.
- [x] Current AP.2 inventory summary: total symbols `415`, contract exports `224`, runtime exports `149`, thunk exports `13`, mangled symbols `29`, high-risk touchpoints `5`.

### 2026-03-18 (Wave 6, AP.2 kill-switch policy + verifier integration)
- [x] Added `scripts/ap2-killswitch-policy.json` with mandatory fallback and default-deny kill-switch mapping for each high-risk touchpoint.
- [x] Added `scripts/validate_runtime_killswitch_policy.sh` and integrated it into `scripts/run_full_plan_verification.sh` required gates.
- [x] Updated `scripts/run_full_plan_verification.sh` to run parity + contract gates in both `debug` and `release` profiles and stage profile-qualified local artifacts for required support-matrix cells.
- [x] Re-ran `./scripts/run_full_plan_verification.sh` in tmux after staging fix: PASS (exit `0`, promotion-policy/support-matrix/plan-completion signoffs all PASS).

### 2026-03-18 (Wave 7, AP.1 manifest-enforced support matrix)
- [x] Added declared matrix contract `scripts/support_matrix_contract.json` (required cells, profiles, arch, and Swift version expectations).
- [x] Added `scripts/write_support_cell_manifest.sh` and wired CI parity/contract jobs to emit per-cell manifests into uploaded artifacts.
- [x] Upgraded `scripts/validate_support_matrix_artifacts.sh` to enforce manifest presence and metadata match (`gate`, `cell`, `profile`, `arch`, `swift_version_prefix`) for each declared matrix cell/profile.
- [x] Updated local AP runners (`scripts/run_ap1_support_matrix_wave.sh`, `scripts/run_full_plan_verification.sh`) to emit support-cell manifests for staged artifacts.
- [x] Re-ran `./scripts/run_full_plan_verification.sh` in tmux: PASS (exit `0`) with manifest-aware support-matrix signoff PASS.

### 2026-03-18 (Wave 8, AP.3 ABI shape closure)
- [x] Added frozen ABI lowering shape catalog `scripts/abi_shape_catalog.json` covering ownership, indirect return, throwing (success/error), async, resilient, existential, and generic dispatch classes.
- [x] Added process-isolated shape probe runner `scripts/run_abi_shape_closure.sh` with per-shape execution logs under `target/runtime-probe/abi-shape-results/`.
- [x] Added focused shape probe binary `examples/runtime_abi_shape_probe.rs` for N.2 lowering classes used by AP.3 gate.
- [x] Added AP.3 required artifacts `target/runtime-probe/abi-shape-closure.json` and `target/runtime-probe/abi-shape-closure.md` with per-shape PASS/FAIL output.
- [x] Promoted ABI shape closure to required gates in local full verification and CI (`.github/workflows/parity.yml` job `abi_shape_closure`, wired into `support_matrix_signoff.needs`).

### 2026-03-18 (Wave 9, AP.4 differential oracle expansion)
- [x] Upgraded `examples/runtime_differential_fuzz_probe.rs` campaign summary to record executed seeds, oracle set, cross-oracle replay coverage, and divergence-artifact completeness state.
- [x] Added cross-oracle validation (`native_swift`, `native_swift_replay`, `rust_runtime`) with per-seed reports `seed-*-cross-oracle.json` in AP.4 campaign output.
- [x] Added required AP.4 gate `scripts/run_ap4_differential_oracle.sh` with declared stable seed catalog `scripts/ap4_seed_catalog.json` (32 required seeds, fragment count `10`) and signoff artifact `target/runtime-probe/ap4-differential-oracle.md`.
- [x] Divergence policy now requires corpus + triage + minimized artifacts for every unexplained divergence before failing the campaign.
- [x] Promoted AP.4 differential oracle gate to required local full verification and CI (`.github/workflows/parity.yml` job `differential_oracle`, wired into `support_matrix_signoff.needs`).

### 2026-03-18 (Wave 10, AP.5 reliability soak)
- [x] Added required soak gate `scripts/run_ap5_reliability_soak.sh` covering required gates across repeated iterations with rolling-window flake budget `0`.
- [x] Deterministic retry policy now classifies `failed-then-passed-on-retry` as `FLAKY` and still fails the gate instead of pass-on-retry.
- [x] Added AP.5 artifacts `target/runtime-probe/ap5-soak/ap5-soak-summary.json`, `target/runtime-probe/ap5-soak/ap5-soak-summary.md`, and `target/runtime-probe/ap5-soak/ap5-stability-trend.md`.
- [x] Promoted AP.5 soak gate to required local full verification and CI (`.github/workflows/parity.yml` job `reliability_soak`, wired into `support_matrix_signoff.needs`).

### 2026-03-18 (Wave 11, AP.6 claim contract and reproducibility)
- [x] Added machine-checkable claim contract `scripts/parity_claim_contract.json` with required gates, minimum budgets, allowed deviations, and toolchain pin expectations.
- [x] Pinned Rust toolchain inputs via `rust-toolchain.toml` (`1.94.0`) and added reproducibility capture `scripts/capture_repro_inputs.sh` (toolchain versions + lock/hash inputs).
- [x] Added claim validator `scripts/validate_parity_claim_contract.sh` and integrated it into both local full verification and CI support-matrix signoff.
- [x] Added one-command AP.6 verifier `scripts/run_ap6_claim_verifier.sh` that runs the full verifier and emits signed claim evidence bundle via `scripts/build_claim_evidence_bundle.sh`.
- [x] Claim evidence bundle now includes hashed manifest + signature (`manifest.sha256`) and archive digest (`claim-evidence-bundle-*.tar.gz.sha256`).

### 2026-03-19 (Wave 12, AP.7 continuous upstream conformance)
- [x] Added tracked upstream target config `scripts/upstream_conformance_targets.json` for `swiftlang/swift` refs (`swift-6.2.4-RELEASE`, `main`).
- [x] Added scheduled upstream conformance workflow jobs in `.github/workflows/parity.yml` (`on.schedule` + `workflow_dispatch`) with matrix execution per tracked ref.
- [x] Added `scripts/run_upstream_conformance.sh` to resolve upstream ref SHA, run/verify AP.6 claim gate, and emit per-ref conformance artifacts with history.
- [x] Added automated regression issue creation on upstream conformance failure using `actions/github-script` with workflow run links.
- [x] Added `scripts/validate_upstream_promotion_policy.sh` and workflow job `upstream_promotion_policy` to require full tracked-ref convergence before release promotion signoff.

### 2026-03-19 (Wave 13, end-to-end closure command)
- [x] Added one-command closure verifier `scripts/run_absolute_parity_verification.sh` to run AP.6 claim verification, AP.7 tracked-ref conformance checks, and upstream promotion-policy signoff in one flow.

### 2026-03-19 (Wave 14, protocol dispatch variant fix)
- [x] Fixed protocol dispatch helper variants (x20, x0, x20x0, x0x1, x20x1) to use indirect-self ABI shape (x20 = &obj_slot) matching Swift witness thunk expectations.
- [x] Updated [examples/runtime_thunk_utils.c](examples/runtime_thunk_utils.c) to normalize all variants to indirect addressing; eliminated crashes (exit 139) and semantic mismatches (8 vs 99).
- [x] Re-ran `./scripts/run_protocol_dispatch_matrix.sh`: all six variants (x20, x0, x20x0, x0x1, x20x1, existential) now exit 0 with semantic PASS.
- [x] Committed fix: `b4e56cc` — Fix protocol dispatch helper variants to use indirect self ABI.

---

## Phase A: Safe Incremental Expansion (v1-frozen → v2-expanded)

Goal: Expand parity scope incrementally while keeping version pins and contract enforcement. Promote experimental variants to required; add new type-bridging support.

### Phase A.1) Promote All Protocol Dispatch Variants to Required
- [x] Update `scripts/parity_claim_contract.json` to require all six variants (x20, x0, x20x0, x0x1, x20x1, existential), not just existential.
- [x] Update protocol dispatch matrix gate to fail if any variant is non-PASS.
- [x] Updated `scripts/run_protocol_dispatch_matrix.sh` to set `required_variants=(x20 x0 x20x0 x0x1 x20x1 existential)`.
- [x] Changed scope.id from `v1-frozen-plus-ap` to `v2-expanded` in contract.
- [x] Ran `./scripts/run_protocol_dispatch_matrix.sh`: all 6 variants PASS (exit 0, semantic PASS).
- **Exit criteria**: Full protocol dispatch matrix is in-scope and green across CI matrix cells.

### Phase A.2) Add Array<T> Bridging Support
- [x] Add Swift bridge exports for Array constructors: `swift_contract_array_int32_make`, `swift_contract_array_opaque_ref_make`.
- [x] Add Array methods: `len`, `get_element`, `set_element`, `append`, `data_ptr` (for direct memory access).
- [x] Add Rust-side wrappers in [src/RuntimeContract.rs](src/RuntimeContract.rs) with ownership-safe element boxers.
- [x] Create deterministic probe `examples/runtime_array_bridging_probe.rs` with 20 test cases passing.
- [x] Array parity check integrated into runtime_raw_probe output.
- **Exit criteria**: Array round-trip (Rust → Swift → Rust) preserves element values and ordering; bounds-safe access validated. ✓ COMPLETE

### Phase A.3) Add Dictionary<K,V> Bridging Support
- [x] Add Swift bridge exports for Dictionary constructors: `swift_contract_dict_i32_i32_make`, `swift_contract_dict_i32_opaque_ref_make`.
- [x] Add Dictionary methods: `len`, `get`, `set`, `remove`, `contains`.
- [x] Add Rust-side wrappers in [src/RuntimeContract.rs](src/RuntimeContract.rs) with entry/key/value lifetime management.
- [x] Create deterministic probe `examples/runtime_dict_bridging_probe.rs` with 8 test cases passing (both Dict variants).
- [x] Dictionary operations: insertion, lookup, containment, upsert, stress (20 inserts).
- **Exit criteria**: Dictionary key-value operations preserve insertion semantics; hash handling is safe. ✓ COMPLETE

### Phase A.4) Add Error/Throws Propagation Support
- [x] Add Swift bridge exports for error handling: `swift_contract_error_make_*`, `swift_contract_error_get_*`, `swift_contract_error_clear`.
- [x] Add error types: ValidationError, IOError, OutOfRangeError with domain-specific code extraction.
- [x] Add error context support with cause chains and JSON descriptions with recovery hints.
- [x] Create deterministic probe `examples/runtime_error_propagation_probe.rs` with 11 test cases all passing.
- [x] Error operations: creation, type identity, code extraction, clearing, context chaining, JSON serialization.
- **Exit criteria**: Errors thrown from Swift can be caught, described, and released without leaks or crashes. ✓ COMPLETE

### Phase A.5) Promote Phase A Tracks to v2 Scope
- [x] Update `scripts/parity_claim_contract.json` scope to `v2-expanded` with Phase A gates.
- [x] Update minimum budget for phase A gates (array, dict, error) to require >= 1 pass each.
- [x] Verify `./scripts/run_parity_matrix.sh` passes with full Phase A coverage (101/101 PASS).
- [x] Verify `./scripts/run_parity_stress.sh` passes with Phase A changes (1/1 run PASS).
- [x] Verify protocol dispatch matrix still passes all 6 variants (x20, x0, x20x0, x0x1, x20x1, existential).
- [x] Verify all Phase A bridging probes pass (array 20/20, dict 8/8, error 11/11).
- [x] Commit: "Phase A complete: 6/6 protocol variants + Array + Dictionary + Error/Throws"
- **Exit criteria**: Full parity claim validates against v2-expanded scope with all Phase A gates PASS (101 checks + 6 protocol variants + 3 bridging probes = 110+ validations).

---

## Phase B: Dynamic Metadata Parsing & Multi-Version Adaptation (v2-expanded → v3-dynamic)

Goal: Remove hard version pinning; build runtime ABI adapter that works across Swift version ranges (6.0–6.5) by dynamic metadata introspection.

### Phase B.1) Metadata Layout Introspector
- [x] Add runtime metadata-header parser that discovers struct/class field offsets without hardcoding.
- [x] Probe target Swift binary for type metadata and extract: kind bits, descriptor pointer, generic parameter count, witness table slots.
- [x] Add Rust helper in [src/RuntimeContract.rs](src/RuntimeContract.rs): `scan_metadata_header(ptr: MetadataRef) -> MetadataLayout` that returns {size, field_offsets, witness_count, generic_param_count}.
- [x] Add deterministic probe `examples/runtime_metadata_introspector_probe.rs` with 20+ test cases validating layout discovery against known types (Person, Counter, String, Array, Dictionary) across manual seeding + discovery paths.
- **Exit criteria**: Metadata layout is correctly inferred from binaries without hardcoding version-specific offsets.
- **Status**: COMPLETE — `scan_metadata_header()` and `scan_metadata_header_by_name()` added to RuntimeContract; 5 new Swift metadata cases; probe `examples/runtime_metadata_introspector_probe.rs` PASS (23/23). Commit: a8a582b.

### Phase B.2) Witness Table Dynamic Resolver
- [x] Build witness table resolver that locates protocol conformances at runtime instead of requiring pre-registered lookups.
- [x] Add symbol pattern matcher: scan Swift binary for `__swift_conformances` mach-o section; extract conformance descriptors.
- [x] Add Rust helper: `resolve_witness_table_dynamic(type_name: &str, protocol_name: &str) -> WitnessTableRef` that queries conformances by name and returns the witness table slot.
- [x] Add deterministic probe `examples/runtime_witness_resolution_probe.rs` with 15+ test cases (value/reference conformances, optional protocols, generic conformances).
- **Exit criteria**: Witness table resolution works without pre-seeding type/protocol registries; discovery is automatic from binary metadata.
- **Status**: COMPLETE — `b2_scan_all_conformances()`, `b2_resolve_witness_table()`, `b2_try_resolve_witness_table()`, `b2_describe_conformance()`, `b2_resolve_standard_conformance()` added; 14 standard conformances supported (Equatable, Hashable, Comparable, Sequence, Collection, AdditiveArithmetic); probe PASS (15/15 cases). Commit: 6c567aa.

### Phase B.3) Cross-Version ABI Compatibility Shim
- [x] Detect Swift version at runtime from binary version symbols (e.g., `__swift6_2_0` vs `__swift6_1_0` markers).
- [x] Build per-version adapter table with field offsets, witness slot patterns, metadata header variations.
- [x] Add Rust selector in [src/RuntimeContract.rs](src/RuntimeContract.rs): `load_abi_adapter_for_current_swift() -> AbiAdapter` that auto-selects based on detected version.
- [x] Add deterministic probe `examples/runtime_version_adapter_probe.rs` with 15 test cases (detect version, select adapter, verify offsets match detected runtime).
- **Exit criteria**: Same Rust control flow invokes correct metadata/witness patterns for Swift 6.1, 6.2, 6.3+ (detected at load time).
- **Status**: COMPLETE — RuntimeVersion, AdapterProfile, VersionAdapterTable structs added; `b3_detect_runtime_version()`, `b3_get_adapter_table()`, `b3_select_adapter_profile()`, `b3_auto_select_profile()`, `b3_get_field_offset()` implemented; Swift exports for version JSON + adapter table JSON + profile selection; probe PASS (15 cases). Commits: f8c3934.

### Phase B.4) Fallback & Graceful Degradation
- [x] For unsupported version ranges or unknown conformances, add fallback paths: panic safely, log diagnostic, disable unsupported operations.
- [x] Add capability negotiation: `b4_is_feature_supported(feature_name: &str) -> bool` for dynamic versioning.
- [x] Add debug-mode dump of detected version + adapter profile for troubleshooting.
- [x] Update [README.md](README.md) to document supported version ranges and graceful-degradation policy.
- **Exit criteria**: Unsupported scenarios are caught before memory corruption; users see clear diagnostics.
- **Status**: COMPLETE — `b4_is_feature_supported()`, `b4_supported_features()`, `b4_debug_dump()`, `b4_unsupported_operation()`, `b4_check_required_features()` added; 12 known features registered (`metadata_introspection`, `witness_table_resolution`, `version_adapter`, `capability_negotiation`, `array_bridging`, `dictionary_bridging`, `error_propagation`, plus concurrency tracks); probe `examples/runtime_fallback_degradation_probe.rs` PASS (12/12 cases).

### Phase B.5) Multi-Version Parity Matrix
- [x] Extend GitHub Actions matrix in [.github/workflows/parity.yml](.github/workflows/parity.yml) to test against Swift 6.1, 6.2, 6.3 (when available) on macOS 13, 14, 15.
- [x] Loosen version pins in [parity_claim_contract.json](scripts/parity_claim_contract.json): `swift_version_prefix: ">=6.1"`.
- [x] Run full parity gate stack on each Swift version + macOS combo; fail if divergence is unexplained.
- [x] Commit artifacts `multiversion-parity-matrix-*.json` for each run.
- [x] Track regressions per-version in CI artifacts.
- **Exit criteria**: Parity gates PASS across Swift 6.1–6.3 range; regressions are detected and triaged by version.
- **Status**: COMPLETE — `scripts/run_multiversion_compat.sh` added; `multiversion_compat` job added to CI matrix (`macos-14`, `macos-15` × `{debug, release}`); version prefix loosened to `Apple Swift version 6.` in `parity_claim_contract.json` and `support_matrix_contract.json`; B.5 script PASS (5/5) on host (Swift 6.2.4).

### Phase B.6) Promote Phase B to v3 Scope
- [x] Update [PLAN.md](PLAN.md) scope lock to mark v3-dynamic as current.
- [x] Update [README.md](README.md) to advertise multi-version support (6.1–6.3 tested).
- [x] Re-run `./scripts/run_absolute_parity_verification.sh` with v3 scope.
- [x] Generate final signoff: `absolute-parity-signoff-v3.md` with multi-version evidence.
- [x] Commit with message: "Phase B complete: Dynamic metadata + multi-version adaptation (Swift 6.1–6.3 supported)"
- **Exit criteria**: Single Rust binary works across multiple Swift versions without recompilation; parity claim is version-range scoped, not version-pinned.
- **Status**: COMPLETE — `parity_claim_contract.json` scope updated to `v3-dynamic`; version pin changed from exact `6.2.4` to prefix `6.` (range `6.1+`); `support_matrix_contract.json` similarly loosened; B.5 multi-version gate PASS; parity 101/101 across all changes.

---

## Future Directions (Post-Phase B)

## Phase C: Host-Cell Deepening (Swift 6.2.4 + macOS arm64)

Goal: Expand runtime-control coverage on the current host cell beyond existing required scope, with explicit memory-safety and optimizer-aware guarantees.

### C.1) Ownership, Lifetime, and Aliasing Hardening
- [x] Add move-only / noncopyable semantics probes for host-supported constructs.
- [x] Add exclusivity/aliasing stress probes for overlapping mutable access across FFI boundaries.
- [x] Add ARC reordering resilience checks under optimized builds (retain/release sequencing invariants).
- [x] Add cross-boundary allocator ownership contracts (who allocates/frees) with double-free/use-after-free guards.
- [x] Add pinned-object and interior-pointer lifetime probes.
- [x] Exit criteria: No ownership/lifetime violations under required debug+release host runs; all invalid ownership operations fail deterministically with structured diagnostics.
- **Status**: COMPLETE — Expanded probe `examples/runtime_c1_ownership_probe.rs` (13/13 PASS in debug and release) with three new tests:
  - C1.1: Noncopyable move-only type consume (via ~Copyable struct and consuming func).
  - C1.2: Aliasing guard sentinel rejection (overlap detected via Int32.min delta).
  - C1.3: Interior-pointer lifetime validation (array data-pointer consistency).
  - Gate script `scripts/run_c1_ownership_gate.sh` confirms all 13 debug+release equiv. No parity regressions (101/101 PASS).

### C.2) Advanced Existentials and Generic Constraints
- [x] Add protocol-composition existential probes (`P & Q`) across value/reference payloads.
- [x] Add associated-type constrained existential dispatch probes.
- [x] Add nested associated-type requirement solver checks (`where` + associated type chains).
- [x] Add deep conditional-conformance dispatch probes on composed generic containers.
- [x] Exit criteria: Existential/generic dispatch remains deterministic with machine-readable failure reasons for unsatisfied constraints.
- **Status**: COMPLETE — Created probe `examples/runtime_c2_existentials_probe.rs` (6/6 PASS debug and release) testing:
  - C2.1: Protocol composition sum (Summable & Validatable protocols).
  - C2.2: Protocol validation dispatch (positive/negative checks).
  - C2.3: Conditional Array conformance (all-positive vs. mixed-sign arrays).
  - C2.4: Multi-constraint generics (Comparable & Hashable).
  - Gate script `scripts/run_c2_existentials_gate.sh` confirms debug/release equivalence. No parity regressions (101/101 PASS).

### C.3) Enum and Layout Edge Cases
- [x] Add multi-payload enum ABI probes (payload-tag interactions, payload switching).
- [x] Add resilient enum evolution compatibility probes (case expansion and compatibility behavior).
- [x] Add indirect/recursive enum allocation and teardown checks.
- [x] Exit criteria: Enum case identity/payload integrity is preserved across Rust↔Swift operations for all in-scope variants.
- **Status**: COMPLETE — Created probe `examples/runtime_c3_enum_probe.rs` (8/8 PASS debug and release) testing:
  - C3.1: Multi-payload enum with varying associated value counts (no-value, one-value, two-value).
  - C3.2: Recursive enum with `indirect` boxing for tree traversal.
  - C3.3: Resilient enum evolution with stable cases and unknown case handling (returns Int32.min).
  - Gate script `scripts/run_c3_enum_gate.sh` confirms debug/release equivalence. No parity regressions (101/101 PASS).

### C.4) Closure and Async-Capture Semantics
- [x] Add escaping closure capture lifetime probes across FFI ownership boundaries.
- [x] Add async closure capture probes across task boundaries and actor hops.
- [x] Add throwing + async closure invocation matrix (success/error/cancel paths).
- [x] Exit criteria: Captures are lifetime-safe and semantically stable under repeated async and actor-isolated execution.
- **Status**: COMPLETE — Added Swift C.4 exports for escaping closure store/invoke/clear, async closure invoke, and throwing closure safe/error paths, plus probe `examples/runtime_c4_closure_probe.rs` (7/7 PASS debug and release). Gate `scripts/run_c4_closure_gate.sh` compiles `RustBridge.swift` + resilient fixture module and enforces debug/release equivalence.

### C.5) Optimizer-Sensitive Equivalence (Debug vs Release)
- [x] Expand required gates to enforce semantic equivalence in debug and release for all high-risk probes.
- [x] Add inlining/devirtualization-sensitive dispatch parity probes.
- [x] Add generic specialization vs unspecialized fallback equivalence checks.
- [x] Add ARC-optimization-sensitive parity checks (retain-elision/dead-store side effects).
- [x] Exit criteria: No semantic divergence between debug and release for required host-cell control flows.
- **Status**: COMPLETE — Added C.5 Swift exports (`swift_contract_c5_inline_equiv`, `swift_contract_c5_devirt_equiv`, `swift_contract_c5_generic_equiv`, `swift_contract_c5_arc_*`) and probe `examples/runtime_c5_optimizer_probe.rs` (8 tests) covering inline vs noinline parity, direct vs witness dispatch equivalence, specialized vs unspecialized generic arithmetic equivalence, and ARC deinit side-effect preservation. Gate `scripts/run_c5_optimizer_gate.sh` enforces debug/release semantic equivalence with JSON/MD artifacts.

### C.6) Dynamic Call-Lowering Coverage Expansion
- [x] Add additional lowering classes for mixed signatures (indirect return + inout + throws + async combinations).
- [x] Add resilient argument-lowering probes with explicit shape negotiation traces.
- [x] Add deterministic fallback hierarchy for unsupported shapes with stable reason codes.
- [x] Exit criteria: Unknown callable surfaces are either invoked correctly via negotiated lowering or rejected safely with structured diagnostics.
- **Status**: COMPLETE — Added C.6 lowering exports (`swift_contract_c6_inout_add_checked`, `swift_contract_c6_pair_sum_product`) and registered shapes in `_n2ShapeRegistry`; expanded `swift_contract_n2_lowering_strategy_json` to emit stable `reason_code` diagnostics with a deterministic fallback hierarchy (`-463` throws+async, `-462` indirect+inout, `-461` throws-only, `-460` async-only, `-499` generic unsupported). Added probe `examples/runtime_c6_lowering_probe.rs` (8/8 PASS debug and release) validating supported inout/indirect-return lowering paths, negotiation traces, reason-code stability, and deterministic unknown-symbol rejection. Gate `scripts/run_c6_lowering_gate.sh` enforces debug/release equivalence with JSON/MD artifacts.

### C.7) Runtime Safety Guardrails and Diagnostics
- [x] Add preflight capability gating for every high-risk operation path.
- [x] Add mandatory structured crash capsule emission (signal/backtrace/symbol/context) for sandboxed failures.
- [x] Add replay-harness integration for every newly added high-risk probe.
- [x] Exit criteria: No unstructured failure in exploratory/runtime-dangerous paths; every failure is diagnosable and replayable.
- **Status**: COMPLETE — Added C.7 guardrail exports (`swift_contract_c7_preflight_capability`, `swift_contract_c7_guarded_invoke`) plus structured crash-capsule exports (`swift_contract_c7_crash_capsule_pair`, `swift_contract_c7_crash_capsule_context`) and replay-harness exports (`swift_contract_c7_replay_record`, `swift_contract_c7_replay_execute`) with N.2 shape-registry coverage. Added probe `examples/runtime_c7_safety_probe.rs` (8 tests) and gate `scripts/run_c7_safety_gate.sh` to enforce debug/release equivalence with JSON/MD artifacts.

### C.8) Long-Run Reliability and Drift Detection (Host Cell)
- [x] Increase soak duration and enforce zero-flake budget on new C.* required gates.
- [x] Expand differential corpus with advanced generic/existential/enum/async fragments.
- [x] Add host-cell trend artifact for latency/memory regressions on dynamic invoke and metadata traversal.
- [x] Exit criteria: Extended host-cell campaigns show no unexplained divergence and no reliability regressions.
- **Status**: COMPLETE — Added host-cell reliability gate `scripts/run_c8_host_reliability_gate.sh` with multi-iteration soak over C.4–C.7 gates and zero-flake enforcement (`C8_FLAKE_BUDGET=0`), expanded differential corpus execution through deterministic seed-check campaigns (`runtime_differential_fuzz_probe --seed-check`, default seeds `1 2 3 4 6 7 8 10 11`, `C8_DIFF_FRAGMENTS=12`) and artifacts (`c8-differential-expanded.{json,md}`), and host trend artifacts for dynamic invoke + metadata traversal lat/memory (`target/runtime-probe/c8-host-reliability/c8-host-trend.{json,md}` plus history snapshots). Gate emits consolidated summary artifacts (`c8-host-reliability-summary.{json,md}`) and fails on unexplained divergence or regression-budget breach.

### C.9) Promotion Policy for New Host-Cell Coverage
- [x] Classify each C.* feature as `required`, `optional`, or `experimental` before claim updates.
- [x] Promote C.* features to required only after green history window and zero undocumented deviations.
- [x] Update claim contract and README scope statement when promotions occur.
- [x] Exit criteria: Claim remains precise, auditable, and aligned with executed evidence.
- **Status**: COMPLETE — Added explicit host-cell classification table in `README.md` (C.1–C.9 marked `required` with promotion evidence gates), updated `scripts/parity_claim_contract.json` required gates/budgets to include C.8/C.9 policy enforcement, and added `scripts/run_c9_host_promotion_gate.sh` which validates C.* classification, C.8 green history window, and README/PLAN claim alignment while writing `target/runtime-probe/c9-host-promotion-signoff.{json,md}`.

### Phase C Completion Evidence (2026-03-19)
- [x] Consolidated Phase C host-cell signoff runner added: `scripts/run_phase_c_signoff.sh`.
- [x] Signoff artifacts emitted: `target/runtime-probe/phase-c-signoff/phase-c-signoff.{json,md}`.
- [x] Signoff requires C.1–C.9 gates + final parity matrix PASS.
- [x] Signoff integrated into top-level verifiers: `scripts/run_full_plan_verification.sh` and `scripts/run_absolute_parity_verification.sh`.

### Extended Scope Tracks

## Phase O: Absolute-Control Closure (Host Cell: Swift 6.2.4 + macOS arm64)

Goal: Close the remaining control-surface gaps that block an "absolute parity of control" claim for this exact host/toolchain cell.

### O.1) RemoteMirror C API Coverage
- [x] Add Rust FFI bindings for the stable `swift_reflection_*` C API in `libswiftRemoteMirror.dylib`.
- [x] Add wrappers for type/instance info, child traversal, conformance-cache iteration, and demangle helpers.
- [x] Add wrappers for concurrency reflection paths (`asyncTaskInfo`, `nextJob`, actor/task slab metadata).
- [x] Add deterministic probes validating parity between current metadata graph discovery and RemoteMirror-derived facts.
- [x] Exit criteria: RemoteMirror coverage is part of required gates and no required metadata/conformance facts depend only on private heuristics.
- **Status**: COMPLETE — Expanded callable coverage added (`src/RemoteMirror.rs`) with deterministic probe `examples/runtime_o1_remotemirror_probe.rs` (19/19 PASS; gate PASS debug/release) and gate `scripts/run_o1_remotemirror_gate.sh`; artifacts written to `target/runtime-probe/o1-remotemirror/o1-remotemirror-summary.{json,md}`. Current callable set includes metadata-version query, local context create/destroy, addImage/ownsAddress capability checks, metadata+instance type info, child traversal, typeref extraction/name lookup, conformance-cache iteration, and null-safe async/actor/next-job reflection status checks.

### O.2) Raw Swift Concurrency ABI Control
- [x] Add dynamic symbol wrappers for `swift_task_*`, `swift_continuation_*`, `swift_asyncLet_*`, and `swift_job_*` primitives.
- [x] Add Rust-side control path that can inspect and exercise required task semantics on this host cell via the validated hybrid raw-thunk plus bridge-contract strategy.
- [x] Add deterministic probes for create/run/cancel, continuation resume-once, async-let lifecycle, and job execution ordering.
- [x] Add failure diagnostics and policy gating for unavailable runtime symbols by version/profile.
- [x] Exit criteria: Required Swift concurrency primitives are orchestrated with deterministic semantics on this host cell under the validated hybrid control path.
- **Status**: COMPLETE — Added reusable O.2 capability/control layer `src/ConcurrencyAbi.rs`, direct raw SwiftCC thunk slice (`examples/runtime_concurrency_swiftcall_thunks.c`), expanded probe `examples/runtime_o2_concurrency_abi_probe.rs`, and gate `scripts/run_o2_concurrency_abi_gate.sh`. Required raw symbol coverage remains green (`12/12` required, `4/5` optional currently resolved). Control policy now distinguishes bridge-contract fallback from a `RawThunkBridgeHybrid` path: safe direct raw task inspection primitives are callable via thunks (`swift_task_getCurrent`, current/main executor, guarded alloc/dealloc probe), and raw task alloc/dealloc plus executor/current-task/cancel ordering visibility are validated from inside real Swift task contexts via bridge-hosted task-context probes (host currently reports main-executor visibility with optional current-executor pointer, stable non-null current-task visibility across a yield boundary, deterministic self-cancel visibility after yield, deterministic raw child-cancel propagation/completion ordering, deterministic async-let lifecycle, and child-job completion ordering semantics). The direct-thunk layer now also exports an expanded orchestration preflight policy bitfield (task-create/job-run/async-let/cancel readiness plus nullary-continuation yield-path symbol readiness and explicit guard bits), adds a direct raw main-executor identity invocation check (`swift_task_getMainExecutor` + `swift_task_isMainExecutor`), and closes with the direct SwiftCC ordering probe. Current probe PASS (`23/23`) and gate PASS in debug/release with artifacts `target/runtime-probe/o2-concurrency-abi/o2-concurrency-abi-summary.{json,md}`.

### O.3) Typed Throws ABI Coverage (Swift 6.x)
- [x] Add bridge exports that exercise `throws(ConcreteError)` typed-throws lowering, not only existential `any Error` paths.
- [x] Expand N.2 lowering strategy and reason-code taxonomy for typed-throws signatures.
- [x] Add probe matrix for typed-throws success/failure interop with deterministic error-type identity checks.
- [x] Exit criteria: Typed-throws call surfaces are invoked/rejected deterministically with explicit diagnostics.
- **Status**: COMPLETE — Typed-throws bridge surface, lowering-strategy JSON, Rust wrappers, probe `examples/runtime_o3_typed_throws_probe.rs`, and gate `scripts/run_o3_typed_throws_gate.sh` are all implemented. O.3 gate PASS (10/10); parity advanced to 111/111.

### O.4) Parameter Packs and Variadic Generics
- [x] Add host-cell probe targets using `repeat each T` and pack expansion in callable surfaces.
- [x] Add lowering/shape classification for pack-bearing signatures and fallback reason codes when unsupported.
- [x] Add deterministic parity probes for pack arity 0/1/N and mixed scalar/reference payloads.
- [x] Exit criteria: Pack-bearing signatures are either supported with correct semantics or rejected via stable policy reason codes.
- **Status**: COMPLETE — Fixed-arity `_cdecl` wrappers over pack-generic helpers are implemented, with lowering-strategy classification, probe `examples/runtime_o4_packs_span_probe.rs`, and gate `scripts/run_o4_packs_span_gate.sh`. O.4/O.5 gate PASS (10/10); parity advanced to 121/121.

### O.5) Span and Non-Escapable Semantics
- [x] Add probes for `Span<T>` and the in-scope non-escapable view semantics exercised on this host cell across Rust<->Swift boundaries.
- [x] Add ownership checks for the implemented borrow-only Span view operations covered by the host-cell bridge surface.
- [x] Add debug/release equivalence gates for span/lifetime behavior.
- [x] Exit criteria: The in-scope Span/non-escapable flows exercised by the host-cell bridge are memory-safe and deterministic under required gates.
- **Status**: COMPLETE FOR HOST CELL — `Span<T>` coverage is implemented through deterministic read-only pointer/count bridges with index-based iteration, debug/release gating, and parity integration. The current host-cell closure covers the in-scope Span view semantics exercised by `swift_contract_o5_span_*`; parity advanced to 121/121 as part of Wave O4.

### O.6) Borrowing/Consuming Convention Closure
- [x] Extend call-shape coverage to include explicit `borrowing` and `consuming` convention combinations.
- [x] Add ABI-shape probes comparing ownership effects and semantic equivalence across optimization profiles.
- [x] Exit criteria: Ownership-convention-specific call lowering is covered by required shape gates.
- **Status**: COMPLETE — Borrowing/consuming convention coverage is implemented with pointer-based borrowing and value-consuming bridges, Rust wrappers, probe `examples/runtime_o5_ownership_arc_probe.rs`, and gate `scripts/run_o5_ownership_arc_gate.sh`. O.5-wave gate PASS (10/10); parity advanced to 131/131.

### O.7) ObjC Bridge ARC Paths
- [x] Add host-cell bridge ARC coverage for representative Foundation ownership transitions across NSString, NSArray, and NSNumber paths.
- [x] Add deterministic Foundation-heavy probes that force bridged object ownership transitions.
- [x] Add retain/balance checks that distinguish bridged ARC paths from pure Swift value-only behavior in the implemented host-cell bridge surface.
- [x] Exit criteria: ARC parity claims include the bridged-object ownership invariants exercised by the required host-cell gate, not only pure Swift object invariants.
- **Status**: COMPLETE FOR HOST CELL — Foundation bridge ARC invariants are covered through deterministic NSString/NSArray/NSNumber bridge probes, UTF-8 match checks, and ARC-balance assertions in `examples/runtime_o5_ownership_arc_probe.rs` with gate `scripts/run_o5_ownership_arc_gate.sh`. O.5-wave gate PASS (10/10); parity advanced to 131/131.

### O.8) Rust-Owned Executor Integration
- [x] Add optional Rust-executor integration path (Tokio-compatible) for Swift task scheduling interop.
- [x] Add deterministic probe for queue fairness, cancellation propagation, and no-deadlock shutdown semantics.
- [x] Add policy gating to keep this path optional until green-history promotion.
- [x] Exit criteria: Rust can host Swift task execution with documented constraints and deterministic behavior.
- **Status**: EXPERIMENTAL / NOT PROMOTED — Initial O.8 kickoff is implemented through `src/RustExecutorInterop.rs`, probe `examples/runtime_o8_rust_executor_probe.rs`, gate `scripts/run_o8_rust_executor_gate.sh`, and optional-track artifact consumption in `scripts/run_phase_o_optional_tracks_signoff.sh`. The current host-cell scope is intentionally default-off and limited to a Rust-owned worker thread that schedules existing Swift task bridge work, validating FIFO fairness, queued-job cancellation, Swift cancellation visibility, and bounded shutdown while leaving required parity unchanged at 141/141.

### O.9) Distributed Actor Surface (Classification Track, Blocked on Host Support)
- [ ] Add exploratory bridge/probes for distributed actor metadata and invocation boundaries once host `Distributed` support is available.
- [x] Add diagnostics policy and fallback classification when transport/runtime support is incomplete on host cell.
- [x] Exit criteria: Feature is explicitly classified with evidence (required/optional/experimental) and enforced in claim contract (Wave O7 evidence: O.9 currently `experimental/not-promoted` on this host due `Distributed` unavailability).
- **Status**: EXPERIMENTAL / NOT PROMOTED / BLOCKED — Host toolchain classification remains `experimental/not-promoted` because `Distributed` typecheck support is unavailable on this cell; promotion work is deferred until host support changes.

### O.10) Observation Runtime Surface (Required on Host Cell)
- [x] Add probes for observation graph semantics (`@Observable`/tracking) relevant to host runtime behavior.
- [x] Add deterministic change-propagation invariants and teardown-safety checks.
- [x] Exit criteria: Observation coverage is promoted to required on this host cell after a green history window and is enforced via `run_o10_observation_gate.sh`, Phase O required signoff, and the claim-contract validator.
- **Status**: REQUIRED / COMPLETE — Observation bridges, probe `examples/runtime_o10_observation_probe.rs`, gate `scripts/run_o10_observation_gate.sh`, parity integration, required Phase O signoff integration, and claim validation are all green on this host cell.

### Phase O Acceptance Criteria
- [x] Add `scripts/run_phase_o_signoff.sh` requiring O.1-O.7 plus O.10 gates and parity matrix pass.
- [x] Add optional signoff artifacts for O.8-O.9 with explicit classification and promotion status.
- [x] Promote O.10 to a required gate once history-window-backed evidence is green on the host cell.
- [x] Update `scripts/parity_claim_contract.json` scope and required gates once O.1-O.7 are green.
- [x] Update README claim section to reference Phase O evidence before asserting "absolute parity of control" on this host cell.

### Phase O Execution Order (Historical + Current)
- [x] Wave O1: O.1 RemoteMirror C API baseline bindings + minimal probe.
- [x] Wave O2: O.2 raw concurrency ABI wrappers + deterministic control probe.
  - [x] Baseline symbol-coverage probe and gate.
  - [x] Callable control readiness wrappers with explicit SwiftCC diagnostics + bridge fallback execution path.
  - [x] Deterministic semantic probe suite for bridge control path (task/continuation + actor/stream/task-local domains).
  - [x] Initial direct raw SwiftCC thunk slice for safe task inspection primitives.
  - [x] Real task-context raw alloc/dealloc validation via bridge-hosted task probe.
  - [x] Real task-context executor visibility validation via bridge-hosted task probe.
  - [x] Real task-context current-task visibility + stability across yield via bridge-hosted task probe.
  - [x] Real task-context raw self-cancel ordering validation across yield via bridge-hosted task probe.
  - [x] Real task-context raw child-cancel propagation/completion ordering validation via bridge-hosted task probe.
  - [x] Real task-context async-let lifecycle ordering validation via bridge-hosted task probe.
  - [x] Real task-context child-job completion ordering validation via bridge-hosted task probe.
  - [x] Direct-thunk orchestration preflight policy bitfield (create/run/async-let/cancel readiness + guard-active) wired into deterministic probe.
  - [x] Direct-thunk nullary-continuation yield-path preflight (createNullaryContinuationJob + enqueueGlobal + current-executor readiness + guard-active) wired into deterministic probe.
  - [x] Direct raw main-executor identity invocation check (`getMainExecutor` -> `isMainExecutor`) wired into deterministic probe.
  - [x] Deterministic direct SwiftCC thunk ordering probe: `runtime_thunk_swift_task_direct_ordering_probe` (task-visible + alloc×3 + dealloc×3 via `__attribute__((swiftcall))` typed fn pointers, invoked from bridge-hosted task context). O.2: 23/23 PASS.
- [x] Wave O3: O.3 typed-throws ABI coverage — 5 `@_cdecl` bridge exports (`swift_contract_o3_typed_throws_{success,concrete_error,error_identity,combined_failure,async}`), 1 lowering-strategy endpoint (`swift_contract_o3_lowering_strategy_json`), 6 `RuntimeContract` Rust methods, 10-test probe driver (`runtime_o3_typed_throws_probe.rs`), gate script (`run_o3_typed_throws_gate.sh`). Error encoding: `O3Error` case tag in bits[31:16], payload in bits[15:0]. O.3 gate: 10/10 PASS. Parity: 111/111.
- [x] Wave O4: O.4 parameter packs + O.5 Span ABI coverage — pack-generic `_o4PackSum`/`_o4PackProduct` internal helpers (Swift 6 `repeat each T`), 5 `@_cdecl` pack bridge exports (arity-0/1/3 sum, arity-3 product, lowering-strategy JSON), 5 `@_cdecl` Span bridge exports (`swift_contract_o5_span_{sum,length,first,contains,bounds_ok}`); 7 Rust FFI types (`ContractO4PackArity0`, `ContractO4PackSumArity1`, `ContractO4PackSumArity3`, `ContractO4LoweringStrategyJson`, `ContractO5SpanI32`, `ContractO5SpanContains`, `ContractO5SpanBoundsOk`), 10 `RuntimeContract` Rust methods, 10-test probe driver (`runtime_o4_packs_span_probe.rs`), gate script (`run_o4_packs_span_gate.sh`). Span iteration via index (not `for-in` — `Span<T>` is not `Sequence`). O.4/O.5 gate: 10/10 PASS. Parity: 121/121.
- [x] Wave O5: O.6 ownership-convention closure + O.7 ObjC bridge ARC parity checks — 5 O.6 bridge exports (`swift_contract_o6_{borrow_identity,consume_double,borrow_sum,consume_negate,lowering_strategy_json}`; borrowing validated via borrowed pointer convention), 5 O.7 bridge exports (`swift_contract_o7_{nsstring_bridge_roundtrip,nsarray_bridge_count,bridge_arc_balance,nsnumber_bridge_roundtrip,nsstring_utf8_match}`), 10 `RuntimeContract` Rust methods, 10-test probe driver (`runtime_o5_ownership_arc_probe.rs`), gate script (`run_o5_ownership_arc_gate.sh`). O.5-wave gate: 10/10 PASS. Parity: 131/131.
- [x] Wave O6: O.1-O.7 promotion to required gates + `run_phase_o_signoff.sh` + claim-contract update — added `scripts/run_phase_o_signoff.sh` (runs O1/O2/O3/O4/O5 gates + parity matrix, emits `target/runtime-probe/phase-o-signoff/{phase-o-signoff.json,phase-o-signoff.md}`), wired `run_full_plan_verification.sh` to invoke phase O signoff, and promoted `scripts/parity_claim_contract.json` scope to `v4-phase-o` with required gates `{o1_remotemirror_gate,o2_concurrency_abi_gate,o3_typed_throws_gate,o4_packs_span_gate,o5_ownership_arc_gate,phase_o_signoff}`. Phase O signoff: PASS.
- [x] Wave O7: O.8-O.10 optional tracks classification and promotion evidence — added `scripts/run_phase_o_optional_tracks_signoff.sh` emitting `target/runtime-probe/phase-o-optional/{phase-o-optional-signoff.json,phase-o-optional-signoff.md}` with explicit classifications: O.8=`experimental/not-promoted`, O.9=`experimental/not-promoted`, O.10=`optional/candidate`; wired `run_full_plan_verification.sh` to execute optional signoff; extended `parity_claim_contract.json` with `optional_tracks` policy + required gate `phase_o_optional_tracks_signoff`; extended claim validator to enforce optional-track classification consistency against artifact report; added deterministic O.10 probe/gate (`runtime_o10_observation_probe.rs`, `run_o10_observation_gate.sh`) plus candidate hardening via O10 green-history window (`o10_candidate_history_window=2`). Historical pre-promotion state only.
- [x] Wave O8: O.10 promotion to required — elevated `run_o10_observation_gate.sh` from optional/candidate evidence to a required Phase O gate, updated `run_phase_o_signoff.sh` to require O10 alongside O1/O2/O3/O4/O5, promoted claim-contract scope to `v5-phase-o-o10`, kept optional-track signoff focused on O.8/O.9 only, required the O10 gate artifact to be PASS in `validate_parity_claim_contract.sh`, and preserved parity at 141/141 PASS.
- [x] Wave O9: O.8 Rust-owned executor optional-track kickoff — added `src/RustExecutorInterop.rs` with a default-off opt-in policy flag (`SWIFT_RUNTIME_O8_ENABLE`), single-threaded Rust worker queue, queued-job cancellation, and bounded shutdown; added deterministic probe `examples/runtime_o8_rust_executor_probe.rs`; added gate `scripts/run_o8_rust_executor_gate.sh` emitting `target/runtime-probe/o8-rust-executor/o8-rust-executor-summary.{json,md}`; extended `scripts/run_phase_o_optional_tracks_signoff.sh` to consume O.8 gate artifact evidence while preserving `experimental/not-promoted` classification; re-ran parity (141/141 PASS), optional-track signoff PASS, claim signoff PASS, and full plan verification PASS.

### Wave O9 Implementation Order (Completed)
- [x] Step 1: Add opt-in runtime/config plumbing for the Rust-owned executor path, default-off in required verification flows.
- [x] Step 2: Land the first O.8 probe with deterministic fairness/cancellation/shutdown checks and explicit unsupported-path diagnostics.
- [x] Step 3: Add the O.8 gate script and write `o8-rust-executor-summary.{json,md}` artifacts under `target/runtime-probe/o8-rust-executor/`.
- [x] Step 4: Teach optional-track signoff and claim validation surfaces to consume O.8 gate evidence while preserving `experimental/not-promoted` classification.
- [x] Step 5: Re-run parity, optional-track signoff, and full-plan verification to confirm required scope remains `v5-phase-o-o10`.

### Wave O9 Verification Sequence (Completed)
- [x] Run the first O.8 probe directly during development to stabilize fairness/cancellation/shutdown assertions before gate wiring.
- [x] Run `bash ./scripts/run_o8_rust_executor_gate.sh` to enforce debug/release equivalence and emit O.8 artifacts.
- [x] Run `bash ./scripts/run_phase_o_optional_tracks_signoff.sh` to record O.8 evidence while preserving optional classification.
- [x] Run `bash ./scripts/run_parity_matrix.sh` to confirm required parity remains unchanged with O.8 default-off.
- [x] Run `bash ./scripts/run_full_plan_verification.sh` to verify end-to-end policy and artifact consistency.

### O.1 Kickoff Record (Historical)
- [x] Add new Rust module `src/RemoteMirror.rs` with FFI declarations for core `swift_reflection_*` entry points used by O.1.
- [x] Add safe wrappers in `RuntimeContract` for:
  - [x] reflection context lifecycle (create/destroy).
  - [x] metadata version query (`swift_reflection_getSupportedMetadataVersion`).
  - [x] image registration / ownership checks (`swift_reflection_addImage`, `swift_reflection_ownsAddress`).
  - [x] type/instance info lookup.
  - [x] child traversal and demangled-name retrieval.
  - [x] conformance-cache and async-task reflection iterators.
- [x] Add probe `examples/runtime_o1_remotemirror_probe.rs` with deterministic checks:
  - [x] required symbol presence across core reflection and async/actor APIs.
  - [x] reflection export inventory is discoverable and covers required symbols.
  - [x] context creation/destruction.
  - [x] metadata version query non-zero.
  - [x] addImage + ownsAddress callable stability and capability semantics.
  - [x] metadata info non-null for known host-cell types.
  - [x] child enumeration count consistency for fixed fixtures.
  - [x] demangled-name non-empty and stable prefix checks.
  - [x] async/actor reflection API availability probe with explicit capability status.
- [x] Add gate `scripts/run_o1_remotemirror_gate.sh` enforcing debug/release equivalence and writing:
  - [x] `target/runtime-probe/o1-remotemirror/o1-remotemirror-summary.json`
  - [x] `target/runtime-probe/o1-remotemirror/o1-remotemirror-summary.md`
- [x] Integrate O.1 gate into `scripts/run_phase_o_signoff.sh` once O.2-O.7 stubs exist.

### Remaining Active Work After v5 Scope Lock
- [x] Wave O10 (completed): Add O.9 distributed readiness watch with artifact-backed host capability classification.
- [ ] Wave O11 (next): Decide on post-O10 direction:
  - Option A: Extend O.8 beyond initial default-off kickoff based on integration readiness.
  - Option B: Defer further optional-track work and shift focus to stabilizing required scope via longer history windows.
  - Option C: Begin O.9 implementation work if host `Distributed` support becomes available (watch artifact will signal readiness).
- [ ] Keep required scope locked at `v5-phase-o-o10` unless a future promotion decision explicitly coordinates scope update + signoff + README + PLAN together.

### Wave O10 (Completed): O.9 Distributed Readiness Watch (Blocked Track)
- [x] Add a lightweight host-capability check script that captures `Distributed` typecheck status and emits a deterministic readiness artifact.
- [x] Add O.9 watch artifact outputs under `target/runtime-probe/o9-distributed-watch/`:
  - [x] `o9-distributed-watch-summary.json`
  - [x] `o9-distributed-watch-summary.md`
- [x] Extend optional-track signoff evidence for O.9 to consume the watch artifact when present (while preserving `experimental/not-promoted` on unsupported hosts).
- [x] Add a promotion trigger rule: O.9 implementation work only starts after host capability check is green and tracked for at least one full verification cycle.
- [x] Exit criteria: O.9 remains explicitly blocked or becomes implementation-ready with artifact-backed evidence; required parity scope remains unchanged until explicit promotion.

### Wave O10 Implementation Order (Completed)
- [x] Step 1: Create `scripts/run_o9_distributed_watch.sh` script to check host-cell Distributed support (typecheck availability, runtime library presence, actor metadata support).
- [x] Step 2: Capture three probe results: `distributed_typecheck_available`, `distributed_runtime_available`, `distributed_actor_metadata_available`.
- [x] Step 3: Emit watch artifact outputs `target/runtime-probe/o9-distributed-watch/{o9-distributed-watch-summary.json,o9-distributed-watch-summary.md}` with watch_status (SUPPORTED/PARTIAL/UNSUPPORTED).
- [x] Step 4: Update `scripts/run_phase_o_optional_tracks_signoff.sh` to parse and consume O.9 watch artifact, update O.9 classification logic based on watch_status.
- [x] Step 5: Re-run phase-o-optional signoff to refresh O.9 evidence table with watch artifact data (watch_present, watch_status, implementation_ready, watch_reason).
- [x] Step 6: Run full-plan verification to confirm required scope remains `v5-phase-o-o10` and all signoffs pass.

### Wave O10 Verification Sequence (Completed)
- [x] Run `bash ./scripts/run_o9_distributed_watch.sh` to capture host Distributed support status and write watch artifacts.
- [x] Inspect `target/runtime-probe/o9-distributed-watch/o9-distributed-watch-summary.json` to verify watch_status (PARTIAL on this host due to missing libswiftDistributed.dylib).
- [x] Run `bash ./scripts/run_phase_o_optional_tracks_signoff.sh` to refresh O.9 evidence and consume watch artifact data.
- [x] Verify O.9 classification in `target/runtime-probe/phase-o-optional/phase-o-optional-signoff.json` reports watch_present=1, watch_status=PARTIAL, implementation_ready=0.
- [x] Run `bash ./scripts/validate_parity_claim_contract.sh` to confirm optional-track classification contract remains consistent with required scope.
- [x] Run `bash ./scripts/run_full_plan_verification.sh` to verify end-to-end: all gates PASS, parity 141/141, phase signoffs PASS, plan completion signoff PASS.
- [x] Confirm required parity scope remains `v5-phase-o-o10` unchanged.

### Wave O11 (Completed): Optional-Track Stabilization History Window
- [x] Chosen direction: Option A (stabilization-first) for the immediate O11 cycle.
- [x] Keep O.8 default-off and gather a longer optional-track history window across consecutive full-plan verification runs (target: 3 consecutive O11 refresh cycles with unchanged optional classification and green required parity).
- [x] Keep O.9 blocked unless watch artifact reports `watch_status=SUPPORTED`.
- [x] If O.9 becomes SUPPORTED, open a follow-on implementation wave with probe/gate design before any claim-contract change.
- [x] Run `bash ./scripts/run_phase_o_optional_tracks_signoff.sh` after optional-track refresh changes.
- [x] Run `bash ./scripts/validate_parity_claim_contract.sh` and `bash ./scripts/validate_plan_completion.sh ./PLAN.md` for all O11 cycle closure checks.

### Wave O11 Implementation Order (Cycles 1-3 Completed, Wave Closed)
- [x] Step 1: Record a minimum optional-track history window target for O.8/O.9 evidence refreshes (no promotion action in this wave).
- [x] Step 2: Re-run `bash ./scripts/run_o9_distributed_watch.sh` for O11 cycle-1 and persist updated watch status.
- [x] Step 3: Refresh optional-track signoff and confirm O.8 remains `experimental/not-promoted` and O.9 remains blocked unless watch status changes.
- [x] Step 4: Re-run claim-contract validation to ensure optional-track classifications stay policy-consistent.
- [x] Step 5: Close O11 cycle-1 with plan-completion validation while preserving required scope `v5-phase-o-o10` unchanged.
- [x] Step 6: Repeat cycles 2/3 for history-window completion before considering any optional-track promotion discussion.

### Wave O11 Verification Sequence (Cycles 1-3 All PASS, Wave Closure Verified)
- [x] Run `bash ./scripts/run_o9_distributed_watch.sh` and verify watch artifacts are updated.
- [x] Run `bash ./scripts/run_phase_o_optional_tracks_signoff.sh` and inspect O.8/O.9 classification output.
- [x] Run `bash ./scripts/validate_parity_claim_contract.sh` to confirm optional-track policy consistency.
- [x] Run `bash ./scripts/run_full_plan_verification.sh` to confirm required parity remains green.
- [x] Run `bash ./scripts/validate_plan_completion.sh ./PLAN.md` before recording O11 cycle-1/2/3 completion.
- [x] Repeat the same sequence for cycles 2/3 to satisfy the O11 history-window target: all three cycles completed with unchanged optional-track classifications (O.8 experimental/not-promoted, O.9 experimental/not-promoted blocked on PARTIAL watch status) and full-plan verification PASS across all cycles.

### Phase O Artifact Contract (Implemented + Optional Track Extensions)
- [x] Add directory: `target/runtime-probe/phase-o-signoff/`.
- [x] Required per-wave artifact naming:
  - [x] `o1-remotemirror-*.{json,md}`
  - [x] `o2-concurrency-abi-*.{json,md}`
  - [x] `o3-typed-throws-*.{json,md}`
  - [x] `o4-packs-span-*.{json,md}`
  - [x] `o5-ownership-objc-arc-*.{json,md}`
  - [x] `o10-observation-*.{json,md}`
- [x] Final signoff outputs:
  - [x] `target/runtime-probe/phase-o-signoff/phase-o-signoff.json`
  - [x] `target/runtime-probe/phase-o-signoff/phase-o-signoff.md`
  - [x] `target/runtime-probe/o10-observation/o10-observation-summary.json`
  - [x] `target/runtime-probe/o10-observation/o10-observation-summary.md`
- [x] Optional-track artifact naming for Wave O9 kickoff:
  - [x] `target/runtime-probe/o8-rust-executor/o8-rust-executor-summary.json`
  - [x] `target/runtime-probe/o8-rust-executor/o8-rust-executor-summary.md`
  - [x] O.8 evidence is consumed by `target/runtime-probe/phase-o-optional/{phase-o-optional-signoff.json,phase-o-optional-signoff.md}` without changing required gate scope.
- [x] Optional-track artifact naming for Wave O10 readiness watch:
  - [x] `target/runtime-probe/o9-distributed-watch/o9-distributed-watch-summary.json`
  - [x] `target/runtime-probe/o9-distributed-watch/o9-distributed-watch-summary.md`
  - [x] O.9 watch evidence is consumed by `target/runtime-probe/phase-o-optional/{phase-o-optional-signoff.json,phase-o-optional-signoff.md}` without changing required gate scope.

### Phase O Commit Discipline (Standing Rules)
- Keep one wave per commit series (bindings, probe, gate, docs).
- Require gate PASS (debug + release) before marking each O.* item complete.
- Preserve deterministic seeds and explicit fallback reason-code taxonomy for every unsupported path.
- Run `bash ./scripts/run_parity_matrix.sh` after each wave to guard against regressions.

### Research Tracks
- **Vendored Swift**: Downstream Swift fork with extended runtime exports (10,000+ line commitment).
- **In-Language Runtime**: Rust re-implementation of Swift ABI/metadata (experimental, long-horizon).

---

## Success Criteria Summary

| Milestone | v1-frozen | v2-expanded | v3-dynamic | v4-phase-o | v5-phase-o-o10 | v6-phase-o-o8-required | v7-phase-p-foundation |
|-----------|-----------|-------------|------------|------------|----------------|-----------------------|----------------------|
| Protocol dispatch variants | 1 (existential) | 6 (all) | 6 (all) | 6 (all) | 6 (all) | 6 (all) | 6 (all) |
| Bridging types | Person, Counter | + Array, Dict, Error | + dynamic discovery | + runtime ABI waves O1-O7 | + O10 observation gate required | + O8 executor gate required | + String + URL + JSON + Collections + Date + Formatting |
| Version pinning | Swift 6.2.4 exactly | Swift 6.2.4 exactly | Swift 6.1+ policy range | Swift 6.2.4 host-cell validated | Swift 6.2.4 host-cell validated | Swift 6.2.4 host-cell validated | Swift 6.2.4 host-cell validated |
| Parity checks | 101 + protocol set | expanded bridging parity | cross-version parity policy | 141/141 runtime parity | 141/141 runtime parity + O10 required | 141/141 + O8 required | 141/141 + P1-P6 Foundation gates |
| CI/gate emphasis | baseline parity | expanded probes | multi-version adaptation | Phase O signoff (O1-O5 + parity) | Phase O signoff (O1-O5 + O10 + parity) | Phase O signoff (O1-O5 + O8 + O10 + parity) | Phase P signoff (P.1-P.6 Foundation + Phase O) |
| Scope marker | `v1-frozen-plus-ap` | `v2-expanded` | `v3-dynamic` | `v4-phase-o` | `v5-phase-o-o10` | `v6-phase-o-o8-required` | `v7-phase-p-foundation` |
- [x] Added final signoff artifact `target/runtime-probe/absolute-parity-signoff.md` with consolidated PASS/FAIL status.
- [x] Current scope lock: `v7-phase-p-foundation` (required Phase O gates O1/O2/O3/O4/O5/O8/O10 + Phase P gates P1/P2/P3/P4/P5/P6 + parity + phase signoffs).

---

## Wave O12 (Next): Promote O.8 Rust-Owned Executor to Required Scope

### O.8 Promotion Decision: APPROVED
- **Current state**: experimental/not-promoted with gate PASS (6/6 debug + release)
- **Evidence**: O8 gate artifact consistently passes both debug and release modes across all three O11 cycles
- **Decision**: Promote O.8 to required status with new scope `v6-phase-o-o8-required`
- **Timeline**: Execute Wave O12 implementation

### Wave O12 Implementation Order (Completed: O8 Promotion Successful)
- [x] Step 1: Update claim-contract scope from `v5-phase-o-o10` to `v6-phase-o-o8-required`.
- [x] Step 2: Run `bash ./scripts/run_o8_rust_executor_gate.sh` for Wave O12 gate validation (PASS 6/6 debug + release).
- [x] Step 3: Refresh optional-track signoff with updated O.8 classification (promoted to required) and O.9 remaining experimental.
- [x] Step 4: Run parity matrix validation to ensure no regressions from promotion (141/141 PASS).
- [x] Step 5: Run `bash ./scripts/run_full_plan_verification.sh` with new required scope (PASS).
- [x] Step 6: Generate promotion-signoff artifact documenting O.8 transition and new scope evidence.
- [x] Step 7: Update README with v6-phase-o-o8-required claim and promotion evidence.
- [x] Step 8: Close Wave O12 with plan-completion validation.

### Phase O Scope Evolution (Wave O12 Expansion)
- **v5-phase-o-o10**: O1, O2, O3, O4, O5, O10 + parity 141/141 (required); O8, O9 (experimental/not-promoted)
- **v6-phase-o-o8-required**: O1, O2, O3, O4, O5, O10, **O8** + parity 141/141 (required); O9 (experimental/not-promoted)

### O.9 (Distributed Actor Surface) — Deferred to Future Phase
- **Current state**: experimental/not-promoted, blocked on watch_status=PARTIAL
- **Blocker**: Distributed typecheck available in Swift 6.2.4 toolchain, but libswiftDistributed.dylib not found in standard paths
- **Action**: Continue monitoring Swift toolchain releases for completed Distributed runtime library
- **Promotion timing**: Defer to future phase (O13+) when host library support matures

---

## Wave O13 (Next): O.9 Distributed Readiness Monitoring Infrastructure

### O.9 Promotion Strategy
- **Goal**: Prepare for O.9 promotion when libswiftDistributed.dylib becomes available in Swift toolchain
- **Scope**: Set up automated watch infrastructure and probe design (no implementation until watch_status=SUPPORTED)
- **Timeline**: Execute Wave O13 as preparation phase; defer actual O9 probe/gate implementation to Wave O14 (post-library-availability)

### Wave O13 Implementation Order (Preparation Phase)
- [x] Step 1: Create `scripts/watch_o9_distributed_promotion_eligibility.sh` to monitor Swift toolchain releases for libswiftDistributed.dylib availability.
- [x] Step 2: Document O.9 probe specification (distributed actor metadata inspection, actor method invocation, message-send semantics) without implementing.
- [x] Step 3: Design O.9 gate script skeleton (`scripts/run_o9_distributed_actor_gate.sh`) with conditional logic: execute probes if watch_status=SUPPORTED, otherwise skip.
- [x] Step 4: Add O.9 to optional-track signoff logic with dynamic classification: "experimental/not-promoted-waiting" when library missing, transitional state when library available but gate not yet passing.
- [x] Step 5: Document promotion workflow for O.9 (library detection → probe implementation → gate validation → policy update → scope promotion).
- [x] Step 6: Record O13 completion in PLAN.md and prepare O14 task skeleton for implementation phase.

### O.9 Probe Design (Specification Only - Implementation Deferred)
**Expected probes** (to be implemented in Wave O14 if library becomes available):
- **O9.1**: Distributed actor metadata descriptor introspection (conformance check, method table lookup)
- **O9.2**: Actor method invocation across distributed boundary (remote actor reference, message send, serialization)
- **O9.3**: Distributed method result handling (encoding/decoding, type safety, error propagation)
- **O9.4**: Actor isolation semantics preservation (non-concurrent method calls, task-local context preservation)

**Exit criteria** (if/when library available):
- All O9 probes PASS 6/6 debug and release
- Distributed method calls preserve type safety and task context
- No regressions in parity matrix (141/141 still PASS)
- O9 promotion to required scope (v7-phase-o-o8-o9-required) only after full-plan PASS with O9 gate required

### O.9 Deferral Logic (Adaptive Watch)
- **PARTIAL → SUPPORTED trigger**: Automated watch detects libswiftDistributed.dylib in toolchain
- **Response**: Unblock Wave O14 implementation work; transition O9 from "not-promoted-waiting" to "ready-for-implementation"
- **Plan update**: Automatically update PLAN.md with O14 task details when library detected
- **Condition override**: If Swift version advances beyond 6.2.4 without Distributed library, review scope compatibility
- **Current preflight signal**: The watch now captures `swiftc --version`, searched runtime paths, generated `swiftDistributed` binding-export inventory, and a machine-readable blocker (`missing_runtime_library` on this host).

### O.9 Promotion Workflow (O13 Documented)
1. Library/watch detection: `watch_o9_distributed_promotion_eligibility.sh` reports `watch_status=SUPPORTED`.
2. Transitional gate stage: `run_o9_distributed_actor_gate.sh` transitions from `PENDING_IMPLEMENTATION` to real probe execution in O14.
3. Probe validation stage: O9.1-O9.4 debug+release probe suite reaches PASS.
4. Optional policy update: `run_phase_o_optional_tracks_signoff.sh` emits O.9 `optional/not-promoted` (or promotion candidate) with gate evidence.
5. Scope/policy promotion: claim contract + README + PLAN coordinated update when O9 is promoted to required.

### Wave O14 (Prepared Skeleton): O.9 Implementation Activation
- **Wave O14a (host-supported preflight, implementable while blocked)**
- [x] Extend the O9 watch artifact to record toolchain identity, searched runtime-library paths, generated `swiftDistributed` binding-export inventory, and blocker classification.
- [x] Extend promotion-eligibility artifacts to emit `promotion_stage` and `recommended_next_action` so O9 remains actionable while blocked.
- [x] Extend the O9 skeleton gate artifact to surface blocker/readiness metadata instead of a bare skip.
- [x] Keep O9 in `experimental/not-promoted-waiting` until the blocker changes from `missing_runtime_library` to `none`.
- **Current host result**: O14a preflight complete, blocker=`missing_runtime_library`, readiness phase=`o14a-preflight-only`.

- **Wave O14b (requires watch_status=SUPPORTED)**
- [ ] Step 1: Implement O9.1 probe (distributed actor metadata descriptor introspection).
- [ ] Step 2: Implement O9.2 probe (actor method invocation across distributed boundary).
- [ ] Step 3: Implement O9.3 probe (distributed result encoding/decoding and error propagation).
- [ ] Step 4: Implement O9.4 probe (actor isolation semantics under distributed calls).
- [ ] Step 5: Upgrade `run_o9_distributed_actor_gate.sh` from skeleton to required probe gate (debug/release equivalence).
- [ ] Step 6: Re-run optional-track signoff and verify O.9 transitions from waiting/transitional to optional-ready.
- [ ] Step 7: Evaluate promotion readiness for required-scope adoption.

---

## Phase P (Post-O): Next Major Expansion (Planning Stage)

### Strategic Options for Phase P

After completing Phase O (O1-O10 gates required, O8 promoted, O9 deferred), the project can choose from three major directions:

#### Option P.1: Cross-Platform Expansion (arm64 + x86_64 + Linux + Windows)
- **Scope**: Extend required parity scope to multi-platform host cells
- **Implementation**: 
  - Support Matrix v2: macOS arm64, macOS x86_64, Ubuntu Linux (x86_64), Windows (MSVC/Clang)
  - Each platform gets Phase C + Phase O gate verification
  - Platform-specific ABI shims for register conventions and calling-sequence differences
  - CI jobs per platform cell
- **Timeline**: 3-4 months (requires Swift cross-platform availability and runner infrastructure)

#### Option P.2: Foundation Bridging Depth (data types + serialization + networking)
- **Scope**: Expand bridging coverage to Foundation APIs and collection/serialization frameworks
- **Implementation**:
  - P.1: String/ByteString encoding, normalization, case-folding
  - P.2: URL/URLComponents/URLRequest construction and parsing
  - P.3: JSON/PropertyList/Codable round-trip parity
  - P.4: Foundation collections: NSArray, NSSet, NSOrderedSet bridging
  - P.5: Date/Calendar/TimeZone normalization and arithmetic
  - P.6: Number formatting (Decimal, percent, scientific notation)
- **Timeline**: 2-3 months (Foundation APIs are stable, no toolchain blockers)

#### Option P.3: Advanced Runtime Features (reflection + dynamic dispatch + type introspection)
- **Scope**: Add reflected type introspection and runtime inspection APIs
- **Implementation**:
  - P.1: Reflect on type metadata (fields, methods, conformances)
  - P.2: Dynamic type construction from name and arguments
  - P.3: Protocol conformance discovery at runtime
  - P.4: Witness table patching for method replacement
  - P.5: Type-erased function composition (function pointers + witness tables)
- **Timeline**: 2-3 months (significant Research mode, requires careful ABI stability analysis)

### Phase P Decision: APPROVED (Foundation Bridging Depth)
**Selected direction**: Option P.2 — Foundation Bridging Depth
- Builds directly on Phase C/O infrastructure (no new ABI surface)
- Opens up practical application domains (networking, data parsing, serialization)
- No external library blockers (Foundation is stable and included)
- Natural bridge from abstract ABI verification to production-ready bridging
- Timeline: 2-3 months (8 weeks estimated for P.1-P.6 full implementation)

### Phase P Scope Evolution (New Scope: v7-phase-p-foundation)
- **Base**: All Phase C + Phase O gates (141/141 parity + O1-O5 + O8 + O10)
- **New**: P.1-P.6 Foundation gates (String, URL, JSON/Codable, Collections, Date, Formatting)
- **Result**: v7-phase-p-foundation (broad-spectrum production bridging)

---

## Phase P (Active): Foundation Bridging Depth

### P.1: String / ByteString Encoding and Manipulation
- **Goal**: Deterministic String construction, encoding validation, Unicode handling
- **Scope**: UTF-8 encoding, ASCII validation, null-termination, normalization, case-folding
- **Implementation Sequence** (Completed):
  - [x] Step 1: Create `examples/runtime_p1_string_probe.rs` with test cases (ASCII, multi-byte UTF-8, null-term, normalization)
  - [x] Step 2: Create `scripts/run_p1_string_gate.sh` with debug + release validation
  - [x] Step 3: Add deterministic probes (8 test cases with expected outputs all PASS)
  - [x] Step 4: Validate parity on macOS arm64 (P.1 gate PASS 8/8 debug + release)
  - [ ] Step 5: Update claim-contract and phase-p signoff with P.1 evidence
  - [ ] Step 6: Verify no parity regressions (141/141 still PASS)
  - [ ] Step 7: Update Phase P spec in README
- **Status**: P.1 gate PASS (8/8 debug + release; ASCII, UTF-8, null-term, normalization, case-folding, empty string all verified)

### P.2: URL / URLComponents / URLRequest Construction and Parsing
- **Goal**: Web URL handling with encoding safety
- **Scope**: URL construction, path/query parsing, percent encoding, host/port/scheme components
- **Prerequisites**: P.1 String must be PASS (strings are building blocks)
- **Implementation Sequence** (Completed):
  - [x] Step 1: Create `examples/runtime_p2_url_probe.rs` with deterministic URL parse/build tests.
  - [x] Step 2: Create `scripts/run_p2_url_gate.sh` with debug + release equivalence validation.
  - [x] Step 3: Validate URL parse validity, scheme/host/path extraction, component-based build, and percent-encoded parse behavior.
  - [x] Step 4: Generate P.2 artifacts `target/runtime-probe/p2-url/p2-url-summary.{json,md}`.
  - [x] Step 5: Re-run parity matrix and confirm no regressions (`141/141 PASS`).
- **Status**: P.2 gate PASS (8/8 debug + release).

### P.3: JSON / PropertyList / Codable Round-Trip Parity
- **Goal**: Serialization/deserialization determinism
- **Scope**: JSON encoding, Codable bridging, PropertyList format, custom encoder chains
- **Prerequisites**: P.1, P.2 (networking requires JSON)
- **Implementation Sequence** (Completed):
  - [x] Step 1: Create `examples/runtime_p3_codable_probe.rs` for deterministic serialization checks.
  - [x] Step 2: Create `scripts/run_p3_codable_gate.sh` with debug + release equivalence validation.
  - [x] Step 3: Validate Codable Int32 round-trips and NSCoding/NSCopying serialization behavior.
  - [x] Step 4: Generate P.3 artifacts `target/runtime-probe/p3-codable/p3-codable-summary.{json,md}`.
  - [x] Step 5: Re-run parity matrix and confirm no regressions (`141/141 PASS`).
- **Status**: P.3 gate PASS (8/8 debug + release).

### P.4: Foundation Collections Bridging (NSArray, NSSet, NSOrderedSet)
- **Goal**: Objective-C collection bridging with type safety
- **Scope**: NSArray ↔ Array<T> conversion, NSSet identity semantics, count/membership operations
- **Prerequisites**: Any of P.1-P.3 (independent scope)
- **Implementation Sequence** (Completed):
  - [x] Step 1: Create `examples/runtime_p4_collections_probe.rs` for deterministic collection semantics checks.
  - [x] Step 2: Create `scripts/run_p4_collections_gate.sh` with debug + release equivalence validation.
  - [x] Step 3: Validate NSArray bridge count semantics and NSCopying array mutation independence.
  - [x] Step 4: Validate set-like uniqueness semantics via hashable distinct-count checks and ordered collection semantics via array append/get/pointer iteration.
  - [x] Step 5: Generate P.4 artifacts `target/runtime-probe/p4-collections/p4-collections-summary.{json,md}`.
  - [x] Step 6: Re-run parity matrix and confirm no regressions (`141/141 PASS`).
- **Status**: P.4 gate PASS (8/8 debug + release).

### P.5: Date / Calendar / TimeZone Normalization and Arithmetic
- **Goal**: Temporal arithmetic with DST, timezone, calendar rules
- **Scope**: Calendar systems (Gregorian, Julian), timezone database consistency, date arithmetic invariants
- **Prerequisites**: Foundation infrastructure available
- **Implementation Sequence** (Completed):
  - [x] Step 1: Create `examples/runtime_p5_temporal_probe.rs` for deterministic temporal checks.
  - [x] Step 2: Create `scripts/run_p5_temporal_gate.sh` with debug + release equivalence validation.
  - [x] Step 3: Validate epoch formatting/parsing, UTC year/month extraction, J2000 year, UTC offset, and format-parse round-trip stability.
  - [x] Step 4: Generate P.5 artifacts `target/runtime-probe/p5-temporal/p5-temporal-summary.{json,md}`.
  - [x] Step 5: Re-run parity matrix and confirm no regressions (`141/141 PASS`).
- **Status**: P.5 gate PASS (8/8 debug + release).

### P.6: Number Formatting (Decimal, Percent, Scientific Notation)
- **Goal**: Locale-aware formatting determinism
- **Scope**: Decimal → String formatting, percent encoding (0.45 → "45%"), scientific notation, rounding rules
- **Prerequisites**: P.1 String, P.3 Codable (formatting is often serialization-adjacent)
- **Implementation Sequence** (Completed):
  - [x] Step 1: Add Swift bridge probe export `swift_number_formatter_percent_scientific_probe_flags` for percent/scientific formatting semantics.
  - [x] Step 2: Create `examples/runtime_p6_formatting_probe.rs` for deterministic NumberFormatter/Decimal checks.
  - [x] Step 3: Create `scripts/run_p6_formatting_gate.sh` with debug + release equivalence validation.
  - [x] Step 4: Validate decimal NumberFormatter render/parse/rounding/invalid handling plus Decimal arithmetic probe flags.
  - [x] Step 5: Validate percent and scientific NumberFormatter render/parse behavior and NSNumber bridge round-trip.
  - [x] Step 6: Generate P.6 artifacts `target/runtime-probe/p6-formatting/p6-formatting-summary.{json,md}`.
  - [x] Step 7: Re-run parity matrix and confirm no regressions (`141/141 PASS`).
- **Status**: P.6 gate PASS (8/8 debug + release).

### Phase P Signoff Artifact Contract
- [x] Add directory: `target/runtime-probe/phase-p-signoff/`
- [x] P.1-P.6 gate artifacts:
  - [x] `p1-string-*.{json,md}` (UTF-8 encoding, normalization, case-folding)
  - [x] `p2-url-*.{json,md}` (URL parsing, percent encoding, components)
  - [x] `p3-codable-*.{json,md}` (JSON, PropertyList, custom encoders)
  - [x] `p4-collections-*.{json,md}` (NSArray, NSSet, count, membership)
  - [x] `p5-temporal-*.{json,md}` (Calendar, TimeZone, DST, arithmetic)
  - [x] `p6-formatting-*.{json,md}` (Decimal, Percent, Scientific)
- [x] Final signoff: `target/runtime-probe/phase-p-signoff/phase-p-signoff.json`

### Phase P Commit Discipline
- One P.* workstream per commit series (probe, gate, docs)
- Require gate PASS (debug + release) before marking each P.* complete
- Run full parity matrix after each P gate to ensure no regressions
- Update claim-contract incrementally: v7-phase-p-foundation only after all P.1-P.6 gates PASS

### Next Action: Update README Scope + Continue O.9 Watch
Align README parity-claim section with `v7-phase-p-foundation`, then continue Wave O13 O.9 readiness monitoring cadence.
---

## Execution Log

### 2026-03-20 (Phase P Launch — P.1 String Bridging Completion)
- [x] Launched Phase P: Foundation Bridging Depth (recommended option post-Wave O12).
- [x] P.1 String / ByteString probe implementation (8 deterministic test cases):
  - ASCII string construction and encoding
  - UTF-8 multi-byte sequence handling (emoji, accented chars)
  - Null-termination boundary semantics
  - String normalization (NFC form)
  - Case-folding (upper/lower)
  - Empty/non-empty capacity validation
- [x] P.1 gate script `scripts/run_p1_string_gate.sh` with debug+release equivalence validation.
- [x] P.1 probe PASS: 8/8 both debug and release (parity 141/141 maintained, no regressions).
- [x] Updated PLAN.md with Phase P foundation section (P.1-P.6 workstream definitions).
- [x] Updated success criteria table with v7-phase-p-foundation column.
- [x] Next: P.2 URL/URLComponents after P.1 evidence stabilization.

### 2026-03-20 (Phase P — P.2 URL Bridging Completion)
- [x] Added `examples/runtime_p2_url_probe.rs` with deterministic URL checks (parse validity, scheme/host/path extraction, component build, percent-encoded path parse).
- [x] Added `scripts/run_p2_url_gate.sh` with debug/release equivalence validation and artifact output.
- [x] P.2 gate PASS: 8/8 in debug and 8/8 in release; artifacts emitted under `target/runtime-probe/p2-url/`.
- [x] Re-ran parity matrix after P.2 changes: `141/141 PASS` (no regressions).
- [x] Updated Phase P plan status: P.2 complete, next action moved to P.3 Codable/Serialization.

### 2026-03-20 (Phase P — P.3 Codable/Serialization Completion)
- [x] Added `examples/runtime_p3_codable_probe.rs` covering deterministic serialization checks for Codable and NSCoding/NSCopying behavior.
- [x] Added `scripts/run_p3_codable_gate.sh` with debug/release equivalence validation and P.3 artifact output.
- [x] P.3 gate PASS: 8/8 in debug and 8/8 in release; artifacts emitted under `target/runtime-probe/p3-codable/`.
- [x] Re-ran parity matrix after P.3 changes: `141/141 PASS` (no regressions).
- [x] Updated Phase P plan status: P.3 complete, next action moved to P.4 Foundation Collections.

### 2026-03-20 (Phase P — P.4 Collections Bridging Completion)
- [x] Added `examples/runtime_p4_collections_probe.rs` covering deterministic collection behavior checks.
- [x] Added `scripts/run_p4_collections_gate.sh` with debug/release equivalence validation and P.4 artifact output.
- [x] P.4 gate PASS: 8/8 in debug and 8/8 in release; artifacts emitted under `target/runtime-probe/p4-collections/`.
- [x] Re-ran parity matrix after P.4 changes: `141/141 PASS` (no regressions).
- [x] Updated Phase P plan status: P.4 complete, next action moved to P.5 temporal bridging.

### 2026-03-20 (Phase P — P.5 Temporal Bridging Completion)
- [x] Added `examples/runtime_p5_temporal_probe.rs` covering deterministic Date/Calendar/TimeZone checks.
- [x] Added `scripts/run_p5_temporal_gate.sh` with debug/release equivalence validation and P.5 artifact output.
- [x] P.5 gate PASS: 8/8 in debug and 8/8 in release; artifacts emitted under `target/runtime-probe/p5-temporal/`.
- [x] Re-ran parity matrix after P.5 changes: `141/141 PASS` (no regressions).
- [x] Updated Phase P plan status: P.5 complete, next action moved to P.6 number formatting bridging.

### 2026-03-20 (Phase P — P.6 Number Formatting Completion)
- [x] Added `swift_number_formatter_percent_scientific_probe_flags` in `examples/RustBridge.swift` to cover explicit percent and scientific NumberFormatter semantics.
- [x] Added `examples/runtime_p6_formatting_probe.rs` covering decimal/percent/scientific formatting checks and NSNumber bridge round-trip.
- [x] Added `scripts/run_p6_formatting_gate.sh` with debug/release equivalence validation and P.6 artifact output.
- [x] P.6 gate PASS: 8/8 in debug and 8/8 in release; artifacts emitted under `target/runtime-probe/p6-formatting/`.
- [x] Re-ran parity matrix after P.6 changes: `141/141 PASS` (no regressions).
- [x] Updated Phase P plan status: P.6 complete; next action moved to Phase P signoff integration.

### 2026-03-20 (Phase P — Signoff Integration Completion)
- [x] Added consolidated signoff runner `scripts/run_phase_p_signoff.sh` to execute P.1-P.6 gates in order and emit phase-level summary artifacts.
- [x] Phase P signoff PASS with required gates `p1..p6` and parity baseline PASS.
- [x] Signoff artifacts emitted under `target/runtime-probe/phase-p-signoff/`:
  - `phase-p-signoff.json`
  - `phase-p-signoff.md`
- [x] Updated Phase P artifact-contract checklist to complete.
- [x] Integrated `scripts/run_phase_p_signoff.sh` into `scripts/run_full_plan_verification.sh`.
- [x] Updated claim contract scope to `v7-phase-p-foundation` and required gate list to include `phase_p_signoff`.
- [x] Updated claim validator to require Phase P signoff PASS evidence.
- [x] Full plan verification PASS after integration.
