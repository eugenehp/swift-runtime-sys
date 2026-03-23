//! Apple GameController — controller input from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 7+, tvOS 9+, visionOS 1+.
//!
//! Wraps GameController for MFi gamepads, keyboard, mouse, and racing wheel input.
//!
//! ```ignore
//! assert!(gamecontroller::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"gamecontroller_available"; "macos", "ios", "tvos", "xros");
