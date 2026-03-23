//! Apple AdSupport — advertising identifier from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 6+, tvOS 9+, visionOS 1+.
//!
//! Wraps AdSupport for reading the IDFA advertising identifier.
//!
//! ```ignore
//! assert!(adsupport::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"adsupport_available"; "macos", "ios", "tvos", "xros");
