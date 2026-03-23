//! Apple CryptoTokenKit — smart cards and crypto tokens from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 13+, tvOS 14+, watchOS 7+.
//!
//! ```ignore
//! assert!(cryptotokenkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"cryptotokenkit_available"; "macos", "ios", "tvos", "watchos");
