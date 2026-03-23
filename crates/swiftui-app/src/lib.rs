//! # swiftui-app — Declarative SwiftUI apps from Rust
//!
//! Build native Apple apps with Metal rendering, configured entirely in `build.rs`.
//! No Swift knowledge required. Works on macOS, iOS, and visionOS.
//!
//! ## How it works
//!
//! 1. Your `build.rs` declares windows, immersive spaces, and volumes
//! 2. The build system generates Swift source and compiles it with `swiftc`
//! 3. At runtime, Rust loads the compiled dylib and registers callbacks
//! 4. SwiftUI manages the app lifecycle; Rust gets `MTLTexture*` each frame
//!
//! ## Quick start
//!
//! **`Cargo.toml`:**
//! ```toml
//! [dependencies]
//! swiftui-app = { path = "../swiftui-app" }
//! ```
//!
//! **`build.rs`:** (the build script generates and compiles Swift for you)
//! ```rust,ignore
//! fn main() {
//!     // Default: single Metal-backed window called "SwiftUI App"
//!     // The build script handles everything automatically.
//! }
//! ```
//!
//! **`src/main.rs`:**
//! ```rust,ignore
//! fn main() {
//!     swiftui_app::launch(|event| match event {
//!         swiftui_app::Event::Init => println!("App ready!"),
//!         swiftui_app::Event::Frame { texture, .. } => {
//!             // texture is an MTLTexture* — render with Metal
//!         },
//!         _ => {}
//!     });
//! }
//! ```
//!
//! ## Custom build.rs for multi-window / visionOS
//!
//! Override the default build script to configure multiple scenes:
//!
//! ```rust,ignore
//! // build.rs
//! fn main() {
//!     let swift = generate_swift_app(
//!         "MyApp",
//!         &[("main", true), ("inspector", true)],  // Metal windows
//!         &["immersive_world"],                      // visionOS immersive spaces
//!         &[("preview", 0.5, 0.5, 0.5)],            // visionOS volumes
//!     );
//!     compile_and_link(&swift);
//! }
//! ```
//!
//! ## Platform behavior
//!
//! | Platform | WindowGroup | ImmersiveSpace | Volume |
//! |----------|-------------|----------------|--------|
//! | macOS    | ✅ MTKView  | skipped (#if)  | skipped |
//! | iOS      | ✅ MTKView  | skipped (#if)  | skipped |
//! | visionOS | ✅ MTKView  | ✅ RealityKit  | ✅ volumetric |
//!
//! On macOS/iOS, visionOS-only scenes are wrapped in `#if os(visionOS)` and
//! compiled out. The same `build.rs` works on all platforms.
//!
//! ## Architecture
//!
//! ```text
//! build.rs                          Runtime
//! ┌────────────────────┐           ┌─────────────────────────────┐
//! │ generate_swift_app()│           │ main()                      │
//! │   ↓                │           │   ↓                         │
//! │ GeneratedApp.swift │           │ dlopen(libGeneratedApp.dylib)│
//! │   ↓                │           │   ↓                         │
//! │ swiftc → .dylib    │           │ register callbacks           │
//! └────────────────────┘           │   ↓                         │
//!                                  │ swiftui_app_launch()         │
//!                                  │   ↓                         │
//!                                  │ RustApp.main() (SwiftUI)     │
//!                                  │   ↓                         │
//!                                  │ MTKView.draw() → on_frame() │
//!                                  │   ↓                         │
//!                                  │ your Rust code (60fps)       │
//!                                  └─────────────────────────────┘
//! ```

#![allow(unused)]

#[cfg(feature = "codegen")]
mod codegen;

#[cfg(feature = "codegen")]
pub use codegen::AppBuilder;

/// Create an [`AppBuilder`] for configuring the app in `build.rs`.
///
/// Only available with the `codegen` feature (used as a build dependency).
#[cfg(feature = "codegen")]
pub fn build() -> AppBuilder {
    AppBuilder::new()
}

// ── Runtime (default, no feature flag) ──────────────────────────────────────

#[cfg(not(feature = "codegen"))]
mod runtime;

#[cfg(not(feature = "codegen"))]
pub use runtime::*;

// ── Test support ────────────────────────────────────────────────────────────

/// Generate Swift source for testing (not public API).
#[doc(hidden)]
pub fn codegen_for_test(
    name: &str,
    windows: &[(&str, bool)],
    immersive: &[&str],
    volumes: &[(&str, f64, f64, f64)],
) -> String {
    // Inline codegen to avoid feature-gating issues in tests
    let mut s = String::new();
    // Use the same generator as build.rs
    generate_swift_source(&mut s, name, windows, immersive, volumes);
    s
}

fn generate_swift_source(
    s: &mut String,
    name: &str,
    windows: &[(&str, bool)],
    immersive: &[&str],
    volumes: &[(&str, f64, f64, f64)],
) {
    use std::fmt::Write;

    writeln!(s, "import SwiftUI").unwrap();
    if !immersive.is_empty() || !volumes.is_empty() {
        writeln!(s, "#if os(visionOS)").unwrap();
        writeln!(s, "import RealityKit").unwrap();
        writeln!(s, "#endif").unwrap();
    }
    writeln!(s).unwrap();

    // Callbacks
    writeln!(s, "private var rustOnInit: (@convention(c) () -> Void)? = nil").unwrap();
    for (id, metal) in windows {
        if *metal {
            writeln!(s, "private var rustOnFrame_{id}: (@convention(c) (UnsafeMutableRawPointer?) -> Void)? = nil").unwrap();
        }
    }
    for sp in immersive {
        writeln!(s, "private var rustOnImmersive_{sp}: (@convention(c) () -> Void)? = nil").unwrap();
    }
    writeln!(s).unwrap();

    // FFI setters
    writeln!(s, r#"@_cdecl("swiftui_app_set_on_init")"#).unwrap();
    writeln!(s, "public func swiftui_app_set_on_init(_ f: @convention(c) () -> Void) {{ rustOnInit = f }}").unwrap();
    for (id, metal) in windows {
        if *metal {
            writeln!(s, r#"@_cdecl("swiftui_app_set_on_frame_{id}")"#).unwrap();
            writeln!(s, "public func swiftui_app_set_on_frame_{id}(_ f: @convention(c) (UnsafeMutableRawPointer?) -> Void) {{ rustOnFrame_{id} = f }}").unwrap();
        }
    }
    writeln!(s).unwrap();

    // Launch
    writeln!(s, r#"@_cdecl("swiftui_app_launch")"#).unwrap();
    writeln!(s, "public func swiftui_app_launch() {{ {name}.main() }}").unwrap();
    writeln!(s).unwrap();

    // App struct
    writeln!(s, "struct {name}: App {{").unwrap();
    writeln!(s, "    var body: some SwiftUI.Scene {{").unwrap();
    for (i, (id, metal)) in windows.iter().enumerate() {
        if i == 0 {
            writeln!(s, "        WindowGroup {{").unwrap();
        } else {
            writeln!(s, r#"        WindowGroup(id: "{id}") {{"#).unwrap();
        }
        if *metal {
            writeln!(s, "            RustMetalView_{id}()").unwrap();
            if i == 0 {
                writeln!(s, "                .onAppear {{ rustOnInit?() }}").unwrap();
            }
        }
        writeln!(s, "        }}").unwrap();
    }

    if !immersive.is_empty() || !volumes.is_empty() {
        writeln!(s, "        #if os(visionOS)").unwrap();
        for sp in immersive {
            writeln!(s, r#"        ImmersiveSpace(id: "{sp}") {{"#).unwrap();
            writeln!(s, "            RealityView {{ content in").unwrap();
            writeln!(s, "                let anchor = AnchorEntity(.head)").unwrap();
            writeln!(s, "                content.add(anchor)").unwrap();
            writeln!(s, "                rustOnImmersive_{sp}?()").unwrap();
            writeln!(s, "            }}").unwrap();
            writeln!(s, "        }}").unwrap();
        }
        for (vid, w, h, d) in volumes {
            writeln!(s, r#"        WindowGroup(id: "{vid}") {{"#).unwrap();
            writeln!(s, "            RealityView {{ content in }}").unwrap();
            writeln!(s, "        }}").unwrap();
            writeln!(s, "        .windowStyle(.volumetric)").unwrap();
            writeln!(s, "        .defaultSize(width: {w}, height: {h}, depth: {d}, in: .meters)").unwrap();
        }
        writeln!(s, "        #endif").unwrap();
    }

    writeln!(s, "    }}").unwrap();
    writeln!(s, "}}").unwrap();
    writeln!(s).unwrap();

    // Metal views
    for (id, metal) in windows {
        if *metal {
            gen_metal_view(s, id);
        }
    }
}

fn gen_metal_view(s: &mut String, id: &str) {
    use std::fmt::Write;
    writeln!(s, "#if canImport(AppKit)").unwrap();
    writeln!(s, "import MetalKit").unwrap();
    writeln!(s, "struct RustMetalView_{id}: NSViewRepresentable {{").unwrap();
    writeln!(s, "    func makeNSView(context: Context) -> MTKView {{").unwrap();
    writeln!(s, "        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())").unwrap();
    writeln!(s, "        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false").unwrap();
    writeln!(s, "        v.delegate = context.coordinator; return v").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    func updateNSView(_ v: MTKView, context: Context) {{}}").unwrap();
    writeln!(s, "    func makeCoordinator() -> Coord_{id} {{ Coord_{id}() }}").unwrap();
    writeln!(s, "}}").unwrap();
    writeln!(s, "#elseif canImport(UIKit)").unwrap();
    writeln!(s, "import MetalKit").unwrap();
    writeln!(s, "struct RustMetalView_{id}: UIViewRepresentable {{").unwrap();
    writeln!(s, "    func makeUIView(context: Context) -> MTKView {{").unwrap();
    writeln!(s, "        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())").unwrap();
    writeln!(s, "        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false").unwrap();
    writeln!(s, "        v.delegate = context.coordinator; return v").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    func updateUIView(_ v: MTKView, context: Context) {{}}").unwrap();
    writeln!(s, "    func makeCoordinator() -> Coord_{id} {{ Coord_{id}() }}").unwrap();
    writeln!(s, "}}").unwrap();
    writeln!(s, "#endif").unwrap();
    writeln!(s, "class Coord_{id}: NSObject, MTKViewDelegate {{").unwrap();
    writeln!(s, "    func mtkView(_ v: MTKView, drawableSizeWillChange s: CGSize) {{}}").unwrap();
    writeln!(s, "    func draw(in v: MTKView) {{").unwrap();
    writeln!(s, "        guard let d = v.currentDrawable else {{ return }}").unwrap();
    writeln!(s, "        rustOnFrame_{id}?(Unmanaged.passUnretained(d.texture).toOpaque())").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "}}").unwrap();
    writeln!(s).unwrap();
}

#[cfg(test)]
mod tests;

