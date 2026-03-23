//! Apple BrowserEngineKit — browser engine integration from Rust.
//!
//! **Platform support:** macOS 15+, iOS 17.4+.
//!
//! ```ignore
//! assert!(browserenginekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"browserenginekit_available"; "macos", "ios");
