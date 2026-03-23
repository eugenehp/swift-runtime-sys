//! Apple Cinematic — cinematic video processing from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+.
//!
//! Wraps Cinematic for processing Cinematic mode video and adjusting depth of field.
//!
//! ```ignore
//! assert!(cinematic::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"cinematic_available"; "macos", "ios");
