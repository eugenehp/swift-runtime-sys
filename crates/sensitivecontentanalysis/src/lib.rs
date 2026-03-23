//! Apple SensitiveContentAnalysis — CSAM/nudity detection from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, visionOS 1+.
//!
//! Wraps SensitiveContentAnalysis for detecting sensitive imagery.
//!
//! ```ignore
//! assert!(sensitivecontentanalysis::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"sensitivecontentanalysis_available"; "macos", "ios", "xros");
