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
