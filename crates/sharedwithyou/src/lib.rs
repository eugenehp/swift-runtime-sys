//! Apple SharedWithYou — Messages collaboration from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+.
//!
//! Wraps SharedWithYou for surfacing content shared via Messages.
//!
//! ```ignore
//! assert!(sharedwithyou::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"sharedwithyou_available"; "macos", "ios", "tvos", "xros");
