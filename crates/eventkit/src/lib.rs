//! Apple EventKit — calendar and reminders from Rust.
//!
//! **Platform support:** macOS 10.8+, iOS 4+, visionOS 1+, watchOS 2+.
//!
//! Wraps EventKit for calendar events, reminders, and alarms.
//!
//! ```ignore
//! assert!(eventkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"eventkit_available"; "macos", "ios", "xros", "watchos");
