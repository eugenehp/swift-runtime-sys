//! Metal triangle — renders a colored triangle using the integrated Metal API.
//!
//! ```bash
//! cargo run -p winit-swift --example metal_triangle
//! ```

use winit_swift::*;

const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut vertex_main(
    const device float2* positions [[buffer(0)]],
    const device float4* colors    [[buffer(1)]],
    uint vid [[vertex_id]]
) {
    VertexOut out;
    out.position = float4(positions[vid], 0.0, 1.0);
    out.color = colors[vid];
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

fn main() {
    let app = App::new();
    let gpu = app.metal_device().expect("No Metal GPU found");
    println!("GPU: {}", gpu.name());

    let window = app.create_window(
        WindowAttributes::new("winit-swift — Metal Triangle", 800.0, 600.0)
    );

    let layer = window.metal_layer().expect("No Metal layer");
    layer.set_vsync(true);

    // Compile shaders
    let lib = gpu.library_from_source(SHADER_SOURCE).expect("Shader compile failed");
    let vert = lib.function("vertex_main").expect("vertex_main not found");
    let frag = lib.function("fragment_main").expect("fragment_main not found");
    let pipeline = gpu.render_pipeline(&vert, &frag, pixel_format::BGRA8_UNORM)
        .expect("Pipeline creation failed");

    // Triangle vertices
    let positions: [f32; 6] = [
         0.0,  0.75,  // top
        -0.75, -0.75, // bottom-left
         0.75, -0.75, // bottom-right
    ];
    let colors: [f32; 16] = [
        1.0, 0.0, 0.0, 1.0, // red
        0.0, 1.0, 0.0, 1.0, // green
        0.0, 0.0, 1.0, 1.0, // blue
        0.0, 0.0, 0.0, 0.0, // padding
    ];

    let pos_buf = gpu.buffer_with_data(&positions, resource_options::SHARED);
    let col_buf = gpu.buffer_with_data(&colors, resource_options::SHARED);

    let queue = gpu.command_queue();

    app.run(move |event, control| {
        match event {
            Event::CloseRequested(_) => control.exit(),
            Event::KeyDown { keycode: 53, .. } => control.exit(), // Escape

            Event::RedrawRequested(_) | Event::Resized { .. } => {
                // Get next drawable
                let Some(drawable) = layer.next_drawable() else { return };

                // Create command buffer and render encoder
                let cmd = queue.command_buffer();
                let enc = cmd.render_encoder(
                    &drawable,
                    0.1, 0.1, 0.12, 1.0, // dark background
                );

                // Draw triangle
                enc.set_pipeline(&pipeline);
                enc.set_vertex_buffer(&pos_buf, 0, 0);
                enc.set_vertex_buffer(&col_buf, 0, 1);
                enc.draw(3, 1);
                enc.end();

                // Present
                cmd.present_and_commit(&drawable);
            }

            _ => {}
        }
    });
}
