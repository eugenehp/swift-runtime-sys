//! Apple AccessorySetupKit — accessory pairing from Rust.
//!
//! **Platform support:** macOS 15+, iOS 18+.
//!
//! Wraps AccessorySetupKit for discovering and pairing Bluetooth/Wi-Fi accessories.
//!
//! ```ignore
//! assert!(accessorysetupkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"accessorysetupkit_available"; "macos", "ios");
