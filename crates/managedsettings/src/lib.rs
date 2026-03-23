//! Apple ManagedSettings — device restrictions from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+.
//!
//! Wraps ManagedSettings for applying Screen Time shields and restrictions.
//!
//! ```ignore
//! assert!(managedsettings::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"managedsettings_available"; "macos", "ios");
