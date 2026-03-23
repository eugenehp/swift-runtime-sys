//! Apple PushKit — VoIP and complication push notifications from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 8+, visionOS 1+, watchOS 6+.
//!
//! Wraps PushKit for receiving VoIP pushes and complication updates.
//!
//! ```ignore
//! assert!(pushkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"pushkit_available"; "macos", "ios", "xros", "watchos");
