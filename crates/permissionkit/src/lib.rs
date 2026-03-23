//! Apple PermissionKit — permission management from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(permissionkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"permissionkit_available"; "macos", "ios");
