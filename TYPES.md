# Type Expansion Plan

## Purpose

The current runtime helper surface is intentionally biased toward `Int32` so the project can validate Swift ABI behavior with a small, predictable set of signatures before widening the matrix.

This document outlines how to expand beyond `Int32` in a staged way.

## Current State

- The reusable thunk catalog is centered on `Int32` method and function shapes.
- The Rust factory API mirrors that narrow thunk surface for convenience.
- The repository already exercises non-`Int32` domains in targeted places, including `Float`, `Double`, strings, pointers, enums, arrays, Objective-C interop, and async runtime probes.
- The main gap is not conceptual support for other types. The gap is a generalized, ergonomic Rust call surface and a broader generated thunk set.

## Why Start With Int32

- `Int32` keeps argument passing and return-value behavior easy to reason about while basic ABI assumptions are being verified.
- It avoids conflating calling convention work with separate problems such as indirect returns, ownership, reference bridging, and resilient layout.
- It makes parity regressions easier to isolate because the signatures are simple and repeatable.

## Expansion Goals

- Add first-class helpers for more primitive scalar types.
- Add pointer-oriented helpers for heap objects and opaque references.
- Add support for mixed-signature calls instead of only same-type pairs.
- Add structured return support where Swift uses tuples or indirect return conventions.
- Keep the parity matrix easy to interpret as coverage expands.

## Proposed Phases

### Phase 1: Scalar Primitives

Add direct helper coverage for the simplest ABI-stable scalar types.

- `Int64` and `UInt64`
- `Float` and `Double`
- `Bool`
- `UInt8` for byte-oriented flags where appropriate

Expected work:

- Extend the Rust factory with typed function aliases and call helpers.
- Extend the thunk generator inputs with the matching signatures.
- Add parity probes that validate both free functions and method dispatch for each new scalar type.

### Phase 2: Pointer and Opaque Handles

Add helpers for signatures that move opaque pointers across the boundary.

- `*mut c_void` and `*const c_void`
- object-returning bridges
- object-accepting bridges

Expected work:

- Replace ad hoc transmute-based call sites with named factory helpers where the ABI is already understood.
- Standardize retain/release expectations for returned references.
- Add explicit nullability checks in probes.

### Phase 3: Mixed Signatures

Add helper coverage for common mixed argument and return combinations.

- examples: `i32 -> bool`, `bool -> i32`, `i64 + i32 -> i64`, `pointer + i32 -> pointer`
- mixed method receivers and free-function forms

Expected work:

- Avoid a combinatorial explosion of manually named helpers.
- Prefer a generator-driven signature catalog with a constrained set of supported patterns.
- Keep symbol naming systematic so probe failures remain readable.

### Phase 4: Structured Values

Add support for value types that do not fit the current scalar-only helpers.

- tuple returns
- small structs passed directly in registers
- indirect returns for larger values
- optionals and enums where layout matters

Expected work:

- Separate calling-convention validation from semantic validation.
- Document which cases are architecture-sensitive.
- Preserve focused parity output instead of collapsing many layout checks into one status bit.

### Phase 5: Bridged and Ownership-Sensitive Types

Add higher-level types only after the low-level calling surface is stable.

- strings
- Foundation and Objective-C bridge types
- error objects
- existential containers and witness-driven calls

Expected work:

- Encode ownership rules directly in helper naming or wrapper types.
- Prefer explicit bridge helpers over generic pointer casts when semantics matter.
- Treat this as a runtime-behavior layer, not just a calling-convention layer.

## Implementation Principles

- Expand the surface in small slices and land parity checks with each slice.
- Prefer generated thunk coverage over one-off handwritten helpers once a pattern repeats.
- Keep unsafe transmute usage localized and replace it with named helpers when the ABI is verified.
- Do not generalize signatures faster than the tests can explain failures.
- Keep the `Int32` path intact until broader coverage is proven stable.

## Near-Term Recommendation

The next concrete step should be:

1. Add `i64`, `u64`, `f64`, `bool`, and pointer-returning helper shapes to the factory.
2. Add matching generated thunk signatures for free functions and `self` methods.
3. Add parity probes that validate argument passing, return values, and nullability.
4. Replace the existing ad hoc pointer transmute call sites where the new helpers make them unnecessary.

## Cross-Cutting Planning Areas

### ABI Portability

- Decide which helper shapes are expected to be stable only on arm64 macOS and which should remain portable across architectures.
- Mark architecture-sensitive cases such as small struct passing, tuple returns, witness dispatch, and indirect returns.
- Record which assumptions are tied to the current Swift release and should be revalidated on upgrade.

### Ownership And Lifetime Rules

- Define ownership expectations for every pointer-returning helper.
- Distinguish borrowed, retained, consumed, and nullable results in helper naming or wrapper types.
- Treat error objects, Objective-C bridges, and Foundation values as explicit lifetime domains instead of generic pointers.

### Signature Generation Strategy

- Plan when the thunk catalog moves from handwritten coverage to generated coverage.
- Keep naming deterministic so generated symbols remain easy to inspect in parity output.
- Constrain the supported signature matrix deliberately instead of trying to support every permutation.

### Test Policy And Coverage Shape

- Decide what minimum coverage each newly supported type requires.
- Separate free-function tests, method-dispatch tests, throwing tests, and raw-layout tests so failures stay local.
- Keep parity output granular enough to identify whether a regression is semantic, ABI-related, or ownership-related.

### Stability Levels

- Mark helpers as stable, experimental, or probe-only.
- Avoid exposing unstable ABI helpers as if they were ordinary library surface.
- Preserve the existing `Int32` path as the baseline until broader helpers are proven reliable.

### Diagnostics And Tooling

- Improve mismatch reporting so symbol lookup failures, calling-convention mismatches, nullability issues, and retain/release bugs are distinguishable.
- Keep the parity scripts readable as the number of status lines grows.
- Plan for targeted probe execution so debugging one signature family does not require rerunning the entire matrix.

### Public API Shape

- Decide whether `RuntimeFactory` should continue growing as explicit typed helpers or whether it should gain a more structured signature abstraction.
- Favor an API that keeps unsafe details visible enough to debug while reducing repetitive boilerplate.
- Avoid introducing a single overly-generic helper that hides ABI differences the project still needs to observe.

### Swift Fixture Organization

- Treat the Swift bridge fixture as a maintained ABI test corpus rather than a loose collection of examples.
- Group fixtures by concern such as scalars, ownership, layout, concurrency, and bridging.
- Keep probe-only helpers separate from examples intended to demonstrate ordinary usage.

### CI And Runtime Cost

- Decide which probes should run on every change and which should remain opt-in or experimental.
- Anticipate slower parity runs as more signature families are added.
- Keep the default matrix representative without making iteration too expensive.

### Documentation Boundaries

- Keep this file focused on roadmap and planning decisions.
- Put end-user usage guidance in README-level docs.
- Put architecture-specific ABI notes in dedicated research or implementation documents where they can evolve without turning this plan into a dump of low-level detail.

## Non-Goals For The First Expansion

- Full generic type erasure for arbitrary Rust and Swift signatures.
- Automatic marshalling for strings or collections.
- A single universal call helper that hides all ABI detail.

Those can come later if the project actually needs them, but they are not the right first step.