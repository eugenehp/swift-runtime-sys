# Runtime Parity Plan (toward 100%)

## Goal
Define and complete all work needed to claim production-grade parity between Rust runtime access and Swift behavior in this repository.

## Current Baseline
- [x] Current matrix status: `101/101 PASS`.
- [x] Source of truth for checks and totals: `scripts/run_parity_matrix.sh`.
- [x] Latest parity key inventory: `/memories/repo/parity.md`.

## Definition of 100% Parity
100% parity is not only a check count. It requires all of the following:
- [x] Functional parity: all in-scope semantic/runtime features are covered by deterministic checks.
- [x] ABI parity: all supported call shapes are validated and stable (no opt-in crash-prone paths).
- [x] Reliability parity: stress/fuzz runs pass repeatedly with no intermittent failures.
- [ ] Platform/version parity: supported Swift and macOS targets pass the same matrix.
- [x] Tooling parity: CI enforces parity gates and preserves history/artifacts.
- [x] Operational parity: known experimental caveats removed or explicitly scoped out of claim.

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
- [ ] Exit criteria:
- [ ] Same required checks pass across all supported cells.
- [ ] Any version-specific deviation is documented and expected.

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
- [ ] Supported version/platform matrix passes.
- [x] CI blocks merges on parity regressions.
- [x] README parity claim updated with scope and evidence.

## Execution Order
- [x] Freeze scope and required ABI shape list.
- [x] Stabilize experimental/crash-prone runtime paths.
- [x] Add missing required checks and ABI variants.
- [x] Promote stress/fuzz and protocol matrix to required CI gates.
- [ ] Validate across support matrix.
- [ ] Publish final parity claim backed by artifacts.
