//! Apple PassKit — Wallet and Apple Pay from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 6+, visionOS 1+, watchOS 2+.
//!
//! Wraps PassKit for Apple Pay, Wallet passes, and payment sheets.
//!
//! ```ignore
//! assert!(passkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"passkit_available"; "macos", "ios", "xros", "watchos");
