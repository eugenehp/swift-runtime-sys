//! Apple WiFiInfrastructure — WiFi infrastructure management from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(wifiinfrastructure::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"wifiinfrastructure_available"; "ios");
