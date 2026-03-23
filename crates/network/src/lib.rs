//! Apple Network framework — modern networking from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 12+, tvOS 12+, visionOS 1+, watchOS 6+.
//!
//! Wraps Network.framework for TCP/UDP/QUIC connections, listeners, and path monitoring.
//!
//! ```ignore
//! assert!(network::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"network_available");
