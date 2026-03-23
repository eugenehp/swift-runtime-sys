//! Apple AutomaticAssessmentConfiguration — exam lockdown from Rust.
//!
//! **Platform support:** macOS 10.15.4+, iOS 13.4+.
//!
//! ```ignore
//! assert!(automaticassessmentconfiguration::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"automaticassessmentconfiguration_available"; "macos", "ios");
