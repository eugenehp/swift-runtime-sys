//! Apple AuthenticationServices — Sign in with Apple from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 12+, tvOS 16+, visionOS 1+, watchOS 6+.
//!
//! Wraps AuthenticationServices for Sign in with Apple, passkeys, and web authentication.
//!
//! ```ignore
//! assert!(authenticationservices::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"authenticationservices_available");
