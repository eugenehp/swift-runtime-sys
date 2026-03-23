//! Apple MetricKit — app diagnostics from Rust.
//!
//! **Platform support:** macOS 12+, iOS 13+, visionOS 1+.
//!
//! Wraps MetricKit for receiving aggregated app performance and diagnostic data.
//!
//! ```ignore
//! assert!(metrickit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"metrickit_available"; "macos", "ios", "xros");
