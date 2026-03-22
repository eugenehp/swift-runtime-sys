#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime numeric and string conversion functions.

use core::ffi::c_char;

unsafe extern "C" {
    /// Convert a float16 to a string.
    pub fn swift_float16ToString(
        buffer: *mut c_char,
        buffer_size: usize,
        value: u16, // __fp16 is 16 bits
        debug: bool,
    ) -> usize;

    /// Convert a float32 to a string.
    pub fn swift_float32ToString(
        buffer: *mut c_char,
        buffer_size: usize,
        value: f32,
        debug: bool,
    ) -> usize;

    /// Convert a float64 to a string.
    pub fn swift_float64ToString(
        buffer: *mut c_char,
        buffer_size: usize,
        value: f64,
        debug: bool,
    ) -> usize;

    /// Convert an int64 to a string.
    pub fn swift_int64ToString(
        buffer: *mut c_char,
        buffer_size: usize,
        value: i64,
        radix: i64,
        uppercase: bool,
    ) -> usize;

    /// Convert a uint64 to a string.
    pub fn swift_uint64ToString(
        buffer: *mut c_char,
        buffer_size: usize,
        value: u64,
        radix: i64,
        uppercase: bool,
    ) -> usize;

    /// Convert an integer to a float32.
    pub fn swift_intToFloat32(value: i64) -> f32;

    /// Convert an integer to a float64.
    pub fn swift_intToFloat64(value: i64) -> f64;
}
