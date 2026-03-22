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
