//! Apple SecureElementCredential — secure element credentials from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(secureelementcredential::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"secureelementcredential_available"; "ios");
