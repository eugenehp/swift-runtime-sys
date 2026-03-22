//! Safe Rust interface to the Swift Runtime.
//!
//! This crate provides safe, ergonomic wrappers around the raw FFI bindings
//! in `swift-runtime-sys`.

pub use swift_runtime_sys as sys;

pub mod metadata;
pub mod string;
pub mod types;
pub mod retain;
