//! Apple BackgroundAssets — background asset downloads from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+.
//!
//! ```ignore
//! assert!(backgroundassets::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"backgroundassets_available"; "macos", "ios");
