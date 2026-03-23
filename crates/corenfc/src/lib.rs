//! Apple Core NFC — NFC tag reading and writing from Rust.
//!
//! **Platform support:** iOS 11+.
//!
//! ```ignore
//! assert!(corenfc::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"corenfc_available"; "ios");
