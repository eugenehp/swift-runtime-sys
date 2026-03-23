//! Apple AVKit — media playback UI from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+.
//!
//! Wraps AVKit for AVPlayerViewController and picture-in-picture.
//!
//! ```ignore
//! assert!(avkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"avkit_available"; "macos", "ios", "tvos", "xros");
