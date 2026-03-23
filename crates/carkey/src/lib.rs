//! Apple CarKey — digital car keys from Rust.
//!
//! **Platform support:** macOS 13.3+, iOS 16.4+, watchOS 9.4+.
//!
//! ```ignore
//! assert!(carkey::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"carkey_available"; "macos", "ios", "watchos");
