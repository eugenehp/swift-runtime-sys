//! GPU compute example — double an array of floats on the GPU.
//!
//! Build the bridge first:
//!   cd crates/metal && ./build_bridge.sh
//!
//! Run:
//!   cargo run -p metal-rs --example gpu_compute

fn main() {
    // Load the Metal bridge
    metal::load("crates/metal/libMetalBridge.dylib");

    // Get the default GPU
    let gpu = metal::Device::system_default().expect("No Metal device found");
    println!("GPU: {}", gpu.name());
    println!("  Unified memory: {}", gpu.has_unified_memory());
    println!("  Max buffer: {} MB", gpu.max_buffer_length() / 1024 / 1024);
    println!("  Max threads/group: {}", gpu.max_threads_per_threadgroup());
    println!("  Supports raytracing: {}", gpu.supports_raytracing());
    println!("  Supports Metal 3: {}", gpu.supports_family(metal::GPUFamily::METAL3));

    // Compile a compute shader
    let source = r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void double_values(
            device float* data [[buffer(0)]],
            uint id [[thread_position_in_grid]]
        ) {
            data[id] *= 2.0;
        }
    "#;

    let lib = gpu.library_from_source(source).expect("Shader compile failed");
    println!("\nLibrary functions: {:?}", lib.function_names());

    let func = lib.function("double_values").expect("Function not found");
    let pipeline = gpu.compute_pipeline(&func).expect("Pipeline creation failed");
    println!("Pipeline thread execution width: {}", pipeline.thread_execution_width());
    println!("Pipeline max threads: {}", pipeline.max_threads());

    // Create a buffer with data
    let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let n = data.len();
    let buf = gpu.buffer_with_data(&data, metal::ResourceOptions::STORAGE_SHARED);
    println!("\nInput:  {:?}", &data);

    // Dispatch compute
    let queue = gpu.command_queue();
    let cmd = queue.command_buffer();
    let enc = cmd.compute_encoder();
    enc.set_pipeline(&pipeline);
    enc.set_buffer(&buf, 0, 0);
    let threads_per_group = pipeline.max_threads().min(n);
    enc.dispatch_threads(n, 1, 1, threads_per_group, 1, 1);
    enc.end();
    cmd.commit();
    cmd.wait();

    // Read results
    let result = buf.as_slice::<f32>();
    println!("Output: {:?}", &result[..n]);
    println!("Status: {:?}", cmd.status());
    println!("GPU time: {:.3} µs", cmd.gpu_duration() * 1_000_000.0);

    assert_eq!(result, &[2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
    println!("\n✓ GPU compute test passed!");
}
