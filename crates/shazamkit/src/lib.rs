//! Apple ShazamKit — music recognition from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, visionOS 1+, watchOS 8+.
//!
//! Wraps ShazamKit for identifying songs from audio and building custom catalogs.
//!
//! ```ignore
//! assert!(shazamkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"shazamkit_available");
