//! Apple MapKit — maps and directions from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 3+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps MapKit for map views, annotations, overlays, and directions.
//!
//! ```ignore
//! assert!(mapkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"mapkit_available");
