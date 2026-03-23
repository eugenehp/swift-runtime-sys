//! Build-time code generation — generates Swift shell + links it.

use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Immersive space style (visionOS).
#[derive(Debug, Clone, Copy)]
pub enum ImmersiveStyle {
    /// Mixed reality — virtual content overlaid on passthrough.
    Mixed,
    /// Progressive — gradual transition from passthrough to virtual.
    Progressive,
    /// Full — completely virtual environment.
    Full,
}

/// A window scene.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub id: String,
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub metal: bool,
}

/// An immersive space scene (visionOS).
#[derive(Debug, Clone)]
pub struct ImmersiveSpaceConfig {
    pub id: String,
    pub style: ImmersiveStyle,
}

/// A volume scene (visionOS).
#[derive(Debug, Clone)]
pub struct VolumeConfig {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

/// Declarative SwiftUI app builder for `build.rs`.
#[derive(Debug)]
pub struct AppBuilder {
    windows: Vec<WindowConfig>,
    immersive_spaces: Vec<ImmersiveSpaceConfig>,
    volumes: Vec<VolumeConfig>,
    app_name: String,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            immersive_spaces: Vec::new(),
            volumes: Vec::new(),
            app_name: "RustApp".into(),
        }
    }

    /// Set the app name (used in Swift struct name).
    pub fn name(mut self, name: &str) -> Self {
        self.app_name = name.into();
        self
    }

    /// Add a window scene with Metal rendering.
    pub fn window(mut self, id: &str, title: &str, width: f64, height: f64) -> Self {
        self.windows.push(WindowConfig {
            id: id.into(),
            title: title.into(),
            width,
            height,
            metal: true,
        });
        self
    }

    /// Add a text-only window (no Metal rendering).
    pub fn text_window(mut self, id: &str, title: &str, width: f64, height: f64) -> Self {
        self.windows.push(WindowConfig {
            id: id.into(),
            title: title.into(),
            width,
            height,
            metal: false,
        });
        self
    }

    /// Add an immersive space (visionOS only).
    pub fn immersive_space(mut self, id: &str) -> Self {
        self.immersive_spaces.push(ImmersiveSpaceConfig {
            id: id.into(),
            style: ImmersiveStyle::Mixed,
        });
        self
    }

    /// Add an immersive space with a specific style.
    pub fn immersive_space_styled(mut self, id: &str, style: ImmersiveStyle) -> Self {
        self.immersive_spaces.push(ImmersiveSpaceConfig {
            id: id.into(),
            style,
        });
        self
    }

    /// Add a volume (visionOS bounded 3D content).
    pub fn volume(mut self, id: &str, width: f64, height: f64, depth: f64) -> Self {
        self.volumes.push(VolumeConfig {
            id: id.into(),
            width,
            height,
            depth,
        });
        self
    }

    /// Generate Swift code, compile, and set up cargo link directives.
    pub fn build(self) {
        let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| ".".into());
        let swift_src = self.generate_swift();
        let swift_path = PathBuf::from(&out_dir).join("GeneratedApp.swift");
        let dylib_path = PathBuf::from(&out_dir).join("libGeneratedApp.dylib");

        std::fs::write(&swift_path, &swift_src).expect("Failed to write Swift source");
        println!("cargo:rerun-if-changed=build.rs");

        // Compile
        let mut cmd = Command::new("swiftc");
        cmd.args([
            "-parse-as-library",
            "-emit-library",
            "-O",
            "-o",
            dylib_path.to_str().unwrap(),
            swift_path.to_str().unwrap(),
            "-Xlinker", "-install_name",
            "-Xlinker", "@rpath/libGeneratedApp.dylib",
        ]);

        let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        match os.as_str() {
            "macos" | "" => {
                cmd.args(["-framework", "MetalKit"]);
            }
            "xros" => {
                let sdk = "/Applications/Xcode.app/Contents/Developer/Platforms/XROS.platform/Developer/SDKs/XROS.sdk";
                cmd.args(["-target", "arm64-apple-xros2.0", "-sdk", sdk]);
            }
            "ios" => {
                let sdk = "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk";
                cmd.args(["-target", "arm64-apple-ios17.0", "-sdk", sdk]);
            }
            _ => {}
        }

        let status = cmd.status().expect("Failed to run swiftc. Is Xcode installed?");
        assert!(status.success(), "swiftc compilation failed");

        // Link directives
        println!("cargo:rustc-link-search=native={out_dir}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{out_dir}");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
        println!("cargo:rustc-link-lib=dylib=swiftCore");

        // Export the dylib path for runtime
        println!("cargo:rustc-env=SWIFTUI_APP_DYLIB={}", dylib_path.display());
    }

    /// Generate the Swift source code.
    fn generate_swift(&self) -> String {
        let mut s = String::new();

        // Imports
        writeln!(s, "import SwiftUI").unwrap();
        writeln!(s, "#if os(visionOS)").unwrap();
        writeln!(s, "import RealityKit").unwrap();
        writeln!(s, "#endif").unwrap();
        writeln!(s).unwrap();

        // Callback storage
        writeln!(s, "// ── Rust callbacks ──").unwrap();
        writeln!(s, "private var rustOnInit: (@convention(c) () -> Void)? = nil").unwrap();
        writeln!(s, "private var rustOnTerminate: (@convention(c) () -> Void)? = nil").unwrap();

        // Per-window frame callbacks
        for w in &self.windows {
            if w.metal {
                writeln!(s, "private var rustOnFrame_{}: (@convention(c) (UnsafeMutableRawPointer?) -> Void)? = nil",
                         w.id).unwrap();
            }
        }

        // Per-immersive-space callbacks
        for sp in &self.immersive_spaces {
            writeln!(s, "private var rustOnImmersive_{}: (@convention(c) () -> Void)? = nil",
                     sp.id).unwrap();
        }

        writeln!(s).unwrap();

        // Configuration FFI
        writeln!(s, "// ── Configuration (called by Rust before launch) ──").unwrap();
        writeln!(s, r#"@_cdecl("swiftui_app_set_on_init")"#).unwrap();
        writeln!(s, "public func swiftui_app_set_on_init(_ f: @convention(c) () -> Void) {{ rustOnInit = f }}").unwrap();
        writeln!(s).unwrap();

        for w in &self.windows {
            if w.metal {
                writeln!(s, r#"@_cdecl("swiftui_app_set_on_frame_{}")"#, w.id).unwrap();
                writeln!(s, "public func swiftui_app_set_on_frame_{}(_ f: @convention(c) (UnsafeMutableRawPointer?) -> Void) {{ rustOnFrame_{} = f }}", w.id, w.id).unwrap();
            }
        }

        writeln!(s).unwrap();

        // Launch function
        writeln!(s, r#"@_cdecl("swiftui_app_launch")"#).unwrap();
        writeln!(s, "public func swiftui_app_launch() {{").unwrap();
        writeln!(s, "    {}.main()", self.app_name).unwrap();
        writeln!(s, "}}").unwrap();
        writeln!(s).unwrap();

        // App struct
        writeln!(s, "struct {}: App {{", self.app_name).unwrap();
        writeln!(s, "    var body: some SwiftUI.Scene {{").unwrap();

        // Windows
        for (i, w) in self.windows.iter().enumerate() {
            if i == 0 {
                writeln!(s, "        WindowGroup {{").unwrap();
            } else {
                writeln!(s, r#"        WindowGroup(id: "{}") {{"#, w.id).unwrap();
            }

            if w.metal {
                writeln!(s, "            RustMetalView_{0}()", w.id).unwrap();
                writeln!(s, "                .onAppear {{ rustOnInit?() }}").unwrap();
            } else {
                writeln!(s, r#"            Text("{}")"#, w.title).unwrap();
                writeln!(s, "                .font(.largeTitle)").unwrap();
                writeln!(s, "                .padding()").unwrap();
            }

            writeln!(s, "        }}").unwrap();
        }

        // Immersive spaces (visionOS only)
        if !self.immersive_spaces.is_empty() {
            writeln!(s, "        #if os(visionOS)").unwrap();
            for sp in &self.immersive_spaces {
                writeln!(s, r#"        ImmersiveSpace(id: "{}") {{"#, sp.id).unwrap();
                writeln!(s, "            RealityView {{ content in").unwrap();
                writeln!(s, "                let anchor = AnchorEntity(.head)").unwrap();
                writeln!(s, "                content.add(anchor)").unwrap();
                writeln!(s, "                rustOnImmersive_{}?()", sp.id).unwrap();
                writeln!(s, "            }}").unwrap();

                match sp.style {
                    ImmersiveStyle::Mixed => writeln!(s, "            .immersionStyle(selection: .constant(.mixed), in: .mixed)").unwrap(),
                    ImmersiveStyle::Progressive => writeln!(s, "            .immersionStyle(selection: .constant(.progressive), in: .progressive)").unwrap(),
                    ImmersiveStyle::Full => writeln!(s, "            .immersionStyle(selection: .constant(.full), in: .full)").unwrap(),
                }

                writeln!(s, "        }}").unwrap();
            }
            writeln!(s, "        #endif").unwrap();
        }

        // Volumes (visionOS only)
        if !self.volumes.is_empty() {
            writeln!(s, "        #if os(visionOS)").unwrap();
            for vol in &self.volumes {
                writeln!(s, r#"        WindowGroup(id: "{}") {{"#, vol.id).unwrap();
                writeln!(s, "            RealityView {{ content in }}").unwrap();
                writeln!(s, "        }}").unwrap();
                writeln!(s, "        .windowStyle(.volumetric)").unwrap();
                writeln!(s, "        .defaultSize(width: {}, height: {}, depth: {}, in: .meters)", vol.width, vol.height, vol.depth).unwrap();
            }
            writeln!(s, "        #endif").unwrap();
        }

        writeln!(s, "    }}").unwrap();
        writeln!(s, "}}").unwrap();
        writeln!(s).unwrap();

        // Metal view structs (one per metal window)
        for w in &self.windows {
            if w.metal {
                self.generate_metal_view(&mut s, &w.id);
            }
        }

        s
    }

    fn generate_metal_view(&self, s: &mut String, window_id: &str) {
        writeln!(s, "// ── Metal view for window '{}' ──", window_id).unwrap();

        // UIKit version (iOS/visionOS)
        writeln!(s, "#if canImport(UIKit) && !canImport(AppKit)").unwrap();
        writeln!(s, "import MetalKit").unwrap();
        writeln!(s, "struct RustMetalView_{0}: UIViewRepresentable {{", window_id).unwrap();
        writeln!(s, "    func makeUIView(context: Context) -> MTKView {{").unwrap();
        writeln!(s, "        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())").unwrap();
        writeln!(s, "        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false").unwrap();
        writeln!(s, "        v.delegate = context.coordinator; return v").unwrap();
        writeln!(s, "    }}").unwrap();
        writeln!(s, "    func updateUIView(_ v: MTKView, context: Context) {{}}").unwrap();
        writeln!(s, "    func makeCoordinator() -> Coord_{0} {{ Coord_{0}() }}", window_id).unwrap();
        writeln!(s, "}}").unwrap();
        writeln!(s, "#endif").unwrap();

        // AppKit version (macOS)
        writeln!(s, "#if canImport(AppKit)").unwrap();
        writeln!(s, "import MetalKit").unwrap();
        writeln!(s, "struct RustMetalView_{0}: NSViewRepresentable {{", window_id).unwrap();
        writeln!(s, "    func makeNSView(context: Context) -> MTKView {{").unwrap();
        writeln!(s, "        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())").unwrap();
        writeln!(s, "        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false").unwrap();
        writeln!(s, "        v.delegate = context.coordinator; return v").unwrap();
        writeln!(s, "    }}").unwrap();
        writeln!(s, "    func updateNSView(_ v: MTKView, context: Context) {{}}").unwrap();
        writeln!(s, "    func makeCoordinator() -> Coord_{0} {{ Coord_{0}() }}", window_id).unwrap();
        writeln!(s, "}}").unwrap();
        writeln!(s, "#endif").unwrap();

        // Shared coordinator
        writeln!(s, "class Coord_{0}: NSObject, MTKViewDelegate {{", window_id).unwrap();
        writeln!(s, "    func mtkView(_ v: MTKView, drawableSizeWillChange s: CGSize) {{}}").unwrap();
        writeln!(s, "    func draw(in v: MTKView) {{").unwrap();
        writeln!(s, "        guard let d = v.currentDrawable else {{ return }}").unwrap();
        writeln!(s, "        rustOnFrame_{0}?(Unmanaged.passUnretained(d.texture).toOpaque())", window_id).unwrap();
        writeln!(s, "    }}").unwrap();
        writeln!(s, "}}").unwrap();
        writeln!(s).unwrap();
    }
}
