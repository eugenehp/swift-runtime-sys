//! Apple Image Playground — AI image generation from Rust.
//!
//! **Platform support:** macOS 15.2+, iOS 18.2+.
//!
//! Wraps Image Playground for on-device AI image generation.
//!
//! ```ignore
//! assert!(imageplayground::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"imageplayground_available"; "macos", "ios");
