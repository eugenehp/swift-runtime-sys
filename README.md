# swift-runtime-sys

An attempt to have Rust bindings generated for the Swift Runtime.

**Status**: WIP!

## SwiftUI from Rust

Not directly.

SwiftUI is a Swift framework built around Swift-only language features such as
generics, protocol conformances, property wrappers, result builders, and
compiler-generated metadata. This crate exposes pieces of the Swift runtime, but
that is not enough to construct or drive SwiftUI views from Rust.

The practical architecture is:

1. Write the UI in SwiftUI.
2. Put application logic, state transitions, parsing, networking, or compute in Rust.
3. Expose Rust through a stable C ABI.
4. Call that Rust API from Swift and adapt it into `ObservableObject` or `@State`.

See SWIFTUI.md for a minimal bridge pattern.

## Pick Swift version

```shell
git clone --depth 1 --branch swift-6.2.4-RELEASE https://github.com/swiftlang/swift.git swift
```

## Build the bindings

```shell
export SWIFT_RUNTIME=$(xcrun -show-sdk-path)/usr/lib/swift
SWIFT_RUNTIME_SYS_GENERATE_BINDINGS=1 cargo build
```

For normal builds after the checked-in bindings have been regenerated, `cargo build`
is sufficient.

## Create Swift Structs / Classes / Functions from Rust (basic)

Directly constructing arbitrary Swift types only through runtime internals is not a
stable API surface. The practical approach is to export explicit C ABI functions from
Swift and call them from Rust.

This repository now includes a minimal bridge example:

- Swift side exports constructors and methods: [examples/RustBridge.swift](examples/RustBridge.swift)
- Rust side wraps those exports with RAII handles: [examples/swift_bridge.rs](examples/swift_bridge.rs)

The example covers:

- Swift `struct` allocation and field access (`Person`)
- Swift `class` allocation, method invocation, and drop (`Counter`)
- Swift global function calls (`swift_add`, `swift_greet`)

Build the Swift bridge as a dynamic library on macOS:

```shell
swiftc -emit-library -o libRustBridge.dylib examples/RustBridge.swift
```

Compile and run the Rust example against that library:

```shell
rustc examples/swift_bridge.rs -L . -l RustBridge -o swift_bridge_demo
DYLD_LIBRARY_PATH=. ./swift_bridge_demo
```

If you want this integrated into Cargo targets, the same ABI can be linked from a
`build.rs` script that compiles the Swift source and emits `cargo:rustc-link-lib` and
`cargo:rustc-link-search` directives.

## Direct Runtime/Memory Mode (experimental)

If you want no Swift `@_cdecl` bridge and instead direct runtime calls from Rust,
use the raw runtime API module:

- [src/RuntimeRaw.rs](src/RuntimeRaw.rs)

This exposes low-level symbols like:

- `swift_allocObject`
- `swift_deallocClassInstance`
- `swift_retain` / `swift_release`
- `swift_getTypeByMangledNameInContext`
- `swift_getTypeByMangledNameInEnvironment`

Important limitations:

- Allocating a class object with `swift_allocObject` does not run Swift initializers.
- There is no stable public ABI for constructing arbitrary Swift `struct` values from
	Rust memory alone, especially for resilient or generic types.
- Calling arbitrary Swift methods/functions directly requires matching Swift calling
	conventions and metadata/witness arguments, which are ABI-sensitive.

In practice, direct runtime mode is good for runtime research and experimentation,
but not yet a stable application integration layer.

### RuntimeFactory API

The crate now exposes a generic runtime-construction module:

- [src/RuntimeFactory.rs](src/RuntimeFactory.rs)

`RuntimeFactory` can load Swift/thunk libraries and perform direct runtime work from Rust:

- class allocating initializers by mangled symbol
- value initializers by mangled symbol
- generated thunk method calls for supported ABI shapes
- retain/release/retainCount
- raw object alloc/dealloc using Swift runtime symbols
- raw symbol address resolution (`symbol_address`)
- direct typed variable memory reads/writes (`read_i32`, `write_i32`, offset variants)
- experimental class-protocol existential container construction

Protocol example in this repo:

- `CounterLike` protocol and `Counter: CounterLike` conformance in
	[examples/RustBridge.swift](examples/RustBridge.swift)
- witness table symbol used by demo:
	`$s10RustBridge7CounterCAA0C4LikeAAWP`

End-to-end demo using crate API:

```shell
./scripts/build_runtime_thunks.sh
swiftc -emit-library -g -o libRustBridge.dylib examples/RustBridge.swift
DYLD_LIBRARY_PATH=. cargo run --example runtime_factory_demo
```

This is the practical way to scale direct Rust control over Swift memory objects.
For protocols and fully generic existential construction, public stable runtime ABI
coverage is still incomplete and remains an active research area.

### Runtime Probe + LLDB Capture

Use the direct-runtime probe:

- [examples/runtime_raw_probe.rs](examples/runtime_raw_probe.rs)

This probe performs:

- direct mangled call to a Swift global function
- direct mangled construction of a Swift value type (`Person`)
- direct mangled allocating init for a Swift class (`Counter`)
- runtime retain/release inspection
- memory footprint dump (`malloc_size` + first object words)

Run probe:

```shell
swiftc -emit-library -g -o libRustBridge.dylib examples/RustBridge.swift
./scripts/build_runtime_thunks.sh
rustc -g examples/runtime_raw_probe.rs -o target/runtime_raw_probe
DYLD_LIBRARY_PATH=. ./target/runtime_raw_probe
```

Optional (experimental): attempt direct class method call through a non-Swift
`swiftcall` thunk (may crash if ABI assumptions drift):

```shell
RUNTIME_TRY_INCREMENT=1 DYLD_LIBRARY_PATH=. ./target/runtime_raw_probe
```

Current status on Apple Silicon:

- `Counter.increment(by:)` works through `libRuntimeThunks.dylib` using an arm64
	register-shaped thunk that places `self` in `x20` and `delta` in `w0` before
	calling the mangled symbol.
- Thunks are generated from [examples/runtime_thunk_methods.txt](examples/runtime_thunk_methods.txt)
	by [scripts/generate_runtime_thunks.sh](scripts/generate_runtime_thunks.sh),
	producing [examples/runtime_swiftcall_thunks.generated.c](examples/runtime_swiftcall_thunks.generated.c)
- Supported generated signatures currently include:
	- `self_i32_to_i32`
	- `self_to_i32`
	- `self_i32_to_void`
	- `self_i32_i32_to_i32`
	- `self_to_void`

Validated through the probe on arm64:

- `Counter.increment(by:) -> Int32`
- `Counter.current() -> Int32`
- `Counter.reset(to:) -> Void`
- `Counter.addPair(_: _:) -> Int32`
- `Counter.clear() -> Void`

Capture debugger trace:

```shell
./scripts/run_lldb_capture.sh
```

Generate a full parity matrix report (probe + tmux lldb):

```shell
./scripts/run_parity_matrix.sh
```

Run stress mode (repeated full matrix runs):

```shell
./scripts/run_parity_stress.sh 20
```

Stress mode now varies randomized probe seeds per run. Tune randomized coverage with:

```shell
FUZZ_CASES=128 ./scripts/run_parity_stress.sh 20
```

Optional fail-fast mode:

```shell
STOP_ON_FAIL=1 ./scripts/run_parity_stress.sh 100
```

Run isolated protocol-dispatch ABI matrix (one variant per process):

```shell
./scripts/run_protocol_dispatch_matrix.sh
```

Artifact:

- `target/runtime-probe/protocol-dispatch-matrix.md`

The parity pipeline now runs the crate example binary `runtime_raw_probe`
through Cargo, so probe/parity/debug paths all exercise `RuntimeFactory`.

Artifacts:

- `target/runtime-probe/probe.log`
- `target/runtime-probe/lldb_tmux.log`
- `target/runtime-probe/parity-report.json`
- `target/runtime-probe/parity-report.md`
- `target/runtime-probe/history/*.json`
- `target/runtime-probe/stress/stress-summary-*.md`
- `target/runtime-probe/stress/run-*.log`

The parity report now also tracks:

- direct class-field memory write/read parity (`Counter.value` offset test)
- protocol witness table resolution parity (non-null witness pointer)
- global Swift variable storage parity (direct symbol-address read/write)
- probe constructor evidence plus clean LLDB exit without `EXC_BAD_ACCESS`
- protocol witness slot inspection parity (detects requirement thunk symbol in witness table)
- raw `swift_allocObject` header parity (object header metadata pointer equals accessor metadata)
- keypath synthesis parity (`read/write/appending(path:)` flag checks)
- property-wrapper synthesis parity (default/memberwise init, clamp, projected value checks)
- result-builder synthesis parity (branch/optional/loop lowering flag checks)
- opaque return-type parity (opaque producer/consumer and stable underlying-type checks)
- task-local runtime parity (outside/inside/nested/restored context value checks)
- dynamic-replacement parity (direct and function-reference replacement dispatch checks)
- sendable concurrency parity (payload transfer across detached and child tasks)
- checked-continuation parity (async callback, synchronous inline, and throwing continuation paths)
- task-group concurrency parity (sum aggregation, throwing group, and max reduction across child tasks)
- `AsyncStream` parity (producer yields 5 values; consumer checks count, sum, and clean termination)
- unsafe memory layout parity (`withUnsafeBytes` field reads at known offsets; `withUnsafeMutablePointer` write+read roundtrip)
- protocol composition existential parity (`any P & Q` dispatch through both witness tables; cast-back to concrete)
- enum raw-value synthesis parity (`Int32`-backed `RawRepresentable`; round-trip, `init(rawValue:)` success/nil, auto-increment)
- `OptionSet` synthesis parity (`contains`, `union`, `intersection`, and `rawValue` round-trip checks)
- `CaseIterable` synthesis parity (`allCases` count/order/endpoints and raw-value aggregate checks)
- set algebra parity (`union`, `intersection`, `subtracting`, and symmetric-difference invariants)
- dictionary semantics parity (lookup, `default:` insertion, update old-value return, and removal/count invariants)
- comparable synthesis parity (sorted order plus `<`, `>`, and `==` invariants)
- result semantics parity (`get` success/failure, `map`, and `mapError` invariants)
- data semantics parity (count/sum/append invariants plus first-byte raw buffer validation)
- UUID semantics parity (parse/normalize/byte-width validation and invalid-input rejection)
- character set semantics parity (digit/non-digit and vowel/non-vowel scalar membership invariants)
- URLComponents semantics parity (scheme/host, port/path, query-item, and fragment parsing invariants)
- calendar semantics parity (UTC Gregorian date construction/decomposition, weekday identity, and leap-year month-day range)
- IndexSet semantics parity (membership, range insertion, removal, and first/last bounds invariants)
- time zone semantics parity (GMT and Asia/Kolkata offset/identifier invariants)
- measurement semantics parity (length/temperature/mass/speed unit conversion invariants)
- date formatter semantics parity (`DateFormatter` fixed-locale round-trip and ISO8601 rendering/parse invariants)
- scanner semantics parity (`Scanner` int/double/token scan progression and end-of-input invariant)
- locale semantics parity (canonical identifier normalization, decimal separator, and language/country component invariants)
- number formatter semantics parity (decimal rendering/parsing, half-up rounding, and invalid-input rejection invariants)
- URL semantics parity (scheme/host/path, query/fragment, absolute-string, and relative-resolution invariants)
- decimal semantics parity (`Decimal` add/multiply/round and invalid-parse rejection invariants)
- URLRequest semantics parity (URL/method/header/timeout/body invariants)
- data base64 semantics parity (encode/decode, ignore-unknown decode, and invalid-input rejection invariants)
- HTTPURLResponse semantics parity (status code, header extraction, URL preservation, content-type parsing)
- JSONEncoder/JSONDecoder semantics parity (encode, decode, nested structures, null-value handling)
- seeded randomized parity fuzz checks (`fuzz parity => ...`) validating add/divide/throw invariants

Note: protocol witness dispatch check is currently an experimental callability
signal (non-crashing dispatch through a witness entry), not yet strict semantic
equality against direct class method results for all ABI shapes.

The parity report now includes both:

- protocol witness dispatch callability
- protocol witness dispatch semantic parity (target value comparison)

Dispatch probing currently tests multiple arm64 register shapes, including
variants that carry witness table in `x1`.

`x1` witness-carrying variants are currently opt-in experimental probes
(`RUNTIME_TRY_WITNESS_X1=1`) because they may crash depending on ABI shape.

LLDB command script used by the capture:

- [scripts/lldb_runtime_cmds.txt](scripts/lldb_runtime_cmds.txt)

Captured output path:

- [target/runtime-probe/lldb.log](target/runtime-probe/lldb.log)

## License

[Apache 2.0](/LICENSE)