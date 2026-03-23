//! Winit-compatible example using the prelude — same code patterns as a real winit app.
//!
//! ```bash
//! cargo run -p winit-swift --example winit_compat
//! ```

use winit_swift::prelude::*;

#[derive(Default)]
struct MyApp {
    window: Option<Window>,
}

impl ApplicationHandler for MyApp {
    fn can_create_surfaces(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_title("winit-swift — Prelude Example")
            .with_surface_size(LogicalSize::new(900.0, 600.0));

        let win = event_loop.create_window(attrs).unwrap();
        println!("Window: {:?}", win.id());
        println!("  title:      {}", win.title());
        println!("  size:       {:?}", win.surface_size());
        println!("  outer_size: {:?}", win.outer_size());
        println!("  position:   {:?}", win.outer_position());
        println!("  safe_area:  {:?}", win.safe_area());
        println!("  theme:      {:?}", win.theme());
        println!("  has_focus:   {}", win.has_focus());
        println!("  resizable:   {}", win.is_resizable());
        println!("  decorated:   {}", win.is_decorated());
        println!("  scale:       {}", win.scale_factor());

        // Query system state
        let acc = accessibility();
        println!("\nAccessibility:");
        println!("  VoiceOver:    {}", acc.voiceover_running);
        println!("  ReduceMotion: {}", acc.reduce_motion);
        println!("  HighContrast: {}", acc.high_contrast);
        println!("  Thermal:      {:?}", thermal_state());
        println!("  LowPower:     {}", is_low_power_mode());

        // Monitor info
        for m in monitors() {
            println!("\nMonitor: {} ({}×{} @{:.0}Hz, {:.1}×)",
                     m.name, m.width, m.height, m.refresh_rate, m.scale_factor);
        }

        self.window = Some(win);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(size) => {
                println!("Resized: {}×{}", size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } if event.state.is_pressed() => {
                println!("Key: {} (repeat={})", event.physical_key, event.repeat);
                if event.physical_key == 53 { event_loop.exit(); } // Escape
            }
            WindowEvent::Focused(f) => println!("Focused: {f}"),
            WindowEvent::ThemeChanged(t) => println!("Theme: {t:?}"),
            WindowEvent::Occluded(o) => println!("Occluded: {o}"),
            WindowEvent::CursorEntered => println!("Cursor entered"),
            WindowEvent::CursorLeft => println!("Cursor left"),
            WindowEvent::PinchGesture { delta } => println!("Pinch: {delta:.3}"),
            WindowEvent::RotationGesture { delta } => println!("Rotate: {delta:.1}°"),
            WindowEvent::RedrawRequested => {
                // Metal rendering would go here
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = MyApp::default();
    event_loop.run_app(&mut app).unwrap();
}
