//! Apple HealthKit — health and fitness data from Rust.
//!
//! **Platform support:** macOS 13+, iOS 8+, visionOS 1+, watchOS 2+.
//!
//! Wraps HealthKit for reading/writing health samples, workouts, and statistics.
//!
//! ```ignore
//! assert!(healthkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"healthkit_available"; "macos", "ios", "xros", "watchos");
