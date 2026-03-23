//! Apple MediaPlayer — music and media playback from Rust.
//!
//! **Platform support:** macOS 10.12+, iOS 3+, tvOS 14+, watchOS 5+.
//!
//! Wraps MediaPlayer for system music player and Now Playing info.
//!
//! ```ignore
//! assert!(mediaplayer::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"mediaplayer_available"; "macos", "ios", "tvos", "watchos");
