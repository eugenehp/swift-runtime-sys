//! Apple Symbols — SF Symbols metadata from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, tvOS 17+, visionOS 1+, watchOS 10+.
//!
//! Wraps Symbols for SF Symbol effects, variable color, and symbol images.
//!
//! ```ignore
//! assert!(symbols::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"symbols_available");
