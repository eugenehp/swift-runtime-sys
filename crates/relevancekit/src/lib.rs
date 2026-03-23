//! Apple RelevanceKit — relevance engine from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(relevancekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"relevancekit_available"; "macos", "ios");
