use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};

const RTLD_NOW: c_int = 0x2;
const RTLD_GLOBAL: c_int = 0x8;
const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

pub type OpaqueSwiftRef = *mut c_void;
pub type MetadataRef = *const c_void;
pub type WitnessTableRef = *const c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ClassProtocolExistential {
    pub object: OpaqueSwiftRef,
    pub witness_table: WitnessTableRef,
}

type SelfI32ToI32 = unsafe extern "C" fn(OpaqueSwiftRef, i32) -> i32;
type SelfToI32 = unsafe extern "C" fn(OpaqueSwiftRef) -> i32;
type SelfI32ToVoid = unsafe extern "C" fn(OpaqueSwiftRef, i32) -> i32;
type SelfI32I32ToI32 = unsafe extern "C" fn(OpaqueSwiftRef, i32, i32) -> i32;
type SelfToVoid = unsafe extern "C" fn(OpaqueSwiftRef) -> i32;
type I32I32ToI32 = unsafe extern "C" fn(i32, i32) -> i32;
type CallSelfToI32X20ByAddress = unsafe extern "C" fn(*const c_void, OpaqueSwiftRef) -> i32;
type CallSelfToI32X0ByAddress = unsafe extern "C" fn(*const c_void, OpaqueSwiftRef) -> i32;
type CallSelfToI32X20X0ByAddress = unsafe extern "C" fn(*const c_void, OpaqueSwiftRef) -> i32;
type CallWitnessSelfX0X1ByAddress =
    unsafe extern "C" fn(*const c_void, OpaqueSwiftRef, WitnessTableRef) -> i32;
type CallWitnessSelfX20X1ByAddress =
    unsafe extern "C" fn(*const c_void, OpaqueSwiftRef, WitnessTableRef) -> i32;
type CallExistentialClassToI32ByAddress =
    unsafe extern "C" fn(*const c_void, OpaqueSwiftRef) -> i32;
type F32ToF32 = unsafe extern "C" fn(f32) -> f32;
type F32F32ToF32 = unsafe extern "C" fn(f32, f32) -> f32;
type I32ToI32 = unsafe extern "C" fn(i32) -> i32;

/// Return value for a Swift `throws` call: either the result or the error object.
#[derive(Debug)]
pub enum ThrowsResult {
    /// Function returned normally; contains the return value.
    Ok(i32),
    /// Function threw; contains the non-null error object (caller must release).
    Threw(OpaqueSwiftRef),
}

/// A pair of i32 values (used for tuple returns).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct I32Pair {
    pub first: i32,
    pub second: i32,
}

type CallThrowsI32I32 = unsafe extern "C" fn(*const c_void, i32, i32, *mut *mut c_void) -> i32;
type CallI32I32ToI32Pair = unsafe extern "C" fn(*const c_void, i32, i32) -> I32Pair;
type SwiftWeakInit = unsafe extern "C" fn(*mut c_void, OpaqueSwiftRef);
type SwiftWeakLoadStrong = unsafe extern "C" fn(*mut c_void) -> OpaqueSwiftRef;
type SwiftWeakDestroy = unsafe extern "C" fn(*mut c_void);
type SwiftConformsToProtocol = unsafe extern "C" fn(MetadataRef, *const c_void) -> WitnessTableRef;

type AllocatingInitI32 = unsafe extern "C" fn(i32) -> OpaqueSwiftRef;
type StructInitI32I32U64 = unsafe extern "C" fn(i32, i32) -> u64;
type MetadataAccessor0 = unsafe extern "C" fn(usize) -> MetadataAccessorResponse;

type SwiftRetain = unsafe extern "C" fn(OpaqueSwiftRef) -> OpaqueSwiftRef;
type SwiftRelease = unsafe extern "C" fn(OpaqueSwiftRef);
type SwiftRetainCount = unsafe extern "C" fn(OpaqueSwiftRef) -> usize;
type SwiftAllocObject = unsafe extern "C" fn(MetadataRef, usize, usize) -> OpaqueSwiftRef;
type SwiftDeallocClassInstance = unsafe extern "C" fn(OpaqueSwiftRef, usize, usize);

#[repr(C)]
#[derive(Copy, Clone)]
struct MetadataAccessorResponse {
    metadata: MetadataRef,
    state: *const c_void,
}

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn dladdr(addr: *const c_void, info: *mut DlInfo) -> c_int;
}

#[repr(C)]
struct DlInfo {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

#[derive(Debug)]
pub enum RuntimeFactoryError {
    OpenLibrary(String),
    ResolveSymbol { symbol: String, error: String },
}

pub struct RuntimeFactory {
    _swift_library: *mut c_void,
    _thunk_library: Option<*mut c_void>,
}

impl RuntimeFactory {
    pub fn new(swift_library_path: &str) -> Result<Self, RuntimeFactoryError> {
        let swift_library = open_library(swift_library_path)?;
        Ok(Self {
            _swift_library: swift_library,
            _thunk_library: None,
        })
    }

    pub fn with_thunk_library(
        swift_library_path: &str,
        thunk_library_path: &str,
    ) -> Result<Self, RuntimeFactoryError> {
        let swift_library = open_library(swift_library_path)?;
        let thunk_library = open_library(thunk_library_path)?;
        Ok(Self {
            _swift_library: swift_library,
            _thunk_library: Some(thunk_library),
        })
    }

    pub fn call_self_i32_to_i32(
        &self,
        thunk_symbol: &str,
        object: OpaqueSwiftRef,
        arg0: i32,
    ) -> Result<i32, RuntimeFactoryError> {
        let thunk: SelfI32ToI32 = resolve_symbol_any(thunk_symbol)?;
        Ok(unsafe { thunk(object, arg0) })
    }

    pub fn call_self_to_i32(
        &self,
        thunk_symbol: &str,
        object: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let thunk: SelfToI32 = resolve_symbol_any(thunk_symbol)?;
        Ok(unsafe { thunk(object) })
    }

    pub fn call_self_i32_to_void(
        &self,
        thunk_symbol: &str,
        object: OpaqueSwiftRef,
        arg0: i32,
    ) -> Result<(), RuntimeFactoryError> {
        let thunk: SelfI32ToVoid = resolve_symbol_any(thunk_symbol)?;
        let _ = unsafe { thunk(object, arg0) };
        Ok(())
    }

    pub fn call_self_i32_i32_to_i32(
        &self,
        thunk_symbol: &str,
        object: OpaqueSwiftRef,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, RuntimeFactoryError> {
        let thunk: SelfI32I32ToI32 = resolve_symbol_any(thunk_symbol)?;
        Ok(unsafe { thunk(object, arg0, arg1) })
    }

    pub fn call_self_to_void(
        &self,
        thunk_symbol: &str,
        object: OpaqueSwiftRef,
    ) -> Result<(), RuntimeFactoryError> {
        let thunk: SelfToVoid = resolve_symbol_any(thunk_symbol)?;
        let _ = unsafe { thunk(object) };
        Ok(())
    }

    pub fn call_i32_i32_to_i32(
        &self,
        symbol: &str,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, RuntimeFactoryError> {
        let function: I32I32ToI32 = resolve_symbol_any(symbol)?;
        Ok(unsafe { function(arg0, arg1) })
    }

    pub fn call_self_to_i32_by_address_x20(
        &self,
        fn_address: *const c_void,
        object: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let call_helper: CallSelfToI32X20ByAddress =
            resolve_symbol_any("runtime_thunk_call_self_to_i32_x20_by_address")?;
        Ok(unsafe { call_helper(fn_address, object) })
    }

    pub fn call_self_to_i32_by_address_x0(
        &self,
        fn_address: *const c_void,
        object: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let call_helper: CallSelfToI32X0ByAddress =
            resolve_symbol_any("runtime_thunk_call_self_to_i32_x0_by_address")?;
        Ok(unsafe { call_helper(fn_address, object) })
    }

    pub fn call_self_to_i32_by_address_x20_x0(
        &self,
        fn_address: *const c_void,
        object: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let call_helper: CallSelfToI32X20X0ByAddress =
            resolve_symbol_any("runtime_thunk_call_self_to_i32_x20_x0_by_address")?;
        Ok(unsafe { call_helper(fn_address, object) })
    }

    pub fn call_witness_self_to_i32_by_address_x0_x1(
        &self,
        fn_address: *const c_void,
        object: OpaqueSwiftRef,
        witness: WitnessTableRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let call_helper: CallWitnessSelfX0X1ByAddress =
            resolve_symbol_any("runtime_thunk_call_witness_self_x0_x1_by_address")?;
        Ok(unsafe { call_helper(fn_address, object, witness) })
    }

    pub fn call_witness_self_to_i32_by_address_x20_x1(
        &self,
        fn_address: *const c_void,
        object: OpaqueSwiftRef,
        witness: WitnessTableRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let call_helper: CallWitnessSelfX20X1ByAddress =
            resolve_symbol_any("runtime_thunk_call_witness_self_x20_x1_by_address")?;
        Ok(unsafe { call_helper(fn_address, object, witness) })
    }

    /// Calls a Swift protocol witness thunk (TW suffix) that uses the existential
    /// indirect-self convention: `x20 = &object_slot`, thunk does `ldr x20,[x20]`.
    pub fn call_existential_class_to_i32_by_address(
        &self,
        fn_address: *const c_void,
        object: OpaqueSwiftRef,
    ) -> Result<i32, RuntimeFactoryError> {
        let call_helper: CallExistentialClassToI32ByAddress =
            resolve_symbol_any("runtime_thunk_call_existential_class_to_i32_by_address")?;
        Ok(unsafe { call_helper(fn_address, object) })
    }

    /// Calls a cdecl function `() -> Int32` (zero-arg, returns i32).
    pub fn call_to_i32(&self, symbol: &str) -> Result<i32, RuntimeFactoryError> {
        type ZeroToI32 = unsafe extern "C" fn() -> i32;
        let f: ZeroToI32 = resolve_symbol_any(symbol)?;
        Ok(unsafe { f() })
    }

    /// Calls a cdecl function `(Float) -> Float`.
    pub fn call_f32_to_f32(&self, symbol: &str, arg0: f32) -> Result<f32, RuntimeFactoryError> {
        let f: F32ToF32 = resolve_symbol_any(symbol)?;
        Ok(unsafe { f(arg0) })
    }

    /// Calls a cdecl function `(Float, Float) -> Float`.
    pub fn call_f32_f32_to_f32(
        &self,
        symbol: &str,
        arg0: f32,
        arg1: f32,
    ) -> Result<f32, RuntimeFactoryError> {
        let f: F32F32ToF32 = resolve_symbol_any(symbol)?;
        Ok(unsafe { f(arg0, arg1) })
    }

    /// Calls a cdecl function `(Int32) -> Int32`.
    pub fn call_i32_to_i32(&self, symbol: &str, arg0: i32) -> Result<i32, RuntimeFactoryError> {
        let f: I32ToI32 = resolve_symbol_any(symbol)?;
        Ok(unsafe { f(arg0) })
    }

    /// Calls a Swift `throws` function `(Int32, Int32) throws -> Int32`.
    pub fn call_throws_i32_i32(
        &self,
        symbol: &str,
        arg0: i32,
        arg1: i32,
    ) -> Result<ThrowsResult, RuntimeFactoryError> {
        let fn_addr = self.symbol_address(symbol)?;
        let thunk: CallThrowsI32I32 = resolve_symbol_any("runtime_thunk_call_throws_i32_i32")?;
        let mut error_ptr: *mut c_void = std::ptr::null_mut();
        let result = unsafe {
            thunk(
                fn_addr as *const c_void,
                arg0,
                arg1,
                &mut error_ptr as *mut *mut c_void,
            )
        };
        if error_ptr.is_null() {
            Ok(ThrowsResult::Ok(result))
        } else {
            Ok(ThrowsResult::Threw(error_ptr))
        }
    }

    /// Calls a Swift free function `(Int32, Int32) -> (Int32, Int32)` (tuple return).
    pub fn call_i32_i32_to_i32_pair(
        &self,
        symbol: &str,
        arg0: i32,
        arg1: i32,
    ) -> Result<I32Pair, RuntimeFactoryError> {
        let fn_addr = self.symbol_address(symbol)?;
        let thunk: CallI32I32ToI32Pair =
            resolve_symbol_any("runtime_thunk_call_i32_i32_to_i32_pair")?;
        Ok(unsafe { thunk(fn_addr as *const c_void, arg0, arg1) })
    }

    /// Reads a single byte at an offset into an object or memory region.
    pub fn read_u8_at_offset(&self, object: *const c_void, byte_offset: usize) -> u8 {
        unsafe { *((object as *const u8).add(byte_offset)) }
    }

    /// Initialises a Swift weak reference slot (8 bytes) pointing to `object`.
    pub fn weak_init(
        &self,
        weak_slot: *mut c_void,
        object: OpaqueSwiftRef,
    ) -> Result<(), RuntimeFactoryError> {
        let f: SwiftWeakInit = resolve_symbol_any("swift_weakInit")?;
        unsafe { f(weak_slot, object) };
        Ok(())
    }

    /// Loads a strong (+1) reference from a Swift weak reference slot.
    pub fn weak_load_strong(
        &self,
        weak_slot: *mut c_void,
    ) -> Result<OpaqueSwiftRef, RuntimeFactoryError> {
        let f: SwiftWeakLoadStrong = resolve_symbol_any("swift_weakLoadStrong")?;
        Ok(unsafe { f(weak_slot) })
    }

    /// Destroys a Swift weak reference slot.
    pub fn weak_destroy(&self, weak_slot: *mut c_void) -> Result<(), RuntimeFactoryError> {
        let f: SwiftWeakDestroy = resolve_symbol_any("swift_weakDestroy")?;
        unsafe { f(weak_slot) };
        Ok(())
    }

    /// Calls `swift_conformsToProtocol`; returns witness table or null.
    pub fn conforms_to_protocol(
        &self,
        type_meta: MetadataRef,
        protocol_descriptor: *const c_void,
    ) -> Result<WitnessTableRef, RuntimeFactoryError> {
        let f: SwiftConformsToProtocol = resolve_symbol_any("swift_conformsToProtocol")?;
        Ok(unsafe { f(type_meta, protocol_descriptor) })
    }

    pub fn symbol_address(&self, symbol: &str) -> Result<*mut c_void, RuntimeFactoryError> {
        let mut ptr = resolve_symbol_raw(RTLD_DEFAULT, symbol)?;
        if ptr.is_null() {
            let prefixed = format!("_{symbol}");
            ptr = resolve_symbol_raw(RTLD_DEFAULT, &prefixed)?;
        }
        if ptr.is_null() {
            return Err(RuntimeFactoryError::ResolveSymbol {
                symbol: symbol.to_string(),
                error: last_dlerror(),
            });
        }
        Ok(ptr)
    }

    pub fn call_allocating_init_i32(
        &self,
        symbol: &str,
        arg0: i32,
    ) -> Result<OpaqueSwiftRef, RuntimeFactoryError> {
        let init: AllocatingInitI32 = resolve_symbol_any(symbol)?;
        Ok(unsafe { init(arg0) })
    }

    pub fn call_struct_init_i32_i32_u64(
        &self,
        symbol: &str,
        arg0: i32,
        arg1: i32,
    ) -> Result<u64, RuntimeFactoryError> {
        let init: StructInitI32I32U64 = resolve_symbol_any(symbol)?;
        Ok(unsafe { init(arg0, arg1) })
    }

    pub fn metadata_from_accessor_0(
        &self,
        symbol: &str,
    ) -> Result<MetadataRef, RuntimeFactoryError> {
        let accessor: MetadataAccessor0 = resolve_symbol_any(symbol)?;
        let response = unsafe { accessor(0) };
        Ok(response.metadata)
    }

    pub fn read_i32(&self, address: *const c_void) -> i32 {
        unsafe { *(address as *const i32) }
    }

    pub fn write_i32(&self, address: *mut c_void, value: i32) {
        unsafe { *(address as *mut i32) = value }
    }

    pub fn read_i32_at_offset(&self, object: OpaqueSwiftRef, byte_offset: usize) -> i32 {
        let address = unsafe { (object as *const u8).add(byte_offset) } as *const c_void;
        self.read_i32(address)
    }

    pub fn write_i32_at_offset(&self, object: OpaqueSwiftRef, byte_offset: usize, value: i32) {
        let address = unsafe { (object as *mut u8).add(byte_offset) } as *mut c_void;
        self.write_i32(address, value)
    }

    pub fn make_class_protocol_existential(
        &self,
        object: OpaqueSwiftRef,
        witness_table: WitnessTableRef,
    ) -> ClassProtocolExistential {
        ClassProtocolExistential {
            object,
            witness_table,
        }
    }

    pub fn read_ptr_at_offset(&self, address: *const c_void, byte_offset: usize) -> *const c_void {
        let slot = unsafe { (address as *const u8).add(byte_offset) } as *const *const c_void;
        unsafe { *slot }
    }

    pub fn symbol_name_for_address(&self, address: *const c_void) -> Option<String> {
        let mut info = DlInfo {
            dli_fname: std::ptr::null(),
            dli_fbase: std::ptr::null_mut(),
            dli_sname: std::ptr::null(),
            dli_saddr: std::ptr::null_mut(),
        };

        let ok = unsafe { dladdr(address, &mut info as *mut DlInfo) };
        if ok == 0 || info.dli_sname.is_null() {
            return None;
        }

        Some(
            unsafe { CStr::from_ptr(info.dli_sname) }
                .to_string_lossy()
                .into_owned(),
        )
    }

    pub fn retain(&self, object: OpaqueSwiftRef) -> Result<OpaqueSwiftRef, RuntimeFactoryError> {
        let retain: SwiftRetain = resolve_symbol_any("swift_retain")?;
        Ok(unsafe { retain(object) })
    }

    pub fn release(&self, object: OpaqueSwiftRef) -> Result<(), RuntimeFactoryError> {
        let release: SwiftRelease = resolve_symbol_any("swift_release")?;
        unsafe { release(object) };
        Ok(())
    }

    pub fn retain_count(&self, object: OpaqueSwiftRef) -> Result<usize, RuntimeFactoryError> {
        let retain_count: SwiftRetainCount = resolve_symbol_any("swift_retainCount")?;
        Ok(unsafe { retain_count(object) })
    }

    pub fn alloc_object(
        &self,
        metadata: MetadataRef,
        size: usize,
        alignment_mask: usize,
    ) -> Result<OpaqueSwiftRef, RuntimeFactoryError> {
        let alloc: SwiftAllocObject = resolve_symbol_any("swift_allocObject")?;
        Ok(unsafe { alloc(metadata, size, alignment_mask) })
    }

    pub fn dealloc_class_instance(
        &self,
        object: OpaqueSwiftRef,
        size: usize,
        alignment_mask: usize,
    ) -> Result<(), RuntimeFactoryError> {
        let dealloc: SwiftDeallocClassInstance = resolve_symbol_any("swift_deallocClassInstance")?;
        unsafe { dealloc(object, size, alignment_mask) };
        Ok(())
    }
}

fn open_library(path: &str) -> Result<*mut c_void, RuntimeFactoryError> {
    let c_path = CString::new(path).map_err(|_| {
        RuntimeFactoryError::OpenLibrary(format!("invalid library path with interior NUL: {path}"))
    })?;
    let handle = unsafe { dlopen(c_path.as_ptr(), RTLD_NOW | RTLD_GLOBAL) };
    if handle.is_null() {
        return Err(RuntimeFactoryError::OpenLibrary(last_dlerror()));
    }
    Ok(handle)
}

fn resolve_symbol_any<T: Sized + Copy>(symbol: &str) -> Result<T, RuntimeFactoryError> {
    let mut ptr = resolve_symbol_raw(RTLD_DEFAULT, symbol)?;
    if ptr.is_null() {
        let prefixed = format!("_{symbol}");
        ptr = resolve_symbol_raw(RTLD_DEFAULT, &prefixed)?;
    }

    if ptr.is_null() {
        return Err(RuntimeFactoryError::ResolveSymbol {
            symbol: symbol.to_string(),
            error: last_dlerror(),
        });
    }

    Ok(unsafe { std::mem::transmute_copy(&ptr) })
}

fn resolve_symbol_raw(
    handle: *mut c_void,
    symbol: &str,
) -> Result<*mut c_void, RuntimeFactoryError> {
    let c_symbol = CString::new(symbol).map_err(|_| RuntimeFactoryError::ResolveSymbol {
        symbol: symbol.to_string(),
        error: "symbol contains interior NUL".to_string(),
    })?;
    Ok(unsafe { dlsym(handle, c_symbol.as_ptr()) })
}

fn last_dlerror() -> String {
    let err_ptr = unsafe { dlerror() };
    if err_ptr.is_null() {
        "unknown dlopen/dlsym error".to_string()
    } else {
        unsafe { CStr::from_ptr(err_ptr) }
            .to_string_lossy()
            .into_owned()
    }
}
