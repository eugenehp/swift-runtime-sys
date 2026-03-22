#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime ObjC bridge utilities.

use core::ffi::{c_char, c_void};

/// Opaque pointer to Swift metadata.
pub type MetadataRef = *const c_void;
/// Opaque pointer to a Swift heap object.
pub type HeapObjectRef = *mut c_void;

/// Instance extents for a class.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClassInstanceExtents {
    pub negative_size_words: i32,
    pub positive_size_words: i32,
}

unsafe extern "C" {
    /// Bridge a Swift error to an NSError.
    pub fn swift_stdlib_bridgeErrorToNSError(error: *mut c_void) -> *mut c_void;

    /// Create a tagged pointer CFString.
    pub fn swift_stdlib_CFStringCreateTaggedPointerString(
        bytes: *const u8,
        length: usize,
    ) -> *const c_void;

    /// Hash a C string as a CFString.
    pub fn swift_stdlib_CFStringHashCString(bytes: *const u8, length: usize) -> usize;

    /// Hash an NSString as a CFString.
    pub fn swift_stdlib_CFStringHashNSString(object: *const c_void) -> usize;

    /// Create an indirect tagged pointer string.
    pub fn swift_stdlib_CreateIndirectTaggedPointerString(
        bytes: *const u8,
        length: usize,
    ) -> *const c_void;

    /// NSObject isEqual.
    pub fn swift_stdlib_NSObject_isEqual(lhs: *const c_void, rhs: *const c_void) -> bool;

    /// NSObject isKindOfClass.
    pub fn swift_stdlib_NSObject_isKindOfClass(object: *const c_void, cls: *const c_void) -> bool;

    /// NSString CString using encoding trampoline.
    pub fn swift_stdlib_NSStringCStringUsingEncodingTrampoline(
        object: *const c_void,
        encoding: usize,
    ) -> *const c_char;

    /// Create an NSString from UTF-8.
    pub fn swift_stdlib_NSStringFromUTF8(bytes: *const c_char, length: usize) -> *mut c_void;

    /// NSString getCString trampoline.
    pub fn swift_stdlib_NSStringGetCStringTrampoline(
        object: *const c_void,
        buffer: *mut c_char,
        max_length: usize,
        encoding: usize,
    ) -> bool;

    /// NSString hash value.
    pub fn swift_stdlib_NSStringHashValue(object: *const c_void, is_ascii: bool) -> usize;

    /// NSString hash value pointer.
    pub fn swift_stdlib_NSStringHashValuePointer(object: *const c_void, is_ascii: bool) -> usize;

    /// NSString length of bytes in encoding trampoline.
    pub fn swift_stdlib_NSStringLengthOfBytesInEncodingTrampoline(
        object: *const c_void,
        encoding: usize,
    ) -> usize;

    /// Check if a pointer is a dyld ObjC constant string.
    pub fn swift_stdlib_dyld_is_objc_constant_string(object: *const c_void) -> bool;

    /// Get the ObjC class of a heap object.
    pub fn swift_classOfObjCHeapObject(object: HeapObjectRef) -> *const c_void;

    /// Check if an ObjC class uses native Swift reference counting.
    pub fn swift_objcClassUsesNativeSwiftReferenceCounting(cls: *const c_void) -> bool;

    /// ObjC implicit entry point (Swift 3 compat).
    pub fn swift_objc_swift3ImplicitObjCEntrypoint(
        object: *const c_void,
        selector: *const c_void,
        file: *const c_char,
        line: usize,
        column: usize,
    );

    /// Get the ObjC class instance extents.
    pub fn swift_getObjCClassInstanceExtents(cls: *const c_void) -> ClassInstanceExtents;

    /// Get the Swift class instance extents.
    pub fn swift_getSwiftClassInstanceExtents(metadata: MetadataRef) -> ClassInstanceExtents;

    /// Root ObjC dealloc.
    pub fn swift_rootObjCDealloc(object: HeapObjectRef);

    /// Get the Optional nil sentinel object for Foundation.
    pub fn swift_Foundation_getOptionalNilSentinelObject() -> *const c_void;

    /// Check if an ObjC type name is serializable.
    pub fn swift_isObjCTypeNameSerializable(cls: *const c_void) -> bool;

    /// Make an AnyHashable by upcasting to the Hashable base type.
    pub fn swift_makeAnyHashableUpcastingToHashableBaseType(
        value: *const c_void,
        value_type: MetadataRef,
        result: *mut c_void,
    );
}
