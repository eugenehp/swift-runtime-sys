//! Apple ScreenCaptureKit — screen recording from Rust.
//!
//! **Platform support:** macOS 12.3+.
//!
//! Wraps ScreenCaptureKit for capturing screen content, windows, and apps.
//!
//! ```ignore
//! assert!(screencapturekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"screencapturekit_available"; "macos");
