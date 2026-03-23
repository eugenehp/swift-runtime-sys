import SwiftUI

#if os(visionOS)
import RealityKit
#endif

private var rustOnFrame: (@convention(c) (UnsafeMutableRawPointer?) -> Void)? = nil
private var rustOnInit: (@convention(c) () -> Void)? = nil

@_cdecl("ws_visionos_configure")
public func ws_visionos_configure(
    onInit: @convention(c) () -> Void,
    onFrame: @convention(c) (UnsafeMutableRawPointer?) -> Void
) {
    rustOnInit = onInit
    rustOnFrame = onFrame
}

@_cdecl("ws_visionos_launch")
public func ws_visionos_launch() {
    RustVisionApp.main()
}

struct RustVisionApp: App {
    var body: some SwiftUI.Scene {
        WindowGroup {
            RustContentView()
                .onAppear { rustOnInit?() }
        }
        #if os(visionOS)
        ImmersiveSpace(id: "rustImmersive") {
            RealityView { content in
                let anchor = AnchorEntity(.head)
                content.add(anchor)
            }
        }
        #endif
    }
}

#if canImport(UIKit) && !canImport(AppKit)
import MetalKit
struct RustContentView: UIViewRepresentable {
    func makeUIView(context: Context) -> MTKView {
        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())
        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false
        v.delegate = context.coordinator; return v
    }
    func updateUIView(_ v: MTKView, context: Context) {}
    func makeCoordinator() -> Coord { Coord() }
    class Coord: NSObject, MTKViewDelegate {
        func mtkView(_ v: MTKView, drawableSizeWillChange s: CGSize) {}
        func draw(in v: MTKView) {
            guard let d = v.currentDrawable else { return }
            rustOnFrame?(Unmanaged.passUnretained(d.texture).toOpaque())
        }
    }
}
#endif

#if canImport(AppKit)
import MetalKit
struct RustContentView: NSViewRepresentable {
    func makeNSView(context: Context) -> MTKView {
        let v = MTKView(frame: .zero, device: MTLCreateSystemDefaultDevice())
        v.colorPixelFormat = .bgra8Unorm; v.isPaused = false; v.enableSetNeedsDisplay = false
        v.delegate = context.coordinator; return v
    }
    func updateNSView(_ v: MTKView, context: Context) {}
    func makeCoordinator() -> Coord { Coord() }
    class Coord: NSObject, MTKViewDelegate {
        func mtkView(_ v: MTKView, drawableSizeWillChange s: CGSize) {}
        func draw(in v: MTKView) {
            guard let d = v.currentDrawable else { return }
            rustOnFrame?(Unmanaged.passUnretained(d.texture).toOpaque())
        }
    }
}
#endif
