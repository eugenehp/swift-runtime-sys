//! Apple WeatherKit — weather data from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+, watchOS 9+.
//!
//! Wraps WeatherKit for current conditions, forecasts, and weather alerts.
//!
//! ```ignore
//! assert!(weatherkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"weatherkit_available");
