# swiftui-app

Build native SwiftUI apps from Rust. Configure scenes in `build.rs`, render with Metal at 60fps.

```rust
// src/main.rs
fn main() {
    swiftui_app::launch(|event| match event {
        swiftui_app::Event::Init => println!("ready!"),
        swiftui_app::Event::Frame { texture, .. } => {
            // texture is MTLTexture* — render with Metal
        },
        _ => {}
    });
}
```

## How it works

1. `build.rs` generates Swift source declaring your app's scenes
2. `swiftc` compiles it to `libGeneratedApp.dylib`
3. At runtime, Rust loads the dylib and registers frame callbacks
4. `SwiftUI.App.main()` enters the event loop
5. Each frame, the `MTKView` calls your Rust function with an `MTLTexture*`

## Multi-window / visionOS

Edit `build.rs` to add more scenes:

```rust
// build.rs
fn main() {
    let swift = generate_swift_app("MyApp",
        &[("main", true), ("inspector", true)],  // Metal windows
        &["world"],                                // visionOS immersive space
        &[("preview", 0.5, 0.5, 0.5)],            // visionOS volume
    );
    compile_and_link(&swift);
}
```

On macOS/iOS, visionOS scenes are `#if os(visionOS)` compiled out.

## Platform support

| Platform | Window | Metal 60fps | Immersive Space | Volume |
|----------|--------|-------------|-----------------|--------|
| macOS    | ✅     | ✅          | —               | —      |
| iOS      | ✅     | ✅          | —               | —      |
| visionOS | ✅     | ✅          | ✅ RealityKit   | ✅     |

## License

Apache-2.0
