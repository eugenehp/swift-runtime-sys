//! Apple StickerKit — iMessage sticker packs from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(stickerkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"stickerkit_available"; "macos", "ios");
