//! Apple Accessibility — assistive technology support from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, visionOS 1+, watchOS 8+.
//!
//! Wraps Accessibility for VoiceOver, Switch Control, and assistive technology attributes.
//!
//! ```ignore
//! assert!(accessibility::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"accessibility_available");
