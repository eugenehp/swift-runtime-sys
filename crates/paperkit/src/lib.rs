//! Apple PaperKit — paper detection and interaction from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(paperkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"paperkit_available"; "macos", "ios");
