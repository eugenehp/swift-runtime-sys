//! Apple AlarmKit — alarm management from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(alarmkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"alarmkit_available"; "ios");
