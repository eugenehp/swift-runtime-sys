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
    runApp(rootView: view, title: title, width: CGFloat(width), height: CGFloat(height))
}

// ── Additional modifiers ──

@_cdecl("swiftui_foreground_color")
public func swiftuiForegroundColor(_ handle: ViewHandle, _ r: Float, _ g: Float, _ b: Float, _ a: Float) -> ViewHandle {
    boxView(unboxView(handle).foregroundColor(Color(red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a))))
}

@_cdecl("swiftui_shadow")
public func swiftuiShadow(_ handle: ViewHandle, _ r: Float, _ g: Float, _ b: Float, _ radius: Float, _ x: Float, _ y: Float) -> ViewHandle {
    boxView(unboxView(handle).shadow(color: Color(red: Double(r), green: Double(g), blue: Double(b)), radius: CGFloat(radius), x: CGFloat(x), y: CGFloat(y)))
}

@_cdecl("swiftui_offset")
public func swiftuiOffset(_ handle: ViewHandle, _ x: Float, _ y: Float) -> ViewHandle {
    boxView(unboxView(handle).offset(x: CGFloat(x), y: CGFloat(y)))
}

@_cdecl("swiftui_scale")
public func swiftuiScale(_ handle: ViewHandle, _ scale: Float) -> ViewHandle {
    boxView(unboxView(handle).scaleEffect(CGFloat(scale)))
}

@_cdecl("swiftui_rotation")
public func swiftuiRotation(_ handle: ViewHandle, _ degrees: Float) -> ViewHandle {
    boxView(unboxView(handle).rotationEffect(.degrees(Double(degrees))))
}

@_cdecl("swiftui_hidden")
public func swiftuiHidden(_ handle: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).hidden())
}

@_cdecl("swiftui_disabled")
public func swiftuiDisabled(_ handle: ViewHandle, _ disabled: Bool) -> ViewHandle {
    boxView(unboxView(handle).disabled(disabled))
}

@_cdecl("swiftui_overlay")
public func swiftuiOverlay(_ handle: ViewHandle, _ overlay: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).overlay(unboxView(overlay)))
}

@_cdecl("swiftui_clip_circle")
public func swiftuiClipCircle(_ handle: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).clipShape(Circle()))
}

@_cdecl("swiftui_font_system")
public func swiftuiFontSystem(_ handle: ViewHandle, _ size: Float, _ weight: Int32) -> ViewHandle {
    let w: Font.Weight = switch weight {
        case 1: .bold
        case 2: .semibold
        case 3: .heavy
        case 4: .light
        case 5: .thin
        case 6: .medium
        default: .regular
    }
    return boxView(unboxView(handle).font(.system(size: CGFloat(size), weight: w)))
}

// ── Additional views ──

@_cdecl("swiftui_label")
public func swiftuiLabel(_ textPtr: UnsafePointer<UInt8>, _ textLen: Int, _ iconPtr: UnsafePointer<UInt8>, _ iconLen: Int) -> ViewHandle {
    let text = String(bytes: UnsafeBufferPointer(start: textPtr, count: textLen), encoding: .utf8) ?? ""
    let icon = String(bytes: UnsafeBufferPointer(start: iconPtr, count: iconLen), encoding: .utf8) ?? ""
    return boxView(Label(text, systemImage: icon))
}

@_cdecl("swiftui_slider")
public func swiftuiSlider(_ value: Float, _ min: Float, _ max: Float) -> ViewHandle {
    boxView(Slider(value: .constant(Double(value)), in: Double(min)...Double(max)))
}

@_cdecl("swiftui_link")
public func swiftuiLink(_ textPtr: UnsafePointer<UInt8>, _ textLen: Int, _ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> ViewHandle {
    let text = String(bytes: UnsafeBufferPointer(start: textPtr, count: textLen), encoding: .utf8) ?? ""
    let url = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    return boxView(Link(text, destination: URL(string: url)!))
}

// ── Reactive state: rebuild callback ──

@_cdecl("swiftui_observable_window")
public func swiftuiObservableWindow(
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ width: Float, _ height: Float,
    _ buildFn: @convention(c) (UnsafeMutableRawPointer?) -> ViewHandle,
    _ userData: UnsafeMutableRawPointer?
) {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    
    let rootView = ReactiveView(buildFn: buildFn, userData: userData)
    runApp(rootView: rootView, title: title, width: CGFloat(width), height: CGFloat(height))
}

@Observable
class ReactiveModel {
    var version: Int = 0
    func bump() { version += 1 }
}

struct ReactiveView: View {
    let buildFn: @convention(c) (UnsafeMutableRawPointer?) -> ViewHandle
    let userData: UnsafeMutableRawPointer?
    @State private var model = ReactiveModel()
    
    var body: some View {
        let _ = model.version // subscribe to changes
        let handle = buildFn(userData)
        let view = unboxView(handle)
        view.environment(model)
    }
}

@_cdecl("swiftui_trigger_update")
public func swiftuiTriggerUpdate(_ modelPtr: UnsafeMutableRawPointer) {
    let model = Unmanaged<ReactiveModel>.fromOpaque(modelPtr).takeUnretainedValue()
    DispatchQueue.main.async { model.bump() }
}

// ── Reactive state: Rust calls buildFn on every state change ──

@Observable
class ReactiveModel2 {
    var version: Int = 0
    func bump() { version += 1 }
}

struct ReactiveView2: View {
    let buildFn: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> ViewHandle
    let userData: UnsafeMutableRawPointer?
    @State private var model = ReactiveModel2()
    
    var body: some View {
        let _ = model.version
        let modelHandle = Unmanaged.passUnretained(model).toOpaque()
        let handle = buildFn(userData, modelHandle)
        unboxView(handle)
    }
}

@_cdecl("swiftui_reactive_window")
public func swiftuiReactiveWindow(
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ width: Float, _ height: Float,
    _ buildFn: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> ViewHandle,
    _ userData: UnsafeMutableRawPointer?
) {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    let rootView = ReactiveView2(buildFn: buildFn, userData: userData)
    runApp(rootView: rootView, title: title, width: CGFloat(width), height: CGFloat(height))
}

@_cdecl("swiftui_trigger_rebuild")
public func swiftuiTriggerRebuild(_ modelPtr: UnsafeMutableRawPointer) {
    let model = Unmanaged<ReactiveModel2>.fromOpaque(modelPtr).takeUnretainedValue()
    DispatchQueue.main.async { model.bump() }
}
