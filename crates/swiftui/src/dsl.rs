//! SwiftUI DSL — free functions for building views.
//!
//! ```ignore
//! use swiftui::dsl::*;
//!
//! let view = vstack(&[
//!     text("Hello").bold().size(24).color(Color::BLUE),
//!     hstack(&[spacer(), text("World"), spacer()]),
//!     button("Click", || println!("clicked!")),
//! ]).padding(16).bg(Color::DARK).rounded(8);
//! ```

pub use crate::color::Color;
pub use crate::color::{hex, rgb, rgba};

pub(crate) use crate::context::with_ui;
use crate::handle::ViewHandle;
use crate::view::View;

// ═══════════════════════════════════════════════════════════════════════════
// View constructors
// ═══════════════════════════════════════════════════════════════════════════

/// Create a `Text` view.
pub fn text(s: &str) -> TextView {
    TextView {
        text: s.to_string(),
        font_size: None,
        weight: 0,
        color: None,
    }
}

/// A text builder with chainable style methods.
pub struct TextView {
    text: String,
    font_size: Option<f32>,
    weight: i32,
    color: Option<Color>,
}

impl TextView {
    pub fn size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }
    pub fn bold(mut self) -> Self {
        self.weight |= 1;
        self
    }
    pub fn italic(mut self) -> Self {
        self.weight |= 2;
        self
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }

    /// Build into a `View`.
    pub fn build(self) -> View {
        with_ui(|ui| {
            let c = self.color.unwrap_or(Color::WHITE);
            let size = self.font_size.unwrap_or(0.0);
            if size > 0.0 || self.weight != 0 || self.color.is_some() {
                View::new(ui.styled_text(
                    &self.text,
                    size.max(14.0),
                    self.weight,
                    c.r,
                    c.g,
                    c.b,
                    c.a,
                ))
            } else {
                View::new(ui.text(&self.text))
            }
        })
    }

    // ── Modifier pass-through (builds first, then applies modifier) ──

    pub fn padding(self, amount: f32) -> View {
        self.build().padding(amount)
    }
    pub fn bg(self, color: Color) -> View {
        self.build().bg(color)
    }
    pub fn rounded(self, radius: f32) -> View {
        self.build().rounded(radius)
    }
    pub fn frame(self, w: f32, h: f32) -> View {
        self.build().frame(w, h)
    }
    pub fn opacity(self, value: f32) -> View {
        self.build().opacity(value)
    }
    pub fn foreground(self, c: Color) -> View {
        self.build().foreground(c)
    }
    pub fn shadow(self, c: Color, r: f32, x: f32, y: f32) -> View {
        self.build().shadow(c, r, x, y)
    }
    pub fn offset(self, x: f32, y: f32) -> View {
        self.build().offset(x, y)
    }
    pub fn scale(self, factor: f32) -> View {
        self.build().scale(factor)
    }
    pub fn rotation(self, degrees: f32) -> View {
        self.build().rotation(degrees)
    }
    pub fn clip_circle(self) -> View {
        self.build().clip_circle()
    }
    pub fn font(self, size: f32, weight: crate::view::FontWeight) -> View {
        self.build().font(size, weight)
    }
    pub fn border(self, c: Color, width: f32) -> View {
        self.build().border(c, width)
    }
    pub fn overlay(self, other: View) -> View {
        self.build().overlay(other)
    }
    pub fn scroll(self) -> View {
        self.build().scroll()
    }
    pub fn frame_max(self) -> View {
        self.build().frame_max()
    }
    pub fn hidden(self) -> View {
        self.build().hidden()
    }
    pub fn disabled(self, d: bool) -> View {
        self.build().disabled(d)
    }
    pub fn on_tap(self, action: impl Fn() + 'static) -> View {
        self.build().on_tap(action)
    }
    pub fn on_long_press(self, action: impl Fn() + 'static) -> View {
        self.build().on_long_press(action)
    }
    pub fn styles(self, presets: &[crate::style::StylePreset]) -> View {
        use crate::style::MultiStyled;
        let view: View = self.into();
        view.styles(presets)
    }
}

/// Convert `TextView` to `View` — allows using it directly in stacks.
impl From<TextView> for View {
    fn from(tv: TextView) -> View {
        tv.build()
    }
}

/// A trait for things that can become a View (TextView, View, etc.)
pub trait IntoView {
    fn into_view(self) -> View;
}

impl IntoView for View {
    fn into_view(self) -> View {
        self
    }
}

impl IntoView for TextView {
    fn into_view(self) -> View {
        self.build()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Conditional helpers that auto-convert TextView → View
// ═══════════════════════════════════════════════════════════════════════════

/// `if/else` that auto-converts both branches to View.
/// No more `.into()` needed:
/// ```ignore
/// // Before: if x { text("a").into() } else { text("b").into() }
/// // After:
/// view_if(x, || text("a"), || text("b"))
/// ```
pub fn view_if<A: IntoView, B: IntoView>(
    cond: bool,
    if_true: impl FnOnce() -> A,
    if_false: impl FnOnce() -> B,
) -> View {
    if cond {
        if_true().into_view()
    } else {
        if_false().into_view()
    }
}

/// `if` with no else — returns empty spacer when false.
/// ```ignore
/// show_if(is_premium, || text("Premium"))
/// ```
pub fn show_if<A: IntoView>(cond: bool, view: impl FnOnce() -> A) -> View {
    if cond {
        view().into_view()
    } else {
        spacer()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Stacks — accept any IntoView
// ═══════════════════════════════════════════════════════════════════════════

/// Create a vertical stack.
pub fn vstack(children: Vec<View>) -> View {
    with_ui(|ui| {
        let handles: Vec<_> = children.iter().map(|v| v.handle()).collect();
        let ptrs: Vec<_> = handles.iter().map(|h| h.as_raw()).collect();
        let raw = unsafe { (ui.fns.vstack)(ptrs.as_ptr(), ptrs.len()) };
        View::new(crate::handle::ViewHandle::new(raw, ui.fns.release))
    })
}

/// Create a horizontal stack.
pub fn hstack(children: Vec<View>) -> View {
    with_ui(|ui| {
        let handles: Vec<_> = children.iter().map(|v| v.handle()).collect();
        let ptrs: Vec<_> = handles.iter().map(|h| h.as_raw()).collect();
        let raw = unsafe { (ui.fns.hstack)(ptrs.as_ptr(), ptrs.len()) };
        View::new(crate::handle::ViewHandle::new(raw, ui.fns.release))
    })
}

/// Create a z-axis stack.
pub fn zstack(children: Vec<View>) -> View {
    with_ui(|ui| {
        let handles: Vec<_> = children.iter().map(|v| v.handle()).collect();
        let ptrs: Vec<_> = handles.iter().map(|h| h.as_raw()).collect();
        let raw = unsafe { (ui.fns.zstack)(ptrs.as_ptr(), ptrs.len()) };
        View::new(crate::handle::ViewHandle::new(raw, ui.fns.release))
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// Macros for nicer stack syntax
// ═══════════════════════════════════════════════════════════════════════════

/// `vstack![view1, view2, ...]` — vertical stack macro.
#[macro_export]
macro_rules! vstack {
    ($($child:expr),* $(,)?) => {
        $crate::dsl::vstack(vec![$($crate::dsl::IntoView::into_view($child)),*])
    };
}

/// `hstack![view1, view2, ...]` — horizontal stack macro.
#[macro_export]
macro_rules! hstack {
    ($($child:expr),* $(,)?) => {
        $crate::dsl::hstack(vec![$($crate::dsl::IntoView::into_view($child)),*])
    };
}

/// `zstack![view1, view2, ...]` — z-axis stack macro.
#[macro_export]
macro_rules! zstack {
    ($($child:expr),* $(,)?) => {
        $crate::dsl::zstack(vec![$($crate::dsl::IntoView::into_view($child)),*])
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// Other views
// ═══════════════════════════════════════════════════════════════════════════

/// Create a spacer.
pub fn spacer() -> View {
    with_ui(|ui| View::new(ui.spacer()))
}

/// Create a divider.
pub fn divider() -> View {
    with_ui(|ui| View::new(ui.divider()))
}

/// Create an SF Symbol image.
pub fn image(system_name: &str) -> View {
    with_ui(|ui| View::new(ui.system_image(system_name)))
}

/// Create a color swatch view.
pub fn color(c: Color) -> View {
    with_ui(|ui| View::new(ui.color(c.r, c.g, c.b, c.a)))
}

/// Create a progress bar.
pub fn progress(value: f32, total: f32) -> View {
    with_ui(|ui| View::new(ui.progress(value, total)))
}

/// Create an indeterminate spinner.
pub fn spinner() -> View {
    with_ui(|ui| View::new(ui.progress(0.0, 0.0)))
}

/// Create a toggle.
pub fn toggle(label: &str, is_on: bool) -> View {
    with_ui(|ui| View::new(ui.toggle(label, is_on)))
}

/// Create a text field.
pub fn textfield(placeholder: &str, value: &str) -> View {
    with_ui(|ui| View::new(ui.textfield(placeholder, value)))
}

/// Create a button with a callback.
pub fn button(label: &str, action: fn()) -> View {
    with_ui(|ui| View::new(ui.button(label, action)))
}

/// Create a menu with a label and content.
pub fn menu(label: &str, content: View) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.menu)(label.as_ptr(), label.len(), content.handle().as_raw()) },
            ui.fns.release,
        ))
    })
}

/// Create a grid layout.
pub fn grid(columns: i32, children: Vec<View>) -> View {
    with_ui(|ui| {
        let ptrs: Vec<_> = children.iter().map(|v| v.handle().as_raw()).collect();
        View::new(ViewHandle::new(
            unsafe { (ui.fns.grid)(ptrs.as_ptr(), ptrs.len(), columns) },
            ui.fns.release,
        ))
    })
}

/// Create a Form.
pub fn form(children: Vec<View>) -> View {
    with_ui(|ui| {
        let ptrs: Vec<_> = children.iter().map(|v| v.handle().as_raw()).collect();
        View::new(ViewHandle::new(
            unsafe { (ui.fns.form)(ptrs.as_ptr(), ptrs.len()) },
            ui.fns.release,
        ))
    })
}

/// Create a Section with a title.
pub fn section(title: &str, children: Vec<View>) -> View {
    with_ui(|ui| {
        let ptrs: Vec<_> = children.iter().map(|v| v.handle().as_raw()).collect();
        View::new(ViewHandle::new(
            unsafe { (ui.fns.section)(title.as_ptr(), title.len(), ptrs.as_ptr(), ptrs.len()) },
            ui.fns.release,
        ))
    })
}

/// Create a real SwiftUI List.
pub fn list(children: Vec<View>) -> View {
    with_ui(|ui| {
        let ptrs: Vec<_> = children.iter().map(|v| v.handle().as_raw()).collect();
        View::new(ViewHandle::new(
            unsafe { (ui.fns.list)(ptrs.as_ptr(), ptrs.len()) },
            ui.fns.release,
        ))
    })
}

/// `list![view1, view2, ...]` macro.
#[macro_export]
macro_rules! list {
    ($($child:expr),* $(,)?) => {
        $crate::dsl::list(vec![$($crate::dsl::IntoView::into_view($child)),*])
    };
}

/// Create a password field.
pub fn secure_field(placeholder: &str, value: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe {
                (ui.fns.secure_field)(
                    placeholder.as_ptr(),
                    placeholder.len(),
                    value.as_ptr(),
                    value.len(),
                )
            },
            ui.fns.release,
        ))
    })
}

/// Create a multiline text editor.
pub fn text_editor(value: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.text_editor)(value.as_ptr(), value.len()) },
            ui.fns.release,
        ))
    })
}

/// Create a stepper.
pub fn stepper(label: &str, value: i32, min: i32, max: i32) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.stepper)(label.as_ptr(), label.len(), value, min, max) },
            ui.fns.release,
        ))
    })
}

/// Create a disclosure group (expandable section).
pub fn disclosure_group(title: &str, content: View) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe {
                (ui.fns.disclosure_group)(title.as_ptr(), title.len(), content.handle().as_raw())
            },
            ui.fns.release,
        ))
    })
}

/// Create a labeled content row.
pub fn labeled_content(label: &str, content: View) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe {
                (ui.fns.labeled_content)(label.as_ptr(), label.len(), content.handle().as_raw())
            },
            ui.fns.release,
        ))
    })
}

/// Create a NavigationSplitView with sidebar and detail.
pub fn navigation_split_view(sidebar: View, detail: View) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe {
                (ui.fns.navigation_split_view)(sidebar.handle().as_raw(), detail.handle().as_raw())
            },
            ui.fns.release,
        ))
    })
}

/// Create a ContentUnavailableView.
pub fn content_unavailable(title: &str, description: &str, system_image: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe {
                (ui.fns.content_unavailable)(
                    title.as_ptr(),
                    title.len(),
                    description.as_ptr(),
                    description.len(),
                    system_image.as_ptr(),
                    system_image.len(),
                )
            },
            ui.fns.release,
        ))
    })
}

/// Create a ShareLink.
pub fn share_link(text: &str, url: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.share_link)(text.as_ptr(), text.len(), url.as_ptr(), url.len()) },
            ui.fns.release,
        ))
    })
}

/// Create a photos picker.
pub fn photos_picker(label: &str, on_select: impl Fn(&[u8]) + 'static) -> View {
    let boxed: Box<Box<dyn Fn(*const u8, usize)>> = Box::new(Box::new(move |ptr, len| {
        let data = unsafe { std::slice::from_raw_parts(ptr, len) };
        on_select(data);
    }));
    let ud = Box::into_raw(boxed) as *mut core::ffi::c_void;
    unsafe extern "C" fn tramp(ptr: *const u8, len: usize, ud: *mut core::ffi::c_void) {
        let f = &*(ud as *const Box<dyn Fn(*const u8, usize)>);
        f(ptr, len);
    }
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.photos_picker)(label.as_ptr(), label.len(), tramp, ud) },
            ui.fns.release,
        ))
    })
}

/// Create a Map view centered on coordinates.
pub fn map(lat: f32, lon: f32, span_lat: f32, span_lon: f32) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.map)(lat, lon, span_lat, span_lon) },
            ui.fns.release,
        ))
    })
}

/// Create a video player from URL.
pub fn video_player(url: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.video_player)(url.as_ptr(), url.len()) },
            ui.fns.release,
        ))
    })
}

/// Create a group box with title.
pub fn group_box(title: &str, content: View) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.group_box)(title.as_ptr(), title.len(), content.handle().as_raw()) },
            ui.fns.release,
        ))
    })
}

/// Create a label with SF Symbol icon.
pub fn label(text: &str, system_image: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe {
                (ui.fns.label)(
                    text.as_ptr(),
                    text.len(),
                    system_image.as_ptr(),
                    system_image.len(),
                )
            },
            ui.fns.release,
        ))
    })
}

/// Create a slider.
pub fn slider(value: f32, min: f32, max: f32) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.slider)(value, min, max) },
            ui.fns.release,
        ))
    })
}

/// Create a link that opens a URL.
pub fn link(text: &str, url: &str) -> View {
    with_ui(|ui| {
        View::new(ViewHandle::new(
            unsafe { (ui.fns.link)(text.as_ptr(), text.len(), url.as_ptr(), url.len()) },
            ui.fns.release,
        ))
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// Window
// ═══════════════════════════════════════════════════════════════════════════

/// Show a view in a window. Blocks until the window is closed.
pub fn window(title: &str, width: f32, height: f32, view: View) {
    with_ui(|ui| ui.show_window(&view.handle, title, width, height));
}

// ═══════════════════════════════════════════════════════════════════════════
// txt! macro — unified text with inline formatting
// ═══════════════════════════════════════════════════════════════════════════

/// `txt!("Hello {name}, count={count}")` — text with inline format args.
/// Replaces both `text()` and `text_fmt!()`.
/// ```ignore
/// txt!("plain text")
/// txt!("Count: {}", count.get())
/// txt!("Hello {name}!")  // if name is a local variable
/// ```
#[macro_export]
macro_rules! txt {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        $crate::dsl::text(&format!($fmt $(, $($arg)*)?))
    };
}

/// Convert any IntoView to View. Eliminates `.into()`:
/// ```ignore
/// // Before: if x { text("a").into() } else { text("b").into() }
/// // After:  if x { v!(text("a")) } else { v!(text("b")) }
/// // Or use view_if:
/// //         view_if(x, || text("a"), || text("b"))
/// ```
#[macro_export]
macro_rules! v {
    ($e:expr) => {
        $crate::dsl::IntoView::into_view($e)
    };
}
