//! Apple LocalAuthentication — biometric auth from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 8+, visionOS 1+, watchOS 3+.
//!
//! Wraps LocalAuthentication for Face ID, Touch ID, and password authentication.
//!
//! ```ignore
//! assert!(localauthentication::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"localauthentication_available"; "macos", "ios", "xros", "watchos");
