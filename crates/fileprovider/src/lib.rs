//! Apple FileProvider — file sync from Rust.
//!
//! **Platform support:** macOS 11+, iOS 11+, visionOS 1+.
//!
//! Wraps FileProvider for cloud file sync, enumeration, and materialization.
//!
//! ```ignore
//! assert!(fileprovider::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"fileprovider_available"; "macos", "ios", "xros");
