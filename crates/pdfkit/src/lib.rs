//! Apple PDFKit — PDF viewing and annotation from Rust.
//!
//! **Platform support:** macOS 10.4+, iOS 11+, visionOS 1+.
//!
//! Wraps PDFKit for displaying, searching, and annotating PDF documents.
//!
//! ```ignore
//! assert!(pdfkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"pdfkit_available"; "macos", "ios", "xros");
