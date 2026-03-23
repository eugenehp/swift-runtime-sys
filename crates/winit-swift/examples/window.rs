//! Basic window — shows event handling, theme detection, and accessibility queries.
//!
//! ```bash
//! # Build the Swift bridge first:
//! cd crates/winit-swift/swift
//! swiftc -emit-library -o libWinitSwift.dylib WinitSwiftBridge.swift \
//!   -framework Foundation -framework QuartzCore \
//!   -framework Metal -framework CoreGraphics \
//!   -framework AppKit -framework CoreHaptics
//!
//! # Run:
//! cargo run -p winit-swift --example window
//! ```

use winit_swift::*;

fn main() {
    let app = App::new();

    // Query system state
    let acc = accessibility();
    println!("VoiceOver: {}", acc.voiceover_running);
    println!("Reduce Motion: {}", acc.reduce_motion);
    println!("High Contrast: {}", acc.high_contrast);
    println!("Thermal: {:?}", thermal_state());
    println!("Low Power: {}", is_low_power_mode());

    // List monitors
    for mon in monitors() {
        println!("Monitor: {} ({}x{} @{:.0}Hz, scale {:.1}x)",
                 mon.name, mon.width, mon.height, mon.refresh_rate, mon.scale_factor);
    }

    // Create window
    let window = app.create_window(
        WindowAttributes::new("winit-swift — Hello", 800.0, 600.0)
    );

    println!("Window created: {:?}", window.id());
    println!("Theme: {:?}", window.theme());
    println!("Surface size: {:?}", window.surface_size());
    println!("Scale factor: {}", window.scale_factor());

    // Event loop
    app.run(move |event, control| {
        match event {
            Event::CloseRequested(_) => {
                println!("Close requested — exiting");
                control.exit();
            }
            Event::Resized { id, width, height } => {
                println!("Resized {:?}: {}x{}", id, width, height);
            }
            Event::KeyDown { keycode, .. } => {
                println!("Key down: {}", keycode);
                if keycode == 53 { // Escape
                    control.exit();
                }
            }
            Event::MouseMoved { x, y, .. } => {
                // Uncomment to see mouse movement:
                // println!("Mouse: {:.0}, {:.0}", x, y);
                let _ = (x, y);
            }
            Event::ThemeChanged { theme, .. } => {
                println!("Theme changed: {:?}", theme);
            }
            Event::Focused(_) => println!("Focused"),
            Event::Unfocused(_) => println!("Unfocused"),
            Event::RedrawRequested(_) => {
                // Would render here
            }
            _ => {}
        }
    });
}
