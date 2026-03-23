//! Apple WirelessInsights — wireless diagnostics from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(wirelessinsights::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"wirelessinsights_available"; "ios");
