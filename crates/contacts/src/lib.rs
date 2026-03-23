//! Apple Contacts — address book access from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Contacts for reading and writing contact records.
//!
//! ```ignore
//! assert!(contacts::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"contacts_available"; "macos", "ios", "xros", "watchos");
