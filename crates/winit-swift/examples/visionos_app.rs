//! visionOS app — Rust owns main(), Swift owns the SwiftUI lifecycle.
//!
//! This is a REAL visionOS app that runs on Apple Vision Pro.
//! On macOS, it runs as a SwiftUI window with Metal rendering.
//!
//! ```bash
//! cargo run -p winit-swift --example visionos_app
//! ```
//!
//! ## Architecture
//!
//! ```text
//! Rust main()
//!   └─ winit_swift::visionos::launch(on_init, on_frame)
//!        └─ ws_visionos_configure(on_init, on_frame)  [C FFI]
//!        └─ ws_visionos_launch()                       [C FFI]
//!             └─ RustVisionApp.main()                  [Swift]
//!                  └─ SwiftUI event loop (never returns)
//!                       └─ MTKView.draw() each frame
//!                            └─ on_frame(MTLTexture*)  [back to Rust]
//! ```

use std::ffi::c_void;

fn main() {
    println!("Launching visionOS app from Rust...");
    println!("Rust owns main(). Swift owns the SwiftUI lifecycle.");
    println!("Metal rendering callback runs in Rust.");
    winit_swift::visionos::launch(on_init, on_frame);
}

extern "C" fn on_init() {
    println!("[Rust] SwiftUI app initialized — surfaces ready");
    println!("[Rust] Metal rendering will begin on next frame");
}

extern "C" fn on_frame(texture_ptr: *mut c_void) {
    if texture_ptr.is_null() {
        return;
    }
    // texture_ptr is an MTLTexture* — you can render into it with Metal.
    //
    // In a real app you would:
    // 1. Create a command buffer from your command queue
    // 2. Create a render pass descriptor targeting this texture
    // 3. Encode your draw calls
    // 4. Commit
    //
    // The MTKView handles presentation automatically.

    // For now, just prove we're getting frames:
    static FRAME_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let frame = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if frame % 60 == 0 {
        println!("[Rust] Frame {frame} — rendering to MTLTexture at {texture_ptr:?}");
    }
}
