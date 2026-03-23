//! Apple AppTrackingTransparency — tracking permission from Rust.
//!
//! **Platform support:** macOS 11+, iOS 14+, tvOS 14+, visionOS 1+.
//!
//! Wraps AppTrackingTransparency for requesting user permission to track.
//!
//! ```ignore
//! assert!(apptrackingtransparency::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"apptrackingtransparency_available"; "macos", "ios", "tvos", "xros");
