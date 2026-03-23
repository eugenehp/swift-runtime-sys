//! Haptic feedback — demonstrates Core Haptics and system feedback.
//!
//! ```bash
//! cargo run -p winit-swift --example haptics
//! ```
//!
//! Press keys 1-5 to trigger different haptic patterns on Force Touch trackpad.

use winit_swift::*;

fn main() {
    let app = App::new();

    let haptics = Haptics::new();
    match &haptics {
        Some(_) => println!("Haptics engine initialized ✓"),
        None => println!("Haptics not supported on this device"),
    }

    let _window = app.create_window(
        WindowAttributes::new("winit-swift — Haptics Demo", 400.0, 300.0)
    );

    println!("Press keys:");
    println!("  1 — Light tap");
    println!("  2 — Selection feedback");
    println!("  3 — Success");
    println!("  4 — Warning");
    println!("  5 — Error");
    println!("  Space — Custom pattern");
    println!("  Esc — Quit");

    app.run(move |event, control| {
        match event {
            Event::CloseRequested(_) => control.exit(),
            Event::KeyDown { keycode, .. } => {
                if let Some(ref h) = haptics {
                    match keycode {
                        18 => { println!("tap"); h.tap(); }           // 1
                        19 => { println!("selection"); h.selection(); } // 2
                        20 => { println!("success"); h.success(); }    // 3
                        21 => { println!("warning"); h.warning(); }    // 4
                        23 => { println!("error"); h.error(); }        // 5
                        49 => {                                         // Space
                            println!("custom pattern");
                            h.play(0.7, 0.2, 0.5);
                        }
                        53 => control.exit(),                           // Esc
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
}
