//! Apple WiFiAware — WiFi Aware networking from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(wifiaware::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"wifiaware_available"; "macos", "ios");
