//! Apple iTunesLibrary — Music library access from Rust.
//!
//! **Platform support:** macOS 10.14+.
//!
//! ```ignore
//! assert!(ituneslibrary::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"ituneslibrary_available"; "macos");
