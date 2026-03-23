//! Apple BackgroundTasks — background work scheduling from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 7+.
//!
//! Wraps BackgroundTasks for scheduling app refresh and processing tasks.
//!
//! ```ignore
//! assert!(backgroundtasks::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"backgroundtasks_available");
