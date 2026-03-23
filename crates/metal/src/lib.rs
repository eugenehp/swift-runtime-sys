//! Apple Metal — GPU programming from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 8+, tvOS 9+, visionOS 1+.
//!
//! Provides safe Rust wrappers around Metal for GPU compute, rendering,
//! buffer/texture management, and shader compilation.
//!
//! # Quick start — GPU compute
//!
//! ```ignore
//! use metal::*;
//!
//! let gpu = Device::system_default().unwrap();
//! println!("GPU: {}", gpu.name());
//!
//! let source = r#"
//!     #include <metal_stdlib>
//!     kernel void double_values(
//!         device float* data [[buffer(0)]],
//!         uint id [[thread_position_in_grid]]
//!     ) { data[id] *= 2.0; }
//! "#;
//!
//! let lib = gpu.library_from_source(source).unwrap();
//! let func = lib.function("double_values").unwrap();
//! let pipeline = gpu.compute_pipeline(&func).unwrap();
//!
//! let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
//! let buf = gpu.buffer_with_data(&data, ResourceOptions::STORAGE_SHARED);
//!
//! let queue = gpu.command_queue();
//! let cmd = queue.command_buffer();
//! let enc = cmd.compute_encoder();
//! enc.set_pipeline(&pipeline);
//! enc.set_buffer(&buf, 0, 0);
//! enc.dispatch_threads(4, 1, 1, pipeline.max_threads().min(4), 1, 1);
//! enc.end();
//! cmd.commit();
//! cmd.wait();
//!
//! let result = buf.as_slice::<f32>();
//! assert_eq!(result, &[2.0, 4.0, 6.0, 8.0]);
//! ```

#![allow(non_snake_case)]

use core::ffi::{c_char, c_void};
use std::ffi::CString;
use std::sync::OnceLock;

type Handle = *mut c_void;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym_from(h: *mut c_void, name: &core::ffi::CStr) -> *mut c_void {
    unsafe { dlsym(h, name.as_ptr()) }
}

// ── Bridge loader ──

static BRIDGE: OnceLock<BridgeFns> = OnceLock::new();

struct BridgeFns {
    // Device
    create_device: unsafe extern "C" fn() -> Handle,
    device_name: unsafe extern "C" fn(Handle, *mut u8, usize) -> usize,
    device_max_threads: unsafe extern "C" fn(Handle) -> usize,
    device_has_unified_memory: unsafe extern "C" fn(Handle) -> bool,
    device_max_buffer_length: unsafe extern "C" fn(Handle) -> usize,
    device_supports_family: unsafe extern "C" fn(Handle, usize) -> bool,
    device_supports_raytracing: unsafe extern "C" fn(Handle) -> bool,
    // Command queue
    make_command_queue: unsafe extern "C" fn(Handle) -> Handle,
    // Command buffer
    queue_command_buffer: unsafe extern "C" fn(Handle) -> Handle,
    command_buffer_commit: unsafe extern "C" fn(Handle),
    command_buffer_wait: unsafe extern "C" fn(Handle),
    command_buffer_status: unsafe extern "C" fn(Handle) -> usize,
    command_buffer_gpu_start: unsafe extern "C" fn(Handle) -> f64,
    command_buffer_gpu_end: unsafe extern "C" fn(Handle) -> f64,
    // Buffer
    make_buffer: unsafe extern "C" fn(Handle, usize, u64) -> Handle,
    make_buffer_bytes: unsafe extern "C" fn(Handle, *const c_void, usize, u64) -> Handle,
    buffer_contents: unsafe extern "C" fn(Handle) -> *mut c_void,
    buffer_length: unsafe extern "C" fn(Handle) -> usize,
    // Texture
    make_texture: unsafe extern "C" fn(Handle, u64, u64, usize, usize, usize, usize, usize, u64, u64) -> Handle,
    texture_width: unsafe extern "C" fn(Handle) -> usize,
    texture_height: unsafe extern "C" fn(Handle) -> usize,
    texture_pixel_format: unsafe extern "C" fn(Handle) -> u64,
    texture_replace_region: unsafe extern "C" fn(Handle, usize, usize, usize, usize, usize, usize, *const c_void),
    // Library
    make_default_library: unsafe extern "C" fn(Handle) -> Handle,
    make_library_source: unsafe extern "C" fn(Handle, *const u8, usize, *mut u8, usize) -> Handle,
    library_make_function: unsafe extern "C" fn(Handle, *const u8, usize) -> Handle,
    library_function_names: unsafe extern "C" fn(Handle, *mut u8, usize) -> usize,
    // Compute pipeline
    make_compute_pipeline: unsafe extern "C" fn(Handle, Handle, *mut u8, usize) -> Handle,
    compute_pipeline_max_threads: unsafe extern "C" fn(Handle) -> usize,
    compute_pipeline_thread_width: unsafe extern "C" fn(Handle) -> usize,
    // Compute encoder
    compute_encoder: unsafe extern "C" fn(Handle) -> Handle,
    compute_set_pipeline: unsafe extern "C" fn(Handle, Handle),
    compute_set_buffer: unsafe extern "C" fn(Handle, Handle, usize, usize),
    compute_set_bytes: unsafe extern "C" fn(Handle, *const c_void, usize, usize),
    compute_set_texture: unsafe extern "C" fn(Handle, Handle, usize),
    compute_dispatch_threads: unsafe extern "C" fn(Handle, usize, usize, usize, usize, usize, usize),
    compute_dispatch_threadgroups: unsafe extern "C" fn(Handle, usize, usize, usize, usize, usize, usize),
    compute_end: unsafe extern "C" fn(Handle),
    // Render pipeline
    make_render_pipeline: unsafe extern "C" fn(Handle, Handle, Handle, u64, *mut u8, usize) -> Handle,
    // Blit encoder
    blit_encoder: unsafe extern "C" fn(Handle) -> Handle,
    blit_copy_buffer: unsafe extern "C" fn(Handle, Handle, usize, Handle, usize, usize),
    blit_fill_buffer: unsafe extern "C" fn(Handle, Handle, usize, usize, u8),
    blit_end: unsafe extern "C" fn(Handle),
    // Release
    release: unsafe extern "C" fn(Handle),
}

fn load_bridge(dylib_path: &str) -> BridgeFns {
    let cpath = CString::new(dylib_path).unwrap();
    let h = unsafe { dlopen(cpath.as_ptr(), 2) };
    assert!(!h.is_null(), "Failed to load Metal bridge: {dylib_path}");

    macro_rules! f {
        ($name:expr) => {
            unsafe { std::mem::transmute(sym_from(h, $name)) }
        };
    }

    BridgeFns {
        create_device: f!(c"mtl_create_system_default_device"),
        device_name: f!(c"mtl_device_name"),
        device_max_threads: f!(c"mtl_device_max_threads_per_threadgroup"),
        device_has_unified_memory: f!(c"mtl_device_has_unified_memory"),
        device_max_buffer_length: f!(c"mtl_device_max_buffer_length"),
        device_supports_family: f!(c"mtl_device_supports_family"),
        device_supports_raytracing: f!(c"mtl_device_supports_raytracing"),
        make_command_queue: f!(c"mtl_device_make_command_queue"),
        queue_command_buffer: f!(c"mtl_command_queue_command_buffer"),
        command_buffer_commit: f!(c"mtl_command_buffer_commit"),
        command_buffer_wait: f!(c"mtl_command_buffer_wait"),
        command_buffer_status: f!(c"mtl_command_buffer_status"),
        command_buffer_gpu_start: f!(c"mtl_command_buffer_gpu_start_time"),
        command_buffer_gpu_end: f!(c"mtl_command_buffer_gpu_end_time"),
        make_buffer: f!(c"mtl_device_make_buffer"),
        make_buffer_bytes: f!(c"mtl_device_make_buffer_with_bytes"),
        buffer_contents: f!(c"mtl_buffer_contents"),
        buffer_length: f!(c"mtl_buffer_length"),
        make_texture: f!(c"mtl_device_make_texture"),
        texture_width: f!(c"mtl_texture_width"),
        texture_height: f!(c"mtl_texture_height"),
        texture_pixel_format: f!(c"mtl_texture_pixel_format"),
        texture_replace_region: f!(c"mtl_texture_replace_region"),
        make_default_library: f!(c"mtl_device_make_default_library"),
        make_library_source: f!(c"mtl_device_make_library_source"),
        library_make_function: f!(c"mtl_library_make_function"),
        library_function_names: f!(c"mtl_library_function_names"),
        make_compute_pipeline: f!(c"mtl_device_make_compute_pipeline"),
        compute_pipeline_max_threads: f!(c"mtl_compute_pipeline_max_threads"),
        compute_pipeline_thread_width: f!(c"mtl_compute_pipeline_thread_execution_width"),
        compute_encoder: f!(c"mtl_command_buffer_compute_encoder"),
        compute_set_pipeline: f!(c"mtl_compute_encoder_set_pipeline"),
        compute_set_buffer: f!(c"mtl_compute_encoder_set_buffer"),
        compute_set_bytes: f!(c"mtl_compute_encoder_set_bytes"),
        compute_set_texture: f!(c"mtl_compute_encoder_set_texture"),
        compute_dispatch_threads: f!(c"mtl_compute_encoder_dispatch_threads"),
        compute_dispatch_threadgroups: f!(c"mtl_compute_encoder_dispatch_threadgroups"),
        compute_end: f!(c"mtl_compute_encoder_end"),
        make_render_pipeline: f!(c"mtl_device_make_render_pipeline"),
        blit_encoder: f!(c"mtl_command_buffer_blit_encoder"),
        blit_copy_buffer: f!(c"mtl_blit_encoder_copy_buffer"),
        blit_fill_buffer: f!(c"mtl_blit_encoder_fill_buffer"),
        blit_end: f!(c"mtl_blit_encoder_end"),
        release: f!(c"mtl_release"),
    }
}

fn fns() -> &'static BridgeFns {
    BRIDGE.get().expect(
        "Metal bridge not loaded. Call metal::load(\"path/to/libMetalBridge.dylib\") first.",
    )
}

/// Load the Metal bridge dylib. Must be called before any other Metal function.
pub fn load(dylib_path: &str) {
    BRIDGE.get_or_init(|| load_bridge(dylib_path));
}

/// Auto-find and load the Metal bridge from common paths.
pub fn auto_load() {
    let candidates = [
        "libMetalBridge.dylib",
        "crates/metal/libMetalBridge.dylib",
        "../../crates/metal/libMetalBridge.dylib",
    ];
    for c in candidates {
        if std::path::Path::new(c).exists() {
            load(c);
            return;
        }
    }
    // Try next to executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("libMetalBridge.dylib");
            if p.exists() {
                load(p.to_str().unwrap());
                return;
            }
        }
    }
    panic!("libMetalBridge.dylib not found. Build it:\n  cd crates/metal && swiftc -emit-library -o libMetalBridge.dylib MetalBridge.swift -framework Metal -framework CoreGraphics");
}

// ── Resource options ──

/// Metal resource storage options.
pub struct ResourceOptions;

impl ResourceOptions {
    pub const STORAGE_SHARED: u64 = 0;
    pub const STORAGE_MANAGED: u64 = 0x10;
    pub const STORAGE_PRIVATE: u64 = 0x20;
    pub const CPU_CACHE_DEFAULT: u64 = 0;
    pub const CPU_CACHE_WRITE_COMBINED: u64 = 0x100;
    pub const HAZARD_TRACKING_DEFAULT: u64 = 0;
    pub const HAZARD_TRACKING_UNTRACKED: u64 = 0x1_0000;
    pub const HAZARD_TRACKING_TRACKED: u64 = 0x2_0000;
}

/// Pixel formats (common subset).
pub struct PixelFormat;

impl PixelFormat {
    pub const BGRA8_UNORM: u64 = 80;
    pub const BGRA8_UNORM_SRGB: u64 = 81;
    pub const RGBA8_UNORM: u64 = 70;
    pub const RGBA8_UNORM_SRGB: u64 = 71;
    pub const RGBA16_FLOAT: u64 = 115;
    pub const RGBA32_FLOAT: u64 = 125;
    pub const R8_UNORM: u64 = 10;
    pub const R16_FLOAT: u64 = 25;
    pub const R32_FLOAT: u64 = 55;
    pub const DEPTH32_FLOAT: u64 = 252;
    pub const DEPTH32_FLOAT_STENCIL8: u64 = 260;
}

/// GPU family identifiers.
pub struct GPUFamily;

impl GPUFamily {
    pub const APPLE1: usize = 1001;
    pub const APPLE2: usize = 1002;
    pub const APPLE3: usize = 1003;
    pub const APPLE4: usize = 1004;
    pub const APPLE5: usize = 1005;
    pub const APPLE6: usize = 1006;
    pub const APPLE7: usize = 1007;
    pub const APPLE8: usize = 1008;
    pub const APPLE9: usize = 1009;
    pub const COMMON1: usize = 3001;
    pub const COMMON2: usize = 3002;
    pub const COMMON3: usize = 3003;
    pub const METAL3: usize = 5001;
}

/// Command buffer execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

impl From<usize> for CommandBufferStatus {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::NotEnqueued,
            1 => Self::Enqueued,
            2 => Self::Committed,
            3 => Self::Scheduled,
            4 => Self::Completed,
            _ => Self::Error,
        }
    }
}

// ── Device ──

/// A Metal GPU device.
pub struct Device {
    handle: Handle,
}

impl Device {
    /// Get the system default Metal device.
    pub fn system_default() -> Option<Self> {
        let h = unsafe { (fns().create_device)() };
        if h.is_null() { None } else { Some(Self { handle: h }) }
    }

    /// GPU name.
    pub fn name(&self) -> String {
        let mut buf = vec![0u8; 256];
        let len = unsafe { (fns().device_name)(self.handle, buf.as_mut_ptr(), buf.len()) };
        String::from_utf8_lossy(&buf[..len]).to_string()
    }

    /// Maximum threads per threadgroup (width dimension).
    pub fn max_threads_per_threadgroup(&self) -> usize {
        unsafe { (fns().device_max_threads)(self.handle) }
    }

    /// Whether the device has unified memory (Apple Silicon).
    pub fn has_unified_memory(&self) -> bool {
        unsafe { (fns().device_has_unified_memory)(self.handle) }
    }

    /// Maximum buffer size in bytes.
    pub fn max_buffer_length(&self) -> usize {
        unsafe { (fns().device_max_buffer_length)(self.handle) }
    }

    /// Check GPU family support.
    pub fn supports_family(&self, family: usize) -> bool {
        unsafe { (fns().device_supports_family)(self.handle, family) }
    }

    /// Whether the device supports ray tracing.
    pub fn supports_raytracing(&self) -> bool {
        unsafe { (fns().device_supports_raytracing)(self.handle) }
    }

    /// Create a command queue.
    pub fn command_queue(&self) -> CommandQueue {
        let h = unsafe { (fns().make_command_queue)(self.handle) };
        assert!(!h.is_null(), "Failed to create command queue");
        CommandQueue { handle: h }
    }

    /// Create a buffer of the given size.
    pub fn buffer(&self, length: usize, options: u64) -> Buffer {
        let h = unsafe { (fns().make_buffer)(self.handle, length, options) };
        assert!(!h.is_null(), "Failed to create buffer");
        Buffer { handle: h, len: length }
    }

    /// Create a buffer initialized with data.
    pub fn buffer_with_data<T: Copy>(&self, data: &[T], options: u64) -> Buffer {
        let len = data.len() * std::mem::size_of::<T>();
        let h = unsafe {
            (fns().make_buffer_bytes)(self.handle, data.as_ptr() as *const c_void, len, options)
        };
        assert!(!h.is_null(), "Failed to create buffer");
        Buffer { handle: h, len }
    }

    /// Create a 2D texture.
    pub fn texture_2d(&self, width: usize, height: usize, pixel_format: u64, usage: u64) -> Texture {
        let h = unsafe {
            (fns().make_texture)(
                self.handle,
                2, // MTLTextureType2D
                pixel_format,
                width, height, 1,
                1, 1, // mip levels, sample count
                usage,
                0, // storage mode default
            )
        };
        assert!(!h.is_null(), "Failed to create texture");
        Texture { handle: h }
    }

    /// Load the default Metal library (.metallib bundled with the app).
    pub fn default_library(&self) -> Option<Library> {
        let h = unsafe { (fns().make_default_library)(self.handle) };
        if h.is_null() { None } else { Some(Library { handle: h }) }
    }

    /// Compile a Metal shader library from source code.
    pub fn library_from_source(&self, source: &str) -> Result<Library, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            (fns().make_library_source)(
                self.handle,
                source.as_ptr(),
                source.len(),
                err_buf.as_mut_ptr(),
                err_buf.len(),
            )
        };
        if h.is_null() {
            let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..err_len]).to_string())
        } else {
            Ok(Library { handle: h })
        }
    }

    /// Create a compute pipeline from a function.
    pub fn compute_pipeline(&self, function: &Function) -> Result<ComputePipeline, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            (fns().make_compute_pipeline)(
                self.handle,
                function.handle,
                err_buf.as_mut_ptr(),
                err_buf.len(),
            )
        };
        if h.is_null() {
            let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..err_len]).to_string())
        } else {
            Ok(ComputePipeline { handle: h })
        }
    }

    /// Create a render pipeline.
    pub fn render_pipeline(
        &self,
        vertex: &Function,
        fragment: &Function,
        pixel_format: u64,
    ) -> Result<RenderPipeline, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            (fns().make_render_pipeline)(
                self.handle,
                vertex.handle,
                fragment.handle,
                pixel_format,
                err_buf.as_mut_ptr(),
                err_buf.len(),
            )
        };
        if h.is_null() {
            let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..err_len]).to_string())
        } else {
            Ok(RenderPipeline { handle: h })
        }
    }

    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Command Queue ──

pub struct CommandQueue {
    handle: Handle,
}

impl CommandQueue {
    /// Create a command buffer.
    pub fn command_buffer(&self) -> CommandBuffer {
        let h = unsafe { (fns().queue_command_buffer)(self.handle) };
        assert!(!h.is_null(), "Failed to create command buffer");
        CommandBuffer { handle: h }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Command Buffer ──

pub struct CommandBuffer {
    handle: Handle,
}

impl CommandBuffer {
    /// Commit the command buffer for execution.
    pub fn commit(&self) {
        unsafe { (fns().command_buffer_commit)(self.handle) }
    }

    /// Wait until the GPU has finished executing.
    pub fn wait(&self) {
        unsafe { (fns().command_buffer_wait)(self.handle) }
    }

    /// Current execution status.
    pub fn status(&self) -> CommandBufferStatus {
        let s = unsafe { (fns().command_buffer_status)(self.handle) };
        CommandBufferStatus::from(s)
    }

    /// GPU start time (seconds since boot).
    pub fn gpu_start_time(&self) -> f64 {
        unsafe { (fns().command_buffer_gpu_start)(self.handle) }
    }

    /// GPU end time (seconds since boot).
    pub fn gpu_end_time(&self) -> f64 {
        unsafe { (fns().command_buffer_gpu_end)(self.handle) }
    }

    /// GPU execution duration in seconds.
    pub fn gpu_duration(&self) -> f64 {
        self.gpu_end_time() - self.gpu_start_time()
    }

    /// Create a compute command encoder.
    pub fn compute_encoder(&self) -> ComputeEncoder {
        let h = unsafe { (fns().compute_encoder)(self.handle) };
        assert!(!h.is_null(), "Failed to create compute encoder");
        ComputeEncoder { handle: h }
    }

    /// Create a blit command encoder.
    pub fn blit_encoder(&self) -> BlitEncoder {
        let h = unsafe { (fns().blit_encoder)(self.handle) };
        assert!(!h.is_null(), "Failed to create blit encoder");
        BlitEncoder { handle: h }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Buffer ──

pub struct Buffer {
    handle: Handle,
    len: usize,
}

impl Buffer {
    /// Raw pointer to buffer contents (CPU-accessible for shared/managed).
    pub fn contents(&self) -> *mut c_void {
        unsafe { (fns().buffer_contents)(self.handle) }
    }

    /// Buffer length in bytes.
    pub fn length(&self) -> usize {
        unsafe { (fns().buffer_length)(self.handle) }
    }

    /// View buffer contents as a typed slice.
    ///
    /// # Safety
    /// The buffer must contain valid data of type `T` and be CPU-accessible.
    pub fn as_slice<T: Copy>(&self) -> &[T] {
        let ptr = self.contents() as *const T;
        let count = self.len / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts(ptr, count) }
    }

    /// View buffer contents as a mutable typed slice.
    ///
    /// # Safety
    /// The buffer must be CPU-accessible and not in use by the GPU.
    pub fn as_mut_slice<T: Copy>(&self) -> &mut [T] {
        let ptr = self.contents() as *mut T;
        let count = self.len / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts_mut(ptr, count) }
    }

    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Texture ──

pub struct Texture {
    handle: Handle,
}

impl Texture {
    pub fn width(&self) -> usize { unsafe { (fns().texture_width)(self.handle) } }
    pub fn height(&self) -> usize { unsafe { (fns().texture_height)(self.handle) } }
    pub fn pixel_format(&self) -> u64 { unsafe { (fns().texture_pixel_format)(self.handle) } }

    /// Upload pixel data to a region of the texture.
    pub fn replace_region(&self, x: usize, y: usize, w: usize, h: usize, mip: usize, bytes_per_row: usize, data: &[u8]) {
        unsafe {
            (fns().texture_replace_region)(self.handle, x, y, w, h, mip, bytes_per_row, data.as_ptr() as *const c_void)
        }
    }

    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Library ──

pub struct Library {
    handle: Handle,
}

impl Library {
    /// Get a function by name.
    pub fn function(&self, name: &str) -> Option<Function> {
        let h = unsafe { (fns().library_make_function)(self.handle, name.as_ptr(), name.len()) };
        if h.is_null() { None } else { Some(Function { handle: h }) }
    }

    /// List all function names in the library.
    pub fn function_names(&self) -> Vec<String> {
        let mut buf = vec![0u8; 8192];
        let len = unsafe { (fns().library_function_names)(self.handle, buf.as_mut_ptr(), buf.len()) };
        if len == 0 {
            return vec![];
        }
        String::from_utf8_lossy(&buf[..len])
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Function ──

pub struct Function {
    handle: Handle,
}

impl Function {
    pub fn raw_handle(&self) -> Handle { self.handle }
}

impl Drop for Function {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Compute Pipeline ──

pub struct ComputePipeline {
    handle: Handle,
}

impl ComputePipeline {
    /// Maximum number of threads per threadgroup for this pipeline.
    pub fn max_threads(&self) -> usize {
        unsafe { (fns().compute_pipeline_max_threads)(self.handle) }
    }

    /// Thread execution width (SIMD width).
    pub fn thread_execution_width(&self) -> usize {
        unsafe { (fns().compute_pipeline_thread_width)(self.handle) }
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Render Pipeline ──

pub struct RenderPipeline {
    handle: Handle,
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe { (fns().release)(self.handle) }
    }
}

// ── Compute Encoder ──

pub struct ComputeEncoder {
    handle: Handle,
}

impl ComputeEncoder {
    /// Set the compute pipeline.
    pub fn set_pipeline(&self, pipeline: &ComputePipeline) {
        unsafe { (fns().compute_set_pipeline)(self.handle, pipeline.handle) }
    }

    /// Bind a buffer at an index.
    pub fn set_buffer(&self, buffer: &Buffer, offset: usize, index: usize) {
        unsafe { (fns().compute_set_buffer)(self.handle, buffer.handle, offset, index) }
    }

    /// Set inline bytes at an index.
    pub fn set_bytes<T: Copy>(&self, data: &[T], index: usize) {
        let len = data.len() * std::mem::size_of::<T>();
        unsafe { (fns().compute_set_bytes)(self.handle, data.as_ptr() as *const c_void, len, index) }
    }

    /// Bind a texture at an index.
    pub fn set_texture(&self, texture: &Texture, index: usize) {
        unsafe { (fns().compute_set_texture)(self.handle, texture.handle, index) }
    }

    /// Dispatch compute threads (non-uniform).
    pub fn dispatch_threads(
        &self,
        grid_x: usize, grid_y: usize, grid_z: usize,
        group_x: usize, group_y: usize, group_z: usize,
    ) {
        unsafe {
            (fns().compute_dispatch_threads)(self.handle, grid_x, grid_y, grid_z, group_x, group_y, group_z)
        }
    }

    /// Dispatch compute threadgroups.
    pub fn dispatch_threadgroups(
        &self,
        groups_x: usize, groups_y: usize, groups_z: usize,
        threads_x: usize, threads_y: usize, threads_z: usize,
    ) {
        unsafe {
            (fns().compute_dispatch_threadgroups)(self.handle, groups_x, groups_y, groups_z, threads_x, threads_y, threads_z)
        }
    }

    /// End encoding.
    pub fn end(&self) {
        unsafe { (fns().compute_end)(self.handle) }
    }
}

// ── Blit Encoder ──

pub struct BlitEncoder {
    handle: Handle,
}

impl BlitEncoder {
    /// Copy between buffers.
    pub fn copy_buffer(
        &self,
        src: &Buffer, src_offset: usize,
        dst: &Buffer, dst_offset: usize,
        size: usize,
    ) {
        unsafe {
            (fns().blit_copy_buffer)(self.handle, src.handle, src_offset, dst.handle, dst_offset, size)
        }
    }

    /// Fill a buffer region with a byte value.
    pub fn fill_buffer(&self, buffer: &Buffer, offset: usize, size: usize, value: u8) {
        unsafe { (fns().blit_fill_buffer)(self.handle, buffer.handle, offset, size, value) }
    }

    /// End encoding.
    pub fn end(&self) {
        unsafe { (fns().blit_end)(self.handle) }
    }
}

// Note: Metal is not available on watchOS. All functions will panic at runtime
// if the bridge is loaded on watchOS (which shouldn't happen).
