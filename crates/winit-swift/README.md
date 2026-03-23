# winit-swift

Windowing for Apple platforms — drop-in winit-shaped API with Metal, haptics, accessibility, HDR, and visionOS support.

## Quick start

```rust
use winit_swift::prelude::*;

struct App { window: Option<Window> }

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, el: &ActiveEventLoop) {
        self.window = Some(el.create_window(
            WindowAttributes::default().with_title("Hello")
        ).unwrap());
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::RedrawRequested => { /* Metal render here */ },
            _ => {}
        }
    }
}

fn main() {
    EventLoop::new().unwrap().run_app(&mut App { window: None }).unwrap();
}
```

## Features beyond winit

| Feature | winit | winit-swift |
|---------|-------|-------------|
| Metal rendering | external | built-in (device, pipeline, buffers, encoders) |
| HDR / Display P3 | ❌ | `layer.set_hdr(true)` |
| Haptics | ❌ | Core Haptics + presets (tap, success, error) |
| Accessibility | ❌ | VoiceOver, reduce motion, high contrast queries |
| Thermal state | ❌ | `thermal_state()`, `is_low_power_mode()` |
| Monitor refresh rate | ❌ | ProMotion 120Hz detection |
| visionOS | ❌ | UIKit windows + ImmersiveSpace (via bridge) |
| Window blur | ❌ | `window.set_blur(true)` |
| Pinch/rotation gestures | ❌ | `PinchGesture`, `RotationGesture` events |
| Safe area | basic | full insets (notch, Dynamic Island) |

## Examples

```bash
cargo run -p winit-swift --example window          # basic window + events
cargo run -p winit-swift --example metal_triangle   # Metal triangle rendering
cargo run -p winit-swift --example haptics          # haptic feedback demo
cargo run -p winit-swift --example winit_compat     # winit-shaped API
cargo run -p winit-swift --example visionos_app     # visionOS app (Swift bridge)
cargo run -p winit-swift --example sil_app          # SIL-level SwiftUI launch
```

## Tests

```bash
cargo test -p winit-swift    # 43 tests — events, DPI, compat types, prelude
```

## License

Apache-2.0
