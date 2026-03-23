//! Apple AdAttributionKit — ad attribution from Rust.
//!
//! **Platform support:** iOS 17.4+.
//!
//! ```ignore
//! assert!(adattributionkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"adattributionkit_available"; "ios");
