//! Apple UserNotifications — push and local notifications from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 10+, tvOS 10+, visionOS 1+, watchOS 3+.
//!
//! Wraps UserNotifications for scheduling local notifications and handling remote push.
//!
//! ```ignore
//! assert!(usernotifications::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"usernotifications_available");
