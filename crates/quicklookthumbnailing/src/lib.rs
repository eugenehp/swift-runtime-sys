//! Apple QuickLook Thumbnailing — file thumbnail generation from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+.
//!
//! ```ignore
//! assert!(quicklookthumbnailing::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"quicklookthumbnailing_available"; "macos", "ios");
