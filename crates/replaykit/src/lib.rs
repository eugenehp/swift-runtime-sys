//! Apple ReplayKit — screen recording and broadcasting from Rust.
//!
//! **Platform support:** macOS 11+, iOS 9+, tvOS 10+.
//!
//! Wraps ReplayKit for in-app screen recording and live broadcasting.
//!
//! ```ignore
//! assert!(replaykit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"replaykit_available"; "macos", "ios", "tvos");
