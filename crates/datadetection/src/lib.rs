//! Apple DataDetection — structured data extraction from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, visionOS 1+.
//!
//! Wraps DataDetection for extracting dates, addresses, links, and phone numbers from text.
//!
//! ```ignore
//! assert!(datadetection::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"datadetection_available"; "macos", "ios", "xros");
