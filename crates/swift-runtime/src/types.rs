//! Swift standard library type accessors.

use crate::metadata::Metadata;

/// Get metadata for `Swift.Int`.
pub fn int() -> Option<Metadata> { crate::metadata::lookup_type(b"Si") }

/// Get metadata for `Swift.Bool`.
pub fn bool() -> Option<Metadata> { crate::metadata::lookup_type(b"Sb") }

/// Get metadata for `Swift.Double`.
pub fn double() -> Option<Metadata> { crate::metadata::lookup_type(b"Sd") }

/// Get metadata for `Swift.Float`.
pub fn float() -> Option<Metadata> { crate::metadata::lookup_type(b"Sf") }

/// Get metadata for `Swift.String`.
pub fn string() -> Option<Metadata> { crate::metadata::lookup_type(b"SS") }

/// Get metadata for `Swift.Optional<T>`.
pub fn optional(element: &Metadata) -> Option<Metadata> {
    swift_runtime_sys::StdlibTypes::optional_metadata(element.as_raw())
        .and_then(Metadata::from_raw)
}

/// Get metadata for `Swift.Array<T>`.
pub fn array(element: &Metadata) -> Option<Metadata> {
    swift_runtime_sys::StdlibTypes::array_metadata(element.as_raw())
        .and_then(Metadata::from_raw)
}

/// Get metadata for `Swift.Dictionary<K, V>`.
pub fn dictionary(key: &Metadata, value: &Metadata) -> Option<Metadata> {
    swift_runtime_sys::StdlibTypes::dictionary_metadata(key.as_raw(), value.as_raw())
        .and_then(Metadata::from_raw)
}
