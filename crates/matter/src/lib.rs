//! Apple Matter — smart home connectivity from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+, tvOS 16+.
//!
//! Wraps Matter/MatterSupport for commissioning and controlling Matter smart home devices.
//!
//! ```ignore
//! assert!(matter::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"matter_available"; "macos", "ios", "tvos");
