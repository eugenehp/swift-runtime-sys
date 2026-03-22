import SwiftUI
import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// View constructors — each takes C types, returns an opaque View handle
// ═══════════════════════════════════════════════════════════════════════════

// All views are stored as AnyView in a retained heap object.
// The handle is an opaque pointer that Rust holds and passes back.

public typealias ViewHandle = UnsafeMutableRawPointer

func boxView<V: View>(_ view: V) -> ViewHandle {
    let erased = AnyView(view)
    let box = Unmanaged.passRetained(erased as AnyObject)
    return box.toOpaque()
}

func unboxView(_ handle: ViewHandle) -> AnyView {
    Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as! AnyView
}

// ── Text ──

@_cdecl("swiftui_text")
public func swiftuiText(_ utf8: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let s = String(bytes: UnsafeBufferPointer(start: utf8, count: len), encoding: .utf8) ?? ""
    return boxView(Text(s))
}

@_cdecl("swiftui_text_styled")
public func swiftuiTextStyled(
    _ utf8: UnsafePointer<UInt8>, _ len: Int,
    _ size: Float, _ weight: Int32, _ r: Float, _ g: Float, _ b: Float, _ a: Float
) -> ViewHandle {
    let s = String(bytes: UnsafeBufferPointer(start: utf8, count: len), encoding: .utf8) ?? ""
    var text = Text(s).font(.system(size: CGFloat(size)))
    switch weight {
    case 1: text = text.bold()
    case 2: text = text.italic()
    case 3: text = text.bold().italic()
    default: break
    }
    return boxView(text.foregroundColor(Color(red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a))))
}

// ── Image ──

@_cdecl("swiftui_system_image")
public func swiftuiSystemImage(_ utf8: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let name = String(bytes: UnsafeBufferPointer(start: utf8, count: len), encoding: .utf8) ?? ""
    return boxView(Image(systemName: name))
}

// ── Spacer ──

@_cdecl("swiftui_spacer")
public func swiftuiSpacer() -> ViewHandle {
    boxView(Spacer())
}

// ── Divider ──

@_cdecl("swiftui_divider")
public func swiftuiDivider() -> ViewHandle {
    boxView(Divider())
}

// ── Stacks ──

@_cdecl("swiftui_vstack")
public func swiftuiVStack(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(
        VStack(alignment: .center, spacing: 8) {
            ForEach(views.indices, id: \.self) { views[$0] }
        }
    )
}

@_cdecl("swiftui_hstack")
public func swiftuiHStack(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(
        HStack(alignment: .center, spacing: 8) {
            ForEach(views.indices, id: \.self) { views[$0] }
        }
    )
}

@_cdecl("swiftui_zstack")
public func swiftuiZStack(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(
        ZStack {
            ForEach(views.indices, id: \.self) { views[$0] }
        }
    )
}

// ── Modifiers ──

@_cdecl("swiftui_padding")
public func swiftuiPadding(_ handle: ViewHandle, _ amount: Float) -> ViewHandle {
    boxView(unboxView(handle).padding(CGFloat(amount)))
}

@_cdecl("swiftui_frame")
public func swiftuiFrame(_ handle: ViewHandle, _ w: Float, _ h: Float) -> ViewHandle {
    let view = unboxView(handle)
    if w > 0 && h > 0 {
        return boxView(view.frame(width: CGFloat(w), height: CGFloat(h)))
    } else if w > 0 {
        return boxView(view.frame(width: CGFloat(w)))
    } else if h > 0 {
        return boxView(view.frame(height: CGFloat(h)))
    }
    return boxView(view.frame(maxWidth: .infinity, maxHeight: .infinity))
}

@_cdecl("swiftui_background_color")
public func swiftuiBackgroundColor(_ handle: ViewHandle, _ r: Float, _ g: Float, _ b: Float, _ a: Float) -> ViewHandle {
    boxView(unboxView(handle).background(Color(red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a))))
}

@_cdecl("swiftui_corner_radius")
public func swiftuiCornerRadius(_ handle: ViewHandle, _ radius: Float) -> ViewHandle {
    boxView(unboxView(handle).cornerRadius(CGFloat(radius)))
}

@_cdecl("swiftui_opacity")
public func swiftuiOpacity(_ handle: ViewHandle, _ opacity: Float) -> ViewHandle {
    boxView(unboxView(handle).opacity(Double(opacity)))
}

@_cdecl("swiftui_border")
public func swiftuiBorder(_ handle: ViewHandle, _ r: Float, _ g: Float, _ b: Float, _ width: Float) -> ViewHandle {
    boxView(unboxView(handle).border(Color(red: Double(r), green: Double(g), blue: Double(b)), width: CGFloat(width)))
}

// ── ScrollView ──

@_cdecl("swiftui_scroll_view")
public func swiftuiScrollView(_ content: ViewHandle) -> ViewHandle {
    boxView(ScrollView { unboxView(content) })
}

// ── Button (with callback) ──

@_cdecl("swiftui_button")
public func swiftuiButton(
    _ labelUtf8: UnsafePointer<UInt8>, _ labelLen: Int,
    _ callback: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelUtf8, count: labelLen), encoding: .utf8) ?? ""
    let ud = userData
    let cb = callback
    return boxView(
        Button(label) { cb(ud) }
    )
}

// ── TextField (read-only display for now) ──

@_cdecl("swiftui_textfield")
public func swiftuiTextField(
    _ placeholderUtf8: UnsafePointer<UInt8>, _ placeholderLen: Int,
    _ valueUtf8: UnsafePointer<UInt8>, _ valueLen: Int
) -> ViewHandle {
    let placeholder = String(bytes: UnsafeBufferPointer(start: placeholderUtf8, count: placeholderLen), encoding: .utf8) ?? ""
    let value = String(bytes: UnsafeBufferPointer(start: valueUtf8, count: valueLen), encoding: .utf8) ?? ""
    return boxView(
        TextField(placeholder, text: .constant(value))
            .textFieldStyle(.roundedBorder)
    )
}

// ── Toggle ──

@_cdecl("swiftui_toggle")
public func swiftuiToggle(
    _ labelUtf8: UnsafePointer<UInt8>, _ labelLen: Int,
    _ isOn: Bool
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelUtf8, count: labelLen), encoding: .utf8) ?? ""
    return boxView(
        Toggle(label, isOn: .constant(isOn))
    )
}

// ── ProgressView ──

@_cdecl("swiftui_progress")
public func swiftuiProgress(_ value: Float, _ total: Float) -> ViewHandle {
    if total > 0 {
        return boxView(ProgressView(value: Double(value), total: Double(total)))
    }
    return boxView(ProgressView()) // indeterminate
}

// ── Picker ──

@_cdecl("swiftui_picker")
public func swiftuiPicker(
    _ labelUtf8: UnsafePointer<UInt8>, _ labelLen: Int,
    _ optionsUtf8: UnsafePointer<UnsafePointer<UInt8>>,
    _ optionsLens: UnsafePointer<Int>,
    _ optionCount: Int,
    _ selected: Int32
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelUtf8, count: labelLen), encoding: .utf8) ?? ""
    let options = (0..<optionCount).map { i in
        String(bytes: UnsafeBufferPointer(start: optionsUtf8[i], count: optionsLens[i]), encoding: .utf8) ?? ""
    }
    return boxView(
        Picker(label, selection: .constant(Int(selected))) {
            ForEach(options.indices, id: \.self) { i in
                Text(options[i]).tag(i)
            }
        }
    )
}

// ── Color as view ──

@_cdecl("swiftui_color")
public func swiftuiColor(_ r: Float, _ g: Float, _ b: Float, _ a: Float) -> ViewHandle {
    boxView(Color(red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a)))
}

// ═══════════════════════════════════════════════════════════════════════════
// View lifecycle
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_retain")
public func swiftuiRetain(_ handle: ViewHandle) {
    _ = Unmanaged<AnyObject>.fromOpaque(handle).retain()
}

@_cdecl("swiftui_release")
public func swiftuiRelease(_ handle: ViewHandle) {
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

// ═══════════════════════════════════════════════════════════════════════════
// Window display
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_show_window")
public func swiftuiShowWindow(
    _ handle: ViewHandle,
    _ titleUtf8: UnsafePointer<UInt8>, _ titleLen: Int,
    _ width: Float, _ height: Float
) {
    let view = unboxView(handle)
    let title = String(bytes: UnsafeBufferPointer(start: titleUtf8, count: titleLen), encoding: .utf8) ?? "SwiftUI"

    NSApplication.shared.setActivationPolicy(.regular)
    let controller = NSHostingController(rootView: view)
    let window = NSWindow(
        contentRect: NSRect(x: 0, y: 0, width: CGFloat(width), height: CGFloat(height)),
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
