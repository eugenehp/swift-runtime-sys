//! SwiftUI view construction API.

use crate::handle::ViewHandle;
use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

type TextFn = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
type StyledTextFn =
    unsafe extern "C" fn(*const u8, usize, f32, i32, f32, f32, f32, f32) -> *mut c_void;
type ImageFn = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
type VoidToViewFn = unsafe extern "C" fn() -> *mut c_void;
type StackFn = unsafe extern "C" fn(*const *mut c_void, usize) -> *mut c_void;
type ModF32Fn = unsafe extern "C" fn(*mut c_void, f32) -> *mut c_void;
type FrameFn = unsafe extern "C" fn(*mut c_void, f32, f32) -> *mut c_void;
type ColorModFn = unsafe extern "C" fn(*mut c_void, f32, f32, f32, f32) -> *mut c_void;
type BorderFn = unsafe extern "C" fn(*mut c_void, f32, f32, f32, f32) -> *mut c_void;
type ScrollFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type ButtonFn = unsafe extern "C" fn(
    *const u8,
    usize,
    unsafe extern "C" fn(*mut c_void),
    *mut c_void,
) -> *mut c_void;
type ToggleFn = unsafe extern "C" fn(*const u8, usize, bool) -> *mut c_void;
type ProgressFn = unsafe extern "C" fn(f32, f32) -> *mut c_void;
type ColorFn = unsafe extern "C" fn(f32, f32, f32, f32) -> *mut c_void;
type TextFieldFn = unsafe extern "C" fn(*const u8, usize, *const u8, usize) -> *mut c_void;
type ShowFn = unsafe extern "C" fn(*mut c_void, *const u8, usize, f32, f32);
type ReleaseFn = unsafe extern "C" fn(*mut c_void);

/// The SwiftUI bridge — create views via a loaded Swift helper dylib.
pub struct SwiftUI {
    text_fn: TextFn,
    styled_text_fn: StyledTextFn,
    image_fn: ImageFn,
    spacer_fn: VoidToViewFn,
    divider_fn: VoidToViewFn,
    pub(crate) vstack_fn: StackFn,
    pub(crate) hstack_fn: StackFn,
    pub(crate) zstack_fn: StackFn,
    padding_fn: ModF32Fn,
    frame_fn: FrameFn,
    bg_color_fn: ColorModFn,
    corner_fn: ModF32Fn,
    opacity_fn: ModF32Fn,
    border_fn: BorderFn,
    scroll_fn: ScrollFn,
    button_fn: ButtonFn,
    toggle_fn: ToggleFn,
    progress_fn: ProgressFn,
    color_fn: ColorFn,
    textfield_fn: TextFieldFn,
    show_fn: ShowFn,
    pub(crate) release_fn: ReleaseFn,
}

fn resolve(h: *mut c_void, name: &[u8]) -> *mut c_void {
    let ptr = unsafe { dlsym(h, name.as_ptr() as *const c_char) };
    assert!(
        !ptr.is_null(),
        "Symbol not found: {}",
        std::str::from_utf8(&name[..name.len() - 1]).unwrap()
    );
    ptr
}

impl SwiftUI {
    /// Load the Swift helper dylib and resolve all view constructor functions.
    pub fn load(helper_path: &str) -> Result<Self, String> {
        unsafe {
            // Load SwiftUI framework
            dlopen(
                c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(),
                1,
            );

            let path = CString::new(helper_path).unwrap();
            let h = dlopen(path.as_ptr(), 2);
            if h.is_null() {
                return Err(format!("Failed to load helper: {helper_path}"));
            }

            Ok(Self {
                text_fn: std::mem::transmute(resolve(h, b"swiftui_text\0")),
                styled_text_fn: std::mem::transmute(resolve(h, b"swiftui_text_styled\0")),
                image_fn: std::mem::transmute(resolve(h, b"swiftui_system_image\0")),
                spacer_fn: std::mem::transmute(resolve(h, b"swiftui_spacer\0")),
                divider_fn: std::mem::transmute(resolve(h, b"swiftui_divider\0")),
                vstack_fn: std::mem::transmute(resolve(h, b"swiftui_vstack\0")),
                hstack_fn: std::mem::transmute(resolve(h, b"swiftui_hstack\0")),
                zstack_fn: std::mem::transmute(resolve(h, b"swiftui_zstack\0")),
                padding_fn: std::mem::transmute(resolve(h, b"swiftui_padding\0")),
                frame_fn: std::mem::transmute(resolve(h, b"swiftui_frame\0")),
                bg_color_fn: std::mem::transmute(resolve(h, b"swiftui_background_color\0")),
                corner_fn: std::mem::transmute(resolve(h, b"swiftui_corner_radius\0")),
                opacity_fn: std::mem::transmute(resolve(h, b"swiftui_opacity\0")),
                border_fn: std::mem::transmute(resolve(h, b"swiftui_border\0")),
                scroll_fn: std::mem::transmute(resolve(h, b"swiftui_scroll_view\0")),
                button_fn: std::mem::transmute(resolve(h, b"swiftui_button\0")),
                toggle_fn: std::mem::transmute(resolve(h, b"swiftui_toggle\0")),
                progress_fn: std::mem::transmute(resolve(h, b"swiftui_progress\0")),
                color_fn: std::mem::transmute(resolve(h, b"swiftui_color\0")),
                textfield_fn: std::mem::transmute(resolve(h, b"swiftui_textfield\0")),
                show_fn: std::mem::transmute(resolve(h, b"swiftui_show_window\0")),
                release_fn: std::mem::transmute(resolve(h, b"swiftui_release\0")),
            })
        }
    }

    fn handle(&self, ptr: *mut c_void) -> ViewHandle {
        ViewHandle::new(ptr, self.release_fn)
    }

    // ── Views ──

    pub fn text(&self, s: &str) -> ViewHandle {
        self.handle(unsafe { (self.text_fn)(s.as_ptr(), s.len()) })
    }

    pub fn styled_text(
        &self,
        s: &str,
        size: f32,
        weight: i32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> ViewHandle {
        self.handle(unsafe { (self.styled_text_fn)(s.as_ptr(), s.len(), size, weight, r, g, b, a) })
    }

    pub fn bold_text(&self, s: &str, size: f32) -> ViewHandle {
        self.styled_text(s, size, 1, 1.0, 1.0, 1.0, 1.0)
    }

    pub fn system_image(&self, name: &str) -> ViewHandle {
        self.handle(unsafe { (self.image_fn)(name.as_ptr(), name.len()) })
    }

    pub fn spacer(&self) -> ViewHandle {
        self.handle(unsafe { (self.spacer_fn)() })
    }

    pub fn divider(&self) -> ViewHandle {
        self.handle(unsafe { (self.divider_fn)() })
    }

    pub fn color(&self, r: f32, g: f32, b: f32, a: f32) -> ViewHandle {
        self.handle(unsafe { (self.color_fn)(r, g, b, a) })
    }

    pub fn progress(&self, value: f32, total: f32) -> ViewHandle {
        self.handle(unsafe { (self.progress_fn)(value, total) })
    }

    pub fn toggle(&self, label: &str, is_on: bool) -> ViewHandle {
        self.handle(unsafe { (self.toggle_fn)(label.as_ptr(), label.len(), is_on) })
    }

    pub fn textfield(&self, placeholder: &str, value: &str) -> ViewHandle {
        self.handle(unsafe {
            (self.textfield_fn)(
                placeholder.as_ptr(),
                placeholder.len(),
                value.as_ptr(),
                value.len(),
            )
        })
    }

    // ── Stacks ──

    pub fn vstack(&self, children: &[ViewHandle]) -> ViewHandle {
        let ptrs: Vec<*mut c_void> = children.iter().map(|h| h.as_raw()).collect();
        self.handle(unsafe { (self.vstack_fn)(ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn hstack(&self, children: &[ViewHandle]) -> ViewHandle {
        let ptrs: Vec<*mut c_void> = children.iter().map(|h| h.as_raw()).collect();
        self.handle(unsafe { (self.hstack_fn)(ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn zstack(&self, children: &[ViewHandle]) -> ViewHandle {
        let ptrs: Vec<*mut c_void> = children.iter().map(|h| h.as_raw()).collect();
        self.handle(unsafe { (self.zstack_fn)(ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn scroll(&self, content: &ViewHandle) -> ViewHandle {
        self.handle(unsafe { (self.scroll_fn)(content.as_raw()) })
    }

    // ── Modifiers ──

    pub fn padding(&self, view: &ViewHandle, amount: f32) -> ViewHandle {
        self.handle(unsafe { (self.padding_fn)(view.as_raw(), amount) })
    }

    pub fn frame(&self, view: &ViewHandle, w: f32, h: f32) -> ViewHandle {
        self.handle(unsafe { (self.frame_fn)(view.as_raw(), w, h) })
    }

    pub fn background(&self, view: &ViewHandle, r: f32, g: f32, b: f32, a: f32) -> ViewHandle {
        self.handle(unsafe { (self.bg_color_fn)(view.as_raw(), r, g, b, a) })
    }

    pub fn corner_radius(&self, view: &ViewHandle, radius: f32) -> ViewHandle {
        self.handle(unsafe { (self.corner_fn)(view.as_raw(), radius) })
    }

    pub fn opacity(&self, view: &ViewHandle, value: f32) -> ViewHandle {
        self.handle(unsafe { (self.opacity_fn)(view.as_raw(), value) })
    }

    pub fn border(&self, view: &ViewHandle, r: f32, g: f32, b: f32, width: f32) -> ViewHandle {
        self.handle(unsafe { (self.border_fn)(view.as_raw(), r, g, b, width) })
    }

    // ── Button ──

    /// Create a button. The callback is called when clicked.
    ///
    /// # Safety
    /// The callback and userdata must remain valid for the lifetime of the button.
    pub fn button_raw(
        &self,
        label: &str,
        callback: unsafe extern "C" fn(*mut c_void),
        userdata: *mut c_void,
    ) -> ViewHandle {
        self.handle(unsafe { (self.button_fn)(label.as_ptr(), label.len(), callback, userdata) })
    }

    /// Create a button with a static callback (no userdata).
    pub fn button(&self, label: &str, callback: fn()) -> ViewHandle {
        unsafe extern "C" fn trampoline(ptr: *mut c_void) {
            let f: fn() = std::mem::transmute(ptr);
            f();
        }
        self.handle(unsafe {
            (self.button_fn)(
                label.as_ptr(),
                label.len(),
                trampoline,
                callback as *mut c_void,
            )
        })
    }

    // ── Window ──

    /// Show the view in a window. This blocks until the window is closed.
    pub fn show_window(&self, view: &ViewHandle, title: &str, width: f32, height: f32) {
        unsafe { (self.show_fn)(view.as_raw(), title.as_ptr(), title.len(), width, height) }
    }
}
