//! Apple UniformTypeIdentifiers — UTI system from Rust.
//!
//! **Platform support:** macOS 11+, iOS 14+, tvOS 14+, visionOS 1+, watchOS 7+.
//!
//! Wraps UniformTypeIdentifiers for declaring and querying file types.
//!
//! ```ignore
//! assert!(uniformtypeidentifiers::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"uniformtypeidentifiers_available");
