//! Pure-Rust SwiftUI App launcher — replicates Swift compiler SIL output.
//!
//! Launches a SwiftUI `App` by constructing the type metadata, protocol
//! conformance descriptor, and witness table that the Swift runtime expects,
//! then calling `SwiftUI.App.main()` directly.
//!
//! No Swift source files. No swiftc. Pure Rust + dlsym.
//!
//! # How it works
//!
//! The Swift compiler turns `@main struct MyApp: App` into:
//!
//! 1. Type metadata (`VMf`) — describes the struct
//! 2. Nominal descriptor (`VMn`) — names the type
//! 3. Protocol conformance descriptor (`Mc`) — proves it conforms to `App`
//! 4. Witness table with `body.getter` and `init` thunks
//! 5. `main()` calls `SwiftUI.App.main<MyApp>(metatype, witness_table)`
//!
//! We replicate all of this from Rust using `swift-runtime-sys` for
//! runtime symbol resolution and asm thunks for Swift CC calls.
//!
//! # Usage
//!
//! ```ignore
//! use winit_swift::sil;
//!
//! fn main() {
//!     sil::launch_app();
//! }
//! ```

pub mod metadata;
pub mod launch;

pub use launch::launch_app;
