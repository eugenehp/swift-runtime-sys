//! Apple IdentityLookup — caller ID and message filtering from Rust.
//!
//! **Platform support:** macOS 12+, iOS 11+.
//!
//! ```ignore
//! assert!(identitylookup::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"identitylookup_available"; "macos", "ios");
