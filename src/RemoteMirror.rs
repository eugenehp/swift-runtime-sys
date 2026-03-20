use core::ffi::{c_char, c_int, c_uint, c_void};
use std::ffi::{CStr, CString};
use std::process::Command;
use std::ptr;

const RTLD_NOW: c_int = 0x2;
const RTLD_GLOBAL: c_int = 0x8;
const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
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

type SwiftAddr = u64;
type SwiftReflectionContextRef = *mut c_void;
type FreeBytesFunction = unsafe extern "C" fn(*mut c_void, *const c_void, *mut c_void);
type ReadBytesFunction =
    unsafe extern "C" fn(*mut c_void, SwiftAddr, u64, *mut *mut c_void) -> *const c_void;
type GetStringLengthFunction = unsafe extern "C" fn(*mut c_void, SwiftAddr) -> u64;
type GetSymbolAddressFunction = unsafe extern "C" fn(*mut c_void, *const c_char, u64) -> SwiftAddr;
type GetSupportedMetadataVersion = unsafe extern "C" fn() -> u16;
type CreateReflectionContext = unsafe extern "C" fn(
    *mut c_void,
    u8,
    FreeBytesFunction,
    ReadBytesFunction,
    GetStringLengthFunction,
    GetSymbolAddressFunction,
) -> SwiftReflectionContextRef;
type DestroyReflectionContext = unsafe extern "C" fn(SwiftReflectionContextRef);
type AddImage = unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr) -> i32;
type OwnsAddress = unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr) -> i32;
type IterateConformanceCache = unsafe extern "C" fn(
    SwiftReflectionContextRef,
    unsafe extern "C" fn(SwiftAddr, SwiftAddr, *mut c_void),
    *mut c_void,
) -> *const c_char;
type AsyncTaskSlabPointer =
    unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr) -> SwiftAsyncTaskSlabReturnRaw;
type AsyncTaskInfoFn =
    unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr) -> SwiftAsyncTaskInfoRaw;
type ActorInfoFn = unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr) -> SwiftActorInfoRaw;
type NextJobFn = unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr) -> SwiftAddr;
type TypeRefForMetadata = unsafe extern "C" fn(SwiftReflectionContextRef, usize) -> SwiftAddr;
type TypeRefForInstance = unsafe extern "C" fn(SwiftReflectionContextRef, usize) -> SwiftAddr;
type CopyNameForTypeRef =
    unsafe extern "C" fn(SwiftReflectionContextRef, SwiftAddr, bool) -> *mut c_char;
type InfoForMetadata = unsafe extern "C" fn(SwiftReflectionContextRef, usize) -> SwiftTypeInfoRaw;
type InfoForInstance = unsafe extern "C" fn(SwiftReflectionContextRef, usize) -> SwiftTypeInfoRaw;
type ChildOfMetadata =
    unsafe extern "C" fn(SwiftReflectionContextRef, usize, u32) -> SwiftChildInfoRaw;
type ChildOfInstance =
    unsafe extern "C" fn(SwiftReflectionContextRef, usize, u32) -> SwiftChildInfoRaw;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct SwiftTypeInfoRaw {
    kind: c_int,
    size: c_uint,
    alignment: c_uint,
    stride: c_uint,
    num_fields: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SwiftChildInfoRaw {
    name: *const c_char,
    offset: c_uint,
    kind: c_int,
    tr: SwiftAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SwiftAsyncTaskSlabReturnRaw {
    error: *const c_char,
    slab_ptr: SwiftAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SwiftAsyncTaskInfoRaw {
    error: *const c_char,
    kind: c_uint,
    enqueue_priority: c_uint,
    is_child_task: bool,
    is_future: bool,
    is_group_child_task: bool,
    is_async_let_task: bool,
    is_synchronous_start_task: bool,
    max_priority: c_uint,
    is_cancelled: bool,
    is_status_record_locked: bool,
    is_escalated: bool,
    has_is_running: bool,
    is_running: bool,
    is_enqueued: bool,
    has_thread_port: bool,
    thread_port: u32,
    id: u64,
    run_job: SwiftAddr,
    allocator_slab_ptr: SwiftAddr,
    child_task_count: c_uint,
    child_tasks: *const SwiftAddr,
    async_backtrace_frames_count: c_uint,
    async_backtrace_frames: *const SwiftAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SwiftActorInfoRaw {
    error: *const c_char,
    state: u8,
    is_distributed_remote: bool,
    is_priority_escalated: bool,
    max_priority: u8,
    first_job: SwiftAddr,
    has_thread_port: bool,
    thread_port: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct SwiftTypeInfo {
    pub kind: i32,
    pub size: u32,
    pub alignment: u32,
    pub stride: u32,
    pub num_fields: u32,
}

#[derive(Clone, Debug)]
pub struct SwiftChildInfo {
    pub name: Option<String>,
    pub offset: u32,
    pub kind: i32,
    pub typeref: u64,
}

#[derive(Clone, Debug)]
pub struct SwiftAsyncTaskSlabInfo {
    pub error: Option<String>,
    pub slab_ptr: u64,
}

#[derive(Clone, Debug)]
pub struct SwiftAsyncTaskInfo {
    pub error: Option<String>,
    pub kind: u32,
    pub enqueue_priority: u32,
    pub is_child_task: bool,
    pub is_future: bool,
    pub is_group_child_task: bool,
    pub is_async_let_task: bool,
    pub is_synchronous_start_task: bool,
    pub max_priority: u32,
    pub is_cancelled: bool,
    pub is_status_record_locked: bool,
    pub is_escalated: bool,
    pub has_is_running: bool,
    pub is_running: bool,
    pub is_enqueued: bool,
    pub has_thread_port: bool,
    pub thread_port: u32,
    pub id: u64,
    pub run_job: u64,
    pub allocator_slab_ptr: u64,
    pub child_tasks: Vec<u64>,
    pub async_backtrace_frames: Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct SwiftActorInfo {
    pub error: Option<String>,
    pub state: u8,
    pub is_distributed_remote: bool,
    pub is_priority_escalated: bool,
    pub max_priority: u8,
    pub first_job: u64,
    pub has_thread_port: bool,
    pub thread_port: u32,
}

#[derive(Debug)]
pub enum RemoteMirrorError {
    OpenLibrary(String),
    ResolveSymbol { symbol: String, error: String },
    CommandError(String),
    ContextCreate(String),
    Dladdr(String),
    NullTypeRef,
    NullName,
}

pub struct RemoteMirrorContext<'a> {
    context: SwiftReflectionContextRef,
    destroy: DestroyReflectionContext,
    _api: &'a RemoteMirrorApi,
}

impl<'a> RemoteMirrorContext<'a> {
    pub fn as_raw(&self) -> SwiftReflectionContextRef {
        self.context
    }
}

impl Drop for RemoteMirrorContext<'_> {
    fn drop(&mut self) {
        if !self.context.is_null() {
            unsafe { (self.destroy)(self.context) };
            self.context = ptr::null_mut();
        }
    }
}

pub struct RemoteMirrorApi {
    handle: *mut c_void,
    library_path: String,
}

impl RemoteMirrorApi {
    pub fn new() -> Result<Self, RemoteMirrorError> {
        let candidates = [
            "/usr/lib/swift/libswiftRemoteMirror.dylib",
            "libswiftRemoteMirror.dylib",
        ];

        for candidate in candidates {
            let Ok(c_path) = CString::new(candidate) else {
                continue;
            };
            let handle = unsafe { dlopen(c_path.as_ptr(), RTLD_NOW | RTLD_GLOBAL) };
            if !handle.is_null() {
                return Ok(Self {
                    handle,
                    library_path: candidate.to_string(),
                });
            }
        }

        Err(RemoteMirrorError::OpenLibrary(last_dlerror()))
    }

    pub fn library_path(&self) -> &str {
        &self.library_path
    }

    pub fn symbol_address(&self, symbol: &str) -> Result<*mut c_void, RemoteMirrorError> {
        let mut ptr = resolve_symbol_raw(self.handle, symbol)?;
        if ptr.is_null() {
            let prefixed = format!("_{symbol}");
            ptr = resolve_symbol_raw(self.handle, &prefixed)?;
        }

        if ptr.is_null() {
            ptr = resolve_symbol_raw(RTLD_DEFAULT, symbol)?;
        }
        if ptr.is_null() {
            let prefixed = format!("_{symbol}");
            ptr = resolve_symbol_raw(RTLD_DEFAULT, &prefixed)?;
        }

        if ptr.is_null() {
            return Err(RemoteMirrorError::ResolveSymbol {
                symbol: symbol.to_string(),
                error: last_dlerror(),
            });
        }

        Ok(ptr)
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.symbol_address(symbol).is_ok()
    }

    pub fn supported_metadata_version(&self) -> Result<u16, RemoteMirrorError> {
        let f: GetSupportedMetadataVersion =
            self.resolve_symbol_any("swift_reflection_getSupportedMetadataVersion")?;
        Ok(unsafe { f() })
    }

    pub fn create_local_context(&self) -> Result<RemoteMirrorContext<'_>, RemoteMirrorError> {
        let create: CreateReflectionContext =
            self.resolve_symbol_any("swift_reflection_createReflectionContext")?;
        let destroy: DestroyReflectionContext =
            self.resolve_symbol_any("swift_reflection_destroyReflectionContext")?;

        let ptr_size = std::mem::size_of::<usize>() as u8;
        let context = unsafe {
            create(
                ptr::null_mut(),
                ptr_size,
                local_free_bytes,
                local_read_bytes,
                local_get_string_length,
                local_get_symbol_address,
            )
        };

        if context.is_null() {
            return Err(RemoteMirrorError::ContextCreate(
                "swift_reflection_createReflectionContext returned null".to_string(),
            ));
        }

        Ok(RemoteMirrorContext {
            context,
            destroy,
            _api: self,
        })
    }

    pub fn image_base_for_symbol(&self, symbol: &str) -> Result<usize, RemoteMirrorError> {
        let address = self.symbol_address(symbol)?;
        let mut info = DlInfo {
            dli_fname: ptr::null(),
            dli_fbase: ptr::null_mut(),
            dli_sname: ptr::null(),
            dli_saddr: ptr::null_mut(),
        };

        let ok = unsafe { dladdr(address as *const c_void, &mut info as *mut DlInfo) };
        if ok == 0 || info.dli_fbase.is_null() {
            return Err(RemoteMirrorError::Dladdr(format!(
                "dladdr failed to resolve image base for symbol {symbol}"
            )));
        }

        Ok(info.dli_fbase as usize)
    }

    pub fn add_image(
        &self,
        context: &RemoteMirrorContext<'_>,
        image_start: usize,
    ) -> Result<bool, RemoteMirrorError> {
        let f: AddImage = self.resolve_symbol_any("swift_reflection_addImage")?;
        Ok(unsafe { f(context.as_raw(), image_start as SwiftAddr) != 0 })
    }

    pub fn owns_address(
        &self,
        context: &RemoteMirrorContext<'_>,
        address: usize,
    ) -> Result<bool, RemoteMirrorError> {
        let f: OwnsAddress = self.resolve_symbol_any("swift_reflection_ownsAddress")?;
        Ok(unsafe { f(context.as_raw(), address as SwiftAddr) != 0 })
    }

    pub fn iterate_conformance_cache(
        &self,
        context: &RemoteMirrorContext<'_>,
    ) -> Result<Vec<(u64, u64)>, RemoteMirrorError> {
        let f: IterateConformanceCache =
            self.resolve_symbol_any("swift_reflection_iterateConformanceCache")?;
        let mut pairs: Vec<(u64, u64)> = Vec::new();
        let error_ptr = unsafe {
            f(
                context.as_raw(),
                conformance_cache_callback,
                &mut pairs as *mut Vec<(u64, u64)> as *mut c_void,
            )
        };
        if !error_ptr.is_null() {
            return Err(RemoteMirrorError::CommandError(
                unsafe { CStr::from_ptr(error_ptr) }
                    .to_string_lossy()
                    .into_owned(),
            ));
        }
        Ok(pairs)
    }

    pub fn async_task_slab_pointer(
        &self,
        context: &RemoteMirrorContext<'_>,
        async_task_ptr: usize,
    ) -> Result<SwiftAsyncTaskSlabInfo, RemoteMirrorError> {
        let f: AsyncTaskSlabPointer =
            self.resolve_symbol_any("swift_reflection_asyncTaskSlabPointer")?;
        let raw = unsafe { f(context.as_raw(), async_task_ptr as SwiftAddr) };
        Ok(SwiftAsyncTaskSlabInfo {
            error: copy_optional_cstr(raw.error),
            slab_ptr: raw.slab_ptr,
        })
    }

    pub fn async_task_info(
        &self,
        context: &RemoteMirrorContext<'_>,
        async_task_ptr: usize,
    ) -> Result<SwiftAsyncTaskInfo, RemoteMirrorError> {
        let f: AsyncTaskInfoFn = self.resolve_symbol_any("swift_reflection_asyncTaskInfo")?;
        let raw = unsafe { f(context.as_raw(), async_task_ptr as SwiftAddr) };
        let child_tasks = copy_u64_slice(raw.child_tasks, raw.child_task_count as usize);
        let async_backtrace_frames = copy_u64_slice(
            raw.async_backtrace_frames,
            raw.async_backtrace_frames_count as usize,
        );
        Ok(SwiftAsyncTaskInfo {
            error: copy_optional_cstr(raw.error),
            kind: raw.kind,
            enqueue_priority: raw.enqueue_priority,
            is_child_task: raw.is_child_task,
            is_future: raw.is_future,
            is_group_child_task: raw.is_group_child_task,
            is_async_let_task: raw.is_async_let_task,
            is_synchronous_start_task: raw.is_synchronous_start_task,
            max_priority: raw.max_priority,
            is_cancelled: raw.is_cancelled,
            is_status_record_locked: raw.is_status_record_locked,
            is_escalated: raw.is_escalated,
            has_is_running: raw.has_is_running,
            is_running: raw.is_running,
            is_enqueued: raw.is_enqueued,
            has_thread_port: raw.has_thread_port,
            thread_port: raw.thread_port,
            id: raw.id,
            run_job: raw.run_job,
            allocator_slab_ptr: raw.allocator_slab_ptr,
            child_tasks,
            async_backtrace_frames,
        })
    }

    pub fn actor_info(
        &self,
        context: &RemoteMirrorContext<'_>,
        actor_ptr: usize,
    ) -> Result<SwiftActorInfo, RemoteMirrorError> {
        let f: ActorInfoFn = self.resolve_symbol_any("swift_reflection_actorInfo")?;
        let raw = unsafe { f(context.as_raw(), actor_ptr as SwiftAddr) };
        Ok(SwiftActorInfo {
            error: copy_optional_cstr(raw.error),
            state: raw.state,
            is_distributed_remote: raw.is_distributed_remote,
            is_priority_escalated: raw.is_priority_escalated,
            max_priority: raw.max_priority,
            first_job: raw.first_job,
            has_thread_port: raw.has_thread_port,
            thread_port: raw.thread_port,
        })
    }

    pub fn next_job(
        &self,
        context: &RemoteMirrorContext<'_>,
        job_ptr: usize,
    ) -> Result<u64, RemoteMirrorError> {
        let f: NextJobFn = self.resolve_symbol_any("swift_reflection_nextJob")?;
        Ok(unsafe { f(context.as_raw(), job_ptr as SwiftAddr) })
    }

    pub fn type_ref_for_metadata(
        &self,
        context: &RemoteMirrorContext<'_>,
        metadata: usize,
    ) -> Result<u64, RemoteMirrorError> {
        let f: TypeRefForMetadata =
            self.resolve_symbol_any("swift_reflection_typeRefForMetadata")?;
        let tr = unsafe { f(context.as_raw(), metadata) };
        if tr == 0 {
            return Err(RemoteMirrorError::NullTypeRef);
        }
        Ok(tr)
    }

    pub fn type_ref_for_instance(
        &self,
        context: &RemoteMirrorContext<'_>,
        object: usize,
    ) -> Result<u64, RemoteMirrorError> {
        let f: TypeRefForInstance =
            self.resolve_symbol_any("swift_reflection_typeRefForInstance")?;
        let tr = unsafe { f(context.as_raw(), object) };
        if tr == 0 {
            return Err(RemoteMirrorError::NullTypeRef);
        }
        Ok(tr)
    }

    pub fn copy_name_for_type_ref(
        &self,
        context: &RemoteMirrorContext<'_>,
        typeref: u64,
        mangled: bool,
    ) -> Result<String, RemoteMirrorError> {
        let f: CopyNameForTypeRef =
            self.resolve_symbol_any("swift_reflection_copyNameForTypeRef")?;
        let ptr = unsafe { f(context.as_raw(), typeref as SwiftAddr, mangled) };
        if ptr.is_null() {
            return Err(RemoteMirrorError::NullName);
        }
        let name = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { libc::free(ptr as *mut c_void) };
        Ok(name)
    }

    pub fn info_for_metadata(
        &self,
        context: &RemoteMirrorContext<'_>,
        metadata: usize,
    ) -> Result<SwiftTypeInfo, RemoteMirrorError> {
        let f: InfoForMetadata = self.resolve_symbol_any("swift_reflection_infoForMetadata")?;
        let raw = unsafe { f(context.as_raw(), metadata) };
        Ok(SwiftTypeInfo {
            kind: raw.kind,
            size: raw.size,
            alignment: raw.alignment,
            stride: raw.stride,
            num_fields: raw.num_fields,
        })
    }

    pub fn info_for_instance(
        &self,
        context: &RemoteMirrorContext<'_>,
        object: usize,
    ) -> Result<SwiftTypeInfo, RemoteMirrorError> {
        let f: InfoForInstance = self.resolve_symbol_any("swift_reflection_infoForInstance")?;
        let raw = unsafe { f(context.as_raw(), object) };
        Ok(SwiftTypeInfo {
            kind: raw.kind,
            size: raw.size,
            alignment: raw.alignment,
            stride: raw.stride,
            num_fields: raw.num_fields,
        })
    }

    pub fn child_of_metadata(
        &self,
        context: &RemoteMirrorContext<'_>,
        metadata: usize,
        index: u32,
    ) -> Result<SwiftChildInfo, RemoteMirrorError> {
        let f: ChildOfMetadata = self.resolve_symbol_any("swift_reflection_childOfMetadata")?;
        let raw = unsafe { f(context.as_raw(), metadata, index) };
        Ok(convert_child(raw))
    }

    pub fn child_of_instance(
        &self,
        context: &RemoteMirrorContext<'_>,
        object: usize,
        index: u32,
    ) -> Result<SwiftChildInfo, RemoteMirrorError> {
        let f: ChildOfInstance = self.resolve_symbol_any("swift_reflection_childOfInstance")?;
        let raw = unsafe { f(context.as_raw(), object, index) };
        Ok(convert_child(raw))
    }

    pub fn required_symbol_report(&self) -> Vec<(&'static str, bool)> {
        required_remote_mirror_symbols()
            .iter()
            .copied()
            .map(|name| (name, self.has_symbol(name)))
            .collect()
    }

    pub fn exported_reflection_symbols(&self) -> Result<Vec<String>, RemoteMirrorError> {
        let output = Command::new("nm")
            .args(["-gU", &self.library_path])
            .output()
            .map_err(|err| {
                RemoteMirrorError::CommandError(format!(
                    "failed to execute nm for RemoteMirror: {err}"
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(RemoteMirrorError::CommandError(format!(
                "nm failed for {}: {}",
                self.library_path, stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut symbols = Vec::new();
        for line in stdout.lines() {
            if !line.contains(" T ") {
                continue;
            }
            if let Some(symbol) = line.split_whitespace().last() {
                if symbol.starts_with("_swift_reflection_") {
                    symbols.push(symbol.to_string());
                }
            }
        }
        symbols.sort();
        symbols.dedup();
        Ok(symbols)
    }

    fn resolve_symbol_any<T: Sized + Copy>(&self, symbol: &str) -> Result<T, RemoteMirrorError> {
        let ptr = self.symbol_address(symbol)?;
        Ok(unsafe { std::mem::transmute_copy(&ptr) })
    }
}

impl Drop for RemoteMirrorApi {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            let _ = unsafe { dlclose(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

fn resolve_symbol_raw(handle: *mut c_void, symbol: &str) -> Result<*mut c_void, RemoteMirrorError> {
    let c_symbol = CString::new(symbol).map_err(|_| RemoteMirrorError::ResolveSymbol {
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

unsafe extern "C" fn local_free_bytes(
    _reader_context: *mut c_void,
    bytes: *const c_void,
    _context: *mut c_void,
) {
    if !bytes.is_null() {
        unsafe { libc::free(bytes as *mut c_void) };
    }
}

unsafe extern "C" fn local_read_bytes(
    _reader_context: *mut c_void,
    address: SwiftAddr,
    size: u64,
    out_free_context: *mut *mut c_void,
) -> *const c_void {
    if !out_free_context.is_null() {
        unsafe { *out_free_context = ptr::null_mut() };
    }

    if address == 0 {
        return ptr::null();
    }

    let alloc_size = size.max(1) as usize;
    let dst = unsafe { libc::malloc(alloc_size) };
    if dst.is_null() {
        return ptr::null();
    }

    if size > 0 {
        unsafe { ptr::copy_nonoverlapping(address as *const u8, dst as *mut u8, size as usize) };
    }

    dst as *const c_void
}

unsafe extern "C" fn local_get_string_length(
    _reader_context: *mut c_void,
    address: SwiftAddr,
) -> u64 {
    if address == 0 {
        return 0;
    }

    const MAX_SCAN: usize = 64 * 1024;
    let mut len = 0usize;
    while len < MAX_SCAN {
        let byte = unsafe { *((address as *const u8).add(len)) };
        if byte == 0 {
            return len as u64;
        }
        len += 1;
    }
    0
}

unsafe extern "C" fn local_get_symbol_address(
    _reader_context: *mut c_void,
    name: *const c_char,
    name_length: u64,
) -> SwiftAddr {
    if name.is_null() || name_length == 0 {
        return 0;
    }

    let bytes = unsafe { std::slice::from_raw_parts(name as *const u8, name_length as usize) };
    let Ok(mut sym) = String::from_utf8(bytes.to_vec()) else {
        return 0;
    };

    // RemoteMirror can ask for symbols with or without leading underscore.
    let mut resolved = resolve_symbol_default(&sym);
    if resolved.is_null() {
        if sym.starts_with('_') {
            sym.remove(0);
            resolved = resolve_symbol_default(&sym);
        } else {
            let prefixed = format!("_{sym}");
            resolved = resolve_symbol_default(&prefixed);
        }
    }

    resolved as SwiftAddr
}

fn resolve_symbol_default(symbol: &str) -> *mut c_void {
    let Ok(c_symbol) = CString::new(symbol) else {
        return ptr::null_mut();
    };
    unsafe { dlsym(RTLD_DEFAULT, c_symbol.as_ptr()) }
}

unsafe extern "C" fn conformance_cache_callback(
    type_ptr: SwiftAddr,
    proto_ptr: SwiftAddr,
    context_ptr: *mut c_void,
) {
    if context_ptr.is_null() {
        return;
    }
    let pairs = unsafe { &mut *(context_ptr as *mut Vec<(u64, u64)>) };
    pairs.push((type_ptr, proto_ptr));
}

fn copy_optional_cstr(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}

fn copy_u64_slice(ptr: *const SwiftAddr, len: usize) -> Vec<u64> {
    if ptr.is_null() || len == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec()
    }
}

fn convert_child(raw: SwiftChildInfoRaw) -> SwiftChildInfo {
    let name = if raw.name.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(raw.name) }
                .to_string_lossy()
                .into_owned(),
        )
    };

    SwiftChildInfo {
        name,
        offset: raw.offset,
        kind: raw.kind,
        typeref: raw.tr,
    }
}

pub fn required_remote_mirror_symbols() -> &'static [&'static str] {
    &[
        "swift_reflection_createReflectionContext",
        "swift_reflection_destroyReflectionContext",
        "swift_reflection_getSupportedMetadataVersion",
        "swift_reflection_addImage",
        "swift_reflection_infoForMetadata",
        "swift_reflection_infoForInstance",
        "swift_reflection_childOfInstance",
        "swift_reflection_childOfMetadata",
        "swift_reflection_iterateConformanceCache",
        "swift_reflection_asyncTaskInfo",
        "swift_reflection_nextJob",
        "swift_reflection_actorInfo",
        "swift_reflection_dumpInfoForTypeRef",
        "swift_reflection_demangle",
    ]
}
