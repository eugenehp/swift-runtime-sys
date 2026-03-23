//! Apple CoreHID — USB and Bluetooth HID devices from Rust.
//!
//! **Platform support:** macOS 15+.
//!
//! ```ignore
//! assert!(corehid::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"corehid_available"; "macos");
