//! Apple BrowserEngineCore — browser engine hosting from Rust.
//!
//! **Platform support:** macOS 15+, iOS 17.4+.
//!
//! ```ignore
//! assert!(browserenginecore::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"browserenginecore_available"; "macos", "ios");
