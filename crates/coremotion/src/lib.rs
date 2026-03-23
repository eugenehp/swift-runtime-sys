//! Apple Core Motion — accelerometer and gyroscope from Rust.
//!
//! **Platform support:** iOS 4+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Motion for accelerometer, gyroscope, pedometer, and activity recognition.
//!
//! ```ignore
//! assert!(coremotion::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coremotion_available"; "ios", "xros", "watchos");
