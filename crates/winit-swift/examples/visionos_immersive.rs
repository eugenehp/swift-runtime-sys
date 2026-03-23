//! visionOS Immersive Space — demonstrates the architecture for visionOS rendering.
//!
//! This example shows how winit-swift would be used for visionOS immersive content.
//! On macOS it falls back to a standard Metal window for development.
//!
//! On visionOS, CompositorServices provides the rendering layer instead of CAMetalLayer,
//! and RealityKit handles 3D entity management.
//!
//! ```bash
//! cargo run -p winit-swift --example visionos_immersive
//! ```
//!
//! ## visionOS architecture
//!
//! ```text
//! ┌──────────────────────────────────────────┐
//! │  winit-swift                             │
//! │  ┌────────────┐  ┌───────────────────┐   │
//! │  │ Window     │  │ ImmersiveSpace    │   │
//! │  │ (2D UI)    │  │ (3D volume)       │   │
//! │  │            │  │                   │   │
//! │  │ CAMetalLayer│ │ CompositorServices│   │
//! │  │ SwiftUI    │  │ + RealityKit      │   │
//! │  └────────────┘  └───────────────────┘   │
//! │                                          │
//! │  ┌──────────────────────────────────┐    │
//! │  │ Metal Device (shared GPU)        │    │
//! │  │ Command Queue / Render Pipeline  │    │
//! │  └──────────────────────────────────┘    │
//! └──────────────────────────────────────────┘
//! ```
//!
//! ## Window types on visionOS
//!
//! - **Window** — standard 2D window (like a floating panel)
//! - **Volume** — bounded 3D content container
//! - **ImmersiveSpace** — unbounded 3D environment (mixed, progressive, or full)
//!
//! winit-swift maps all three to the same `Window` type with different attributes.

use winit_swift::*;

fn main() {
    let app = App::new();
    let gpu = app.metal_device().expect("No GPU");
    println!("GPU: {}", gpu.name());

    // On visionOS, you would use:
    //   WindowAttributes::new("Scene", 1.0, 1.0)
    //       .immersive_space(ImmersiveStyle::Mixed)
    // which creates a CompositorServices-backed rendering context.
    //
    // On macOS, we just create a standard Metal window for development:
    let window = app.create_window(
        WindowAttributes::new("winit-swift — visionOS Preview", 1200.0, 800.0)
            .fullsize_content()
            .titlebar_hidden()
    );

    let layer = window.metal_layer().expect("No Metal layer");

    // Build a simple shader for clear-color rendering
    let shader = r#"
        #include <metal_stdlib>
        using namespace metal;

        struct VertexOut {
            float4 position [[position]];
            float4 color;
        };

        vertex VertexOut fullscreen_vert(uint vid [[vertex_id]]) {
            float2 positions[] = {
                float2(-1, -1), float2(3, -1), float2(-1, 3)
            };
            VertexOut out;
            out.position = float4(positions[vid], 0.0, 1.0);

            // Gradient: space purple at top, deep blue at bottom
            float t = positions[vid].y * 0.5 + 0.5;
            out.color = mix(
                float4(0.05, 0.02, 0.15, 1.0),  // deep purple
                float4(0.0, 0.0, 0.05, 1.0),     // near black
                t
            );
            return out;
        }

        fragment float4 fullscreen_frag(VertexOut in [[stage_in]]) {
            return in.color;
        }
    "#;

    let lib = gpu.library_from_source(shader).expect("Shader error");
    let vert = lib.function("fullscreen_vert").unwrap();
    let frag = lib.function("fullscreen_frag").unwrap();
    let pipeline = gpu.render_pipeline(&vert, &frag, pixel_format::BGRA8_UNORM).unwrap();
    let queue = gpu.command_queue();

    println!("Rendering immersive preview...");
    println!("On visionOS, this would be a full immersive space.");
    println!("Press Esc to exit.");

    app.run(move |event, control| {
        match event {
            Event::CloseRequested(_) | Event::KeyDown { keycode: 53, .. } => {
                control.exit();
            }
            Event::RedrawRequested(_) | Event::Resized { .. } => {
                let Some(drawable) = layer.next_drawable() else { return };
                let cmd = queue.command_buffer();
                let enc = cmd.render_encoder(&drawable, 0.0, 0.0, 0.0, 1.0);
                enc.set_pipeline(&pipeline);
                enc.draw(3, 1); // fullscreen triangle
                enc.end();
                cmd.present_and_commit(&drawable);
            }
            _ => {}
        }
    });
}
