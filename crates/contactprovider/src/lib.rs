//! Apple ContactProvider — contact provider extensions from Rust.
//!
//! **Platform support:** iOS 18+.
//!
//! ```ignore
//! assert!(contactprovider::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"contactprovider_available"; "ios");
