// Platform abstraction for iOS / macOS window hosting.

import SwiftUI

#if os(iOS)
import UIKit

func showSwiftUIWindow(
    rootView: some View,
    title: String,
    width: CGFloat,
    height: CGFloat
) {
    let controller = UIHostingController(rootView: rootView)
    
    guard let windowScene = UIApplication.shared.connectedScenes
        .compactMap({ $0 as? UIWindowScene }).first else { return }
    
    let window = UIWindow(windowScene: windowScene)
    window.rootViewController = controller
    window.makeKeyAndVisible()
}

func runApp(rootView: some View, title: String, width: CGFloat, height: CGFloat) {
    // On iOS, the app lifecycle is managed by UIApplicationMain.
    // We store the root view and present it when the scene connects.
    AppState.shared.pendingRootView = AnyView(rootView)
    AppState.shared.pendingTitle = title

    // If we're already running (called from SceneDelegate), just show.
    if UIApplication.shared.connectedScenes.isEmpty == false {
        showSwiftUIWindow(rootView: rootView, title: title, width: width, height: height)
    }
}

class AppState {
    static let shared = AppState()
    var pendingRootView: AnyView?
    var pendingTitle: String = ""
}

// Scene delegate for iOS app lifecycle
class SwiftUISceneDelegate: UIResponder, UIWindowSceneDelegate {
    var window: UIWindow?
    
    func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options connectionOptions: UIScene.ConnectionOptions) {
        guard let windowScene = scene as? UIWindowScene,
              let rootView = AppState.shared.pendingRootView else { return }
        
        let controller = UIHostingController(rootView: rootView)
        let window = UIWindow(windowScene: windowScene)
        window.rootViewController = controller
        window.makeKeyAndVisible()
        self.window = window
    }
}

#else // macOS

import AppKit

func showSwiftUIWindow(
    rootView: some View,
    title: String,
    width: CGFloat,
    height: CGFloat
) {
    NSApplication.shared.setActivationPolicy(.regular)
    let controller = NSHostingController(rootView: rootView)
    let window = NSWindow(
        contentRect: NSRect(x: 0, y: 0, width: width, height: height),
        styleMask: [.titled, .closable, .resizable, .miniaturizable],
        backing: .buffered, defer: false
    )
    window.contentViewController = controller
    window.title = title
    window.center()
    window.makeKeyAndOrderFront(nil)
    NSApplication.shared.activate(ignoringOtherApps: true)
    NSApplication.shared.run()
}

func runApp(rootView: some View, title: String, width: CGFloat, height: CGFloat) {
    showSwiftUIWindow(rootView: rootView, title: title, width: width, height: height)
}

#endif
