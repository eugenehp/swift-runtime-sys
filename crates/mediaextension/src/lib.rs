//! Apple MediaExtension — media codec extensions from Rust.
//!
//! **Platform support:** macOS 15+.
//!
//! ```ignore
//! assert!(mediaextension::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"mediaextension_available"; "macos");
