//! Apple AdServices — ad attribution from Rust.
//!
//! **Platform support:** macOS 14+, iOS 14.3+.
//!
//! Wraps AdServices for Apple Search Ads attribution tokens.
//!
//! ```ignore
//! assert!(adservices::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"adservices_available"; "macos", "ios");
