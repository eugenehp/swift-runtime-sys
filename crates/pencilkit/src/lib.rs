//! Apple PencilKit — drawing and handwriting from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, visionOS 1+.
//!
//! Wraps PencilKit for canvas drawing, stroke recognition, and Apple Pencil input.
//!
//! ```ignore
//! assert!(pencilkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"pencilkit_available"; "macos", "ios", "xros");
