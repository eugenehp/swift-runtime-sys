//! Apple SoundAnalysis — audio classification from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 6+.
//!
//! Wraps SoundAnalysis for classifying sounds (speech, music, environment).
//!
//! ```ignore
//! assert!(soundanalysis::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"soundanalysis_available");
