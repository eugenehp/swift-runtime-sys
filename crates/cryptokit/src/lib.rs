//! Apple CryptoKit — cryptography from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 6+.
//!
//! Wraps CryptoKit for hashing, signing, encryption, and key agreement.
//!
//! ```ignore
//! assert!(cryptokit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"cryptokit_available");
