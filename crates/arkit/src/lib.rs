//! Apple ARKit — augmented reality from Rust.
//!
//! **Platform support:** iOS 11+, visionOS 1+.
//!
//! Wraps ARKit for world tracking, plane detection, face tracking, and body tracking.
//!
//! ```ignore
//! assert!(arkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"arkit_available"; "ios", "xros");
