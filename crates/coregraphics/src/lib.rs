//! Apple Core Graphics — 2D drawing from Rust.
//!
//! **Platform support:** macOS 10.0+, iOS 2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Graphics (Quartz 2D) for paths, colors, images, and PDF generation.
//!
//! ```ignore
//! assert!(coregraphics::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coregraphics_available");
