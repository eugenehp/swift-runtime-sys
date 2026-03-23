//! Apple GameKit — Game Center from Rust.
//!
//! **Platform support:** macOS 10.8+, iOS 4+, tvOS 9+, visionOS 1+.
//!
//! Wraps GameKit for leaderboards, achievements, matchmaking, and Game Center.
//!
//! ```ignore
//! assert!(gamekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"gamekit_available"; "macos", "ios", "tvos", "xros");
