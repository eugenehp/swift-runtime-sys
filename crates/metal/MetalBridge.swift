// Metal bridge — @_cdecl wrappers for Metal API
// Compile: swiftc -emit-library -o libMetalBridge.dylib MetalBridge.swift -framework Metal -framework CoreGraphics

import Metal
import Foundation

public typealias Handle = UnsafeMutableRawPointer

func box<T: AnyObject>(_ obj: T) -> Handle { Unmanaged.passRetained(obj).toOpaque() }
func unbox<T: AnyObject>(_ h: Handle) -> T { Unmanaged<T>.fromOpaque(h).takeUnretainedValue() }
func unboxProto<T>(_ h: Handle) -> T { Unmanaged<AnyObject>.fromOpaque(h).takeUnretainedValue() as! T }

// ── Device ──

@_cdecl("mtl_create_system_default_device")
public func mtl_create_system_default_device() -> Handle? {
    guard let device = MTLCreateSystemDefaultDevice() else { return nil }
    return box(device as AnyObject)
}

@_cdecl("mtl_device_name")
public func mtl_device_name(_ device: Handle, _ out: UnsafeMutablePointer<UInt8>, _ cap: Int) -> Int {
    let d: MTLDevice = unboxProto(device)
    let name = d.name
    let bytes = Array(name.utf8)
    let len = min(bytes.count, cap)
    for i in 0..<len { out[i] = bytes[i] }
    return len
}

@_cdecl("mtl_device_max_threads_per_threadgroup")
public func mtl_device_max_threads_per_threadgroup(_ device: Handle) -> Int {
    let d: MTLDevice = unboxProto(device)
    return d.maxThreadsPerThreadgroup.width
}

@_cdecl("mtl_device_has_unified_memory")
public func mtl_device_has_unified_memory(_ device: Handle) -> Bool {
    let d: MTLDevice = unboxProto(device)
    return d.hasUnifiedMemory
}

@_cdecl("mtl_device_max_buffer_length")
public func mtl_device_max_buffer_length(_ device: Handle) -> Int {
    let d: MTLDevice = unboxProto(device)
    return d.maxBufferLength
}

@_cdecl("mtl_device_supports_family")
public func mtl_device_supports_family(_ device: Handle, _ family: Int) -> Bool {
    let d: MTLDevice = unboxProto(device)
    guard let f = MTLGPUFamily(rawValue: family) else { return false }
    return d.supportsFamily(f)
}

@_cdecl("mtl_device_supports_raytracing")
public func mtl_device_supports_raytracing(_ device: Handle) -> Bool {
    let d: MTLDevice = unboxProto(device)
    return d.supportsRaytracing
}

// ── Command Queue ──

@_cdecl("mtl_device_make_command_queue")
public func mtl_device_make_command_queue(_ device: Handle) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    guard let q = d.makeCommandQueue() else { return nil }
    return box(q as AnyObject)
}

@_cdecl("mtl_device_make_command_queue_max")
public func mtl_device_make_command_queue_max(_ device: Handle, _ maxCmds: Int) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    guard let q = d.makeCommandQueue(maxCommandBufferCount: maxCmds) else { return nil }
    return box(q as AnyObject)
}

// ── Command Buffer ──

@_cdecl("mtl_command_queue_command_buffer")
public func mtl_command_queue_command_buffer(_ queue: Handle) -> Handle? {
    let q: MTLCommandQueue = unboxProto(queue)
    guard let cb = q.makeCommandBuffer() else { return nil }
    return box(cb as AnyObject)
}

@_cdecl("mtl_command_buffer_commit")
public func mtl_command_buffer_commit(_ buf: Handle) {
    let cb: MTLCommandBuffer = unboxProto(buf)
    cb.commit()
}

@_cdecl("mtl_command_buffer_wait")
public func mtl_command_buffer_wait(_ buf: Handle) {
    let cb: MTLCommandBuffer = unboxProto(buf)
    cb.waitUntilCompleted()
}

@_cdecl("mtl_command_buffer_status")
public func mtl_command_buffer_status(_ buf: Handle) -> Int {
    let cb: MTLCommandBuffer = unboxProto(buf)
    return Int(cb.status.rawValue)
}

@_cdecl("mtl_command_buffer_gpu_start_time")
public func mtl_command_buffer_gpu_start_time(_ buf: Handle) -> Double {
    let cb: MTLCommandBuffer = unboxProto(buf)
    return cb.gpuStartTime
}

@_cdecl("mtl_command_buffer_gpu_end_time")
public func mtl_command_buffer_gpu_end_time(_ buf: Handle) -> Double {
    let cb: MTLCommandBuffer = unboxProto(buf)
    return cb.gpuEndTime
}

// ── Buffer ──

@_cdecl("mtl_device_make_buffer")
public func mtl_device_make_buffer(_ device: Handle, _ length: Int, _ options: UInt) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    guard let buf = d.makeBuffer(length: length, options: MTLResourceOptions(rawValue: options)) else { return nil }
    return box(buf as AnyObject)
}

@_cdecl("mtl_device_make_buffer_with_bytes")
public func mtl_device_make_buffer_with_bytes(_ device: Handle, _ ptr: UnsafeRawPointer, _ length: Int, _ options: UInt) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    guard let buf = d.makeBuffer(bytes: ptr, length: length, options: MTLResourceOptions(rawValue: options)) else { return nil }
    return box(buf as AnyObject)
}

@_cdecl("mtl_buffer_contents")
public func mtl_buffer_contents(_ buf: Handle) -> UnsafeMutableRawPointer {
    let b: MTLBuffer = unboxProto(buf)
    return b.contents()
}

@_cdecl("mtl_buffer_length")
public func mtl_buffer_length(_ buf: Handle) -> Int {
    let b: MTLBuffer = unboxProto(buf)
    return b.length
}

// ── Texture ──

@_cdecl("mtl_device_make_texture")
public func mtl_device_make_texture(
    _ device: Handle,
    _ textureType: UInt, _ pixelFormat: UInt,
    _ width: Int, _ height: Int, _ depth: Int,
    _ mipmapLevels: Int, _ sampleCount: Int,
    _ usage: UInt, _ storageMode: UInt
) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    let desc = MTLTextureDescriptor()
    desc.textureType = MTLTextureType(rawValue: textureType)!
    desc.pixelFormat = MTLPixelFormat(rawValue: pixelFormat)!
    desc.width = width
    desc.height = height
    desc.depth = depth
    desc.mipmapLevelCount = mipmapLevels
    desc.sampleCount = sampleCount
    desc.usage = MTLTextureUsage(rawValue: usage)
    desc.storageMode = MTLStorageMode(rawValue: storageMode)!
    guard let tex = d.makeTexture(descriptor: desc) else { return nil }
    return box(tex as AnyObject)
}

@_cdecl("mtl_texture_width")
public func mtl_texture_width(_ tex: Handle) -> Int {
    let t: MTLTexture = unboxProto(tex)
    return t.width
}

@_cdecl("mtl_texture_height")
public func mtl_texture_height(_ tex: Handle) -> Int {
    let t: MTLTexture = unboxProto(tex)
    return t.height
}

@_cdecl("mtl_texture_pixel_format")
public func mtl_texture_pixel_format(_ tex: Handle) -> UInt {
    let t: MTLTexture = unboxProto(tex)
    return t.pixelFormat.rawValue
}

@_cdecl("mtl_texture_replace_region")
public func mtl_texture_replace_region(
    _ tex: Handle,
    _ x: Int, _ y: Int, _ w: Int, _ h: Int,
    _ mip: Int, _ bytesPerRow: Int, _ data: UnsafeRawPointer
) {
    let t: MTLTexture = unboxProto(tex)
    let region = MTLRegionMake2D(x, y, w, h)
    t.replace(region: region, mipmapLevel: mip, withBytes: data, bytesPerRow: bytesPerRow)
}

// ── Library & Function ──

@_cdecl("mtl_device_make_default_library")
public func mtl_device_make_default_library(_ device: Handle) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    guard let lib = d.makeDefaultLibrary() else { return nil }
    return box(lib as AnyObject)
}

@_cdecl("mtl_device_make_library_source")
public func mtl_device_make_library_source(_ device: Handle, _ src: UnsafePointer<UInt8>, _ len: Int, _ errOut: UnsafeMutablePointer<UInt8>, _ errCap: Int) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    let source = String(bytes: UnsafeBufferPointer(start: src, count: len), encoding: .utf8) ?? ""
    do {
        let lib = try d.makeLibrary(source: source, options: nil)
        return box(lib as AnyObject)
    } catch {
        let msg = Array(error.localizedDescription.utf8)
        let n = min(msg.count, errCap)
        for i in 0..<n { errOut[i] = msg[i] }
        return nil
    }
}

@_cdecl("mtl_library_make_function")
public func mtl_library_make_function(_ lib: Handle, _ name: UnsafePointer<UInt8>, _ len: Int) -> Handle? {
    let l: MTLLibrary = unboxProto(lib)
    let funcName = String(bytes: UnsafeBufferPointer(start: name, count: len), encoding: .utf8) ?? ""
    guard let f = l.makeFunction(name: funcName) else { return nil }
    return box(f as AnyObject)
}

@_cdecl("mtl_library_function_names")
public func mtl_library_function_names(_ lib: Handle, _ out: UnsafeMutablePointer<UInt8>, _ cap: Int) -> Int {
    let l: MTLLibrary = unboxProto(lib)
    let joined = l.functionNames.joined(separator: ",")
    let bytes = Array(joined.utf8)
    let n = min(bytes.count, cap)
    for i in 0..<n { out[i] = bytes[i] }
    return n
}

// ── Compute Pipeline ──

@_cdecl("mtl_device_make_compute_pipeline")
public func mtl_device_make_compute_pipeline(_ device: Handle, _ function: Handle, _ errOut: UnsafeMutablePointer<UInt8>, _ errCap: Int) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    let f: MTLFunction = unboxProto(function)
    do {
        let pso = try d.makeComputePipelineState(function: f)
        return box(pso as AnyObject)
    } catch {
        let msg = Array(error.localizedDescription.utf8)
        let n = min(msg.count, errCap)
        for i in 0..<n { errOut[i] = msg[i] }
        return nil
    }
}

@_cdecl("mtl_compute_pipeline_max_threads")
public func mtl_compute_pipeline_max_threads(_ pso: Handle) -> Int {
    let p: MTLComputePipelineState = unboxProto(pso)
    return p.maxTotalThreadsPerThreadgroup
}

@_cdecl("mtl_compute_pipeline_thread_execution_width")
public func mtl_compute_pipeline_thread_execution_width(_ pso: Handle) -> Int {
    let p: MTLComputePipelineState = unboxProto(pso)
    return p.threadExecutionWidth
}

// ── Compute Command Encoder ──

@_cdecl("mtl_command_buffer_compute_encoder")
public func mtl_command_buffer_compute_encoder(_ buf: Handle) -> Handle? {
    let cb: MTLCommandBuffer = unboxProto(buf)
    guard let enc = cb.makeComputeCommandEncoder() else { return nil }
    return box(enc as AnyObject)
}

@_cdecl("mtl_compute_encoder_set_pipeline")
public func mtl_compute_encoder_set_pipeline(_ enc: Handle, _ pso: Handle) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    let p: MTLComputePipelineState = unboxProto(pso)
    e.setComputePipelineState(p)
}

@_cdecl("mtl_compute_encoder_set_buffer")
public func mtl_compute_encoder_set_buffer(_ enc: Handle, _ buf: Handle, _ offset: Int, _ index: Int) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    let b: MTLBuffer = unboxProto(buf)
    e.setBuffer(b, offset: offset, index: index)
}

@_cdecl("mtl_compute_encoder_set_bytes")
public func mtl_compute_encoder_set_bytes(_ enc: Handle, _ ptr: UnsafeRawPointer, _ length: Int, _ index: Int) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    e.setBytes(ptr, length: length, index: index)
}

@_cdecl("mtl_compute_encoder_set_texture")
public func mtl_compute_encoder_set_texture(_ enc: Handle, _ tex: Handle, _ index: Int) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    let t: MTLTexture = unboxProto(tex)
    e.setTexture(t, index: index)
}

@_cdecl("mtl_compute_encoder_dispatch_threads")
public func mtl_compute_encoder_dispatch_threads(
    _ enc: Handle,
    _ gx: Int, _ gy: Int, _ gz: Int,
    _ tx: Int, _ ty: Int, _ tz: Int
) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    e.dispatchThreads(
        MTLSizeMake(gx, gy, gz),
        threadsPerThreadgroup: MTLSizeMake(tx, ty, tz)
    )
}

@_cdecl("mtl_compute_encoder_dispatch_threadgroups")
public func mtl_compute_encoder_dispatch_threadgroups(
    _ enc: Handle,
    _ gx: Int, _ gy: Int, _ gz: Int,
    _ tx: Int, _ ty: Int, _ tz: Int
) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    e.dispatchThreadgroups(
        MTLSizeMake(gx, gy, gz),
        threadsPerThreadgroup: MTLSizeMake(tx, ty, tz)
    )
}

@_cdecl("mtl_compute_encoder_end")
public func mtl_compute_encoder_end(_ enc: Handle) {
    let e: MTLComputeCommandEncoder = unboxProto(enc)
    e.endEncoding()
}

// ── Render Pipeline ──

@_cdecl("mtl_device_make_render_pipeline")
public func mtl_device_make_render_pipeline(
    _ device: Handle,
    _ vertexFn: Handle, _ fragmentFn: Handle,
    _ pixelFormat: UInt,
    _ errOut: UnsafeMutablePointer<UInt8>, _ errCap: Int
) -> Handle? {
    let d: MTLDevice = unboxProto(device)
    let vf: MTLFunction = unboxProto(vertexFn)
    let ff: MTLFunction = unboxProto(fragmentFn)
    let desc = MTLRenderPipelineDescriptor()
    desc.vertexFunction = vf
    desc.fragmentFunction = ff
    desc.colorAttachments[0].pixelFormat = MTLPixelFormat(rawValue: pixelFormat)!
    do {
        let pso = try d.makeRenderPipelineState(descriptor: desc)
        return box(pso as AnyObject)
    } catch {
        let msg = Array(error.localizedDescription.utf8)
        let n = min(msg.count, errCap)
        for i in 0..<n { errOut[i] = msg[i] }
        return nil
    }
}

// ── Blit Command Encoder ──

@_cdecl("mtl_command_buffer_blit_encoder")
public func mtl_command_buffer_blit_encoder(_ buf: Handle) -> Handle? {
    let cb: MTLCommandBuffer = unboxProto(buf)
    guard let enc = cb.makeBlitCommandEncoder() else { return nil }
    return box(enc as AnyObject)
}

@_cdecl("mtl_blit_encoder_copy_buffer")
public func mtl_blit_encoder_copy_buffer(
    _ enc: Handle,
    _ src: Handle, _ srcOffset: Int,
    _ dst: Handle, _ dstOffset: Int,
    _ size: Int
) {
    let e: MTLBlitCommandEncoder = unboxProto(enc)
    let s: MTLBuffer = unboxProto(src)
    let d: MTLBuffer = unboxProto(dst)
    e.copy(from: s, sourceOffset: srcOffset, to: d, destinationOffset: dstOffset, size: size)
}

@_cdecl("mtl_blit_encoder_fill_buffer")
public func mtl_blit_encoder_fill_buffer(_ enc: Handle, _ buf: Handle, _ offset: Int, _ size: Int, _ value: UInt8) {
    let e: MTLBlitCommandEncoder = unboxProto(enc)
    let b: MTLBuffer = unboxProto(buf)
    let range = offset..<(offset + size)
    e.fill(buffer: b, range: range, value: value)
}

@_cdecl("mtl_blit_encoder_end")
public func mtl_blit_encoder_end(_ enc: Handle) {
    let e: MTLBlitCommandEncoder = unboxProto(enc)
    e.endEncoding()
}

// ── Release ──

@_cdecl("mtl_release")
public func mtl_release(_ h: Handle) {
    Unmanaged<AnyObject>.fromOpaque(h).release()
}
