//! Full visionOS app — window + immersive space + volume.
//!
//! The build.rs would configure:
//! ```ignore
//! swiftui_app::build()
//!     .window("main", "Vision App", 800.0, 600.0)
//!     .immersive_space("world")
//!     .volume("preview", 0.5, 0.5, 0.5)
//!     .build();
//! ```
//!
//! On macOS, only the window scene runs (immersive/volume are visionOS-only).
//! On visionOS, all three scenes are available.
//!
//! ```bash
//! cargo run -p swiftui-app --example visionos_full
//! ```

fn main() {
    println!("visionOS app: window + immersive space + volume");
    println!("(On macOS, only the window scene is active)");

    swiftui_app::launch(|event| match event {
        swiftui_app::Event::Init => {
            println!("App ready — all scenes configured");
        }
        swiftui_app::Event::Frame { window, texture } => {
            static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let frame = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if frame % 60 == 0 {
                println!("[{window}] Frame {frame}");
            }
        }
        _ => {}
    });
}
