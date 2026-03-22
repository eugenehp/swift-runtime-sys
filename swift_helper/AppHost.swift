// AppHost.swift — Swift-owned app lifecycle with Rust-provided content.
//
// Compile as a standalone app binary (not dylib):
//   xcrun swiftc AppHost.swift SwiftUIHelper.swift Platform.swift \
//     -o MyApp -target arm64-apple-macosx15.0 \
//     -sdk $(xcrun -sdk macosx --show-sdk-path) \
//     -L . -lRustApp
//
// Where libRustApp.dylib exports:
//   rust_register_scenes(registrar)
//   rust_build_main(cx, trigger) -> ViewHandle
//   rust_build_settings(cx, trigger) -> ViewHandle  (optional)
//   rust_build_immersive(cx, trigger) -> ViewHandle  (optional)

import SwiftUI

// ═══════════════════════════════════════════════════════════════════════════
// Scene registry — Rust tells Swift what scenes to create
// ═══════════════════════════════════════════════════════════════════════════

class SceneRegistry {
    static let shared = SceneRegistry()

    struct SceneEntry {
        let id: String
        let title: String
        let kind: SceneKind
        let buildFn: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> ViewHandle
        let userData: UnsafeMutableRawPointer?
        let width: CGFloat
        let height: CGFloat
    }

    enum SceneKind: Int32 {
        case windowGroup = 0
        case settings = 1
        case menuBarExtra = 2
        case documentGroup = 3
        #if os(visionOS)
        case immersiveSpace = 4
        case volumetric = 5
        #endif
    }

    var scenes: [SceneEntry] = []
    var menuBarImage: String = ""
}

// ═══════════════════════════════════════════════════════════════════════════
// C API — called from Rust before app launch
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("scene_register_window")
public func sceneRegisterWindow(
    _ idPtr: UnsafePointer<UInt8>, _ idLen: Int,
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ width: Float, _ height: Float,
    _ buildFn: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> ViewHandle,
    _ userData: UnsafeMutableRawPointer?
) {
    let id = String(bytes: UnsafeBufferPointer(start: idPtr, count: idLen), encoding: .utf8) ?? ""
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    SceneRegistry.shared.scenes.append(.init(
        id: id, title: title, kind: .windowGroup,
        buildFn: buildFn, userData: userData,
        width: CGFloat(width), height: CGFloat(height)
    ))
}

@_cdecl("scene_register_settings")
public func sceneRegisterSettings(
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ buildFn: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> ViewHandle,
    _ userData: UnsafeMutableRawPointer?
) {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    SceneRegistry.shared.scenes.append(.init(
        id: "settings", title: title, kind: .settings,
        buildFn: buildFn, userData: userData,
        width: 400, height: 300
    ))
}

@_cdecl("scene_register_menu_bar")
public func sceneRegisterMenuBar(
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ imagePtr: UnsafePointer<UInt8>, _ imageLen: Int,
    _ buildFn: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> ViewHandle,
    _ userData: UnsafeMutableRawPointer?
) {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    SceneRegistry.shared.menuBarImage = String(bytes: UnsafeBufferPointer(start: imagePtr, count: imageLen), encoding: .utf8) ?? "star"
    SceneRegistry.shared.scenes.append(.init(
        id: "menubar", title: title, kind: .menuBarExtra,
        buildFn: buildFn, userData: userData,
        width: 300, height: 400
    ))
}

@_cdecl("scene_launch")
public func sceneLaunch() {
    RustHostApp.main()
}

// ═══════════════════════════════════════════════════════════════════════════
// SwiftUI App — uses registered scenes
// ═══════════════════════════════════════════════════════════════════════════

struct RustHostApp: App {
    var body: some Scene {
        WindowGroup {
            RootView()
        }
    }
}

struct RootView: View {
    var body: some View {
        let entry = SceneRegistry.shared.scenes.first(where: { $0.kind == .windowGroup })
        if let entry = entry {
            RustReactiveView(entry: entry)
                .frame(minWidth: entry.width, minHeight: entry.height)
        } else {
            Text("No scene registered")
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Reactive view that calls the Rust build function
// ═══════════════════════════════════════════════════════════════════════════

@Observable
class SceneModel {
    var version: Int = 0
    func bump() { version += 1 }
}

struct RustReactiveView: View {
    let entry: SceneRegistry.SceneEntry
    @State private var model = SceneModel()

    var body: some View {
        let _ = model.version
        let handle = entry.buildFn(entry.userData, Unmanaged.passUnretained(model).toOpaque())
        unboxView(handle)
    }
}

@_cdecl("scene_trigger_rebuild")
public func sceneTriggerRebuild(_ modelPtr: UnsafeMutableRawPointer) {
    let model = Unmanaged<SceneModel>.fromOpaque(modelPtr).takeUnretainedValue()
    DispatchQueue.main.async { model.bump() }
}
