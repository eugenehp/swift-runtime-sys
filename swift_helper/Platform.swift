// Platform abstraction — configurable app container.

import SwiftUI

// ═══════════════════════════════════════════════════════════════════════════
// App configuration — set from Rust before launching
// ═══════════════════════════════════════════════════════════════════════════

class HostConfig {
    static let shared = HostConfig()

    var rootView: AnyView = AnyView(EmptyView())
    var title: String = "SwiftUI"
    var width: CGFloat = 400
    var height: CGFloat = 300

    // Window style
    var windowStyle: Int32 = 0  // 0=default, 1=borderless, 2=fullscreen, 3=floating, 4=transparent
    var resizable: Bool = true
    var minimizable: Bool = true
    var closable: Bool = true
    var minWidth: CGFloat = 0
    var minHeight: CGFloat = 0
    var maxWidth: CGFloat = 0
    var maxHeight: CGFloat = 0

    #if os(macOS)
    // macOS-specific
    var menuBarExtra: AnyView? = nil
    var menuBarExtraTitle: String = ""
    var menuBarExtraImage: String = ""
    var hideMenuBar: Bool = false
    var hideDock: Bool = false
    var titleBarHidden: Bool = false
    var backgroundMaterial: Int32 = 0 // 0=none, 1=thin, 2=regular, 3=thick, 4=ultra
    #endif

    // Multi-window
    var secondaryWindows: [(AnyView, String, CGFloat, CGFloat)] = []

    // Lifecycle callbacks
    var onAppear: (@convention(c) (UnsafeMutableRawPointer?) -> Void)? = nil
    var onDisappear: (@convention(c) (UnsafeMutableRawPointer?) -> Void)? = nil
    var onAppearData: UnsafeMutableRawPointer? = nil
    var onDisappearData: UnsafeMutableRawPointer? = nil
}

// ═══════════════════════════════════════════════════════════════════════════
// Configuration C API — called from Rust before app launch
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("host_set_window_style")
public func hostSetWindowStyle(_ style: Int32) {
    HostConfig.shared.windowStyle = style
}

@_cdecl("host_set_resizable")
public func hostSetResizable(_ resizable: Bool) {
    HostConfig.shared.resizable = resizable
}

@_cdecl("host_set_min_size")
public func hostSetMinSize(_ w: Float, _ h: Float) {
    HostConfig.shared.minWidth = CGFloat(w)
    HostConfig.shared.minHeight = CGFloat(h)
}

@_cdecl("host_set_max_size")
public func hostSetMaxSize(_ w: Float, _ h: Float) {
    HostConfig.shared.maxWidth = CGFloat(w)
    HostConfig.shared.maxHeight = CGFloat(h)
}

@_cdecl("host_set_on_appear")
public func hostSetOnAppear(
    _ callback: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ data: UnsafeMutableRawPointer?
) {
    HostConfig.shared.onAppear = callback
    HostConfig.shared.onAppearData = data
}

@_cdecl("host_set_on_disappear")
public func hostSetOnDisappear(
    _ callback: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ data: UnsafeMutableRawPointer?
) {
    HostConfig.shared.onDisappear = callback
    HostConfig.shared.onDisappearData = data
}

#if os(macOS)
@_cdecl("host_set_titlebar_hidden")
public func hostSetTitlebarHidden(_ hidden: Bool) {
    HostConfig.shared.titleBarHidden = hidden
}

@_cdecl("host_set_background_material")
public func hostSetBackgroundMaterial(_ material: Int32) {
    HostConfig.shared.backgroundMaterial = material
}

@_cdecl("host_hide_dock_icon")
public func hostHideDockIcon(_ hide: Bool) {
    HostConfig.shared.hideDock = hide
}

@_cdecl("host_set_menu_bar_extra")
public func hostSetMenuBarExtra(
    _ handle: ViewHandle,
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ imagePtr: UnsafePointer<UInt8>, _ imageLen: Int
) {
    HostConfig.shared.menuBarExtra = unboxView(handle)
    HostConfig.shared.menuBarExtraTitle = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    HostConfig.shared.menuBarExtraImage = String(bytes: UnsafeBufferPointer(start: imagePtr, count: imageLen), encoding: .utf8) ?? ""
}
#endif

@_cdecl("host_add_window")
public func hostAddWindow(
    _ handle: ViewHandle,
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ width: Float, _ height: Float
) {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    HostConfig.shared.secondaryWindows.append((unboxView(handle), title, CGFloat(width), CGFloat(height)))
}

// ═══════════════════════════════════════════════════════════════════════════
// App launch — uses HostConfig
// ═══════════════════════════════════════════════════════════════════════════

func runApp(rootView: some View, title: String, width: CGFloat, height: CGFloat) {
    let config = HostConfig.shared
    config.title = title
    config.width = width
    config.height = height

    // Wrap with lifecycle callbacks
    var wrapped: AnyView
    if config.onAppear != nil || config.onDisappear != nil {
        let onAppear = config.onAppear
        let onAppearData = config.onAppearData
        let onDisappear = config.onDisappear
        let onDisappearData = config.onDisappearData
        wrapped = AnyView(
            AnyView(rootView)
                .onAppear { onAppear?(onAppearData) }
                .onDisappear { onDisappear?(onDisappearData) }
        )
    } else {
        wrapped = AnyView(rootView)
    }

    config.rootView = wrapped

    #if os(macOS)
    launchMacOS(config)
    #else
    launchIOS(config)
    #endif
}

// ═══════════════════════════════════════════════════════════════════════════
// macOS launch
// ═══════════════════════════════════════════════════════════════════════════

#if os(macOS)
import AppKit

func launchMacOS(_ config: HostConfig) {
    if config.hideDock {
        NSApplication.shared.setActivationPolicy(.accessory)
    } else {
        NSApplication.shared.setActivationPolicy(.regular)
    }

    let controller = NSHostingController(rootView: config.rootView)

    // Window style mask
    var styleMask: NSWindow.StyleMask = []
    if config.closable { styleMask.insert(.closable) }
    if config.resizable { styleMask.insert(.resizable) }
    if config.minimizable { styleMask.insert(.miniaturizable) }

    switch config.windowStyle {
    case 1: // borderless
        styleMask = .borderless
    case 2: // fullscreen
        styleMask.insert(.titled)
    case 3: // floating
        styleMask.insert(.titled)
    case 4: // transparent
        styleMask = [.borderless, .fullSizeContentView]
    default:
        styleMask.insert(.titled)
    }

    if config.titleBarHidden {
        styleMask.insert(.fullSizeContentView)
    }

    let window = NSWindow(
        contentRect: NSRect(x: 0, y: 0, width: config.width, height: config.height),
        styleMask: styleMask,
        backing: .buffered, defer: false
    )
    window.contentViewController = controller
    window.title = config.title
    window.center()

    if config.titleBarHidden {
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
    }

    if config.windowStyle == 4 {
        window.isOpaque = false
        window.backgroundColor = .clear
    }

    if config.windowStyle == 3 {
        window.level = .floating
    }

    if config.minWidth > 0 || config.minHeight > 0 {
        window.minSize = NSSize(width: config.minWidth, height: config.minHeight)
    }
    if config.maxWidth > 0 || config.maxHeight > 0 {
        window.maxSize = NSSize(width: config.maxWidth, height: config.maxHeight)
    }

    window.makeKeyAndOrderFront(nil)

    if config.windowStyle == 2 {
        window.toggleFullScreen(nil)
    }

    // Secondary windows
    for (view, title, w, h) in config.secondaryWindows {
        let ctrl = NSHostingController(rootView: view)
        let win = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: w, height: h),
            styleMask: [.titled, .closable, .resizable],
            backing: .buffered, defer: false
        )
        win.contentViewController = ctrl
        win.title = title
        win.center()
        win.makeKeyAndOrderFront(nil)
    }

    NSApplication.shared.activate(ignoringOtherApps: true)
    NSApplication.shared.run()
}

#endif

// ═══════════════════════════════════════════════════════════════════════════
// iOS launch
// ═══════════════════════════════════════════════════════════════════════════

#if os(iOS)
import UIKit

func launchIOS(_ config: HostConfig) {
    AppState.shared.config = config

    if UIApplication.shared.connectedScenes.isEmpty == false {
        if let scene = UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene }).first {
            presentInScene(scene, config: config)
        }
    }
}

class AppState {
    static let shared = AppState()
    var config = HostConfig()
}

func presentInScene(_ scene: UIWindowScene, config: HostConfig) {
    let controller = UIHostingController(rootView: config.rootView)

    if config.windowStyle == 2 { // fullscreen
        controller.modalPresentationStyle = .fullScreen
    }

    let window = UIWindow(windowScene: scene)
    window.rootViewController = controller
    window.makeKeyAndVisible()
}

class SwiftUISceneDelegate: UIResponder, UIWindowSceneDelegate {
    var window: UIWindow?

    func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options: UIScene.ConnectionOptions) {
        guard let windowScene = scene as? UIWindowScene else { return }
        presentInScene(windowScene, config: AppState.shared.config)
    }
}

#endif
