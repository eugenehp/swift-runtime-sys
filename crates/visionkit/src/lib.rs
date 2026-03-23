//! Apple VisionKit — document scanning and visual lookup from Rust.
//!
//! **Platform support:** macOS 13+, iOS 13+, visionOS 1+.
//!
//! Wraps VisionKit for document camera, data scanner, and Live Text.
//!
//! ```ignore
//! assert!(visionkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"visionkit_available"; "macos", "ios", "xros");
