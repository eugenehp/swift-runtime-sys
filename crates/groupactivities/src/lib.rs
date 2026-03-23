//! Apple GroupActivities — SharePlay from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, visionOS 1+.
//!
//! Wraps GroupActivities for SharePlay sessions and shared experiences.
//!
//! ```ignore
//! assert!(groupactivities::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"groupactivities_available"; "macos", "ios", "tvos", "xros");
