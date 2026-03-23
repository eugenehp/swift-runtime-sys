//! Apple QuickLook — file previews from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 4+, visionOS 1+.
//!
//! Wraps QuickLook for previewing documents, images, and 3D models.
//!
//! ```ignore
//! assert!(quicklook::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"quicklook_available"; "macos", "ios", "xros");
