//! Apple LiveCommunicationKit — live calling from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+.
//!
//! Wraps LiveCommunicationKit for VoIP calling with modern async API.
//!
//! ```ignore
//! assert!(livecommunicationkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"livecommunicationkit_available"; "macos", "ios");
