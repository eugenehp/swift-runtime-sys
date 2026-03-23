//! Apple ManagedApp — managed app configuration from Rust.
//!
//! **Platform support:** iOS 18+.
//!
//! ```ignore
//! assert!(managedapp::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"managedapp_available"; "ios");
