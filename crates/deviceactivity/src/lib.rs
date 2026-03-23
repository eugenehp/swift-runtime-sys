//! Apple DeviceActivity — Screen Time monitoring from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+.
//!
//! Wraps DeviceActivity for monitoring app and website usage (Screen Time API).
//!
//! ```ignore
//! assert!(deviceactivity::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"deviceactivity_available"; "macos", "ios");
