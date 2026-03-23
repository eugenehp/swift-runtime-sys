//! Metal GPU integration — device, command queue, pipelines, buffers, rendering.

use core::ffi::c_void;
use crate::{fns, Handle, MetalDrawable};

/// Common Metal pixel formats.
pub mod pixel_format {
    pub const BGRA8_UNORM: u64 = 80;
    pub const BGRA8_UNORM_SRGB: u64 = 81;
    pub const RGBA8_UNORM: u64 = 70;
    pub const RGBA16_FLOAT: u64 = 115;
    pub const RGBA32_FLOAT: u64 = 125;
    pub const R8_UNORM: u64 = 10;
    pub const R16_FLOAT: u64 = 25;
    pub const R32_FLOAT: u64 = 55;
    pub const DEPTH32_FLOAT: u64 = 252;
}

/// Metal resource storage options.
pub mod resource_options {
    pub const SHARED: u64 = 0;
    pub const MANAGED: u64 = 0x10;
    pub const PRIVATE: u64 = 0x20;
}

/// The Metal GPU device.
pub struct MetalDevice {
    pub(crate) handle: Handle,
}

impl MetalDevice {
    /// GPU name.
    pub fn name(&self) -> String {
        let mut buf = vec![0u8; 256];
        let len = unsafe { (fns().metal_device_name)(buf.as_mut_ptr(), buf.len()) };
        String::from_utf8_lossy(&buf[..len]).to_string()
    }

    /// Create a command queue.
    pub fn command_queue(&self) -> CommandQueue {
        let h = unsafe { (fns().metal_create_command_queue)() };
        assert!(!h.is_null(), "Failed to create command queue");
        CommandQueue { handle: h }
    }

    /// Compile a shader library from Metal Shading Language source.
    pub fn library_from_source(&self, source: &str) -> Result<ShaderLibrary, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            (fns().metal_make_library)(
                source.as_ptr(), source.len(),
                err_buf.as_mut_ptr(), err_buf.len(),
            )
        };
        if h.is_null() {
            let len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..len]).to_string())
        } else {
            Ok(ShaderLibrary { handle: h })
        }
    }

    /// Create a GPU buffer of the given size.
    pub fn buffer(&self, length: usize, options: u64) -> MetalBuffer {
        let h = unsafe { (fns().metal_make_buffer)(length, options) };
        assert!(!h.is_null(), "Failed to create buffer");
        MetalBuffer { handle: h, len: length }
    }

    /// Create a GPU buffer initialized with data.
    pub fn buffer_with_data<T: Copy>(&self, data: &[T], options: u64) -> MetalBuffer {
        let len = data.len() * std::mem::size_of::<T>();
        let h = unsafe {
            (fns().metal_make_buffer_data)(data.as_ptr() as *const c_void, len, options)
        };
        assert!(!h.is_null(), "Failed to create buffer");
        MetalBuffer { handle: h, len }
    }

    /// Create a render pipeline.
    pub fn render_pipeline(
        &self,
        vertex: &ShaderFunction,
        fragment: &ShaderFunction,
        pixel_format: u64,
    ) -> Result<RenderPipeline, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            (fns().metal_make_render_pipeline)(
                vertex.handle, fragment.handle, pixel_format,
                err_buf.as_mut_ptr(), err_buf.len(),
            )
        };
        if h.is_null() {
            let len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..len]).to_string())
        } else {
            Ok(RenderPipeline { handle: h })
        }
    }

    /// Create a compute pipeline.
    pub fn compute_pipeline(&self, function: &ShaderFunction) -> Result<ComputePipeline, String> {
        let mut err_buf = vec![0u8; 2048];
        let h = unsafe {
            (fns().metal_make_compute_pipeline)(
                function.handle, err_buf.as_mut_ptr(), err_buf.len(),
            )
        };
        if h.is_null() {
            let len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
            Err(String::from_utf8_lossy(&err_buf[..len]).to_string())
        } else {
            Ok(ComputePipeline { handle: h })
        }
    }

    /// Raw MTLDevice pointer.
    pub fn raw(&self) -> Handle { self.handle }
}

/// A Metal command queue.
pub struct CommandQueue {
    handle: Handle,
}

impl CommandQueue {
    /// Create a command buffer.
    pub fn command_buffer(&self) -> CommandBuffer {
        let h = unsafe { (fns().metal_command_buffer)(self.handle) };
        assert!(!h.is_null(), "Failed to create command buffer");
        CommandBuffer { handle: h }
    }
}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A Metal command buffer.
pub struct CommandBuffer {
    handle: Handle,
}

impl CommandBuffer {
    /// Create a render command encoder targeting a drawable's texture.
    pub fn render_encoder(
        &self,
        drawable: &MetalDrawable,
        clear_r: f64, clear_g: f64, clear_b: f64, clear_a: f64,
    ) -> RenderEncoder {
        let texture = drawable.texture();
        let h = unsafe {
            (fns().metal_render_encoder)(
                self.handle, texture,
                clear_r, clear_g, clear_b, clear_a,
            )
        };
        assert!(!h.is_null(), "Failed to create render encoder");
        RenderEncoder { handle: h }
    }

    /// Present a drawable and commit.
    pub fn present_and_commit(&self, drawable: &MetalDrawable) {
        unsafe {
            (fns().metal_present_drawable)(self.handle, drawable.handle);
            (fns().metal_commit)(self.handle);
        }
    }

    /// Commit for execution.
    pub fn commit(&self) {
        unsafe { (fns().metal_commit)(self.handle) };
    }

    /// Wait until GPU finishes.
    pub fn wait(&self) {
        unsafe { (fns().metal_wait)(self.handle) };
    }

    /// Raw MTLCommandBuffer pointer.
    pub fn raw(&self) -> Handle { self.handle }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A compiled shader library.
pub struct ShaderLibrary {
    handle: Handle,
}

impl ShaderLibrary {
    /// Get a function by name.
    pub fn function(&self, name: &str) -> Option<ShaderFunction> {
        let h = unsafe {
            (fns().metal_make_function)(self.handle, name.as_ptr(), name.len())
        };
        if h.is_null() { None } else { Some(ShaderFunction { handle: h }) }
    }
}

impl Drop for ShaderLibrary {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A shader function from a library.
pub struct ShaderFunction {
    pub(crate) handle: Handle,
}

impl Drop for ShaderFunction {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A render pipeline state.
pub struct RenderPipeline {
    handle: Handle,
}

impl RenderPipeline {
    pub fn raw(&self) -> Handle { self.handle }
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A compute pipeline state.
pub struct ComputePipeline {
    handle: Handle,
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A GPU buffer.
pub struct MetalBuffer {
    handle: Handle,
    len: usize,
}

impl MetalBuffer {
    /// Raw pointer to buffer contents.
    pub fn contents(&self) -> *mut c_void {
        unsafe { (fns().metal_buffer_contents)(self.handle) }
    }

    /// View buffer contents as a typed slice.
    pub fn as_slice<T: Copy>(&self) -> &[T] {
        let ptr = self.contents() as *const T;
        let count = self.len / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts(ptr, count) }
    }

    /// View buffer contents as a mutable typed slice.
    pub fn as_mut_slice<T: Copy>(&mut self) -> &mut [T] {
        let ptr = self.contents() as *mut T;
        let count = self.len / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts_mut(ptr, count) }
    }

    /// Raw MTLBuffer pointer.
    pub fn raw(&self) -> Handle { self.handle }
}

impl Drop for MetalBuffer {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}

/// A render command encoder.
pub struct RenderEncoder {
    handle: Handle,
}

impl RenderEncoder {
    /// Set the render pipeline.
    pub fn set_pipeline(&self, pipeline: &RenderPipeline) {
        unsafe { (fns().metal_render_set_pipeline)(self.handle, pipeline.handle) };
    }

    /// Bind a vertex buffer at an index.
    pub fn set_vertex_buffer(&self, buffer: &MetalBuffer, offset: usize, index: usize) {
        unsafe { (fns().metal_render_set_vertex_buffer)(self.handle, buffer.handle, offset, index) };
    }

    /// Draw primitives (triangles).
    pub fn draw(&self, vertex_count: usize, instance_count: usize) {
        unsafe { (fns().metal_render_draw)(self.handle, vertex_count, instance_count) };
    }

    /// End encoding.
    pub fn end(&self) {
        unsafe { (fns().metal_render_end)(self.handle) };
    }
}
