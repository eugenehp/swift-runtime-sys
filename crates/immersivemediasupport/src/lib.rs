//! Apple ImmersiveMediaSupport — immersive media playback from Rust.
//!
//! **Platform support:** macOS 26+, visionOS 2+.
//!
//! ```ignore
//! assert!(immersivemediasupport::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"immersivemediasupport_available"; "macos", "xros");
