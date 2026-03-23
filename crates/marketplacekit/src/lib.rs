//! Apple MarketplaceKit — alternative app marketplace from Rust.
//!
//! **Platform support:** iOS 17.4+.
//!
//! ```ignore
//! assert!(marketplacekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"marketplacekit_available"; "ios");
