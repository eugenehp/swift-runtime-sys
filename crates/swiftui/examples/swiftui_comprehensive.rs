//! Comprehensive SwiftUI test — builds a complex UI entirely from Rust.
//!
//! Build helper first:
//!   xcrun swiftc -emit-library swift_helper/SwiftUIHelper.swift \
//!     -o swift_helper/libSwiftUIHelper.dylib \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)

use std::ffi::{c_char, c_void};

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    fn NSApplicationLoad() -> bool;
}
unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

type V = *mut c_void;

macro_rules! func {
    ($h:expr, $name:literal, $t:ty) => {
        std::mem::transmute::<_, $t>(dlsym($h, concat!($name, "\0").as_ptr() as _))
    };
}

fn main() {
    unsafe {
        NSApplicationLoad();
        dlopen(
            c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(),
            1,
        );
        let h = dlopen(c"swift_helper/libSwiftUIHelper.dylib".as_ptr(), 2);
        assert!(!h.is_null(), "Build the helper first — see doc comment");

        // Resolve all functions
        type TextFn = unsafe extern "C" fn(*const u8, usize) -> V;
        type StyledTextFn =
            unsafe extern "C" fn(*const u8, usize, f32, i32, f32, f32, f32, f32) -> V;
        type ImageFn = unsafe extern "C" fn(*const u8, usize) -> V;
        type SpacerFn = unsafe extern "C" fn() -> V;
        type DividerFn = unsafe extern "C" fn() -> V;
        type StackFn = unsafe extern "C" fn(*const V, usize) -> V;
        type PaddingFn = unsafe extern "C" fn(V, f32) -> V;
        type FrameFn = unsafe extern "C" fn(V, f32, f32) -> V;
        type BgColorFn = unsafe extern "C" fn(V, f32, f32, f32, f32) -> V;
        type CornerFn = unsafe extern "C" fn(V, f32) -> V;
        type OpacityFn = unsafe extern "C" fn(V, f32) -> V;
        type BorderFn = unsafe extern "C" fn(V, f32, f32, f32, f32) -> V;
        type ScrollFn = unsafe extern "C" fn(V) -> V;
        type ButtonFn = unsafe extern "C" fn(
            *const u8,
            usize,
            unsafe extern "C" fn(*mut c_void),
            *mut c_void,
        ) -> V;
        type ToggleFn = unsafe extern "C" fn(*const u8, usize, bool) -> V;
        type ProgressFn = unsafe extern "C" fn(f32, f32) -> V;
        type ColorFn = unsafe extern "C" fn(f32, f32, f32, f32) -> V;
        type ShowFn = unsafe extern "C" fn(V, *const u8, usize, f32, f32);
        type TextFieldFn = unsafe extern "C" fn(*const u8, usize, *const u8, usize) -> V;

        let text: TextFn = func!(h, "swiftui_text", TextFn);
        let styled_text: StyledTextFn = func!(h, "swiftui_text_styled", StyledTextFn);
        let image: ImageFn = func!(h, "swiftui_system_image", ImageFn);
        let spacer: SpacerFn = func!(h, "swiftui_spacer", SpacerFn);
        let divider: DividerFn = func!(h, "swiftui_divider", DividerFn);
        let vstack: StackFn = func!(h, "swiftui_vstack", StackFn);
        let hstack: StackFn = func!(h, "swiftui_hstack", StackFn);
        let padding: PaddingFn = func!(h, "swiftui_padding", PaddingFn);
        let frame: FrameFn = func!(h, "swiftui_frame", FrameFn);
        let bg_color: BgColorFn = func!(h, "swiftui_background_color", BgColorFn);
        let corner: CornerFn = func!(h, "swiftui_corner_radius", CornerFn);
        let opacity: OpacityFn = func!(h, "swiftui_opacity", OpacityFn);
        let border: BorderFn = func!(h, "swiftui_border", BorderFn);
        let scroll: ScrollFn = func!(h, "swiftui_scroll_view", ScrollFn);
        let button: ButtonFn = func!(h, "swiftui_button", ButtonFn);
        let toggle: ToggleFn = func!(h, "swiftui_toggle", ToggleFn);
        let progress: ProgressFn = func!(h, "swiftui_progress", ProgressFn);
        let color: ColorFn = func!(h, "swiftui_color", ColorFn);
        let show: ShowFn = func!(h, "swiftui_show_window", ShowFn);
        let textfield: TextFieldFn = func!(h, "swiftui_textfield", TextFieldFn);

        // ── Build the UI tree from Rust ──

        let s = |text: &str| -> (*const u8, usize) { (text.as_ptr(), text.len()) };

        // Header
        let (p, l) = s("🦀 SwiftUI from Rust");
        let title = styled_text(p, l, 28.0, 1, 0.2, 0.4, 0.8, 1.0);

        let (p, l) = s("Built entirely with Rust + swift-runtime-sys");
        let subtitle = styled_text(p, l, 14.0, 2, 0.5, 0.5, 0.5, 1.0);

        let header = {
            let items = [title, subtitle];
            let stack = vstack(items.as_ptr(), items.len());
            padding(stack, 16.0)
        };

        // Stats row
        let (p, l) = s("chart.bar.fill");
        let chart_icon = image(p, l);
        let (p, l) = s("66 tests passing");
        let stat1 = text(p, l);
        let (p, l) = s("763 symbols bound");
        let stat2 = text(p, l);
        let stats_row = {
            let items = [chart_icon, stat1, spacer(), stat2];
            let row = hstack(items.as_ptr(), items.len());
            let row = padding(row, 12.0);
            let row = bg_color(row, 0.15, 0.15, 0.2, 1.0);
            corner(row, 8.0)
        };

        // Controls section
        let (p, l) = s("Click me!");
        unsafe extern "C" fn on_click(_: *mut c_void) {
            println!("Button clicked from Rust!");
        }
        let btn = button(p, l, on_click, std::ptr::null_mut());

        let tgl = {
            let (p, l) = s("Enable feature");
            toggle(p, l, true)
        };

        let prog = progress(0.7, 1.0);

        let (p, l) = s("Enter text");
        let (p2, l2) = s("Hello world");
        let tf = textfield(p, l, p2, l2);

        let controls = {
            let (p, l) = s("Controls");
            let label = styled_text(p, l, 18.0, 1, 1.0, 1.0, 1.0, 1.0);
            let items = [label, btn, tgl, prog, tf];
            let stack = vstack(items.as_ptr(), items.len());
            let stack = padding(stack, 16.0);
            let stack = bg_color(stack, 0.1, 0.1, 0.15, 1.0);
            corner(stack, 12.0)
        };

        // Color swatches
        let swatch = |r: f32, g: f32, b: f32, name: &str| {
            let c = color(r, g, b, 1.0);
            let c = frame(c, 40.0, 40.0);
            let c = corner(c, 6.0);
            let (p, l) = s(name);
            let label = styled_text(p, l, 10.0, 0, 0.7, 0.7, 0.7, 1.0);
            let items = [c, label];
            vstack(items.as_ptr(), items.len())
        };

        let swatches = {
            let items = [
                swatch(1.0, 0.3, 0.3, "Red"),
                swatch(0.3, 0.8, 0.3, "Green"),
                swatch(0.3, 0.5, 1.0, "Blue"),
                swatch(1.0, 0.8, 0.2, "Yellow"),
                swatch(0.8, 0.4, 1.0, "Purple"),
            ];
            let row = hstack(items.as_ptr(), items.len());
            padding(row, 8.0)
        };

        // Footer
        let (p, l) = s("swift-runtime-sys v0.0.3 • 65 source files • arm64 inline asm thunks");
        let footer = styled_text(p, l, 11.0, 0, 0.4, 0.4, 0.4, 1.0);
        let footer = padding(footer, 8.0);

        // Assemble everything
        let content = {
            let items = [
                header,
                divider(),
                stats_row,
                controls,
                swatches,
                spacer(),
                divider(),
                footer,
            ];
            let stack = vstack(items.as_ptr(), items.len());
            padding(stack, 20.0)
        };

        let root = scroll(content);
        let root = bg_color(root, 0.05, 0.05, 0.08, 1.0);
        let root = frame(root, -1.0, -1.0); // maxWidth/maxHeight infinity

        // Show it!
        let (p, l) = s("SwiftUI from Rust 🦀");
        show(root, p, l, 500.0, 600.0);
    }
}
