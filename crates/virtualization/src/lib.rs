//! Apple Virtualization — virtual machines from Rust.
//!
//! **Platform support:** macOS 11+.
//!
//! Wraps Virtualization.framework for running Linux and macOS VMs.
//!
//! ```ignore
//! assert!(virtualization::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"virtualization_available"; "macos");
