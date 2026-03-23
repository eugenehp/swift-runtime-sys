//! Apple AVFAudio — audio playback, recording, and processing from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 7+.
//!
//! Wraps AVFAudio for audio engine, players, recorders, and audio sessions.
//!
//! ```ignore
//! assert!(avfaudio::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"avfaudio_available");
