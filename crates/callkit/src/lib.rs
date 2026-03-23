//! Apple CallKit — VoIP call integration from Rust.
//!
//! **Platform support:** macOS 13+, iOS 10+, watchOS 9+.
//!
//! Wraps CallKit for VoIP call UI, call directory, and blocking/identification.
//!
//! ```ignore
//! assert!(callkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"callkit_available"; "macos", "ios", "watchos");
