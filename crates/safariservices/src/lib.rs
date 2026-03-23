//! Apple SafariServices — in-app browser from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 7+, visionOS 1+.
//!
//! Wraps SafariServices for SFSafariViewController, content blockers, and web extensions.
//!
//! ```ignore
//! assert!(safariservices::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"safariservices_available"; "macos", "ios", "xros");
