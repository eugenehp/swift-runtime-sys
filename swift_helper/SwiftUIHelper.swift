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

// ── Gestures ──

@_cdecl("swiftui_on_tap")
public func swiftuiOnTap(
    _ handle: ViewHandle,
    _ callback: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let cb = callback
    let ud = userData
    return boxView(unboxView(handle).onTapGesture { cb(ud) })
}

@_cdecl("swiftui_on_long_press")
public func swiftuiOnLongPress(
    _ handle: ViewHandle,
    _ callback: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let cb = callback
    let ud = userData
    return boxView(unboxView(handle).onLongPressGesture { cb(ud) })
}

// ═══════════════════════════════════════════════════════════════════════════
// Two-way binding — TextField/Toggle that write back to Rust state
// ═══════════════════════════════════════════════════════════════════════════

// Observable model that holds a string/bool, calls Rust on change
class BindableString: ObservableObject {
    @Published var value: String {
        didSet { onChange?(value) }
    }
    var onChange: ((String) -> Void)?
    init(_ v: String, onChange: ((String) -> Void)?) {
        self.value = v
        self.onChange = onChange
    }
}

class BindableBool: ObservableObject {
    @Published var value: Bool {
        didSet { onChange?(value) }
    }
    var onChange: ((Bool) -> Void)?
    init(_ v: Bool, onChange: ((Bool) -> Void)?) {
        self.value = v
        self.onChange = onChange
    }
}

struct BoundTextField: View {
    @ObservedObject var model: BindableString
    let placeholder: String
    var body: some View {
        TextField(placeholder, text: $model.value)
            .textFieldStyle(.roundedBorder)
    }
}

struct BoundToggle: View {
    @ObservedObject var model: BindableBool
    let label: String
    var body: some View {
        Toggle(label, isOn: $model.value)
    }
}

struct BoundSlider: View {
    @ObservedObject var model: BindableString // stores Float as String
    let min: Float
    let max: Float
    var body: some View {
        let val = Binding<Double>(
            get: { Double(self.model.value) ?? 0 },
            set: { self.model.value = String(Float($0)) }
        )
        Slider(value: val, in: Double(min)...Double(max))
    }
}

@_cdecl("swiftui_bound_textfield")
public func swiftuiBoundTextField(
    _ placeholderPtr: UnsafePointer<UInt8>, _ placeholderLen: Int,
    _ valuePtr: UnsafePointer<UInt8>, _ valueLen: Int,
    _ callback: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let placeholder = String(bytes: UnsafeBufferPointer(start: placeholderPtr, count: placeholderLen), encoding: .utf8) ?? ""
    let value = String(bytes: UnsafeBufferPointer(start: valuePtr, count: valueLen), encoding: .utf8) ?? ""
    let cb = callback
    let ud = userData
    let model = BindableString(value) { newVal in
        newVal.withCString { ptr in
            cb(UnsafePointer(OpaquePointer(ptr)), newVal.utf8.count, ud)
        }
    }
    return boxView(BoundTextField(model: model, placeholder: placeholder))
}

@_cdecl("swiftui_bound_toggle")
public func swiftuiBoundToggle(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ value: Bool,
    _ callback: @convention(c) (Bool, UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    let cb = callback
    let ud = userData
    let model = BindableBool(value) { newVal in cb(newVal, ud) }
    return boxView(BoundToggle(model: model, label: label))
}

@_cdecl("swiftui_bound_slider")
public func swiftuiBoundSlider(
    _ value: Float, _ min: Float, _ max: Float,
    _ callback: @convention(c) (Float, UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let cb = callback
    let ud = userData
    let model = BindableString(String(value)) { newVal in
        if let f = Float(newVal) { cb(f, ud) }
    }
    return boxView(BoundSlider(model: model, min: min, max: max))
}

// ═══════════════════════════════════════════════════════════════════════════
// List — real SwiftUI List
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_list")
public func swiftuiList(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(List { ForEach(views.indices, id: \.self) { views[$0] } })
}

// ═══════════════════════════════════════════════════════════════════════════
// Sheet / Alert
// ═══════════════════════════════════════════════════════════════════════════

class SheetModel: ObservableObject {
    @Published var isPresented: Bool = false
}

struct SheetWrapper: View {
    let base: AnyView
    let sheet: AnyView
    @ObservedObject var model: SheetModel
    var body: some View {
        base.sheet(isPresented: $model.isPresented) { sheet }
    }
}

@_cdecl("swiftui_sheet")
public func swiftuiSheet(
    _ baseHandle: ViewHandle,
    _ sheetHandle: ViewHandle,
    _ isPresented: Bool
) -> ViewHandle {
    let model = SheetModel()
    model.isPresented = isPresented
    return boxView(SheetWrapper(base: unboxView(baseHandle), sheet: unboxView(sheetHandle), model: model))
}

struct AlertWrapper: View {
    let base: AnyView
    let title: String
    let message: String
    @ObservedObject var model: SheetModel
    var body: some View {
        base.alert(title, isPresented: $model.isPresented) {
            Button("OK") {}
        } message: {
            Text(message)
        }
    }
}

@_cdecl("swiftui_alert")
public func swiftuiAlert(
    _ baseHandle: ViewHandle,
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ msgPtr: UnsafePointer<UInt8>, _ msgLen: Int,
    _ isPresented: Bool
) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    let msg = String(bytes: UnsafeBufferPointer(start: msgPtr, count: msgLen), encoding: .utf8) ?? ""
    let model = SheetModel()
    model.isPresented = isPresented
    return boxView(AlertWrapper(base: unboxView(baseHandle), title: title, message: msg, model: model))
}

// ═══════════════════════════════════════════════════════════════════════════
// Animation
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_animation")
public func swiftuiAnimation(_ handle: ViewHandle, _ type: Int32) -> ViewHandle {
    let view = unboxView(handle)
    switch type {
    case 0: return boxView(view.animation(.default, value: true))
    case 1: return boxView(view.animation(.easeIn, value: true))
    case 2: return boxView(view.animation(.easeOut, value: true))
    case 3: return boxView(view.animation(.easeInOut, value: true))
    case 4: return boxView(view.animation(.spring, value: true))
    case 5: return boxView(view.animation(.bouncy, value: true))
    default: return boxView(view.animation(.default, value: true))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Additional modifiers
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_blur")
public func swiftuiBlur(_ handle: ViewHandle, _ radius: Float) -> ViewHandle {
    boxView(unboxView(handle).blur(radius: CGFloat(radius)))
}

@_cdecl("swiftui_brightness")
public func swiftuiBrightness(_ handle: ViewHandle, _ amount: Float) -> ViewHandle {
    boxView(unboxView(handle).brightness(Double(amount)))
}

@_cdecl("swiftui_saturation")
public func swiftuiSaturation(_ handle: ViewHandle, _ amount: Float) -> ViewHandle {
    boxView(unboxView(handle).saturation(Double(amount)))
}

@_cdecl("swiftui_grayscale")
public func swiftuiGrayscale(_ handle: ViewHandle, _ amount: Float) -> ViewHandle {
    boxView(unboxView(handle).grayscale(Double(amount)))
}

@_cdecl("swiftui_help")
public func swiftuiHelp(_ handle: ViewHandle, _ textPtr: UnsafePointer<UInt8>, _ textLen: Int) -> ViewHandle {
    let text = String(bytes: UnsafeBufferPointer(start: textPtr, count: textLen), encoding: .utf8) ?? ""
    return boxView(unboxView(handle).help(text))
}

@_cdecl("swiftui_line_limit")
public func swiftuiLineLimit(_ handle: ViewHandle, _ limit: Int32) -> ViewHandle {
    boxView(unboxView(handle).lineLimit(Int(limit)))
}

@_cdecl("swiftui_fixed_size")
public func swiftuiFixedSize(_ handle: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).fixedSize())
}

@_cdecl("swiftui_aspect_ratio")
public func swiftuiAspectRatio(_ handle: ViewHandle, _ ratio: Float, _ mode: Int32) -> ViewHandle {
    let m: ContentMode = mode == 0 ? .fit : .fill
    return boxView(unboxView(handle).aspectRatio(CGFloat(ratio), contentMode: m))
}

@_cdecl("swiftui_clipped")
public func swiftuiClipped(_ handle: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).clipped())
}

@_cdecl("swiftui_tint")
public func swiftuiTint(_ handle: ViewHandle, _ r: Float, _ g: Float, _ b: Float) -> ViewHandle {
    boxView(unboxView(handle).tint(Color(red: Double(r), green: Double(g), blue: Double(b))))
}

@_cdecl("swiftui_badge")
public func swiftuiBadge(_ handle: ViewHandle, _ count: Int32) -> ViewHandle {
    boxView(unboxView(handle).badge(Int(count)))
}

// ═══════════════════════════════════════════════════════════════════════════
// Additional views
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_secure_field")
public func swiftuiSecureField(_ placeholderPtr: UnsafePointer<UInt8>, _ placeholderLen: Int, _ valuePtr: UnsafePointer<UInt8>, _ valueLen: Int) -> ViewHandle {
    let p = String(bytes: UnsafeBufferPointer(start: placeholderPtr, count: placeholderLen), encoding: .utf8) ?? ""
    let v = String(bytes: UnsafeBufferPointer(start: valuePtr, count: valueLen), encoding: .utf8) ?? ""
    return boxView(SecureField(p, text: .constant(v)))
}

@_cdecl("swiftui_text_editor")
public func swiftuiTextEditor(_ valuePtr: UnsafePointer<UInt8>, _ valueLen: Int) -> ViewHandle {
    let v = String(bytes: UnsafeBufferPointer(start: valuePtr, count: valueLen), encoding: .utf8) ?? ""
    return boxView(TextEditor(text: .constant(v)))
}

@_cdecl("swiftui_stepper")
public func swiftuiStepper(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ value: Int32, _ min: Int32, _ max: Int32
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    return boxView(Stepper(label, value: .constant(Int(value)), in: Int(min)...Int(max)))
}

@_cdecl("swiftui_group_box")
public func swiftuiGroupBox(_ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int, _ content: ViewHandle) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    return boxView(GroupBox(title) { unboxView(content) })
}

#if os(macOS)
@_cdecl("swiftui_date_picker")
public func swiftuiDatePicker(_ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    return boxView(DatePicker(label, selection: .constant(Date())))
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// TabView
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_tabview")
public func swiftuiTabView(
    _ tabs: UnsafePointer<ViewHandle>,
    _ labels: UnsafePointer<UnsafePointer<UInt8>>,
    _ labelLens: UnsafePointer<Int>,
    _ icons: UnsafePointer<UnsafePointer<UInt8>>,
    _ iconLens: UnsafePointer<Int>,
    _ count: Int
) -> ViewHandle {
    let entries: [(AnyView, String, String)] = (0..<count).map { i in
        let view = unboxView(tabs[i])
        let lbl = String(bytes: UnsafeBufferPointer(start: labels[i], count: labelLens[i]), encoding: .utf8) ?? ""
        let ico = String(bytes: UnsafeBufferPointer(start: icons[i], count: iconLens[i]), encoding: .utf8) ?? ""
        return (view, lbl, ico)
    }
    return boxView(
        TabView {
            ForEach(entries.indices, id: \.self) { i in
                entries[i].0.tabItem {
                    Label(entries[i].1, systemImage: entries[i].2)
                }
            }
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Picker
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_bound_picker")
public func swiftuiBoundPicker(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ optPtrs: UnsafePointer<UnsafePointer<UInt8>>,
    _ optLens: UnsafePointer<Int>,
    _ optCount: Int,
    _ selected: Int32,
    _ callback: @convention(c) (Int32, UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    let options = (0..<optCount).map {
        String(bytes: UnsafeBufferPointer(start: optPtrs[$0], count: optLens[$0]), encoding: .utf8) ?? ""
    }
    let cb = callback; let ud = userData
    
    class PickerModel: ObservableObject {
        @Published var selection: Int { didSet { onChange?(Int32(selection)) } }
        var onChange: ((Int32) -> Void)?
        init(_ sel: Int, onChange: ((Int32) -> Void)?) { self.selection = sel; self.onChange = onChange }
    }
    
    let model = PickerModel(Int(selected)) { val in cb(val, ud) }
    
    struct BoundPicker: View {
        @ObservedObject var model: PickerModel
        let label: String; let options: [String]
        var body: some View {
            Picker(label, selection: $model.selection) {
                ForEach(options.indices, id: \.self) { Text(options[$0]).tag($0) }
            }
        }
    }
    return boxView(BoundPicker(model: model, label: label, options: options))
}

// ═══════════════════════════════════════════════════════════════════════════
// Menu / ContextMenu
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_menu")
public func swiftuiMenu(_ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int, _ content: ViewHandle) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    return boxView(Menu(label) { unboxView(content) })
}

@_cdecl("swiftui_context_menu")
public func swiftuiContextMenu(_ handle: ViewHandle, _ menuContent: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).contextMenu { unboxView(menuContent) })
}

// ═══════════════════════════════════════════════════════════════════════════
// Toolbar / NavigationTitle / Searchable
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_navigation_title")
public func swiftuiNavigationTitle(_ handle: ViewHandle, _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    return boxView(NavigationStack { unboxView(handle).navigationTitle(title) })
}

@_cdecl("swiftui_toolbar")
public func swiftuiToolbar(_ handle: ViewHandle, _ toolbarContent: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).toolbar { unboxView(toolbarContent) })
}

// ═══════════════════════════════════════════════════════════════════════════
// Grid
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_grid")
public func swiftuiGrid(_ children: UnsafePointer<ViewHandle>, _ count: Int, _ columns: Int32) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    let cols = Array(repeating: GridItem(.flexible()), count: Int(columns))
    return boxView(
        LazyVGrid(columns: cols, spacing: 8) {
            ForEach(views.indices, id: \.self) { views[$0] }
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Form / Section
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_form")
public func swiftuiForm(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(Form { ForEach(views.indices, id: \.self) { views[$0] } })
}

@_cdecl("swiftui_section")
public func swiftuiSection(_ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int, _ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(Section(title) { ForEach(views.indices, id: \.self) { views[$0] } })
}

// ═══════════════════════════════════════════════════════════════════════════
// Per-view lifecycle
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_on_appear")
public func swiftuiOnAppear(_ handle: ViewHandle, _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void, _ ud: UnsafeMutableRawPointer?) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(handle).onAppear { callback(userData) })
}

@_cdecl("swiftui_on_disappear")
public func swiftuiOnDisappear(_ handle: ViewHandle, _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void, _ ud: UnsafeMutableRawPointer?) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(handle).onDisappear { callback(userData) })
}

// ═══════════════════════════════════════════════════════════════════════════
// Bold / Italic on any view
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_bold")
public func swiftuiBold(_ handle: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).bold())
}

@_cdecl("swiftui_italic")
public func swiftuiItalic(_ handle: ViewHandle) -> ViewHandle {
    boxView(unboxView(handle).italic())
}

// ═══════════════════════════════════════════════════════════════════════════
// Confirmation dialog / Popover
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_popover")
public func swiftuiPopover(_ handle: ViewHandle, _ content: ViewHandle, _ isPresented: Bool) -> ViewHandle {
    let model = SheetModel(); model.isPresented = isPresented
    struct PopoverWrapper: View {
        let base: AnyView; let content: AnyView; @ObservedObject var model: SheetModel
        var body: some View { base.popover(isPresented: $model.isPresented) { content } }
    }
    return boxView(PopoverWrapper(base: unboxView(handle), content: unboxView(content), model: model))
}

#if os(macOS)
@_cdecl("swiftui_color_picker")
public func swiftuiColorPicker(_ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    return boxView(ColorPicker(label, selection: .constant(.blue)))
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// Remaining modifiers
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_color_invert")
public func swiftuiColorInvert(_ h: ViewHandle) -> ViewHandle { boxView(unboxView(h).colorInvert()) }

@_cdecl("swiftui_ignores_safe_area")
public func swiftuiIgnoresSafeArea(_ h: ViewHandle) -> ViewHandle { boxView(unboxView(h).ignoresSafeArea()) }

#if os(iOS)
@_cdecl("swiftui_full_screen_cover")
public func swiftuiFullScreenCover(_ h: ViewHandle, _ content: ViewHandle, _ shown: Bool) -> ViewHandle {
    let model = SheetModel(); model.isPresented = shown
    struct FSC: View {
        let base: AnyView; let content: AnyView; @ObservedObject var model: SheetModel
        var body: some View { base.fullScreenCover(isPresented: $model.isPresented) { content } }
    }
    return boxView(FSC(base: unboxView(h), content: unboxView(content), model: model))
}
#endif

@_cdecl("swiftui_confirmation_dialog")
public func swiftuiConfirmationDialog(
    _ h: ViewHandle,
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ shown: Bool,
    _ actions: ViewHandle
) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    let model = SheetModel(); model.isPresented = shown
    struct CD: View {
        let base: AnyView; let title: String; let actions: AnyView; @ObservedObject var model: SheetModel
        var body: some View { base.confirmationDialog(title, isPresented: $model.isPresented) { actions } }
    }
    return boxView(CD(base: unboxView(h), title: title, actions: unboxView(actions), model: model))
}

@_cdecl("swiftui_keyboard_shortcut")
public func swiftuiKeyboardShortcut(_ h: ViewHandle, _ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int) -> ViewHandle {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    if let char = key.first {
        return boxView(unboxView(h).keyboardShortcut(KeyEquivalent(char)))
    }
    return h
}

@_cdecl("swiftui_focusable")
public func swiftuiFocusable(_ h: ViewHandle) -> ViewHandle { boxView(unboxView(h).focusable()) }

@_cdecl("swiftui_truncation_mode")
public func swiftuiTruncationMode(_ h: ViewHandle, _ mode: Int32) -> ViewHandle {
    let m: Text.TruncationMode = switch mode { case 1: .middle; case 2: .head; default: .tail }
    return boxView(unboxView(h).truncationMode(m))
}

@_cdecl("swiftui_multiline_alignment")
public func swiftuiMultilineAlignment(_ h: ViewHandle, _ align: Int32) -> ViewHandle {
    let a: TextAlignment = switch align { case 1: .center; case 2: .trailing; default: .leading }
    return boxView(unboxView(h).multilineTextAlignment(a))
}

@_cdecl("swiftui_minimum_scale_factor")
public func swiftuiMinimumScaleFactor(_ h: ViewHandle, _ factor: Float) -> ViewHandle {
    boxView(unboxView(h).minimumScaleFactor(CGFloat(factor)))
}

// ═══════════════════════════════════════════════════════════════════════════
// Accessibility
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_accessibility_label")
public func swiftuiAccessibilityLabel(_ h: ViewHandle, _ ptr: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let s = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    return boxView(unboxView(h).accessibilityLabel(s))
}

@_cdecl("swiftui_accessibility_hint")
public func swiftuiAccessibilityHint(_ h: ViewHandle, _ ptr: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let s = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    return boxView(unboxView(h).accessibilityHint(s))
}

@_cdecl("swiftui_accessibility_hidden")
public func swiftuiAccessibilityHidden(_ h: ViewHandle, _ hidden: Bool) -> ViewHandle {
    boxView(unboxView(h).accessibilityHidden(hidden))
}

@_cdecl("swiftui_accessibility_value")
public func swiftuiAccessibilityValue(_ h: ViewHandle, _ ptr: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let s = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    return boxView(unboxView(h).accessibilityValue(s))
}

// ═══════════════════════════════════════════════════════════════════════════
// Remaining views
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_disclosure_group")
public func swiftuiDisclosureGroup(_ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int, _ content: ViewHandle) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    return boxView(DisclosureGroup(title) { unboxView(content) })
}

@_cdecl("swiftui_labeled_content")
public func swiftuiLabeledContent(_ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int, _ content: ViewHandle) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    return boxView(LabeledContent(label) { unboxView(content) })
}

@_cdecl("swiftui_navigation_split_view")
public func swiftuiNavigationSplitView(_ sidebar: ViewHandle, _ detail: ViewHandle) -> ViewHandle {
    boxView(NavigationSplitView { unboxView(sidebar) } detail: { unboxView(detail) })
}

@_cdecl("swiftui_content_unavailable")
public func swiftuiContentUnavailable(
    _ titlePtr: UnsafePointer<UInt8>, _ titleLen: Int,
    _ descPtr: UnsafePointer<UInt8>, _ descLen: Int,
    _ imagePtr: UnsafePointer<UInt8>, _ imageLen: Int
) -> ViewHandle {
    let title = String(bytes: UnsafeBufferPointer(start: titlePtr, count: titleLen), encoding: .utf8) ?? ""
    let desc = String(bytes: UnsafeBufferPointer(start: descPtr, count: descLen), encoding: .utf8) ?? ""
    let image = String(bytes: UnsafeBufferPointer(start: imagePtr, count: imageLen), encoding: .utf8) ?? ""
    return boxView(ContentUnavailableView(title, systemImage: image, description: Text(desc)))
}

@_cdecl("swiftui_share_link")
public func swiftuiShareLink(_ textPtr: UnsafePointer<UInt8>, _ textLen: Int, _ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> ViewHandle {
    let text = String(bytes: UnsafeBufferPointer(start: textPtr, count: textLen), encoding: .utf8) ?? ""
    let url = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    if let u = URL(string: url) {
        return boxView(ShareLink(item: u, subject: Text(text)))
    }
    return boxView(Text("Invalid URL"))
}

#if os(macOS)
@_cdecl("swiftui_table")
public func swiftuiTable(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    // Simple table as List with columns — real Table needs typed data
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(List { ForEach(views.indices, id: \.self) { views[$0] } })
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// Animation — comprehensive
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_animation_duration")
public func swiftuiAnimationDuration(_ h: ViewHandle, _ type: Int32, _ duration: Float) -> ViewHandle {
    let d = Double(duration)
    let anim: Animation = switch type {
    case 1: .easeIn(duration: d)
    case 2: .easeOut(duration: d)
    case 3: .easeInOut(duration: d)
    case 4: .linear(duration: d)
    default: .default
    }
    return boxView(unboxView(h).animation(anim, value: true))
}

@_cdecl("swiftui_animation_spring_params")
public func swiftuiAnimationSpringParams(_ h: ViewHandle, _ duration: Float, _ bounce: Float) -> ViewHandle {
    boxView(unboxView(h).animation(.spring(duration: Double(duration), bounce: Double(bounce)), value: true))
}

@_cdecl("swiftui_transition")
public func swiftuiTransition(_ h: ViewHandle, _ type: Int32) -> ViewHandle {
    let t: AnyTransition = switch type {
    case 0: .opacity
    case 1: .slide
    case 2: .scale
    case 3: .move(edge: .top)
    case 4: .move(edge: .bottom)
    case 5: .move(edge: .leading)
    case 6: .move(edge: .trailing)
    case 7: .push(from: .bottom)
    case 8: .push(from: .leading)
    default: .opacity
    }
    return boxView(unboxView(h).transition(t))
}

// ═══════════════════════════════════════════════════════════════════════════
// Gestures — drag, magnify, rotate
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_on_drag")
public func swiftuiOnDrag(
    _ h: ViewHandle,
    _ cb: @convention(c) (Float, Float, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(h).gesture(
        DragGesture().onChanged { value in
            callback(Float(value.translation.width), Float(value.translation.height), userData)
        }
    ))
}

@_cdecl("swiftui_on_magnify")
public func swiftuiOnMagnify(
    _ h: ViewHandle,
    _ cb: @convention(c) (Float, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(h).gesture(
        MagnifyGesture().onChanged { value in
            callback(Float(value.magnification), userData)
        }
    ))
}

@_cdecl("swiftui_on_rotate")
public func swiftuiOnRotate(
    _ h: ViewHandle,
    _ cb: @convention(c) (Float, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(h).gesture(
        RotateGesture().onChanged { value in
            callback(Float(value.rotation.degrees), userData)
        }
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// Canvas — custom drawing
// ═══════════════════════════════════════════════════════════════════════════

// Canvas needs a draw callback — we pass (context_ptr, width, height) to Rust
// and Rust calls back with draw commands. For now, provide predefined shapes.

@_cdecl("swiftui_canvas")
public func swiftuiCanvas(
    _ width: Float, _ height: Float,
    _ drawCb: @convention(c) (UnsafeMutableRawPointer?, Float, Float) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    // Since we can't pass GraphicsContext to Rust, use a fixed callback approach.
    // The callback receives (userData, width, height) and we just render a placeholder.
    let w = CGFloat(width); let h = CGFloat(height)
    return boxView(
        Canvas { context, size in
            // Notify Rust that canvas drew (for lifecycle tracking)
            drawCb(ud, Float(size.width), Float(size.height))
        }
        .frame(width: w, height: h)
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Geometry reader callback
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_geometry_reader")
public func swiftuiGeometryReader(
    _ buildCb: @convention(c) (Float, Float, UnsafeMutableRawPointer?) -> ViewHandle,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let cb = buildCb; let userData = ud
    struct GR: View {
        let cb: @convention(c) (Float, Float, UnsafeMutableRawPointer?) -> ViewHandle
        let ud: UnsafeMutableRawPointer?
        var body: some View {
            GeometryReader { geo in
                let handle = cb(Float(geo.size.width), Float(geo.size.height), ud)
                unboxView(handle)
            }
        }
    }
    return boxView(GR(cb: cb, ud: userData))
}

// ═══════════════════════════════════════════════════════════════════════════
// ScrollViewReader
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_scroll_view_reader")
public func swiftuiScrollViewReader(_ content: ViewHandle) -> ViewHandle {
    boxView(ScrollViewReader { _ in unboxView(content) })
}

@_cdecl("swiftui_scrollable_id")
public func swiftuiScrollableId(_ h: ViewHandle, _ idPtr: UnsafePointer<UInt8>, _ idLen: Int) -> ViewHandle {
    let id = String(bytes: UnsafeBufferPointer(start: idPtr, count: idLen), encoding: .utf8) ?? ""
    return boxView(unboxView(h).id(id))
}

// ═══════════════════════════════════════════════════════════════════════════
// TimelineView
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_timeline_view")
public func swiftuiTimelineView(
    _ intervalSeconds: Float,
    _ buildCb: @convention(c) (Float, UnsafeMutableRawPointer?) -> ViewHandle,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let cb = buildCb; let userData = ud
    struct TV: View {
        let interval: TimeInterval
        let cb: @convention(c) (Float, UnsafeMutableRawPointer?) -> ViewHandle
        let ud: UnsafeMutableRawPointer?
        var body: some View {
            TimelineView(.periodic(from: .now, by: interval)) { timeline in
                let elapsed = Float(timeline.date.timeIntervalSince1970.truncatingRemainder(dividingBy: 3600))
                let handle = cb(elapsed, ud)
                unboxView(handle)
            }
        }
    }
    return boxView(TV(interval: TimeInterval(intervalSeconds), cb: cb, ud: userData))
}

// ═══════════════════════════════════════════════════════════════════════════
// Map (MapKit)
// ═══════════════════════════════════════════════════════════════════════════

import MapKit

@_cdecl("swiftui_map")
public func swiftuiMap(_ lat: Float, _ lon: Float, _ spanLat: Float, _ spanLon: Float) -> ViewHandle {
    let region = MKCoordinateRegion(
        center: CLLocationCoordinate2D(latitude: Double(lat), longitude: Double(lon)),
        span: MKCoordinateSpan(latitudeDelta: Double(spanLat), longitudeDelta: Double(spanLon))
    )
    return boxView(Map(initialPosition: .region(region)))
}

// ═══════════════════════════════════════════════════════════════════════════
// VideoPlayer (AVKit)
// ═══════════════════════════════════════════════════════════════════════════

import AVKit

@_cdecl("swiftui_video_player")
public func swiftuiVideoPlayer(_ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> ViewHandle {
    let urlStr = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    if let url = URL(string: urlStr) {
        return boxView(VideoPlayer(player: AVPlayer(url: url)))
    }
    return boxView(Text("Invalid video URL"))
}

// ═══════════════════════════════════════════════════════════════════════════
// .searchable / .refreshable / .swipeActions / .onChange
// ═══════════════════════════════════════════════════════════════════════════

class SearchModel: ObservableObject {
    @Published var text: String = ""
    var onChange: ((String) -> Void)?
    init(_ cb: ((String) -> Void)?) { self.onChange = cb }
}

struct SearchableWrapper: View {
    let base: AnyView
    @ObservedObject var model: SearchModel
    var body: some View {
        base.searchable(text: $model.text)
            .onChange(of: model.text) { _, new in model.onChange?(new) }
    }
}

@_cdecl("swiftui_searchable")
public func swiftuiSearchable(
    _ h: ViewHandle,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    let model = SearchModel { text in
        text.withCString { ptr in
            callback(UnsafePointer(OpaquePointer(ptr)), text.utf8.count, userData)
        }
    }
    return boxView(SearchableWrapper(base: unboxView(h), model: model))
}

@_cdecl("swiftui_refreshable")
public func swiftuiRefreshable(
    _ h: ViewHandle,
    _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(h).refreshable { callback(userData) })
}

@_cdecl("swiftui_swipe_actions_delete")
public func swiftuiSwipeActionsDelete(
    _ h: ViewHandle,
    _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(h).swipeActions(edge: .trailing) {
        Button(role: .destructive) { callback(userData) } label: { Label("Delete", systemImage: "trash") }
    })
}

@_cdecl("swiftui_swipe_actions_custom")
public func swiftuiSwipeActionsCustom(_ h: ViewHandle, _ actions: ViewHandle, _ edge: Int32) -> ViewHandle {
    let e: HorizontalEdge = edge == 0 ? .leading : .trailing
    return boxView(unboxView(h).swipeActions(edge: e) { unboxView(actions) })
}

// ═══════════════════════════════════════════════════════════════════════════
// withAnimation — wraps a Rust callback in an animation block
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_with_animation")
public func swiftuiWithAnimation(
    _ type: Int32, _ duration: Float,
    _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let d = Double(duration)
    let anim: Animation? = switch type {
    case 0: .default
    case 1: .easeIn(duration: d)
    case 2: .easeOut(duration: d)
    case 3: .easeInOut(duration: d)
    case 4: .linear(duration: d)
    case 5: .spring(duration: d)
    case 6: .bouncy
    default: .default
    }
    withAnimation(anim) { cb(ud) }
}

// ═══════════════════════════════════════════════════════════════════════════
// matchedGeometryEffect — uses a shared namespace per window
// ═══════════════════════════════════════════════════════════════════════════

struct MatchedGeoWrapper: View {
    let base: AnyView
    let id: String
    @Namespace var ns
    var body: some View { base.matchedGeometryEffect(id: id, in: ns) }
}

@_cdecl("swiftui_matched_geometry")
public func swiftuiMatchedGeometry(_ h: ViewHandle, _ idPtr: UnsafePointer<UInt8>, _ idLen: Int) -> ViewHandle {
    let id = String(bytes: UnsafeBufferPointer(start: idPtr, count: idLen), encoding: .utf8) ?? ""
    return boxView(MatchedGeoWrapper(base: unboxView(h), id: id))
}

// ═══════════════════════════════════════════════════════════════════════════
// Canvas with draw commands — Rust sends draw ops via callback
// ═══════════════════════════════════════════════════════════════════════════

// Draw command types sent from Rust:
// 0 = fill_rect(x, y, w, h, r, g, b, a)
// 1 = fill_circle(cx, cy, radius, r, g, b, a)
// 2 = stroke_rect(x, y, w, h, r, g, b, a, lineWidth)
// 3 = fill_rounded_rect(x, y, w, h, cornerRadius, r, g, b, a)

@_cdecl("swiftui_canvas_drawing")
public func swiftuiCanvasDrawing(
    _ width: Float, _ height: Float,
    _ drawCb: @convention(c) (UnsafeMutableRawPointer, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let cb = drawCb; let userData = ud
    let w = CGFloat(width); let h = CGFloat(height)
    
    struct DrawingCanvas: View {
        let w: CGFloat; let h: CGFloat
        let cb: @convention(c) (UnsafeMutableRawPointer, UnsafeMutableRawPointer?) -> Void
        let ud: UnsafeMutableRawPointer?
        
        var body: some View {
            Canvas { context, size in
                var cmds = DrawCommands()
                let ptr = Unmanaged.passUnretained(cmds as AnyObject).toOpaque()
                // In practice, we'd need a way to collect commands.
                // For now, just invoke the callback for side effects.
                cb(ptr, ud)
                
                // Simple: draw a rect to prove canvas works
                context.fill(Path(CGRect(x: 0, y: 0, width: size.width, height: size.height)),
                            with: .color(.clear))
            }
            .frame(width: w, height: h)
        }
    }
    
    class DrawCommands {}
    
    return boxView(DrawingCanvas(w: w, h: h, cb: cb, ud: userData))
}

// ═══════════════════════════════════════════════════════════════════════════
// .task {} — async lifecycle modifier
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_task")
public func swiftuiTask(
    _ h: ViewHandle,
    _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(unboxView(h).task { callback(userData) })
}

// ═══════════════════════════════════════════════════════════════════════════
// PhotosPicker
// ═══════════════════════════════════════════════════════════════════════════

import PhotosUI

class PhotoPickerModel: ObservableObject {
    @Published var selection: PhotosPickerItem? = nil {
        didSet {
            if let item = selection {
                Task {
                    if let data = try? await item.loadTransferable(type: Data.self) {
                        await MainActor.run {
                            onSelect?(data)
                        }
                    }
                }
            }
        }
    }
    var onSelect: ((Data) -> Void)?
}

struct PhotoPickerWrapper: View {
    let label: String
    @ObservedObject var model: PhotoPickerModel
    var body: some View {
        PhotosPicker(selection: $model.selection, matching: .images) {
            Label(label, systemImage: "photo")
        }
    }
}

@_cdecl("swiftui_photos_picker")
public func swiftuiPhotosPicker(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    let callback = cb; let userData = ud
    let model = PhotoPickerModel()
    model.onSelect = { data in
        data.withUnsafeBytes { buf in
            callback(buf.baseAddress!.assumingMemoryBound(to: UInt8.self), buf.count, userData)
        }
    }
    return boxView(PhotoPickerWrapper(label: label, model: model))
}

// ═══════════════════════════════════════════════════════════════════════════
// Missing modifiers batch
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_blend_mode")
public func swiftuiBlendMode(_ h: ViewHandle, _ mode: Int32) -> ViewHandle {
    let m: BlendMode = switch mode {
    case 1: .multiply; case 2: .screen; case 3: .overlay
    case 4: .darken; case 5: .lighten; case 6: .colorDodge
    case 7: .colorBurn; case 8: .softLight; case 9: .hardLight
    case 10: .difference; case 11: .exclusion
    default: .normal
    }
    return boxView(unboxView(h).blendMode(m))
}

@_cdecl("swiftui_mask")
public func swiftuiMask(_ h: ViewHandle, _ mask: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).mask { unboxView(mask) })
}

@_cdecl("swiftui_drawing_group")
public func swiftuiDrawingGroup(_ h: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).drawingGroup())
}

@_cdecl("swiftui_allows_hit_testing")
public func swiftuiAllowsHitTesting(_ h: ViewHandle, _ enabled: Bool) -> ViewHandle {
    boxView(unboxView(h).allowsHitTesting(enabled))
}

@_cdecl("swiftui_content_shape")
public func swiftuiContentShape(_ h: ViewHandle, _ shape: Int32) -> ViewHandle {
    switch shape {
    case 1: return boxView(unboxView(h).contentShape(Circle()))
    case 2: return boxView(unboxView(h).contentShape(Capsule()))
    default: return boxView(unboxView(h).contentShape(Rectangle()))
    }
}

@_cdecl("swiftui_safe_area_inset_bottom")
public func swiftuiSafeAreaInsetBottom(_ h: ViewHandle, _ content: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).safeAreaInset(edge: .bottom) { unboxView(content) })
}

@_cdecl("swiftui_safe_area_inset_top")
public func swiftuiSafeAreaInsetTop(_ h: ViewHandle, _ content: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).safeAreaInset(edge: .top) { unboxView(content) })
}

@_cdecl("swiftui_list_row_background")
public func swiftuiListRowBackground(_ h: ViewHandle, _ bg: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).listRowBackground(unboxView(bg)))
}

@_cdecl("swiftui_list_row_separator")
public func swiftuiListRowSeparator(_ h: ViewHandle, _ visible: Bool) -> ViewHandle {
    boxView(unboxView(h).listRowSeparator(visible ? .visible : .hidden))
}

@_cdecl("swiftui_overlay_aligned")
public func swiftuiOverlayAligned(_ h: ViewHandle, _ content: ViewHandle, _ align: Int32) -> ViewHandle {
    let a: Alignment = switch align {
    case 1: .topLeading; case 2: .top; case 3: .topTrailing
    case 4: .leading; case 5: .center; case 6: .trailing
    case 7: .bottomLeading; case 8: .bottom; case 9: .bottomTrailing
    default: .center
    }
    return boxView(unboxView(h).overlay(alignment: a) { unboxView(content) })
}

@_cdecl("swiftui_background_aligned")
public func swiftuiBackgroundAligned(_ h: ViewHandle, _ content: ViewHandle, _ align: Int32) -> ViewHandle {
    let a: Alignment = switch align {
    case 1: .topLeading; case 2: .top; case 3: .topTrailing
    case 4: .leading; case 5: .center; case 6: .trailing
    case 7: .bottomLeading; case 8: .bottom; case 9: .bottomTrailing
    default: .center
    }
    return boxView(unboxView(h).background(alignment: a) { unboxView(content) })
}

// ═══════════════════════════════════════════════════════════════════════════
// @Environment / @AppStorage / Timer
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_preferred_color_scheme")
public func swiftuiPreferredColorScheme(_ h: ViewHandle, _ dark: Bool) -> ViewHandle {
    boxView(unboxView(h).preferredColorScheme(dark ? .dark : .light))
}

@_cdecl("swiftui_environment_dismiss")
public func swiftuiEnvironmentDismiss(_ h: ViewHandle) -> ViewHandle {
    // Wrap in a view that can dismiss itself
    struct Dismissable: View {
        let content: AnyView
        @Environment(\.dismiss) var dismiss
        var body: some View { content }
    }
    return boxView(Dismissable(content: unboxView(h)))
}

// AppStorage bridge: read/write UserDefaults
@_cdecl("swiftui_app_storage_get_string")
public func swiftuiAppStorageGetString(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>, _ outLen: UnsafeMutablePointer<Int>) -> Bool {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    guard let val = UserDefaults.standard.string(forKey: key) else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: val.utf8.count, alignment: 1)
    val.withCString { ptr in buf.copyMemory(from: ptr, byteCount: val.utf8.count) }
    outPtr.pointee = buf; outLen.pointee = val.utf8.count
    return true
}

@_cdecl("swiftui_app_storage_set_string")
public func swiftuiAppStorageSetString(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ valPtr: UnsafePointer<UInt8>, _ valLen: Int) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    let val = String(bytes: UnsafeBufferPointer(start: valPtr, count: valLen), encoding: .utf8) ?? ""
    UserDefaults.standard.set(val, forKey: key)
}

@_cdecl("swiftui_app_storage_get_int")
public func swiftuiAppStorageGetInt(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int) -> Int {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    return UserDefaults.standard.integer(forKey: key)
}

@_cdecl("swiftui_app_storage_set_int")
public func swiftuiAppStorageSetInt(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ val: Int) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    UserDefaults.standard.set(val, forKey: key)
}

@_cdecl("swiftui_app_storage_get_bool")
public func swiftuiAppStorageGetBool(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int) -> Bool {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    return UserDefaults.standard.bool(forKey: key)
}

@_cdecl("swiftui_app_storage_set_bool")
public func swiftuiAppStorageSetBool(_ keyPtr: UnsafePointer<UInt8>, _ keyLen: Int, _ val: Bool) {
    let key = String(bytes: UnsafeBufferPointer(start: keyPtr, count: keyLen), encoding: .utf8) ?? ""
    UserDefaults.standard.set(val, forKey: key)
}

// ═══════════════════════════════════════════════════════════════════════════
// NavigationStack with path
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_navigation_stack")
public func swiftuiNavigationStack(_ content: ViewHandle) -> ViewHandle {
    boxView(NavigationStack { unboxView(content) })
}

@_cdecl("swiftui_navigation_link")
public func swiftuiNavigationLink(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ destination: ViewHandle
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    return boxView(NavigationLink(label) { unboxView(destination) })
}

// ═══════════════════════════════════════════════════════════════════════════
// Symbol effects (iOS 17+ / macOS 14+)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_symbol_effect_bounce")
public func swiftuiSymbolEffectBounce(_ h: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).symbolEffect(.bounce))
}

@_cdecl("swiftui_symbol_effect_pulse")
public func swiftuiSymbolEffectPulse(_ h: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).symbolEffect(.pulse))
}

@_cdecl("swiftui_symbol_effect_variable_color")
public func swiftuiSymbolEffectVariableColor(_ h: ViewHandle) -> ViewHandle {
    boxView(unboxView(h).symbolEffect(.variableColor))
}

#if os(iOS)
@_cdecl("swiftui_sensory_feedback")
public func swiftuiSensoryFeedback(_ h: ViewHandle, _ type: Int32, _ trigger: Bool) -> ViewHandle {
    let f: SensoryFeedback = switch type {
    case 1: .success; case 2: .warning; case 3: .error
    case 4: .selection; case 5: .impact
    default: .success
    }
    return boxView(unboxView(h).sensoryFeedback(f, trigger: trigger))
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// onChange(of:) — observe state changes per-view
// ═══════════════════════════════════════════════════════════════════════════

class OnChangeModel: ObservableObject {
    @Published var value: Int = 0
}

@_cdecl("swiftui_on_change_int")
public func swiftuiOnChangeInt(
    _ h: ViewHandle, _ currentValue: Int,
    _ cb: @convention(c) (Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud; let val = currentValue
    return boxView(unboxView(h).onChange(of: val) { _, new in callback(new, userData) })
}

// ═══════════════════════════════════════════════════════════════════════════
// containerRelativeFrame
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_container_relative_frame")
public func swiftuiContainerRelativeFrame(_ h: ViewHandle, _ axes: Int32) -> ViewHandle {
    let a: Axis.Set = axes == 0 ? .horizontal : (axes == 1 ? .vertical : [.horizontal, .vertical])
    return boxView(unboxView(h).containerRelativeFrame(a))
}

// ═══════════════════════════════════════════════════════════════════════════
// Canvas draw commands — Rust builds a command buffer, Swift executes it
// ═══════════════════════════════════════════════════════════════════════════

// Command format: [type:u8][...params as f32s]
// Types:
//   0 = fill_rect(x, y, w, h, r, g, b, a)
//   1 = fill_circle(cx, cy, radius, r, g, b, a)
//   2 = stroke_rect(x, y, w, h, r, g, b, a, lineWidth)
//   3 = fill_rounded_rect(x, y, w, h, cornerRadius, r, g, b, a)
//   4 = fill_ellipse(x, y, w, h, r, g, b, a)
//   5 = stroke_line(x1, y1, x2, y2, r, g, b, a, lineWidth)
//   6 = fill_text(text_ptr, text_len, x, y, size, r, g, b, a) — special

@_cdecl("swiftui_canvas_commands")
public func swiftuiCanvasCommands(
    _ width: Float, _ height: Float,
    _ cmdPtr: UnsafePointer<Float>, _ cmdCount: Int
) -> ViewHandle {
    // Parse commands into an array
    let cmds = Array(UnsafeBufferPointer(start: cmdPtr, count: cmdCount))
    let w = CGFloat(width); let h = CGFloat(height)
    
    struct CmdCanvas: View {
        let w: CGFloat; let h: CGFloat; let cmds: [Float]
        var body: some View {
            Canvas { ctx, size in
                var i = 0
                while i < cmds.count {
                    let type = Int(cmds[i]); i += 1
                    switch type {
                    case 0: // fill_rect
                        guard i + 8 <= cmds.count else { return }
                        let rect = CGRect(x: CGFloat(cmds[i]), y: CGFloat(cmds[i+1]), width: CGFloat(cmds[i+2]), height: CGFloat(cmds[i+3]))
                        let color = Color(red: Double(cmds[i+4]), green: Double(cmds[i+5]), blue: Double(cmds[i+6]), opacity: Double(cmds[i+7]))
                        ctx.fill(Path(rect), with: .color(color)); i += 8
                    case 1: // fill_circle
                        guard i + 7 <= cmds.count else { return }
                        let r = CGFloat(cmds[i+2])
                        let rect = CGRect(x: CGFloat(cmds[i])-r, y: CGFloat(cmds[i+1])-r, width: r*2, height: r*2)
                        let color = Color(red: Double(cmds[i+3]), green: Double(cmds[i+4]), blue: Double(cmds[i+5]), opacity: Double(cmds[i+6]))
                        ctx.fill(Path(ellipseIn: rect), with: .color(color)); i += 7
                    case 2: // stroke_rect
                        guard i + 9 <= cmds.count else { return }
                        let rect = CGRect(x: CGFloat(cmds[i]), y: CGFloat(cmds[i+1]), width: CGFloat(cmds[i+2]), height: CGFloat(cmds[i+3]))
                        let color = Color(red: Double(cmds[i+4]), green: Double(cmds[i+5]), blue: Double(cmds[i+6]), opacity: Double(cmds[i+7]))
                        ctx.stroke(Path(rect), with: .color(color), lineWidth: CGFloat(cmds[i+8])); i += 9
                    case 3: // fill_rounded_rect
                        guard i + 9 <= cmds.count else { return }
                        let rect = CGRect(x: CGFloat(cmds[i]), y: CGFloat(cmds[i+1]), width: CGFloat(cmds[i+2]), height: CGFloat(cmds[i+3]))
                        let cr = CGFloat(cmds[i+4])
                        let color = Color(red: Double(cmds[i+5]), green: Double(cmds[i+6]), blue: Double(cmds[i+7]), opacity: Double(cmds[i+8]))
                        ctx.fill(Path(roundedRect: rect, cornerRadius: cr), with: .color(color)); i += 9
                    case 4: // fill_ellipse
                        guard i + 8 <= cmds.count else { return }
                        let rect = CGRect(x: CGFloat(cmds[i]), y: CGFloat(cmds[i+1]), width: CGFloat(cmds[i+2]), height: CGFloat(cmds[i+3]))
                        let color = Color(red: Double(cmds[i+4]), green: Double(cmds[i+5]), blue: Double(cmds[i+6]), opacity: Double(cmds[i+7]))
                        ctx.fill(Path(ellipseIn: rect), with: .color(color)); i += 8
                    case 5: // stroke_line
                        guard i + 9 <= cmds.count else { return }
                        var path = Path()
                        path.move(to: CGPoint(x: CGFloat(cmds[i]), y: CGFloat(cmds[i+1])))
                        path.addLine(to: CGPoint(x: CGFloat(cmds[i+2]), y: CGFloat(cmds[i+3])))
                        let color = Color(red: Double(cmds[i+4]), green: Double(cmds[i+5]), blue: Double(cmds[i+6]), opacity: Double(cmds[i+7]))
                        ctx.stroke(path, with: .color(color), lineWidth: CGFloat(cmds[i+8])); i += 9
                    default:
                        return
                    }
                }
            }.frame(width: w, height: h)
        }
    }
    return boxView(CmdCanvas(w: w, h: h, cmds: cmds))
}

// ═══════════════════════════════════════════════════════════════════════════
// Keyframe / Phase animations
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_phase_animation")
public func swiftuiPhaseAnimation(_ h: ViewHandle, _ phaseCount: Int32) -> ViewHandle {
    // Phase animation cycles through opacity values
    let phases = (0..<Int(phaseCount)).map { Double($0) / Double(phaseCount) }
    return boxView(unboxView(h).phaseAnimator(phases) { content, phase in
        content.opacity(0.3 + phase * 0.7)
    })
}

@_cdecl("swiftui_phase_animation_scale")
public func swiftuiPhaseAnimationScale(_ h: ViewHandle, _ scales: UnsafePointer<Float>, _ count: Int) -> ViewHandle {
    let s = (0..<count).map { Double(scales[$0]) }
    return boxView(unboxView(h).phaseAnimator(s) { content, phase in
        content.scaleEffect(phase)
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// Custom bezier timing curve
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_animation_bezier")
public func swiftuiAnimationBezier(_ h: ViewHandle, _ x1: Float, _ y1: Float, _ x2: Float, _ y2: Float, _ duration: Float) -> ViewHandle {
    let curve = UnitCurve.bezier(startControlPoint: UnitPoint(x: Double(x1), y: Double(y1)), endControlPoint: UnitPoint(x: Double(x2), y: Double(y2)))
    return boxView(unboxView(h).animation(.timingCurve(curve, duration: Double(duration)), value: true))
}

// ═══════════════════════════════════════════════════════════════════════════
// KeyframeAnimator — offset/scale/rotation keyframes
// ═══════════════════════════════════════════════════════════════════════════

struct KFState {
    var offsetX: CGFloat = 0; var offsetY: CGFloat = 0
    var scale: CGFloat = 1; var rotation: CGFloat = 0; var opacity: CGFloat = 1
}

@_cdecl("swiftui_keyframe_animation")
public func swiftuiKeyframeAnimation(
    _ h: ViewHandle,
    _ keyframes: UnsafePointer<Float>, _ count: Int, _ trigger: Bool
) -> ViewHandle {
    // Keyframe format: [duration, offsetX, offsetY, scale, rotation, opacity] × N
    let stride = 6
    let kfCount = count / stride
    var kfs: [(Double, CGFloat, CGFloat, CGFloat, CGFloat, CGFloat)] = []
    for i in 0..<kfCount {
        let base = i * stride
        kfs.append((
            Double(keyframes[base]),
            CGFloat(keyframes[base+1]), CGFloat(keyframes[base+2]),
            CGFloat(keyframes[base+3]), CGFloat(keyframes[base+4]),
            CGFloat(keyframes[base+5])
        ))
    }
    
    struct KFView: View {
        let content: AnyView; let kfs: [(Double, CGFloat, CGFloat, CGFloat, CGFloat, CGFloat)]; let trigger: Bool
        var body: some View {
            content.keyframeAnimator(initialValue: KFState(), trigger: trigger) { view, value in
                view.offset(x: value.offsetX, y: value.offsetY)
                    .scaleEffect(value.scale)
                    .rotationEffect(.degrees(value.rotation))
                    .opacity(value.opacity)
            } keyframes: { _ in
                KeyframeTrack(\.offsetX) {
                    for kf in kfs { CubicKeyframe(kf.1, duration: kf.0) }
                }
                KeyframeTrack(\.offsetY) {
                    for kf in kfs { CubicKeyframe(kf.2, duration: kf.0) }
                }
                KeyframeTrack(\.scale) {
                    for kf in kfs { CubicKeyframe(kf.3, duration: kf.0) }
                }
                KeyframeTrack(\.rotation) {
                    for kf in kfs { CubicKeyframe(kf.4, duration: kf.0) }
                }
                KeyframeTrack(\.opacity) {
                    for kf in kfs { CubicKeyframe(kf.5, duration: kf.0) }
                }
            }
        }
    }
    return boxView(KFView(content: unboxView(h), kfs: kfs, trigger: trigger))
}

// ═══════════════════════════════════════════════════════════════════════════
// ScrollViewReader.scrollTo
// ═══════════════════════════════════════════════════════════════════════════

class ScrollProxyHolder: ObservableObject {
    var proxy: ScrollViewProxy?
    var pendingScroll: String?
}

struct ScrollReaderWrapper: View {
    let content: AnyView
    @ObservedObject var holder: ScrollProxyHolder
    var body: some View {
        ScrollViewReader { proxy in
            content.onAppear {
                holder.proxy = proxy
                if let id = holder.pendingScroll {
                    proxy.scrollTo(id, anchor: .top)
                    holder.pendingScroll = nil
                }
            }
        }
    }
}

@_cdecl("swiftui_scroll_reader_create")
public func swiftuiScrollReaderCreate(_ content: ViewHandle) -> ViewHandle {
    let holder = ScrollProxyHolder()
    let wrapper = ScrollReaderWrapper(content: unboxView(content), holder: holder)
    let handle = boxView(wrapper)
    // Store holder reference for scrollTo calls
    scrollProxyStore[handle] = holder
    return handle
}

@_cdecl("swiftui_scroll_to")
public func swiftuiScrollTo(_ readerHandle: ViewHandle, _ idPtr: UnsafePointer<UInt8>, _ idLen: Int) {
    let id = String(bytes: UnsafeBufferPointer(start: idPtr, count: idLen), encoding: .utf8) ?? ""
    if let holder = scrollProxyStore[readerHandle] {
        if let proxy = holder.proxy {
            DispatchQueue.main.async { proxy.scrollTo(id, anchor: .top) }
        } else {
            holder.pendingScroll = id
        }
    }
}

private var scrollProxyStore: [ViewHandle: ScrollProxyHolder] = [:]

// ═══════════════════════════════════════════════════════════════════════════
// @FocusState
// ═══════════════════════════════════════════════════════════════════════════

class FocusModel: ObservableObject {
    @Published var focusedField: String? = nil
}

struct FocusableTextField: View {
    let placeholder: String
    @ObservedObject var model: BindableString
    let fieldId: String
    @ObservedObject var focusModel: FocusModel
    @FocusState private var isFocused: Bool
    
    var body: some View {
        TextField(placeholder, text: $model.value)
            .textFieldStyle(.roundedBorder)
            .focused($isFocused)
            .onChange(of: focusModel.focusedField) { _, newValue in
                isFocused = (newValue == fieldId)
            }
            .onChange(of: isFocused) { _, newValue in
                if newValue { focusModel.focusedField = fieldId }
            }
    }
}

@_cdecl("swiftui_focusable_textfield")
public func swiftuiFocusableTextField(
    _ placeholderPtr: UnsafePointer<UInt8>, _ placeholderLen: Int,
    _ valuePtr: UnsafePointer<UInt8>, _ valueLen: Int,
    _ fieldIdPtr: UnsafePointer<UInt8>, _ fieldIdLen: Int,
    _ onChangeCb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ userData: UnsafeMutableRawPointer?,
    _ focusModelPtr: UnsafeMutableRawPointer?
) -> ViewHandle {
    let placeholder = String(bytes: UnsafeBufferPointer(start: placeholderPtr, count: placeholderLen), encoding: .utf8) ?? ""
    let value = String(bytes: UnsafeBufferPointer(start: valuePtr, count: valueLen), encoding: .utf8) ?? ""
    let fieldId = String(bytes: UnsafeBufferPointer(start: fieldIdPtr, count: fieldIdLen), encoding: .utf8) ?? ""
    let cb = onChangeCb; let ud = userData
    let stringModel = BindableString(value) { newVal in
        newVal.withCString { ptr in cb(UnsafePointer(OpaquePointer(ptr)), newVal.utf8.count, ud) }
    }
    let focusModel: FocusModel
    if let ptr = focusModelPtr {
        focusModel = Unmanaged<FocusModel>.fromOpaque(ptr).takeUnretainedValue()
    } else {
        focusModel = FocusModel()
    }
    return boxView(FocusableTextField(placeholder: placeholder, model: stringModel, fieldId: fieldId, focusModel: focusModel))
}

@_cdecl("swiftui_focus_model_create")
public func swiftuiFocusModelCreate() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(FocusModel()).toOpaque()
}

@_cdecl("swiftui_focus_model_set")
public func swiftuiFocusModelSet(_ ptr: UnsafeMutableRawPointer, _ idPtr: UnsafePointer<UInt8>, _ idLen: Int) {
    let model = Unmanaged<FocusModel>.fromOpaque(ptr).takeUnretainedValue()
    let id = String(bytes: UnsafeBufferPointer(start: idPtr, count: idLen), encoding: .utf8) ?? ""
    DispatchQueue.main.async { model.focusedField = id }
}

@_cdecl("swiftui_focus_model_clear")
public func swiftuiFocusModelClear(_ ptr: UnsafeMutableRawPointer) {
    let model = Unmanaged<FocusModel>.fromOpaque(ptr).takeUnretainedValue()
    DispatchQueue.main.async { model.focusedField = nil }
}

// ═══════════════════════════════════════════════════════════════════════════
// Typed Table (macOS)
// ═══════════════════════════════════════════════════════════════════════════

#if os(macOS)
struct SimpleTableRow: Identifiable {
    let id: Int
    let columns: [String]
}

@_cdecl("swiftui_typed_table")
public func swiftuiTypedTable(
    _ headerPtrs: UnsafePointer<UnsafePointer<UInt8>>,
    _ headerLens: UnsafePointer<Int>,
    _ headerCount: Int,
    _ cellPtrs: UnsafePointer<UnsafePointer<UInt8>>,
    _ cellLens: UnsafePointer<Int>,
    _ rowCount: Int
) -> ViewHandle {
    let headers = (0..<headerCount).map {
        String(bytes: UnsafeBufferPointer(start: headerPtrs[$0], count: headerLens[$0]), encoding: .utf8) ?? ""
    }
    let colCount = headerCount
    var rows: [SimpleTableRow] = []
    for r in 0..<rowCount {
        var cols: [String] = []
        for c in 0..<colCount {
            let idx = r * colCount + c
            if idx < rowCount * colCount {
                cols.append(String(bytes: UnsafeBufferPointer(start: cellPtrs[idx], count: cellLens[idx]), encoding: .utf8) ?? "")
            }
        }
        rows.append(SimpleTableRow(id: r, columns: cols))
    }
    
    // Build a Table with dynamic columns
    // SwiftUI Table requires static column count, so we use a List with HStack
    return boxView(
        VStack(alignment: .leading, spacing: 0) {
            // Header
            HStack {
                ForEach(headers.indices, id: \.self) { i in
                    Text(headers[i]).bold().frame(maxWidth: .infinity, alignment: .leading)
                }
            }.padding(.horizontal, 8).padding(.vertical, 4).background(Color.gray.opacity(0.2))
            Divider()
            // Rows
            List(rows) { row in
                HStack {
                    ForEach(row.columns.indices, id: \.self) { i in
                        Text(row.columns[i]).frame(maxWidth: .infinity, alignment: .leading)
                    }
                }
            }
        }
    )
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// Missing views: AsyncImage, ColorPicker, DatePicker, EditButton,
// EmptyView, GridRow, LazyHGrid, Table (real), PasteButton, RenameButton
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_async_image")
public func swiftuiAsyncImage(_ urlPtr: UnsafePointer<UInt8>, _ urlLen: Int) -> ViewHandle {
    let s = String(bytes: UnsafeBufferPointer(start: urlPtr, count: urlLen), encoding: .utf8) ?? ""
    if let url = URL(string: s) {
        return boxView(AsyncImage(url: url))
    }
    return boxView(Text("Invalid URL"))
}

@_cdecl("swiftui_color_picker_bound")
public func swiftuiColorPickerBound(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ r: Float, _ g: Float, _ b: Float,
    _ cb: @convention(c) (Float, Float, Float, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    let callback = cb; let userData = ud
    class CM: ObservableObject {
        @Published var color: Color { didSet { onChange?(color) } }
        var onChange: ((Color) -> Void)?
        init(_ c: Color, _ cb: ((Color) -> Void)?) { self.color = c; self.onChange = cb }
    }
    let model = CM(Color(red: Double(r), green: Double(g), blue: Double(b))) { c in
        // Extract RGB — approximate via NSColor
        #if os(macOS)
        if let ns = NSColor(c).usingColorSpace(.deviceRGB) {
            callback(Float(ns.redComponent), Float(ns.greenComponent), Float(ns.blueComponent), userData)
        }
        #endif
    }
    struct BCP: View {
        let label: String; @ObservedObject var model: CM
        var body: some View { ColorPicker(label, selection: $model.color) }
    }
    return boxView(BCP(label: label, model: model))
}

@_cdecl("swiftui_date_picker_bound")
public func swiftuiDatePickerBound(
    _ labelPtr: UnsafePointer<UInt8>, _ labelLen: Int,
    _ timestamp: Double,
    _ cb: @convention(c) (Double, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: labelPtr, count: labelLen), encoding: .utf8) ?? ""
    let callback = cb; let userData = ud
    class DM: ObservableObject {
        @Published var date: Date { didSet { onChange?(date) } }
        var onChange: ((Date) -> Void)?
        init(_ d: Date, _ cb: ((Date) -> Void)?) { self.date = d; self.onChange = cb }
    }
    let model = DM(Date(timeIntervalSince1970: timestamp)) { d in
        callback(d.timeIntervalSince1970, userData)
    }
    struct BDP: View {
        let label: String; @ObservedObject var model: DM
        var body: some View { DatePicker(label, selection: $model.date) }
    }
    return boxView(BDP(label: label, model: model))
}

@_cdecl("swiftui_empty_view")
public func swiftuiEmptyView() -> ViewHandle { boxView(EmptyView()) }

@_cdecl("swiftui_lazy_hgrid")
public func swiftuiLazyHGrid(_ children: UnsafePointer<ViewHandle>, _ count: Int, _ rows: Int32) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    let r = Array(repeating: GridItem(.flexible()), count: Int(rows))
    return boxView(LazyHGrid(rows: r, spacing: 8) { ForEach(views.indices, id: \.self) { views[$0] } })
}

// ═══════════════════════════════════════════════════════════════════════════
// Timer — periodic Rust callback
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_timer_start")
public func swiftuiTimerStart(
    _ intervalSeconds: Float,
    _ cb: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer {
    let callback = cb; let userData = ud
    let timer = Timer.scheduledTimer(withTimeInterval: TimeInterval(intervalSeconds), repeats: true) { _ in
        callback(userData)
    }
    return Unmanaged.passRetained(timer as AnyObject).toOpaque()
}

@_cdecl("swiftui_timer_stop")
public func swiftuiTimerStop(_ timerPtr: UnsafeMutableRawPointer) {
    let timer = Unmanaged<AnyObject>.fromOpaque(timerPtr).takeRetainedValue() as! Timer
    timer.invalidate()
}

// ═══════════════════════════════════════════════════════════════════════════
// Final missing views
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_any_view")
public func swiftuiAnyView(_ h: ViewHandle) -> ViewHandle { h } // already AnyView

#if os(iOS)
@_cdecl("swiftui_edit_button")
public func swiftuiEditButton() -> ViewHandle { boxView(EditButton()) }
#endif

@_cdecl("swiftui_grid_row")
public func swiftuiGridRow(_ children: UnsafePointer<ViewHandle>, _ count: Int) -> ViewHandle {
    let views = (0..<count).map { unboxView(children[$0]) }
    return boxView(GridRow { ForEach(views.indices, id: \.self) { views[$0] } })
}

// ═══════════════════════════════════════════════════════════════════════════
// @Environment — read common environment values
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("swiftui_env_color_scheme")
public func swiftuiEnvColorScheme(_ h: ViewHandle, _ cb: @convention(c) (Bool, UnsafeMutableRawPointer?) -> Void, _ ud: UnsafeMutableRawPointer?) -> ViewHandle {
    let callback = cb; let userData = ud
    struct EnvReader: View {
        let content: AnyView
        let cb: @convention(c) (Bool, UnsafeMutableRawPointer?) -> Void
        let ud: UnsafeMutableRawPointer?
        @Environment(\.colorScheme) var colorScheme
        var body: some View {
            content.onAppear { cb(colorScheme == .dark, ud) }
        }
    }
    return boxView(EnvReader(content: unboxView(h), cb: callback, ud: userData))
}

@_cdecl("swiftui_env_locale")
public func swiftuiEnvLocale(_ h: ViewHandle, _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void, _ ud: UnsafeMutableRawPointer?) -> ViewHandle {
    let callback = cb; let userData = ud
    struct LocaleReader: View {
        let content: AnyView
        let cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void
        let ud: UnsafeMutableRawPointer?
        @Environment(\.locale) var locale
        var body: some View {
            content.onAppear {
                let id = locale.identifier
                id.withCString { ptr in cb(UnsafePointer(OpaquePointer(ptr)), id.utf8.count, ud) }
            }
        }
    }
    return boxView(LocaleReader(content: unboxView(h), cb: callback, ud: userData))
}

#if os(macOS)
@_cdecl("swiftui_paste_button")
public func swiftuiPasteButton(
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) -> ViewHandle {
    let callback = cb; let userData = ud
    return boxView(PasteButton(payloadType: String.self) { strings in
        for s in strings {
            s.withCString { ptr in callback(UnsafePointer(OpaquePointer(ptr)), s.utf8.count, userData) }
        }
    })
}
#endif

@_cdecl("swiftui_rename_button")
public func swiftuiRenameButton() -> ViewHandle {
    boxView(RenameButton())
}
