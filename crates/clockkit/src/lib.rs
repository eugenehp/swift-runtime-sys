//! Apple ClockKit — watchOS complications from Rust (deprecated).
//!
//! **Platform support:** watchOS 2+ (deprecated, use WidgetKit).
//!
//! ```ignore
//! assert!(clockkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"clockkit_available"; "watchos");
