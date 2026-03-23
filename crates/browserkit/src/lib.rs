//! Apple BrowserKit — browser kit from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(browserkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"browserkit_available"; "ios");
