//! Apple CoreTransferable — drag-and-drop and sharing from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+, watchOS 9+.
//!
//! Wraps CoreTransferable for the Transferable protocol, drag-and-drop, and copy/paste.
//!
//! ```ignore
//! assert!(coretransferable::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coretransferable_available");
