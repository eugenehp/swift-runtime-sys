//! Apple Vision framework — image analysis and computer vision from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 11+, tvOS 11+, visionOS 1+.
//!
//! Wraps Vision for face detection, text recognition, image classification, and more.
//!
//! ```ignore
//! assert!(vision::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"vision_available"; "macos", "ios", "tvos", "xros");
