//! Apple TelephonyMessagingKit — telephony messaging from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(telephonymessagingkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"telephonymessagingkit_available"; "macos", "ios");
