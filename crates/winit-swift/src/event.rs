//! Event types — winit-compatible event model.

/// Window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// Theme (light/dark mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Theme {
    Light,
    Dark,
}

/// Application and window events.
#[derive(Debug, Clone)]
pub enum Event {
    /// Window surface was resized (physical pixels).
    Resized { id: WindowId, width: u32, height: u32 },
    /// Window was moved.
    Moved { id: WindowId, x: i32, y: i32 },
    /// Window close was requested.
    CloseRequested(WindowId),
    /// Window was destroyed.
    Destroyed(WindowId),
    /// Window gained focus.
    Focused(WindowId),
    /// Window lost focus.
    Unfocused(WindowId),
    /// Key pressed (raw keycode).
    KeyDown { id: WindowId, keycode: u16 },
    /// Key released (raw keycode).
    KeyUp { id: WindowId, keycode: u16 },
    /// Mouse/cursor moved (logical coordinates).
    MouseMoved { id: WindowId, x: f64, y: f64 },
    /// Mouse button pressed.
    MouseButtonDown { id: WindowId, button: MouseButton },
    /// Mouse button released.
    MouseButtonUp { id: WindowId, button: MouseButton },
    /// Scroll / trackpad input.
    Scroll { id: WindowId, dx: f64, dy: f64 },
    /// Scale factor (DPI) changed.
    ScaleFactorChanged { id: WindowId, scale: f64 },
    /// Redraw requested — render your frame now.
    RedrawRequested(WindowId),
    /// Touch started.
    TouchStart { id: WindowId, x: f64, y: f64 },
    /// Touch moved.
    TouchMove { id: WindowId, x: f64, y: f64 },
    /// Touch ended.
    TouchEnd { id: WindowId, x: f64, y: f64 },
    /// System theme changed.
    ThemeChanged { id: WindowId, theme: Theme },
    /// App resumed (became active).
    Resumed,
    /// App suspended (became inactive).
    Suspended,
    /// File drag entered window.
    DragEntered(WindowId),
    /// File drag left window.
    DragLeft(WindowId),
    /// File dropped on window.
    Dropped(WindowId),
    /// Window occluded/unoccluded.
    Occluded { id: WindowId, occluded: bool },
    /// Modifier keys changed.
    ModifiersChanged { id: WindowId, modifiers: u64 },
    /// Cursor entered the window.
    CursorEntered(WindowId),
    /// Cursor left the window.
    CursorLeft(WindowId),
    /// Pinch/magnify gesture.
    PinchGesture { id: WindowId, delta: f64 },
    /// Rotation gesture.
    RotationGesture { id: WindowId, delta: f64 },
    /// Unknown event.
    Unknown(u32),
}

impl Event {
    pub fn from_raw(event_type: u32, window_id: u64, a: i64, b: i64, x: f64, y: f64) -> Self {
        let id = WindowId(window_id);
        match event_type {
            1  => Event::Resized { id, width: a as u32, height: b as u32 },
            2  => Event::Moved { id, x: a as i32, y: b as i32 },
            3  => Event::CloseRequested(id),
            4  => Event::Destroyed(id),
            5  => Event::Focused(id),
            6  => Event::Unfocused(id),
            7  => Event::KeyDown { id, keycode: a as u16 },
            8  => Event::KeyUp { id, keycode: a as u16 },
            9  => Event::MouseMoved { id, x, y },
            10 => Event::MouseButtonDown { id, button: button_from(a) },
            11 => Event::MouseButtonUp { id, button: button_from(a) },
            12 => Event::Scroll { id, dx: x, dy: y },
            13 => Event::ScaleFactorChanged { id, scale: x },
            14 => Event::RedrawRequested(id),
            15 => Event::TouchStart { id, x, y },
            16 => Event::TouchMove { id, x, y },
            17 => Event::TouchEnd { id, x, y },
            18 => Event::ThemeChanged { id, theme: if a == 1 { Theme::Dark } else { Theme::Light } },
            19 => Event::Resumed,
            20 => Event::Suspended,
            21 => Event::DragEntered(id),
            22 => Event::DragLeft(id),
            23 => Event::Dropped(id),
            24 => Event::Occluded { id, occluded: a != 0 },
            25 => Event::ModifiersChanged { id, modifiers: a as u64 },
            26 => Event::CursorEntered(id),
            27 => Event::CursorLeft(id),
            28 => Event::PinchGesture { id, delta: x },
            29 => Event::RotationGesture { id, delta: x },
            _  => Event::Unknown(event_type),
        }
    }

    /// Get the window ID if this is a window event.
    pub fn window_id(&self) -> Option<WindowId> {
        match self {
            Event::Resized { id, .. } | Event::Moved { id, .. } |
            Event::CloseRequested(id) | Event::Destroyed(id) |
            Event::Focused(id) | Event::Unfocused(id) |
            Event::KeyDown { id, .. } | Event::KeyUp { id, .. } |
            Event::MouseMoved { id, .. } |
            Event::MouseButtonDown { id, .. } | Event::MouseButtonUp { id, .. } |
            Event::Scroll { id, .. } | Event::ScaleFactorChanged { id, .. } |
            Event::RedrawRequested(id) |
            Event::TouchStart { id, .. } | Event::TouchMove { id, .. } | Event::TouchEnd { id, .. } |
            Event::ThemeChanged { id, .. } |
            Event::DragEntered(id) | Event::DragLeft(id) | Event::Dropped(id) |
            Event::Occluded { id, .. } | Event::ModifiersChanged { id, .. } |
            Event::CursorEntered(id) | Event::CursorLeft(id) |
            Event::PinchGesture { id, .. } | Event::RotationGesture { id, .. } => Some(*id),
            _ => None,
        }
    }
}

fn button_from(raw: i64) -> MouseButton {
    match raw {
        0 => MouseButton::Left,
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        n => MouseButton::Other(n as u16),
    }
}
