//! Apple AVFoundation — media capture, playback, and editing from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.
//!
//! Wraps AVFoundation for camera capture, video playback, and media composition.
//!
//! ```ignore
//! assert!(avfoundation::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"avfoundation_available"; "macos", "ios", "tvos", "xros");
