//! Apple MultipeerConnectivity — peer-to-peer networking from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 7+, tvOS 10+, visionOS 1+.
//!
//! Wraps MultipeerConnectivity for discovering and communicating with nearby devices.
//!
//! ```ignore
//! assert!(multipeerconnectivity::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"multipeerconnectivity_available"; "macos", "ios", "tvos", "xros");
