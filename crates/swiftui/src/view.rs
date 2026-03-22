//! The `View` trait — chainable modifiers on any SwiftUI view.

use crate::color::Color;
use crate::context::with_ui;
pub use crate::handle::ViewHandle;

/// A SwiftUI view with chainable modifiers.
///
/// Every view constructor returns a `View`, and modifiers return a new `View`.
///
/// ```ignore
/// text("Hello").bold().size(24).padding(16).bg(Color::DARK).rounded(8)
/// ```
pub struct View {
    pub(crate) handle: ViewHandle,
}

impl View {
    pub(crate) fn new(handle: ViewHandle) -> Self {
        Self { handle }
    }

    /// Get the raw handle (for stack assembly).
    pub fn handle(&self) -> &ViewHandle {
        &self.handle
    }

    /// Consume and return the handle.
    pub fn into_handle(self) -> ViewHandle {
        self.handle
    }

    // ── Layout modifiers ──

    pub fn padding(self, amount: f32) -> Self {
        with_ui(|ui| Self::new(ui.padding(&self.handle, amount)))
    }

    pub fn frame(self, w: f32, h: f32) -> Self {
        with_ui(|ui| Self::new(ui.frame(&self.handle, w, h)))
    }

    pub fn frame_max(self) -> Self {
        self.frame(-1.0, -1.0)
    }

    /// Convenience: apply padding, bg, and corner radius in one call.
    pub fn card(self, padding: f32, bg: crate::color::Color, radius: f32) -> Self {
        self.padding(padding).bg(bg).rounded(radius)
    }

    // ── Appearance modifiers ──

    pub fn bg(self, color: Color) -> Self {
        with_ui(|ui| Self::new(ui.background(&self.handle, color.r, color.g, color.b, color.a)))
    }

    pub fn rounded(self, radius: f32) -> Self {
        with_ui(|ui| Self::new(ui.corner_radius(&self.handle, radius)))
    }

    pub fn opacity(self, value: f32) -> Self {
        with_ui(|ui| Self::new(ui.opacity(&self.handle, value)))
    }

    pub fn border(self, color: Color, width: f32) -> Self {
        with_ui(|ui| Self::new(ui.border(&self.handle, color.r, color.g, color.b, width)))
    }

    pub fn foreground(self, color: Color) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.foreground_color)(
                        self.handle.as_raw(),
                        color.r,
                        color.g,
                        color.b,
                        color.a,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    pub fn shadow(self, color: Color, radius: f32, x: f32, y: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.shadow)(
                        self.handle.as_raw(),
                        color.r,
                        color.g,
                        color.b,
                        radius,
                        x,
                        y,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    pub fn offset(self, x: f32, y: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.offset)(self.handle.as_raw(), x, y) },
                ui.fns.release,
            ))
        })
    }

    pub fn scale(self, factor: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.scale)(self.handle.as_raw(), factor) },
                ui.fns.release,
            ))
        })
    }

    pub fn rotation(self, degrees: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.rotation)(self.handle.as_raw(), degrees) },
                ui.fns.release,
            ))
        })
    }

    pub fn hidden(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.hidden)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn disabled(self, disabled: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.disabled)(self.handle.as_raw(), disabled) },
                ui.fns.release,
            ))
        })
    }

    pub fn overlay(self, overlay: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.overlay)(self.handle.as_raw(), overlay.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn clip_circle(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.clip_circle)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn font(self, size: f32, weight: FontWeight) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.font_system)(self.handle.as_raw(), size, weight as i32) },
                ui.fns.release,
            ))
        })
    }

    // ── Gestures ──

    /// Tap gesture with a closure.
    pub fn on_tap(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_tap)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    /// Long press gesture with a closure.
    pub fn on_long_press(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_long_press)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    // ── Visual effects ──

    pub fn blur(self, radius: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.blur)(self.handle.as_raw(), radius) },
                ui.fns.release,
            ))
        })
    }

    pub fn brightness(self, amount: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.brightness)(self.handle.as_raw(), amount) },
                ui.fns.release,
            ))
        })
    }

    pub fn saturation(self, amount: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.saturation)(self.handle.as_raw(), amount) },
                ui.fns.release,
            ))
        })
    }

    pub fn grayscale(self, amount: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.grayscale)(self.handle.as_raw(), amount) },
                ui.fns.release,
            ))
        })
    }

    // ── Text layout ──

    pub fn line_limit(self, limit: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.line_limit)(self.handle.as_raw(), limit) },
                ui.fns.release,
            ))
        })
    }

    pub fn fixed_size_mod(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.fixed_size_mod)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Layout ──

    pub fn aspect_ratio(self, ratio: f32, fit: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.aspect_ratio)(self.handle.as_raw(), ratio, if fit { 0 } else { 1 })
                },
                ui.fns.release,
            ))
        })
    }

    pub fn clipped(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.clipped)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Misc ──

    pub fn tint(self, c: Color) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.tint)(self.handle.as_raw(), c.r, c.g, c.b) },
                ui.fns.release,
            ))
        })
    }

    pub fn badge(self, count: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.badge)(self.handle.as_raw(), count) },
                ui.fns.release,
            ))
        })
    }

    pub fn help(self, text: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.help_text)(self.handle.as_raw(), text.as_ptr(), text.len()) },
                ui.fns.release,
            ))
        })
    }

    // ── Animation ──

    pub fn animated(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation)(self.handle.as_raw(), 0) },
                ui.fns.release,
            ))
        })
    }

    pub fn spring(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation)(self.handle.as_raw(), 4) },
                ui.fns.release,
            ))
        })
    }

    pub fn bouncy(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation)(self.handle.as_raw(), 5) },
                ui.fns.release,
            ))
        })
    }

    // ── Presentation ──

    pub fn sheet(self, content: View, is_presented: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.sheet)(self.handle.as_raw(), content.handle.as_raw(), is_presented)
                },
                ui.fns.release,
            ))
        })
    }

    pub fn alert(self, title: &str, message: &str, is_presented: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.alert)(
                        self.handle.as_raw(),
                        title.as_ptr(),
                        title.len(),
                        message.as_ptr(),
                        message.len(),
                        is_presented,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    // ── Navigation / Toolbar ──

    pub fn navigation_title(self, title: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.navigation_title)(self.handle.as_raw(), title.as_ptr(), title.len())
                },
                ui.fns.release,
            ))
        })
    }

    pub fn toolbar(self, content: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.toolbar)(self.handle.as_raw(), content.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn context_menu(self, content: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.context_menu)(self.handle.as_raw(), content.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn popover(self, content: View, is_presented: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.popover)(self.handle.as_raw(), content.handle.as_raw(), is_presented)
                },
                ui.fns.release,
            ))
        })
    }

    // ── Typography ──

    pub fn bold_mod(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.bold_mod)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn italic_mod(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.italic_mod)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Lifecycle ──

    pub fn on_appear(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_appear)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    pub fn on_disappear(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_disappear)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    // ── Remaining modifiers ──

    pub fn color_invert(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.color_invert)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn ignores_safe_area(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.ignores_safe_area)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn confirmation_dialog(self, title: &str, is_presented: bool, actions: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.confirmation_dialog)(
                        self.handle.as_raw(),
                        title.as_ptr(),
                        title.len(),
                        is_presented,
                        actions.handle.as_raw(),
                    )
                },
                ui.fns.release,
            ))
        })
    }

    pub fn keyboard_shortcut(self, key: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.keyboard_shortcut)(self.handle.as_raw(), key.as_ptr(), key.len())
                },
                ui.fns.release,
            ))
        })
    }

    pub fn focusable(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.focusable)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    /// 0=tail, 1=middle, 2=head
    pub fn truncation_mode(self, mode: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.truncation_mode)(self.handle.as_raw(), mode) },
                ui.fns.release,
            ))
        })
    }

    /// 0=leading, 1=center, 2=trailing
    pub fn multiline_alignment(self, align: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.multiline_alignment)(self.handle.as_raw(), align) },
                ui.fns.release,
            ))
        })
    }

    pub fn minimum_scale_factor(self, factor: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.minimum_scale_factor)(self.handle.as_raw(), factor) },
                ui.fns.release,
            ))
        })
    }

    // ── Accessibility ──

    pub fn accessibility_label(self, label: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.accessibility_label)(self.handle.as_raw(), label.as_ptr(), label.len())
                },
                ui.fns.release,
            ))
        })
    }

    pub fn accessibility_hint(self, hint: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.accessibility_hint)(self.handle.as_raw(), hint.as_ptr(), hint.len())
                },
                ui.fns.release,
            ))
        })
    }

    pub fn accessibility_hidden(self, hidden: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.accessibility_hidden)(self.handle.as_raw(), hidden) },
                ui.fns.release,
            ))
        })
    }

    pub fn accessibility_value(self, value: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.accessibility_value)(self.handle.as_raw(), value.as_ptr(), value.len())
                },
                ui.fns.release,
            ))
        })
    }

    // ── Animation extended ──

    /// Animate with duration. type: 1=easeIn, 2=easeOut, 3=easeInOut, 4=linear
    pub fn ease_in(self, duration: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation_duration)(self.handle.as_raw(), 1, duration) },
                ui.fns.release,
            ))
        })
    }

    pub fn ease_out(self, duration: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation_duration)(self.handle.as_raw(), 2, duration) },
                ui.fns.release,
            ))
        })
    }

    pub fn ease_in_out(self, duration: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation_duration)(self.handle.as_raw(), 3, duration) },
                ui.fns.release,
            ))
        })
    }

    pub fn linear(self, duration: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation_duration)(self.handle.as_raw(), 4, duration) },
                ui.fns.release,
            ))
        })
    }

    pub fn spring_params(self, duration: f32, bounce: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation_spring_params)(self.handle.as_raw(), duration, bounce) },
                ui.fns.release,
            ))
        })
    }

    /// Transition. 0=opacity, 1=slide, 2=scale, 3..6=move(edge), 7..8=push
    pub fn transition_opacity(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.transition)(self.handle.as_raw(), 0) },
                ui.fns.release,
            ))
        })
    }

    pub fn transition_slide(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.transition)(self.handle.as_raw(), 1) },
                ui.fns.release,
            ))
        })
    }

    pub fn transition_scale(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.transition)(self.handle.as_raw(), 2) },
                ui.fns.release,
            ))
        })
    }

    // ── Gestures extended ──

    pub fn on_drag(self, action: impl Fn(f32, f32) + 'static) -> Self {
        let boxed: Box<Box<dyn Fn(f32, f32)>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(x: f32, y: f32, p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn(f32, f32)>);
            f(x, y);
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_drag)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    pub fn on_magnify(self, action: impl Fn(f32) + 'static) -> Self {
        let boxed: Box<Box<dyn Fn(f32)>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(v: f32, p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn(f32)>);
            f(v);
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_magnify)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    pub fn on_rotate(self, action: impl Fn(f32) + 'static) -> Self {
        let boxed: Box<Box<dyn Fn(f32)>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(v: f32, p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn(f32)>);
            f(v);
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_rotate)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    /// Set an ID for ScrollViewReader targeting.
    pub fn scroll_id(self, id: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.scrollable_id)(self.handle.as_raw(), id.as_ptr(), id.len()) },
                ui.fns.release,
            ))
        })
    }

    // ── Searchable / Refreshable / SwipeActions ──

    pub fn searchable(self, on_change: impl Fn(&str) + 'static) -> Self {
        let boxed: Box<Box<dyn Fn(*const u8, usize)>> = Box::new(Box::new(move |ptr, len| {
            let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) };
            on_change(s);
        }));
        let ud = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(ptr: *const u8, len: usize, ud: *mut core::ffi::c_void) {
            let f = &*(ud as *const Box<dyn Fn(*const u8, usize)>);
            f(ptr, len);
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.searchable)(self.handle.as_raw(), tramp, ud) },
                ui.fns.release,
            ))
        })
    }

    pub fn refreshable(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.refreshable)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    pub fn swipe_delete(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.swipe_actions_delete)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    pub fn swipe_actions(self, actions: View, leading: bool) -> Self {
        let edge = if leading { 0 } else { 1 };
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.swipe_actions_custom)(
                        self.handle.as_raw(),
                        actions.handle.as_raw(),
                        edge,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    // ── Matched geometry / Task ──

    pub fn matched_geometry(self, id: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.matched_geometry)(self.handle.as_raw(), id.as_ptr(), id.len()) },
                ui.fns.release,
            ))
        })
    }

    pub fn task(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.task)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    // ── Blend / Mask / Drawing ──

    /// 0=normal,1=multiply,2=screen,3=overlay,4=darken,5=lighten,6=colorDodge,7=colorBurn,8=softLight,9=hardLight,10=difference,11=exclusion
    pub fn blend_mode(self, mode: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.blend_mode)(self.handle.as_raw(), mode) },
                ui.fns.release,
            ))
        })
    }
    pub fn mask(self, mask_view: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.mask)(self.handle.as_raw(), mask_view.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }
    pub fn drawing_group(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.drawing_group)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }
    pub fn allows_hit_testing(self, enabled: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.allows_hit_testing)(self.handle.as_raw(), enabled) },
                ui.fns.release,
            ))
        })
    }
    /// 0=rectangle, 1=circle, 2=capsule
    pub fn content_shape(self, shape: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.content_shape)(self.handle.as_raw(), shape) },
                ui.fns.release,
            ))
        })
    }
    pub fn safe_area_inset_bottom(self, content: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.safe_area_inset_bottom)(self.handle.as_raw(), content.handle.as_raw())
                },
                ui.fns.release,
            ))
        })
    }
    pub fn safe_area_inset_top(self, content: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.safe_area_inset_top)(self.handle.as_raw(), content.handle.as_raw())
                },
                ui.fns.release,
            ))
        })
    }
    pub fn list_row_background(self, bg: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.list_row_background)(self.handle.as_raw(), bg.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }
    pub fn list_row_separator(self, visible: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.list_row_separator)(self.handle.as_raw(), visible) },
                ui.fns.release,
            ))
        })
    }
    /// alignment: 1=topLeading..9=bottomTrailing (3x3 grid, row-major)
    pub fn overlay_aligned(self, content: View, alignment: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.overlay_aligned)(
                        self.handle.as_raw(),
                        content.handle.as_raw(),
                        alignment,
                    )
                },
                ui.fns.release,
            ))
        })
    }
    pub fn background_aligned(self, content: View, alignment: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.background_aligned)(
                        self.handle.as_raw(),
                        content.handle.as_raw(),
                        alignment,
                    )
                },
                ui.fns.release,
            ))
        })
    }
    pub fn preferred_color_scheme(self, dark: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.preferred_color_scheme)(self.handle.as_raw(), dark) },
                ui.fns.release,
            ))
        })
    }

    // ── Symbol effects ──

    pub fn symbol_bounce(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.symbol_effect_bounce)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }
    pub fn symbol_pulse(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.symbol_effect_pulse)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }
    pub fn symbol_variable_color(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.symbol_effect_variable_color)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Navigation ──

    pub fn navigation_stack(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.navigation_stack)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Container modifiers ──

    pub fn scroll(self) -> Self {
        with_ui(|ui| Self::new(ui.scroll(&self.handle)))
    }
}

/// Font weight for the `.font()` modifier.
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum FontWeight {
    Regular = 0,
    Bold = 1,
    Semibold = 2,
    Heavy = 3,
    Light = 4,
    Thin = 5,
    Medium = 6,
}
