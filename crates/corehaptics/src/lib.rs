//! Apple Core Haptics — haptic feedback from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+.
//!
//! Wraps Core Haptics for custom haptic patterns and audio-haptic experiences.
//!
//! ```ignore
//! assert!(corehaptics::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"corehaptics_available"; "macos", "ios");
