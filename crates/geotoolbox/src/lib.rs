//! Apple GeoToolbox — geographic utilities from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(geotoolbox::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"geotoolbox_available"; "macos", "ios");
