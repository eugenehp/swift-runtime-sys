//! Apple WebKit — web views from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 8+, visionOS 1+.
//!
//! Wraps WebKit/WKWebView for embedding web content, navigation, and JavaScript evaluation.
//!
//! ```ignore
//! assert!(webkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"webkit_available"; "macos", "ios", "xros");
