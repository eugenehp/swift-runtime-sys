//! Apple Speech — speech recognition from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 10+, visionOS 1+.
//!
//! Wraps Speech framework for on-device and server-based speech recognition.
//!
//! ```ignore
//! assert!(speech::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"speech_available"; "macos", "ios", "xros");
