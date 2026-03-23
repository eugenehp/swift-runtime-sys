/// Build script that generates and compiles the Swift app shell.
///
/// Users override this in their own crate's build.rs by depending on
/// swiftui-app with the `codegen` feature and calling `swiftui_app::build()`.
///
/// This default build.rs creates a single-window Metal-backed app.

fn main() {
    // Generate a minimal Swift app with one Metal window
    let swift_src = generate_swift_app("RustApp", &[("main", true)], &[], &[]);
    compile_and_link(&swift_src);
}

fn generate_swift_app(
    name: &str,
    windows: &[(&str, bool)],  // (id, metal?)
    _immersive: &[&str],
    _volumes: &[(&str, f64, f64, f64)],
) -> String {
    let mut s = String::new();
    s.push_str("import SwiftUI\n\n");

    // Callbacks
    s.push_str("private var rustOnInit: (@convention(c) () -> Void)? = nil\n");
    for (id, metal) in windows {
        if *metal {
            s.push_str(&format!(
                "private var rustOnFrame_{id}: (@convention(c) (UnsafeMutableRawPointer?) -> Void)? = nil\n"
            ));
        }
    }

    // Set callbacks FFI
    s.push_str("\n@_cdecl(\"swiftui_app_set_on_init\")\n");
    s.push_str("public func swiftui_app_set_on_init(_ f: @convention(c) () -> Void) { rustOnInit = f }\n\n");

    for (id, metal) in windows {
        if *metal {
            s.push_str(&format!(
                "@_cdecl(\"swiftui_app_set_on_frame_{id}\")\n\
                 public func swiftui_app_set_on_frame_{id}(_ f: @convention(c) (UnsafeMutableRawPointer?) -> Void) {{ rustOnFrame_{id} = f }}\n\n"
            ));
        }
    }

    // Launch
    s.push_str("@_cdecl(\"swiftui_app_launch\")\n");
    s.push_str(&format!("public func swiftui_app_launch() {{ {name}.main() }}\n\n"));

    // App struct
    s.push_str(&format!("struct {name}: App {{\n"));
    s.push_str("    var body: some SwiftUI.Scene {\n");

    for (i, (id, metal)) in windows.iter().enumerate() {
        if i == 0 {
            s.push_str("        WindowGroup {\n");
        } else {
            s.push_str(&format!("        WindowGroup(id: \"{id}\") {{\n"));
        }

        if *metal {
            s.push_str(&format!("            RustMetalView_{id}()\n"));
            if i == 0 {
                s.push_str("                .onAppear { rustOnInit?() }\n");
            }
        }

        s.push_str("        }\n");
    }

    s.push_str("    }\n}\n\n");

    // Metal views
    for (id, metal) in windows {
        if *metal {
            generate_metal_view(&mut s, id);
        }
    }

    s
}

fn generate_metal_view(s: &mut String, id: &str) {
    // AppKit (macOS)
    s.push_str("#if canImport(AppKit)\n");
    s.push_str("import MetalKit\n");
    s.push_str(&format!("struct RustMetalView_{id}: NSViewRepresentable {{\n"));
    s.push_str("    func makeNSView(context: Context) -> MTKView {\n");
    s.push_str("        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())\n");
    s.push_str("        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false\n");
    s.push_str("        v.delegate = context.coordinator; return v\n");
    s.push_str("    }\n");
    s.push_str("    func updateNSView(_ v: MTKView, context: Context) {}\n");
    s.push_str(&format!("    func makeCoordinator() -> Coord_{id} {{ Coord_{id}() }}\n"));
    s.push_str("}\n");
    s.push_str("#elseif canImport(UIKit)\n");
    s.push_str("import MetalKit\n");
    s.push_str(&format!("struct RustMetalView_{id}: UIViewRepresentable {{\n"));
    s.push_str("    func makeUIView(context: Context) -> MTKView {\n");
    s.push_str("        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())\n");
    s.push_str("        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false\n");
    s.push_str("        v.delegate = context.coordinator; return v\n");
    s.push_str("    }\n");
    s.push_str("    func updateUIView(_ v: MTKView, context: Context) {}\n");
    s.push_str(&format!("    func makeCoordinator() -> Coord_{id} {{ Coord_{id}() }}\n"));
    s.push_str("}\n");
    s.push_str("#endif\n");
    s.push_str(&format!("class Coord_{id}: NSObject, MTKViewDelegate {{\n"));
    s.push_str("    func mtkView(_ v: MTKView, drawableSizeWillChange s: CGSize) {}\n");
    s.push_str("    func draw(in v: MTKView) {\n");
    s.push_str("        guard let d = v.currentDrawable else { return }\n");
    s.push_str(&format!("        rustOnFrame_{id}?(Unmanaged.passUnretained(d.texture).toOpaque())\n"));
    s.push_str("    }\n");
    s.push_str("}\n\n");
}

fn compile_and_link(swift_src: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| ".".into());
    let swift_path = format!("{out_dir}/GeneratedApp.swift");
    let dylib_path = format!("{out_dir}/libGeneratedApp.dylib");

    std::fs::write(&swift_path, swift_src).expect("Failed to write Swift source");
    println!("cargo:rerun-if-changed=build.rs");

    let mut cmd = std::process::Command::new("swiftc");
    cmd.args([
        "-parse-as-library", "-emit-library", "-O",
        "-o", &dylib_path,
        &swift_path,
        "-framework", "MetalKit",
        "-Xlinker", "-install_name",
        "-Xlinker", "@rpath/libGeneratedApp.dylib",
    ]);

    let status = cmd.status().expect("Failed to run swiftc");
    assert!(status.success(), "swiftc failed to compile generated app");

    println!("cargo:rustc-link-arg=-Wl,-rpath,{out_dir}");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    println!("cargo:rustc-env=SWIFTUI_APP_DYLIB={dylib_path}");
    println!("cargo:warning=Generated SwiftUI app compiled ✓");
}
