//! Simplest possible SwiftUI app from Rust.
//!
//! The build.rs generates a single-window SwiftUI app.
//! This just launches it and prints frame events.
//!
//! ```bash
//! cargo run -p swiftui-app --example hello
//! ```

fn main() {
    println!("Launching SwiftUI app from Rust...");
    swiftui_app::launch(|event| match event {
        swiftui_app::Event::Init => {
            println!("SwiftUI app initialized ✓");
        }
        swiftui_app::Event::Frame { texture, .. } => {
            // texture is MTLTexture* — render with Metal here
            static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n % 60 == 0 {
                println!("Frame {n} — texture at {texture:?}");
            }
        }
        _ => {}
    });
}
