// WinitSwiftBridge.swift — unified Apple platform bridge for winit-swift
// Supports macOS (AppKit), iOS/tvOS (UIKit), and visionOS (UIKit + CompositorServices)
//
// macOS:    swiftc -emit-library -o libWinitSwift.dylib WinitSwiftBridge.swift \
//            -framework Foundation -framework QuartzCore -framework Metal \
//            -framework CoreGraphics -framework AppKit -framework CoreHaptics -O
//
// visionOS: swiftc -emit-library -o libWinitSwift.dylib WinitSwiftBridge.swift \
//            -framework Foundation -framework QuartzCore -framework Metal \
//            -framework UIKit -framework CoreHaptics -framework RealityKit \
//            -framework CompositorServices -O

import Foundation
import QuartzCore
import Metal
import CoreGraphics

#if canImport(AppKit)
import AppKit
#endif

#if canImport(UIKit)
import UIKit
#endif

#if canImport(CoreHaptics)
import CoreHaptics
#endif

#if canImport(RealityKit)
import RealityKit
#endif

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Shared State
// ═══════════════════════════════════════════════════════════════════════════

private var activeWindows: [UInt64: AnyObject] = [:]
private var nextWindowId: UInt64 = 1
private var metalDevice: MTLDevice?
private var eventCallback: (
    @convention(c) (UInt32, UInt64, Int64, Int64, Double, Double) -> Void
)?

// Event constants (shared across all platforms)
let EV_RESIZED: UInt32           = 1
let EV_MOVED: UInt32             = 2
let EV_CLOSE_REQUESTED: UInt32   = 3
let EV_DESTROYED: UInt32         = 4
let EV_FOCUSED: UInt32           = 5
let EV_UNFOCUSED: UInt32         = 6
let EV_KEY_DOWN: UInt32          = 7
let EV_KEY_UP: UInt32            = 8
let EV_MOUSE_MOVED: UInt32       = 9
let EV_MOUSE_BTN_DOWN: UInt32    = 10
let EV_MOUSE_BTN_UP: UInt32      = 11
let EV_SCROLL: UInt32            = 12
let EV_SCALE_FACTOR: UInt32      = 13
let EV_REDRAW: UInt32            = 14
let EV_TOUCH_START: UInt32       = 15
let EV_TOUCH_MOVE: UInt32        = 16
let EV_TOUCH_END: UInt32         = 17
let EV_THEME_CHANGED: UInt32     = 18
let EV_RESUMED: UInt32           = 19
let EV_SUSPENDED: UInt32         = 20
let EV_DRAG_ENTERED: UInt32      = 21
let EV_DRAG_LEFT: UInt32         = 22
let EV_DROPPED: UInt32           = 23
let EV_OCCLUDED: UInt32          = 24
let EV_MODIFIERS: UInt32         = 25
let EV_MOUSE_ENTERED: UInt32     = 26
let EV_MOUSE_LEFT: UInt32        = 27
let EV_PINCH: UInt32             = 28
let EV_ROTATION: UInt32          = 29

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Init (all platforms)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("ws_init")
public func ws_init(
    callback: @convention(c) (UInt32, UInt64, Int64, Int64, Double, Double) -> Void
) -> Bool {
    eventCallback = callback
    metalDevice = MTLCreateSystemDefaultDevice()
    return true
}

@_cdecl("ws_metal_device")
public func ws_metal_device() -> UnsafeMutableRawPointer? {
    guard let d = metalDevice else { return nil }
    return Unmanaged.passUnretained(d).toOpaque()
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - macOS Window (AppKit)
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(AppKit) && !targetEnvironment(macCatalyst)

class WinitView: NSView, CALayerDelegate {
    var metalLayer: CAMetalLayer?
    var wid: UInt64 = 0
    var trk: NSTrackingArea?
    var wantsHDR = false

    override var acceptsFirstResponder: Bool { true }
    override var wantsUpdateLayer: Bool { true }

    override func makeBackingLayer() -> CALayer {
        let l = CAMetalLayer()
        l.device = metalDevice
        l.pixelFormat = wantsHDR ? .rgba16Float : .bgra8Unorm
        l.framebufferOnly = true
        l.contentsScale = window?.backingScaleFactor ?? 2.0
        if wantsHDR {
            l.wantsExtendedDynamicRangeContent = true
            if let cs = CGColorSpace(name: CGColorSpace.extendedLinearDisplayP3) { l.colorspace = cs }
        }
        metalLayer = l; return l
    }

    override func updateTrackingAreas() {
        if let t = trk { removeTrackingArea(t) }
        let t = NSTrackingArea(rect: bounds, options: [.activeAlways, .mouseMoved, .mouseEnteredAndExited, .inVisibleRect], owner: self, userInfo: nil)
        addTrackingArea(t); trk = t
    }

    override func viewDidChangeBackingProperties() {
        super.viewDidChangeBackingProperties()
        guard let l = metalLayer, let w = window else { return }
        l.contentsScale = w.backingScaleFactor
        eventCallback?(EV_SCALE_FACTOR, wid, 0, 0, Double(w.backingScaleFactor), 0)
    }

    override func setFrameSize(_ s: NSSize) {
        super.setFrameSize(s)
        let b = convertToBacking(bounds).size; metalLayer?.drawableSize = b
        eventCallback?(EV_RESIZED, wid, Int64(b.width), Int64(b.height), 0, 0)
    }

    private func loc(_ e: NSEvent) -> (Double, Double) {
        let p = convert(e.locationInWindow, from: nil)
        return (Double(p.x), Double(bounds.height - p.y))
    }

    override func mouseMoved(with e: NSEvent)        { let p = loc(e); eventCallback?(EV_MOUSE_MOVED, wid, 0, 0, p.0, p.1) }
    override func mouseDragged(with e: NSEvent)       { let p = loc(e); eventCallback?(EV_MOUSE_MOVED, wid, 0, 0, p.0, p.1) }
    override func rightMouseDragged(with e: NSEvent)  { let p = loc(e); eventCallback?(EV_MOUSE_MOVED, wid, 0, 0, p.0, p.1) }
    override func mouseDown(with e: NSEvent)          { eventCallback?(EV_MOUSE_BTN_DOWN, wid, 0, 0, 0, 0) }
    override func mouseUp(with e: NSEvent)            { eventCallback?(EV_MOUSE_BTN_UP, wid, 0, 0, 0, 0) }
    override func rightMouseDown(with e: NSEvent)     { eventCallback?(EV_MOUSE_BTN_DOWN, wid, 1, 0, 0, 0) }
    override func rightMouseUp(with e: NSEvent)       { eventCallback?(EV_MOUSE_BTN_UP, wid, 1, 0, 0, 0) }
    override func otherMouseDown(with e: NSEvent)     { eventCallback?(EV_MOUSE_BTN_DOWN, wid, Int64(e.buttonNumber), 0, 0, 0) }
    override func otherMouseUp(with e: NSEvent)       { eventCallback?(EV_MOUSE_BTN_UP, wid, Int64(e.buttonNumber), 0, 0, 0) }
    override func mouseEntered(with e: NSEvent)       { eventCallback?(EV_MOUSE_ENTERED, wid, 0, 0, 0, 0) }
    override func mouseExited(with e: NSEvent)        { eventCallback?(EV_MOUSE_LEFT, wid, 0, 0, 0, 0) }
    override func scrollWheel(with e: NSEvent)        { eventCallback?(EV_SCROLL, wid, 0, 0, Double(e.scrollingDeltaX), Double(e.scrollingDeltaY)) }
    override func keyDown(with e: NSEvent)            { eventCallback?(EV_KEY_DOWN, wid, Int64(e.keyCode), e.isARepeat ? 1 : 0, 0, 0) }
    override func keyUp(with e: NSEvent)              { eventCallback?(EV_KEY_UP, wid, Int64(e.keyCode), 0, 0, 0) }
    override func flagsChanged(with e: NSEvent)       { eventCallback?(EV_MODIFIERS, wid, Int64(e.modifierFlags.rawValue), 0, 0, 0) }
    override func magnify(with e: NSEvent)            { eventCallback?(EV_PINCH, wid, 0, 0, Double(e.magnification), 0) }
    override func rotate(with e: NSEvent)             { eventCallback?(EV_ROTATION, wid, 0, 0, Double(e.rotation), 0) }
}

class WinitDelegate: NSObject, NSWindowDelegate {
    var wid: UInt64 = 0
    func windowShouldClose(_ s: NSWindow) -> Bool { eventCallback?(EV_CLOSE_REQUESTED, wid, 0, 0, 0, 0); return false }
    func windowDidBecomeKey(_ n: Notification)    { eventCallback?(EV_FOCUSED, wid, 0, 0, 0, 0) }
    func windowDidResignKey(_ n: Notification)    { eventCallback?(EV_UNFOCUSED, wid, 0, 0, 0, 0) }
    func windowDidMove(_ n: Notification) {
        guard let w = n.object as? NSWindow else { return }
        eventCallback?(EV_MOVED, wid, Int64(w.frame.origin.x), Int64(w.frame.origin.y), 0, 0)
    }
    func windowDidChangeOcclusionState(_ n: Notification) {
        guard let w = n.object as? NSWindow else { return }
        eventCallback?(EV_OCCLUDED, wid, w.occlusionState.contains(.visible) ? 0 : 1, 0, 0, 0)
    }
    func windowDidChangeEffectiveAppearance(_ n: Notification) {
        guard let w = n.object as? NSWindow else { return }
        let dark = w.effectiveAppearance.bestMatch(from: [.darkAqua, .aqua]) == .darkAqua
        eventCallback?(EV_THEME_CHANGED, wid, dark ? 1 : 0, 0, 0, 0)
    }
}

@_cdecl("ws_create_window")
public func ws_create_window(_ tP: UnsafePointer<UInt8>, _ tL: Int, _ w: Double, _ h: Double, _ flags: UInt64) -> UInt64 {
    let title = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? "Window"
    let id = nextWindowId; nextWindowId += 1

    var style: NSWindow.StyleMask = [.titled, .closable, .miniaturizable, .resizable]
    if flags & 4 != 0 { style.insert(.fullSizeContentView) }

    let win = NSWindow(contentRect: NSRect(x: 0, y: 0, width: w, height: h), styleMask: style, backing: .buffered, defer: false)
    win.title = title; win.center()
    if flags & 1 != 0 { win.isOpaque = false; win.backgroundColor = .clear }
    if flags & 2 != 0 { win.titlebarAppearsTransparent = true; win.titleVisibility = .hidden }

    let view = WinitView(frame: win.contentView!.bounds)
    view.wid = id; view.wantsHDR = flags & 8 != 0; view.wantsLayer = true
    view.autoresizingMask = [.width, .height]; win.contentView = view

    let del = WinitDelegate(); del.wid = id; win.delegate = del
    win.makeKeyAndOrderFront(nil)
    activeWindows[id] = win; activeWindows[id + 100000] = del
    return id
}

@_cdecl("ws_destroy_window")
public func ws_destroy_window(_ id: UInt64) {
    (activeWindows[id] as? NSWindow)?.close()
    activeWindows.removeValue(forKey: id); activeWindows.removeValue(forKey: id + 100000)
    eventCallback?(EV_DESTROYED, id, 0, 0, 0, 0)
}

private func win(_ id: UInt64) -> NSWindow? { activeWindows[id] as? NSWindow }
private func mview(_ id: UInt64) -> WinitView? { win(id)?.contentView as? WinitView }

@_cdecl("ws_window_metal_layer")
public func ws_window_metal_layer(_ id: UInt64) -> UnsafeMutableRawPointer? {
    guard let l = mview(id)?.metalLayer else { return nil }; return Unmanaged.passUnretained(l).toOpaque()
}

@_cdecl("ws_window_set_title")
public func ws_window_set_title(_ id: UInt64, _ p: UnsafePointer<UInt8>, _ l: Int) {
    win(id)?.title = String(bytes: UnsafeBufferPointer(start: p, count: l), encoding: .utf8) ?? ""
}

@_cdecl("ws_window_title")
public func ws_window_title(_ id: UInt64, _ buf: UnsafeMutablePointer<UInt8>, _ len: Int) -> Int {
    guard let t = win(id)?.title else { return 0 }
    let b = Array(t.utf8); let c = min(b.count, len); for i in 0..<c { buf[i] = b[i] }; return c
}

@_cdecl("ws_window_size")
public func ws_window_size(_ id: UInt64, _ oW: UnsafeMutablePointer<UInt32>, _ oH: UnsafeMutablePointer<UInt32>) {
    guard let v = mview(id) else { return }
    let s = v.convertToBacking(v.bounds).size; oW.pointee = UInt32(s.width); oH.pointee = UInt32(s.height)
}

@_cdecl("ws_window_scale_factor")
public func ws_window_scale_factor(_ id: UInt64) -> Double { Double(win(id)?.backingScaleFactor ?? 2.0) }

@_cdecl("ws_window_outer_position")
public func ws_window_outer_position(_ id: UInt64, _ oX: UnsafeMutablePointer<Int32>, _ oY: UnsafeMutablePointer<Int32>) {
    guard let f = win(id)?.frame else { return }; oX.pointee = Int32(f.origin.x); oY.pointee = Int32(f.origin.y)
}

@_cdecl("ws_window_set_outer_position")
public func ws_window_set_outer_position(_ id: UInt64, _ x: Int32, _ y: Int32) {
    win(id)?.setFrameOrigin(NSPoint(x: Int(x), y: Int(y)))
}

@_cdecl("ws_window_outer_size")
public func ws_window_outer_size(_ id: UInt64, _ oW: UnsafeMutablePointer<UInt32>, _ oH: UnsafeMutablePointer<UInt32>) {
    guard let f = win(id)?.frame else { return }; oW.pointee = UInt32(f.width); oH.pointee = UInt32(f.height)
}

@_cdecl("ws_window_safe_area")
public func ws_window_safe_area(_ id: UInt64, _ t: UnsafeMutablePointer<UInt32>, _ l: UnsafeMutablePointer<UInt32>,
                                 _ b: UnsafeMutablePointer<UInt32>, _ r: UnsafeMutablePointer<UInt32>) {
    guard let v = mview(id) else { return }
    if v.responds(to: Selector(("safeAreaInsets"))) {
        let ins = v.safeAreaInsets; let sf = win(id)?.backingScaleFactor ?? 1
        t.pointee = UInt32(ins.top * sf); l.pointee = UInt32(ins.left * sf)
        b.pointee = UInt32(ins.bottom * sf); r.pointee = UInt32(ins.right * sf)
    }
}

@_cdecl("ws_window_theme")
public func ws_window_theme(_ id: UInt64) -> UInt8 {
    win(id)?.effectiveAppearance.bestMatch(from: [.darkAqua, .aqua]) == .darkAqua ? 1 : 0
}

@_cdecl("ws_window_has_focus")
public func ws_window_has_focus(_ id: UInt64) -> Bool { win(id)?.isKeyWindow ?? false }
@_cdecl("ws_window_is_visible")
public func ws_window_is_visible(_ id: UInt64) -> Bool { win(id)?.isVisible ?? false }
@_cdecl("ws_window_is_minimized")
public func ws_window_is_minimized(_ id: UInt64) -> Bool { win(id)?.isMiniaturized ?? false }
@_cdecl("ws_window_is_maximized")
public func ws_window_is_maximized(_ id: UInt64) -> Bool { win(id)?.isZoomed ?? false }
@_cdecl("ws_window_is_fullscreen")
public func ws_window_is_fullscreen(_ id: UInt64) -> Bool { win(id)?.styleMask.contains(.fullScreen) ?? false }
@_cdecl("ws_window_is_resizable")
public func ws_window_is_resizable(_ id: UInt64) -> Bool { win(id)?.styleMask.contains(.resizable) ?? true }
@_cdecl("ws_window_is_decorated")
public func ws_window_is_decorated(_ id: UInt64) -> Bool { win(id)?.styleMask.contains(.titled) ?? true }
@_cdecl("ws_window_raw_handle")
public func ws_window_raw_handle(_ id: UInt64) -> UnsafeMutableRawPointer? {
    guard let v = win(id)?.contentView else { return nil }; return Unmanaged.passUnretained(v).toOpaque()
}

@_cdecl("ws_window_set_visible")
public func ws_window_set_visible(_ id: UInt64, _ v: Bool) { if v { win(id)?.makeKeyAndOrderFront(nil) } else { win(id)?.orderOut(nil) } }
@_cdecl("ws_window_set_fullscreen")
public func ws_window_set_fullscreen(_ id: UInt64, _ fs: Bool) {
    guard let w = win(id) else { return }; if fs != w.styleMask.contains(.fullScreen) { w.toggleFullScreen(nil) }
}
@_cdecl("ws_window_set_minimized")
public func ws_window_set_minimized(_ id: UInt64, _ v: Bool) { if v { win(id)?.miniaturize(nil) } else { win(id)?.deminiaturize(nil) } }
@_cdecl("ws_window_set_maximized")
public func ws_window_set_maximized(_ id: UInt64, _ v: Bool) {
    guard let w = win(id) else { return }; if v != w.isZoomed { w.zoom(nil) }
}
@_cdecl("ws_window_set_resizable")
public func ws_window_set_resizable(_ id: UInt64, _ v: Bool) {
    guard let w = win(id) else { return }; if v { w.styleMask.insert(.resizable) } else { w.styleMask.remove(.resizable) }
}
@_cdecl("ws_window_request_redraw")
public func ws_window_request_redraw(_ id: UInt64) { eventCallback?(EV_REDRAW, id, 0, 0, 0, 0) }
@_cdecl("ws_window_set_min_size")
public func ws_window_set_min_size(_ id: UInt64, _ w: Double, _ h: Double) { win(id)?.minSize = NSSize(width: w, height: h) }
@_cdecl("ws_window_set_max_size")
public func ws_window_set_max_size(_ id: UInt64, _ w: Double, _ h: Double) { win(id)?.maxSize = NSSize(width: w, height: h) }
@_cdecl("ws_window_focus")
public func ws_window_focus(_ id: UInt64) { win(id)?.makeKeyAndOrderFront(nil) }
@_cdecl("ws_window_set_decorations")
public func ws_window_set_decorations(_ id: UInt64, _ v: Bool) {
    guard let w = win(id) else { return }
    if v { w.styleMask.insert([.titled, .closable, .miniaturizable]) } else { w.styleMask.remove([.titled, .closable, .miniaturizable]) }
}
@_cdecl("ws_window_set_blur")
public func ws_window_set_blur(_ id: UInt64, _ v: Bool) {
    guard let w = win(id) else { return }
    typealias BlurFn = @convention(c) (UnsafeMutableRawPointer?, Int, Int) -> Int32
    typealias ConnFn = @convention(c) () -> UnsafeMutableRawPointer?
    if let blurSym = dlsym(dlopen(nil, 2), "CGSSetWindowBackgroundBlurRadius"),
       let connSym = dlsym(dlopen(nil, 2), "CGSMainConnectionID") {
        let getConn: ConnFn = unsafeBitCast(connSym, to: ConnFn.self)
        let setBlur: BlurFn = unsafeBitCast(blurSym, to: BlurFn.self)
        _ = setBlur(getConn(), w.windowNumber, v ? 20 : 0)
    }
}
@_cdecl("ws_window_set_content_protected")
public func ws_window_set_content_protected(_ id: UInt64, _ v: Bool) { win(id)?.sharingType = v ? .none : .readOnly }
@_cdecl("ws_window_set_window_level")
public func ws_window_set_window_level(_ id: UInt64, _ level: Int32) {
    switch level { case 1: win(id)?.level = .floating; case 2: win(id)?.level = .modalPanel; default: win(id)?.level = .normal }
}
@_cdecl("ws_window_request_attention")
public func ws_window_request_attention(_ id: UInt64, _ critical: Bool) {
    NSApp.requestUserAttention(critical ? .criticalRequest : .informationalRequest)
}
@_cdecl("ws_window_set_cursor_visible")
public func ws_window_set_cursor_visible(_ id: UInt64, _ v: Bool) { if v { NSCursor.unhide() } else { NSCursor.hide() } }
@_cdecl("ws_window_set_cursor_position")
public func ws_window_set_cursor_position(_ id: UInt64, _ x: Double, _ y: Double) {
    guard let w = win(id), let screen = w.screen else { return }
    CGWarpMouseCursorPosition(CGPoint(x: w.frame.origin.x + x, y: screen.frame.height - w.frame.origin.y - y))
}
@_cdecl("ws_window_set_theme")
public func ws_window_set_theme(_ id: UInt64, _ dark: Int8) {
    switch dark { case 0: win(id)?.appearance = NSAppearance(named: .aqua); case 1: win(id)?.appearance = NSAppearance(named: .darkAqua); default: win(id)?.appearance = nil }
}
@_cdecl("ws_window_set_transparent")
public func ws_window_set_transparent(_ id: UInt64, _ v: Bool) {
    guard let w = win(id) else { return }; w.isOpaque = !v; w.backgroundColor = v ? .clear : .windowBackgroundColor
}
@_cdecl("ws_window_drag")
public func ws_window_drag(_ id: UInt64) {
    guard let w = win(id), let e = NSApp.currentEvent else { return }; w.performDrag(with: e)
}

// ── macOS Event Loop ──

@_cdecl("ws_run_event_loop")
public func ws_run_event_loop() {
    let app = NSApplication.shared
    app.setActivationPolicy(.regular); app.activate(ignoringOtherApps: true); app.run()
}

@_cdecl("ws_poll_events")
public func ws_poll_events() -> Bool {
    let app = NSApplication.shared
    while let e = app.nextEvent(matching: .any, until: nil, inMode: .default, dequeue: true) { app.sendEvent(e) }
    return true
}

@_cdecl("ws_poll_events_timeout")
public func ws_poll_events_timeout(_ secs: Double) -> Bool {
    let app = NSApplication.shared; let dl = Date(timeIntervalSinceNow: secs)
    while let e = app.nextEvent(matching: .any, until: dl, inMode: .default, dequeue: true) { app.sendEvent(e) }
    return true
}

@_cdecl("ws_stop_event_loop")
public func ws_stop_event_loop() {
    NSApplication.shared.stop(nil)
    if let e = NSEvent.otherEvent(with: .applicationDefined, location: .zero, modifierFlags: [],
                                   timestamp: 0, windowNumber: 0, context: nil, subtype: 0, data1: 0, data2: 0) {
        NSApplication.shared.postEvent(e, atStart: true)
    }
}

// ── macOS Monitors ──

@_cdecl("ws_monitor_count")
public func ws_monitor_count() -> Int { NSScreen.screens.count }

@_cdecl("ws_monitor_info")
public func ws_monitor_info(_ idx: Int, _ x: UnsafeMutablePointer<Int32>, _ y: UnsafeMutablePointer<Int32>,
                             _ w: UnsafeMutablePointer<UInt32>, _ h: UnsafeMutablePointer<UInt32>,
                             _ scale: UnsafeMutablePointer<Double>, _ np: UnsafeMutablePointer<UInt8>, _ nl: Int) -> Int {
    let screens = NSScreen.screens; guard idx < screens.count else { return 0 }
    let s = screens[idx]; let f = s.frame
    x.pointee = Int32(f.origin.x); y.pointee = Int32(f.origin.y)
    w.pointee = UInt32(f.width); h.pointee = UInt32(f.height)
    scale.pointee = Double(s.backingScaleFactor)
    let n = s.localizedName; let b = Array(n.utf8); let c = min(b.count, nl)
    for i in 0..<c { np[i] = b[i] }; return c
}

@_cdecl("ws_monitor_refresh_rate")
public func ws_monitor_refresh_rate(_ idx: Int) -> Double {
    let screens = NSScreen.screens; guard idx < screens.count else { return 60.0 }
    return Double(screens[idx].maximumFramesPerSecond)
}

// ── macOS Accessibility ──

@_cdecl("ws_accessibility_is_voiceover_running")
public func ws_accessibility_is_voiceover_running() -> Bool { NSWorkspace.shared.isVoiceOverEnabled }
@_cdecl("ws_accessibility_is_reduce_motion")
public func ws_accessibility_is_reduce_motion() -> Bool { NSWorkspace.shared.accessibilityDisplayShouldReduceMotion }
@_cdecl("ws_accessibility_is_reduce_transparency")
public func ws_accessibility_is_reduce_transparency() -> Bool { NSWorkspace.shared.accessibilityDisplayShouldReduceTransparency }
@_cdecl("ws_accessibility_is_high_contrast")
public func ws_accessibility_is_high_contrast() -> Bool { NSWorkspace.shared.accessibilityDisplayShouldIncreaseContrast }

#endif // canImport(AppKit)

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - iOS / visionOS Window (UIKit)
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(UIKit) && !canImport(AppKit)

class WinitUIView: UIView {
    var wid: UInt64 = 0

    override class var layerClass: AnyClass { CAMetalLayer.self }

    var metalLayer: CAMetalLayer { layer as! CAMetalLayer }

    override func didMoveToWindow() {
        super.didMoveToWindow()
        guard let w = window else { return }
        let l = metalLayer
        l.device = metalDevice
        l.pixelFormat = .bgra8Unorm
        l.contentsScale = w.screen.scale
        l.drawableSize = CGSize(width: bounds.width * w.screen.scale, height: bounds.height * w.screen.scale)
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        guard let w = window else { return }
        let scale = w.screen.scale
        metalLayer.drawableSize = CGSize(width: bounds.width * scale, height: bounds.height * scale)
        eventCallback?(EV_RESIZED, wid, Int64(bounds.width * scale), Int64(bounds.height * scale), 0, 0)
    }

    // Touch handling
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        for t in touches {
            let p = t.location(in: self)
            eventCallback?(EV_TOUCH_START, wid, 0, 0, Double(p.x), Double(p.y))
        }
    }
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        for t in touches {
            let p = t.location(in: self)
            eventCallback?(EV_TOUCH_MOVE, wid, 0, 0, Double(p.x), Double(p.y))
        }
    }
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        for t in touches {
            let p = t.location(in: self)
            eventCallback?(EV_TOUCH_END, wid, 0, 0, Double(p.x), Double(p.y))
        }
    }
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        for t in touches {
            let p = t.location(in: self)
            eventCallback?(EV_TOUCH_END, wid, 0, 0, Double(p.x), Double(p.y))
        }
    }

    // Keyboard (with connected keyboard on iPad/visionOS)
    override var canBecomeFirstResponder: Bool { true }
    override func pressesBegan(_ presses: Set<UIPress>, with event: UIPressesEvent?) {
        for p in presses {
            if let key = p.key { eventCallback?(EV_KEY_DOWN, wid, Int64(key.keyCode.rawValue), 0, 0, 0) }
        }
        super.pressesBegan(presses, with: event)
    }
    override func pressesEnded(_ presses: Set<UIPress>, with event: UIPressesEvent?) {
        for p in presses {
            if let key = p.key { eventCallback?(EV_KEY_UP, wid, Int64(key.keyCode.rawValue), 0, 0, 0) }
        }
        super.pressesEnded(presses, with: event)
    }
}

// Store UIWindow + scene delegate refs
private var uiWindows: [UInt64: UIWindow] = [:]

@_cdecl("ws_create_window")
public func ws_create_window(_ tP: UnsafePointer<UInt8>, _ tL: Int, _ w: Double, _ h: Double, _ flags: UInt64) -> UInt64 {
    let id = nextWindowId; nextWindowId += 1

    // On iOS/visionOS we need a UIWindowScene
    guard let scene = UIApplication.shared.connectedScenes.first as? UIWindowScene else {
        // Fallback: create without scene (limited)
        let uiWin = UIWindow(frame: UIScreen.main.bounds)
        let view = WinitUIView(frame: uiWin.bounds)
        view.wid = id; view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        let vc = UIViewController(); vc.view = view
        uiWin.rootViewController = vc
        uiWin.makeKeyAndVisible()
        uiWindows[id] = uiWin
        activeWindows[id] = uiWin
        return id
    }

    let uiWin = UIWindow(windowScene: scene)
    let view = WinitUIView(frame: uiWin.bounds)
    view.wid = id; view.autoresizingMask = [.flexibleWidth, .flexibleHeight]
    let vc = UIViewController(); vc.view = view
    uiWin.rootViewController = vc
    uiWin.makeKeyAndVisible()
    uiWindows[id] = uiWin
    activeWindows[id] = uiWin
    return id
}

@_cdecl("ws_destroy_window")
public func ws_destroy_window(_ id: UInt64) {
    uiWindows[id]?.isHidden = true
    uiWindows.removeValue(forKey: id)
    activeWindows.removeValue(forKey: id)
    eventCallback?(EV_DESTROYED, id, 0, 0, 0, 0)
}

private func uiWin(_ id: UInt64) -> UIWindow? { uiWindows[id] }
private func uiView(_ id: UInt64) -> WinitUIView? { uiWin(id)?.rootViewController?.view as? WinitUIView }

@_cdecl("ws_window_metal_layer")
public func ws_window_metal_layer(_ id: UInt64) -> UnsafeMutableRawPointer? {
    guard let v = uiView(id) else { return nil }; return Unmanaged.passUnretained(v.metalLayer).toOpaque()
}

@_cdecl("ws_window_set_title")
public func ws_window_set_title(_ id: UInt64, _ p: UnsafePointer<UInt8>, _ l: Int) { /* titles not visible on iOS/visionOS */ }

@_cdecl("ws_window_title")
public func ws_window_title(_ id: UInt64, _ buf: UnsafeMutablePointer<UInt8>, _ len: Int) -> Int { return 0 }

@_cdecl("ws_window_size")
public func ws_window_size(_ id: UInt64, _ oW: UnsafeMutablePointer<UInt32>, _ oH: UnsafeMutablePointer<UInt32>) {
    guard let v = uiView(id), let w = v.window else { return }
    let scale = w.screen.scale
    oW.pointee = UInt32(v.bounds.width * scale); oH.pointee = UInt32(v.bounds.height * scale)
}

@_cdecl("ws_window_scale_factor")
public func ws_window_scale_factor(_ id: UInt64) -> Double {
    Double(uiWin(id)?.screen.scale ?? UIScreen.main.scale)
}

@_cdecl("ws_window_outer_position")
public func ws_window_outer_position(_ id: UInt64, _ oX: UnsafeMutablePointer<Int32>, _ oY: UnsafeMutablePointer<Int32>) {
    oX.pointee = 0; oY.pointee = 0 // iOS/visionOS windows are always at origin
}

@_cdecl("ws_window_set_outer_position")
public func ws_window_set_outer_position(_ id: UInt64, _ x: Int32, _ y: Int32) { /* no-op on iOS/visionOS */ }

@_cdecl("ws_window_outer_size")
public func ws_window_outer_size(_ id: UInt64, _ oW: UnsafeMutablePointer<UInt32>, _ oH: UnsafeMutablePointer<UInt32>) {
    guard let f = uiWin(id)?.frame else { return }; oW.pointee = UInt32(f.width); oH.pointee = UInt32(f.height)
}

@_cdecl("ws_window_safe_area")
public func ws_window_safe_area(_ id: UInt64, _ t: UnsafeMutablePointer<UInt32>, _ l: UnsafeMutablePointer<UInt32>,
                                 _ b: UnsafeMutablePointer<UInt32>, _ r: UnsafeMutablePointer<UInt32>) {
    guard let v = uiView(id), let w = v.window else { return }
    let ins = v.safeAreaInsets; let scale = w.screen.scale
    t.pointee = UInt32(ins.top * scale); l.pointee = UInt32(ins.left * scale)
    b.pointee = UInt32(ins.bottom * scale); r.pointee = UInt32(ins.right * scale)
}

@_cdecl("ws_window_theme")
public func ws_window_theme(_ id: UInt64) -> UInt8 {
    uiWin(id)?.traitCollection.userInterfaceStyle == .dark ? 1 : 0
}

@_cdecl("ws_window_has_focus")
public func ws_window_has_focus(_ id: UInt64) -> Bool { uiWin(id)?.isKeyWindow ?? false }
@_cdecl("ws_window_is_visible")
public func ws_window_is_visible(_ id: UInt64) -> Bool { !(uiWin(id)?.isHidden ?? true) }
@_cdecl("ws_window_is_minimized")
public func ws_window_is_minimized(_ id: UInt64) -> Bool { false }
@_cdecl("ws_window_is_maximized")
public func ws_window_is_maximized(_ id: UInt64) -> Bool { true }
@_cdecl("ws_window_is_fullscreen")
public func ws_window_is_fullscreen(_ id: UInt64) -> Bool { true }
@_cdecl("ws_window_is_resizable")
public func ws_window_is_resizable(_ id: UInt64) -> Bool { false }
@_cdecl("ws_window_is_decorated")
public func ws_window_is_decorated(_ id: UInt64) -> Bool { false }
@_cdecl("ws_window_raw_handle")
public func ws_window_raw_handle(_ id: UInt64) -> UnsafeMutableRawPointer? {
    guard let v = uiView(id) else { return nil }; return Unmanaged.passUnretained(v).toOpaque()
}

@_cdecl("ws_window_set_visible")
public func ws_window_set_visible(_ id: UInt64, _ v: Bool) { uiWin(id)?.isHidden = !v }
@_cdecl("ws_window_set_fullscreen")
public func ws_window_set_fullscreen(_ id: UInt64, _ fs: Bool) { /* always fullscreen */ }
@_cdecl("ws_window_set_minimized")
public func ws_window_set_minimized(_ id: UInt64, _ v: Bool) { /* no-op */ }
@_cdecl("ws_window_set_maximized")
public func ws_window_set_maximized(_ id: UInt64, _ v: Bool) { /* no-op */ }
@_cdecl("ws_window_set_resizable")
public func ws_window_set_resizable(_ id: UInt64, _ v: Bool) { /* no-op */ }
@_cdecl("ws_window_request_redraw")
public func ws_window_request_redraw(_ id: UInt64) { eventCallback?(EV_REDRAW, id, 0, 0, 0, 0) }
@_cdecl("ws_window_set_min_size")
public func ws_window_set_min_size(_ id: UInt64, _ w: Double, _ h: Double) { /* no-op */ }
@_cdecl("ws_window_set_max_size")
public func ws_window_set_max_size(_ id: UInt64, _ w: Double, _ h: Double) { /* no-op */ }
@_cdecl("ws_window_focus")
public func ws_window_focus(_ id: UInt64) { uiWin(id)?.makeKeyAndVisible() }
@_cdecl("ws_window_set_decorations")
public func ws_window_set_decorations(_ id: UInt64, _ v: Bool) { /* no-op */ }
@_cdecl("ws_window_set_blur")
public func ws_window_set_blur(_ id: UInt64, _ v: Bool) { /* no-op */ }
@_cdecl("ws_window_set_content_protected")
public func ws_window_set_content_protected(_ id: UInt64, _ v: Bool) { /* no-op */ }
@_cdecl("ws_window_set_window_level")
public func ws_window_set_window_level(_ id: UInt64, _ level: Int32) { /* no-op */ }
@_cdecl("ws_window_request_attention")
public func ws_window_request_attention(_ id: UInt64, _ critical: Bool) { /* no-op */ }
@_cdecl("ws_window_set_cursor_visible")
public func ws_window_set_cursor_visible(_ id: UInt64, _ v: Bool) { /* no cursor on iOS/visionOS */ }
@_cdecl("ws_window_set_cursor_position")
public func ws_window_set_cursor_position(_ id: UInt64, _ x: Double, _ y: Double) { /* no cursor */ }
@_cdecl("ws_window_set_theme")
public func ws_window_set_theme(_ id: UInt64, _ dark: Int8) {
    switch dark { case 0: uiWin(id)?.overrideUserInterfaceStyle = .light; case 1: uiWin(id)?.overrideUserInterfaceStyle = .dark; default: uiWin(id)?.overrideUserInterfaceStyle = .unspecified }
}
@_cdecl("ws_window_set_transparent")
public func ws_window_set_transparent(_ id: UInt64, _ v: Bool) {
    uiWin(id)?.isOpaque = !v; uiWin(id)?.backgroundColor = v ? .clear : .systemBackground
}
@_cdecl("ws_window_drag")
public func ws_window_drag(_ id: UInt64) { /* no-op */ }

// ── iOS/visionOS Event Loop ──

@_cdecl("ws_run_event_loop")
public func ws_run_event_loop() {
    // On iOS/visionOS, the run loop is managed by UIApplication.
    // We use a CADisplayLink to drive redraws.
    RunLoop.main.run()
}

@_cdecl("ws_poll_events")
public func ws_poll_events() -> Bool {
    RunLoop.main.run(mode: .default, before: Date())
    return true
}

@_cdecl("ws_poll_events_timeout")
public func ws_poll_events_timeout(_ secs: Double) -> Bool {
    RunLoop.main.run(mode: .default, before: Date(timeIntervalSinceNow: secs))
    return true
}

@_cdecl("ws_stop_event_loop")
public func ws_stop_event_loop() {
    CFRunLoopStop(CFRunLoopGetMain())
}

// ── iOS/visionOS Monitors ──

@_cdecl("ws_monitor_count")
public func ws_monitor_count() -> Int { 1 }

@_cdecl("ws_monitor_info")
public func ws_monitor_info(_ idx: Int, _ x: UnsafeMutablePointer<Int32>, _ y: UnsafeMutablePointer<Int32>,
                             _ w: UnsafeMutablePointer<UInt32>, _ h: UnsafeMutablePointer<UInt32>,
                             _ scale: UnsafeMutablePointer<Double>, _ np: UnsafeMutablePointer<UInt8>, _ nl: Int) -> Int {
    let s = UIScreen.main; let b = s.bounds
    x.pointee = 0; y.pointee = 0
    w.pointee = UInt32(b.width); h.pointee = UInt32(b.height)
    scale.pointee = Double(s.scale)
    let name = "Main Display"; let bytes = Array(name.utf8); let c = min(bytes.count, nl)
    for i in 0..<c { np[i] = bytes[i] }; return c
}

@_cdecl("ws_monitor_refresh_rate")
public func ws_monitor_refresh_rate(_ idx: Int) -> Double { Double(UIScreen.main.maximumFramesPerSecond) }

// ── iOS/visionOS Accessibility ──

@_cdecl("ws_accessibility_is_voiceover_running")
public func ws_accessibility_is_voiceover_running() -> Bool { UIAccessibility.isVoiceOverRunning }
@_cdecl("ws_accessibility_is_reduce_motion")
public func ws_accessibility_is_reduce_motion() -> Bool { UIAccessibility.isReduceMotionEnabled }
@_cdecl("ws_accessibility_is_reduce_transparency")
public func ws_accessibility_is_reduce_transparency() -> Bool { UIAccessibility.isReduceTransparencyEnabled }
@_cdecl("ws_accessibility_is_high_contrast")
public func ws_accessibility_is_high_contrast() -> Bool { UIAccessibility.isDarkerSystemColorsEnabled }

#endif // canImport(UIKit)

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Metal (all platforms)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("ws_metal_create_command_queue")
public func ws_metal_create_command_queue() -> UnsafeMutableRawPointer? {
    guard let q = metalDevice?.makeCommandQueue() else { return nil }; return Unmanaged.passRetained(q).toOpaque()
}

@_cdecl("ws_metal_next_drawable")
public func ws_metal_next_drawable(_ lp: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    guard let d = Unmanaged<CAMetalLayer>.fromOpaque(lp).takeUnretainedValue().nextDrawable() else { return nil }
    return Unmanaged.passRetained(d).toOpaque()
}

@_cdecl("ws_metal_drawable_texture")
public func ws_metal_drawable_texture(_ dp: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    Unmanaged.passUnretained(Unmanaged<CAMetalDrawable>.fromOpaque(dp).takeUnretainedValue().texture).toOpaque()
}

@_cdecl("ws_metal_present_drawable")
public func ws_metal_present_drawable(_ cp: UnsafeMutableRawPointer, _ dp: UnsafeMutableRawPointer) {
    Unmanaged<MTLCommandBuffer>.fromOpaque(cp).takeUnretainedValue().present(
        Unmanaged<CAMetalDrawable>.fromOpaque(dp).takeUnretainedValue())
}

@_cdecl("ws_metal_command_buffer")
public func ws_metal_command_buffer(_ qp: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    guard let b = Unmanaged<MTLCommandQueue>.fromOpaque(qp).takeUnretainedValue().makeCommandBuffer() else { return nil }
    return Unmanaged.passRetained(b).toOpaque()
}

@_cdecl("ws_metal_commit")
public func ws_metal_commit(_ p: UnsafeMutableRawPointer) { Unmanaged<MTLCommandBuffer>.fromOpaque(p).takeUnretainedValue().commit() }
@_cdecl("ws_metal_wait")
public func ws_metal_wait(_ p: UnsafeMutableRawPointer) { Unmanaged<MTLCommandBuffer>.fromOpaque(p).takeUnretainedValue().waitUntilCompleted() }

@_cdecl("ws_metal_make_library")
public func ws_metal_make_library(_ sp: UnsafePointer<UInt8>, _ sl: Int, _ ep: UnsafeMutablePointer<UInt8>, _ el: Int) -> UnsafeMutableRawPointer? {
    let src = String(bytes: UnsafeBufferPointer(start: sp, count: sl), encoding: .utf8) ?? ""
    do { guard let lib = try metalDevice?.makeLibrary(source: src, options: nil) else { return nil }; return Unmanaged.passRetained(lib).toOpaque()
    } catch { let b = Array(error.localizedDescription.utf8); let c = min(b.count, el); for i in 0..<c { ep[i] = b[i] }; return nil }
}

@_cdecl("ws_metal_make_function")
public func ws_metal_make_function(_ lp: UnsafeMutableRawPointer, _ np: UnsafePointer<UInt8>, _ nl: Int) -> UnsafeMutableRawPointer? {
    let name = String(bytes: UnsafeBufferPointer(start: np, count: nl), encoding: .utf8) ?? ""
    guard let f = Unmanaged<MTLLibrary>.fromOpaque(lp).takeUnretainedValue().makeFunction(name: name) else { return nil }
    return Unmanaged.passRetained(f).toOpaque()
}

@_cdecl("ws_metal_make_render_pipeline")
public func ws_metal_make_render_pipeline(_ vp: UnsafeMutableRawPointer, _ fp: UnsafeMutableRawPointer, _ pf: UInt64,
                                           _ ep: UnsafeMutablePointer<UInt8>, _ el: Int) -> UnsafeMutableRawPointer? {
    let d = MTLRenderPipelineDescriptor()
    d.vertexFunction = Unmanaged<MTLFunction>.fromOpaque(vp).takeUnretainedValue()
    d.fragmentFunction = Unmanaged<MTLFunction>.fromOpaque(fp).takeUnretainedValue()
    d.colorAttachments[0].pixelFormat = MTLPixelFormat(rawValue: UInt(pf)) ?? .bgra8Unorm
    do { guard let p = try metalDevice?.makeRenderPipelineState(descriptor: d) else { return nil }; return Unmanaged.passRetained(p).toOpaque()
    } catch { let b = Array(error.localizedDescription.utf8); let c = min(b.count, el); for i in 0..<c { ep[i] = b[i] }; return nil }
}

@_cdecl("ws_metal_make_compute_pipeline")
public func ws_metal_make_compute_pipeline(_ fp: UnsafeMutableRawPointer, _ ep: UnsafeMutablePointer<UInt8>, _ el: Int) -> UnsafeMutableRawPointer? {
    do { guard let p = try metalDevice?.makeComputePipelineState(function: Unmanaged<MTLFunction>.fromOpaque(fp).takeUnretainedValue()) else { return nil }
        return Unmanaged.passRetained(p).toOpaque()
    } catch { let b = Array(error.localizedDescription.utf8); let c = min(b.count, el); for i in 0..<c { ep[i] = b[i] }; return nil }
}

@_cdecl("ws_metal_make_buffer")
public func ws_metal_make_buffer(_ len: Int, _ opt: UInt64) -> UnsafeMutableRawPointer? {
    guard let b = metalDevice?.makeBuffer(length: len, options: MTLResourceOptions(rawValue: UInt(opt))) else { return nil }
    return Unmanaged.passRetained(b).toOpaque()
}

@_cdecl("ws_metal_make_buffer_data")
public func ws_metal_make_buffer_data(_ p: UnsafeRawPointer, _ len: Int, _ opt: UInt64) -> UnsafeMutableRawPointer? {
    guard let b = metalDevice?.makeBuffer(bytes: p, length: len, options: MTLResourceOptions(rawValue: UInt(opt))) else { return nil }
    return Unmanaged.passRetained(b).toOpaque()
}

@_cdecl("ws_metal_buffer_contents")
public func ws_metal_buffer_contents(_ p: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    Unmanaged<MTLBuffer>.fromOpaque(p).takeUnretainedValue().contents()
}

@_cdecl("ws_metal_render_encoder")
public func ws_metal_render_encoder(_ cp: UnsafeMutableRawPointer, _ tp: UnsafeMutableRawPointer,
                                     _ r: Double, _ g: Double, _ b: Double, _ a: Double) -> UnsafeMutableRawPointer? {
    let d = MTLRenderPassDescriptor()
    d.colorAttachments[0].texture = Unmanaged<MTLTexture>.fromOpaque(tp).takeUnretainedValue()
    d.colorAttachments[0].loadAction = .clear; d.colorAttachments[0].storeAction = .store
    d.colorAttachments[0].clearColor = MTLClearColorMake(r, g, b, a)
    guard let e = Unmanaged<MTLCommandBuffer>.fromOpaque(cp).takeUnretainedValue().makeRenderCommandEncoder(descriptor: d) else { return nil }
    return Unmanaged.passRetained(e).toOpaque()
}

@_cdecl("ws_metal_render_set_pipeline")
public func ws_metal_render_set_pipeline(_ ep: UnsafeMutableRawPointer, _ pp: UnsafeMutableRawPointer) {
    Unmanaged<MTLRenderCommandEncoder>.fromOpaque(ep).takeUnretainedValue().setRenderPipelineState(
        Unmanaged<MTLRenderPipelineState>.fromOpaque(pp).takeUnretainedValue())
}

@_cdecl("ws_metal_render_set_vertex_buffer")
public func ws_metal_render_set_vertex_buffer(_ ep: UnsafeMutableRawPointer, _ bp: UnsafeMutableRawPointer, _ off: Int, _ idx: Int) {
    Unmanaged<MTLRenderCommandEncoder>.fromOpaque(ep).takeUnretainedValue().setVertexBuffer(
        Unmanaged<MTLBuffer>.fromOpaque(bp).takeUnretainedValue(), offset: off, index: idx)
}

@_cdecl("ws_metal_render_draw")
public func ws_metal_render_draw(_ ep: UnsafeMutableRawPointer, _ vc: Int, _ ic: Int) {
    Unmanaged<MTLRenderCommandEncoder>.fromOpaque(ep).takeUnretainedValue().drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: vc, instanceCount: ic)
}

@_cdecl("ws_metal_render_end")
public func ws_metal_render_end(_ ep: UnsafeMutableRawPointer) {
    Unmanaged<MTLRenderCommandEncoder>.fromOpaque(ep).takeUnretainedValue().endEncoding()
}

@_cdecl("ws_metal_release")
public func ws_metal_release(_ p: UnsafeMutableRawPointer) { Unmanaged<AnyObject>.fromOpaque(p).release() }

@_cdecl("ws_metal_device_name")
public func ws_metal_device_name(_ buf: UnsafeMutablePointer<UInt8>, _ len: Int) -> Int {
    guard let n = metalDevice?.name else { return 0 }
    let b = Array(n.utf8); let c = min(b.count, len); for i in 0..<c { buf[i] = b[i] }; return c
}

@_cdecl("ws_metal_layer_set_vsync")
public func ws_metal_layer_set_vsync(_ p: UnsafeMutableRawPointer, _ v: Bool) {
    #if canImport(AppKit)
    Unmanaged<CAMetalLayer>.fromOpaque(p).takeUnretainedValue().displaySyncEnabled = v
    #endif
    // UIKit: vsync is always enabled
}

@_cdecl("ws_metal_layer_set_pixel_format")
public func ws_metal_layer_set_pixel_format(_ p: UnsafeMutableRawPointer, _ f: UInt64) {
    Unmanaged<CAMetalLayer>.fromOpaque(p).takeUnretainedValue().pixelFormat = MTLPixelFormat(rawValue: UInt(f)) ?? .bgra8Unorm
}

@_cdecl("ws_metal_layer_set_drawable_count")
public func ws_metal_layer_set_drawable_count(_ p: UnsafeMutableRawPointer, _ c: Int) {
    Unmanaged<CAMetalLayer>.fromOpaque(p).takeUnretainedValue().maximumDrawableCount = c
}

@_cdecl("ws_metal_layer_set_hdr")
public func ws_metal_layer_set_hdr(_ p: UnsafeMutableRawPointer, _ v: Bool) {
    let l = Unmanaged<CAMetalLayer>.fromOpaque(p).takeUnretainedValue()
    l.wantsExtendedDynamicRangeContent = v
    l.pixelFormat = v ? .rgba16Float : .bgra8Unorm
    l.colorspace = v ? CGColorSpace(name: CGColorSpace.extendedLinearDisplayP3) : CGColorSpace(name: CGColorSpace.sRGB)
}

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - Haptics (all platforms)
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(CoreHaptics)
private var hapticEngine: CHHapticEngine?

@_cdecl("ws_haptic_init")
public func ws_haptic_init() -> Bool {
    guard CHHapticEngine.capabilitiesForHardware().supportsHaptics else { return false }
    do { hapticEngine = try CHHapticEngine(); try hapticEngine?.start(); return true } catch { return false }
}

@_cdecl("ws_haptic_play")
public func ws_haptic_play(_ intensity: Float, _ sharpness: Float, _ duration: Float) -> Bool {
    guard let e = hapticEngine else { return false }
    do {
        let ev = CHHapticEvent(eventType: .hapticContinuous, parameters: [
            CHHapticEventParameter(parameterID: .hapticIntensity, value: intensity),
            CHHapticEventParameter(parameterID: .hapticSharpness, value: sharpness),
        ], relativeTime: 0, duration: TimeInterval(duration))
        try e.makePlayer(with: try CHHapticPattern(events: [ev], parameters: [])).start(atTime: CHHapticTimeImmediate)
        return true
    } catch { return false }
}

@_cdecl("ws_haptic_impact")
public func ws_haptic_impact(_ style: UInt8) {
    #if canImport(AppKit) && !targetEnvironment(macCatalyst)
    NSHapticFeedbackManager.defaultPerformer.perform(
        NSHapticFeedbackManager.FeedbackPattern(rawValue: Int(style)) ?? .generic, performanceTime: .now)
    #elseif canImport(UIKit)
    let gen: UIImpactFeedbackGenerator
    switch style {
    case 0: gen = UIImpactFeedbackGenerator(style: .light)
    case 1: gen = UIImpactFeedbackGenerator(style: .medium)
    case 2: gen = UIImpactFeedbackGenerator(style: .heavy)
    default: gen = UIImpactFeedbackGenerator(style: .medium)
    }
    gen.impactOccurred()
    #endif
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// MARK: - System (all platforms)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("ws_thermal_state")
public func ws_thermal_state() -> UInt8 { UInt8(ProcessInfo.processInfo.thermalState.rawValue) }
@_cdecl("ws_is_low_power_mode")
public func ws_is_low_power_mode() -> Bool { ProcessInfo.processInfo.isLowPowerModeEnabled }

@_cdecl("winit_swift_available")
public func winit_swift_available() -> Bool { true }
