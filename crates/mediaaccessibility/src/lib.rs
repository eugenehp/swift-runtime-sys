//! Apple MediaAccessibility — closed captions and audio descriptions from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 7+, tvOS 9+.
//!
//! ```ignore
//! assert!(mediaaccessibility::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"mediaaccessibility_available"; "macos", "ios", "tvos");
