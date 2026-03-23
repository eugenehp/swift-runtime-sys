//! Apple MLCompute — ML compute operations from Rust (deprecated).
//!
//! **Platform support:** macOS 11+, iOS 14+ (deprecated, use MetalPerformanceShadersGraph).
//!
//! ```ignore
//! assert!(mlcompute::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"mlcompute_available"; "macos", "ios");
