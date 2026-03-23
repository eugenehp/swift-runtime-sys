//! Apple AutomatedDeviceEnrollment — MDM enrollment from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(automateddeviceenrollment::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"automateddeviceenrollment_available"; "ios");
