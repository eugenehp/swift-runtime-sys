//! Apple Assignables — education assignment management from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(assignables::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"assignables_available"; "ios");
