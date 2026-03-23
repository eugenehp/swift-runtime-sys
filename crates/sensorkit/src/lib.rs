//! Apple SensorKit — research sensor data from Rust.
//!
//! **Platform support:** iOS 14+.
//!
//! Wraps SensorKit for ambient light, accelerometer, and keyboard metrics (research use).
//!
//! ```ignore
//! assert!(sensorkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"sensorkit_available"; "ios");
