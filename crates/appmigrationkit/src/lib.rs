//! Apple AppMigrationKit — app migration utilities from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(appmigrationkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"appmigrationkit_available"; "ios");
