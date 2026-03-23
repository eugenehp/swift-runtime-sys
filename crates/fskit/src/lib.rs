//! Apple FSKit — file system extensions from Rust.
//!
//! **Platform support:** macOS 15+.
//!
//! ```ignore
//! assert!(fskit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"fskit_available"; "macos");
