//! Metal rendering in a SwiftUI window — declared in build.rs.
//!
//! ```bash
//! cargo run -p swiftui-app --example metal_render
//! ```

use std::ffi::c_void;

fn main() {
    println!("Metal rendering in SwiftUI window");

    swiftui_app::launch(|event| match event {
        swiftui_app::Event::Init => {
            println!("GPU surfaces ready — Metal rendering active");
        }
        swiftui_app::Event::Frame { window, texture } => {
            // In a real app, you'd:
            // 1. Create a command buffer from your command queue
            // 2. Create a render pass descriptor with this texture
            // 3. Encode your draw calls
            // 4. The MTKView handles presentation

            static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let frame = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if frame % 120 == 0 {
                println!("[{window}] Frame {frame}: MTLTexture at {texture:?}");
            }
        }
        _ => {}
    });
}
