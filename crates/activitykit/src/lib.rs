//! Apple ActivityKit — Live Activities and Dynamic Island from Rust.
//!
//! **Platform support:** iOS 16.1+ only (not available on macOS, tvOS, or visionOS).
//!
//! ```ignore
//! assert!(activitykit::is_available());
//! ```
//!
//! Note: Creating Live Activities requires defining ActivityAttributes
//! via the @available Swift protocol, which needs compiler macro support.
//! This crate provides availability checking. Full Live Activity support
//! requires a Swift extension target in the app bundle.

apple_sys_helpers::apple_framework!(c"activitykit_available"; "ios");
