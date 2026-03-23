//! Apple ProximityReader — Tap to Pay on iPhone from Rust.
//!
//! **Platform support:** iOS 15.4+.
//!
//! ```ignore
//! assert!(proximityreader::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"proximityreader_available"; "ios");
