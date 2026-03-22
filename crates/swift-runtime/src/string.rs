//! Swift string creation from Rust.

/// Create a Swift small string (≤15 bytes) as raw bytes.
pub fn create_small(s: &str) -> Option<[u8; 16]> {
    swift_runtime_sys::StdlibTypes::create_swift_string(s)
}

/// Extract the contents of a Swift small string.
pub fn extract_small(buf: &[u8; 16]) -> Option<&str> {
    swift_runtime_sys::StdlibTypes::extract_small_string(buf)
}

/// Create a Swift.String via the runtime (supports any length).
///
/// Returns the 16-byte String representation.
pub fn create(s: &str) -> Option<[u8; 16]> {
    unsafe { swift_runtime_sys::SwiftUIBridge::create_swift_string(s).ok() }
}
