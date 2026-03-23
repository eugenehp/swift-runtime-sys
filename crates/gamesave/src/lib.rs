//! Apple GameSave — game save management from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+, tvOS 26+.
//!
//! ```ignore
//! assert!(gamesave::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"gamesave_available"; "macos", "ios", "tvos");
