//! Apple NaturalLanguage — text processing and NLP from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 12+, tvOS 12+, visionOS 1+, watchOS 5+.
//!
//! Wraps NaturalLanguage for language detection, tokenization, and sentiment analysis.
//!
//! ```ignore
//! assert!(naturallanguage::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"naturallanguage_available");
