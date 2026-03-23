//! Apple FamilyControls — parental controls from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+.
//!
//! Wraps FamilyControls for requesting Screen Time authorization and app restrictions.
//!
//! ```ignore
//! assert!(familycontrols::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"familycontrols_available"; "macos", "ios");
