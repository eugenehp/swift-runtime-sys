//! Winit API compatibility layer.
//!
//! Re-exports types and traits that match winit's public API, so you can write:
//!
//! ```ignore
//! use winit_swift::compat::*;
//!
//! // Same code as you'd write with winit:
//! let event_loop = EventLoop::new().unwrap();
//! event_loop.run_app(&mut MyApp::default()).unwrap();
//! ```
//!
//! # Migration from winit
//!
//! Replace `use winit::` with `use winit_swift::compat::` and add `use winit_swift::platform::*`
//! for Apple-specific extensions (Metal, haptics, etc.)


// ── Re-export dpi ───────────────────────────────────────────────────────────

pub mod dpi {
    /// A size in physical pixels.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PhysicalSize<T> {
        pub width: T,
        pub height: T,
    }

    impl<T> PhysicalSize<T> {
        pub fn new(width: T, height: T) -> Self {
            Self { width, height }
        }
    }

    /// A size in logical pixels (before scale factor).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct LogicalSize<T> {
        pub width: T,
        pub height: T,
    }

    impl<T> LogicalSize<T> {
        pub fn new(width: T, height: T) -> Self {
            Self { width, height }
        }
    }

    impl LogicalSize<f64> {
        pub fn to_physical(&self, scale: f64) -> PhysicalSize<u32> {
            PhysicalSize {
                width: (self.width * scale) as u32,
                height: (self.height * scale) as u32,
            }
        }
    }

    /// A position in physical pixels.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PhysicalPosition<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> PhysicalPosition<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    /// Physical insets (safe area).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PhysicalInsets<T> {
        pub top: T,
        pub left: T,
        pub bottom: T,
        pub right: T,
    }

    /// Size — either physical or logical.
    #[derive(Debug, Clone, Copy)]
    pub enum Size {
        Physical(PhysicalSize<u32>),
        Logical(LogicalSize<f64>),
    }

    impl From<PhysicalSize<u32>> for Size {
        fn from(s: PhysicalSize<u32>) -> Self { Size::Physical(s) }
    }
    impl From<LogicalSize<f64>> for Size {
        fn from(s: LogicalSize<f64>) -> Self { Size::Logical(s) }
    }

    /// Position — either physical or logical.
    #[derive(Debug, Clone, Copy)]
    pub enum Position {
        Physical(PhysicalPosition<i32>),
    }
}

// ── Event types matching winit ──────────────────────────────────────────────

pub mod event {
    use super::dpi::*;
    pub use crate::event::MouseButton;

    /// Describes the reason the event loop is resuming.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum StartCause {
        ResumeTimeReached {
            start: std::time::Instant,
            requested_resume: std::time::Instant,
        },
        WaitCancelled {
            start: std::time::Instant,
            requested_resume: Option<std::time::Instant>,
        },
        Poll,
        Init,
    }

    /// Element (key/button) state.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ElementState {
        Pressed,
        Released,
    }

    impl ElementState {
        pub fn is_pressed(self) -> bool {
            self == ElementState::Pressed
        }
    }

    /// A keyboard event.
    #[derive(Debug, Clone)]
    pub struct KeyEvent {
        /// The physical key code.
        pub physical_key: u16,
        /// The key state.
        pub state: ElementState,
        /// Whether this is a repeat.
        pub repeat: bool,
    }

    /// Window events.
    #[derive(Debug, Clone)]
    pub enum WindowEvent {
        /// Surface was resized.
        SurfaceResized(PhysicalSize<u32>),
        /// Window was moved.
        Moved(PhysicalPosition<i32>),
        /// Close button was pressed.
        CloseRequested,
        /// Window was destroyed.
        Destroyed,
        /// Window focus changed.
        Focused(bool),
        /// Keyboard input.
        KeyboardInput {
            event: KeyEvent,
            is_synthetic: bool,
        },
        /// Cursor moved.
        CursorMoved {
            position: PhysicalPosition<i32>,
        },
        /// Mouse button input.
        MouseInput {
            state: ElementState,
            button: MouseButton,
        },
        /// Scroll / trackpad.
        MouseWheel {
            delta_x: f64,
            delta_y: f64,
        },
        /// Scale factor changed.
        ScaleFactorChanged {
            scale_factor: f64,
        },
        /// Redraw requested.
        RedrawRequested,
        /// Theme changed.
        ThemeChanged(Theme),
        /// Touch started/moved/ended.
        Touch {
            phase: TouchPhase,
            location: PhysicalPosition<i32>,
        },
        /// Window occluded/unoccluded.
        Occluded(bool),
        /// Cursor entered the window.
        CursorEntered,
        /// Cursor left the window.
        CursorLeft,
        /// Pinch/magnify gesture.
        PinchGesture { delta: f64 },
        /// Rotation gesture.
        RotationGesture { delta: f64 },
    }

    /// Touch phase.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum TouchPhase {
        Started,
        Moved,
        Ended,
        Cancelled,
    }

    /// Theme (light/dark).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Theme {
        Light,
        Dark,
    }
}

// ── Window ──────────────────────────────────────────────────────────────────

pub mod window {
    
    use super::event::Theme;

    /// Window identifier.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WindowId(pub(crate) u64);

    impl WindowId {
        pub const fn into_raw(self) -> u64 { self.0 }
        pub const fn from_raw(id: u64) -> Self { Self(id) }
    }

    /// Window creation attributes — winit-compatible builder.
    #[derive(Debug, Clone)]
    pub struct WindowAttributes {
        pub(crate) title: String,
        pub(crate) width: f64,
        pub(crate) height: f64,
        pub(crate) transparent: bool,
        pub(crate) decorations: bool,
        pub(crate) visible: bool,
        pub(crate) resizable: bool,
        pub(crate) maximized: bool,
        pub(crate) theme: Option<Theme>,
        pub(crate) fullsize_content: bool,
        pub(crate) titlebar_hidden: bool,
        pub(crate) hdr: bool,
    }

    impl Default for WindowAttributes {
        fn default() -> Self {
            Self {
                title: "winit-swift".into(),
                width: 800.0,
                height: 600.0,
                transparent: false,
                decorations: true,
                visible: true,
                resizable: true,
                maximized: false,
                theme: None,
                fullsize_content: false,
                titlebar_hidden: false,
                hdr: false,
            }
        }
    }

    impl WindowAttributes {
        pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
            self.title = title.into();
            self
        }

        pub fn with_surface_size<S: Into<super::dpi::Size>>(mut self, size: S) -> Self {
            match size.into() {
                super::dpi::Size::Physical(s) => {
                    self.width = s.width as f64;
                    self.height = s.height as f64;
                }
                super::dpi::Size::Logical(s) => {
                    self.width = s.width;
                    self.height = s.height;
                }
            }
            self
        }

        pub fn with_transparent(mut self, transparent: bool) -> Self {
            self.transparent = transparent;
            self
        }

        pub fn with_decorations(mut self, decorations: bool) -> Self {
            self.decorations = decorations;
            self
        }

        pub fn with_visible(mut self, visible: bool) -> Self {
            self.visible = visible;
            self
        }

        pub fn with_resizable(mut self, resizable: bool) -> Self {
            self.resizable = resizable;
            self
        }

        pub fn with_maximized(mut self, maximized: bool) -> Self {
            self.maximized = maximized;
            self
        }

        pub fn with_theme(mut self, theme: Option<Theme>) -> Self {
            self.theme = theme;
            self
        }

        // Apple-specific extensions
        pub fn with_fullsize_content(mut self, v: bool) -> Self {
            self.fullsize_content = v;
            self
        }

        pub fn with_titlebar_hidden(mut self, v: bool) -> Self {
            self.titlebar_hidden = v;
            self
        }

        pub fn with_hdr(mut self, v: bool) -> Self {
            self.hdr = v;
            self
        }
    }

    /// A window — wraps our native Window.
    pub struct Window {
        pub(crate) inner: crate::Window,
    }

    impl Window {
        pub fn id(&self) -> WindowId {
            WindowId(self.inner.id().0)
        }

        pub fn scale_factor(&self) -> f64 {
            self.inner.scale_factor()
        }

        pub fn request_redraw(&self) {
            self.inner.request_redraw();
        }

        pub fn surface_size(&self) -> super::dpi::PhysicalSize<u32> {
            let (w, h) = self.inner.surface_size();
            super::dpi::PhysicalSize::new(w, h)
        }

        pub fn set_title(&self, title: &str) {
            self.inner.set_title(title);
        }

        pub fn set_visible(&self, visible: bool) {
            self.inner.set_visible(visible);
        }

        pub fn set_fullscreen(&self, fullscreen: bool) {
            self.inner.set_fullscreen(fullscreen);
        }

        pub fn set_decorations(&self, decorations: bool) {
            self.inner.set_decorations(decorations);
        }

        pub fn focus_window(&self) {
            self.inner.focus();
        }

        pub fn theme(&self) -> Option<super::event::Theme> {
            Some(match self.inner.theme() {
                crate::Theme::Light => super::event::Theme::Light,
                crate::Theme::Dark => super::event::Theme::Dark,
            })
        }

        /// Access the Metal layer for rendering.
        pub fn metal_layer(&self) -> Option<crate::MetalLayer> {
            self.inner.metal_layer()
        }

        pub fn raw_view_handle(&self) -> *mut std::ffi::c_void {
            self.inner.raw_view_handle()
        }

        // ── Full winit Window parity ──

        pub fn title(&self) -> String { self.inner.title() }

        pub fn outer_position(&self) -> super::dpi::PhysicalPosition<i32> {
            let (x, y) = self.inner.outer_position();
            super::dpi::PhysicalPosition::new(x, y)
        }

        pub fn set_outer_position(&self, x: i32, y: i32) { self.inner.set_outer_position(x, y); }

        pub fn outer_size(&self) -> super::dpi::PhysicalSize<u32> {
            let (w, h) = self.inner.outer_size();
            super::dpi::PhysicalSize::new(w, h)
        }

        pub fn safe_area(&self) -> super::dpi::PhysicalInsets<u32> {
            let (t, l, b, r) = self.inner.safe_area();
            super::dpi::PhysicalInsets { top: t, left: l, bottom: b, right: r }
        }

        pub fn set_min_surface_size(&self, w: f64, h: f64) { self.inner.set_min_size(w, h); }
        pub fn set_max_surface_size(&self, w: f64, h: f64) { self.inner.set_max_size(w, h); }

        pub fn set_resizable(&self, v: bool) { self.inner.set_resizable(v); }
        pub fn is_resizable(&self) -> bool { self.inner.is_resizable() }

        pub fn set_minimized(&self, v: bool) { self.inner.set_minimized(v); }
        pub fn is_minimized(&self) -> Option<bool> { Some(self.inner.is_minimized()) }

        pub fn set_maximized(&self, v: bool) { self.inner.set_maximized(v); }
        pub fn is_maximized(&self) -> bool { self.inner.is_maximized() }

        pub fn is_decorated(&self) -> bool { self.inner.is_decorated() }
        pub fn is_visible(&self) -> Option<bool> { Some(self.inner.is_visible()) }

        pub fn has_focus(&self) -> bool { self.inner.has_focus() }

        pub fn set_transparent(&self, v: bool) { self.inner.set_transparent(v); }
        pub fn set_blur(&self, v: bool) { self.inner.set_blur(v); }
        pub fn set_content_protected(&self, v: bool) { self.inner.set_content_protected(v); }

        pub fn set_window_level(&self, level: crate::window::WindowLevel) { self.inner.set_window_level(level); }
        pub fn set_theme(&self, theme: Option<super::event::Theme>) {
            self.inner.set_theme(theme.map(|t| match t {
                super::event::Theme::Dark => crate::Theme::Dark,
                super::event::Theme::Light => crate::Theme::Light,
            }));
        }

        pub fn set_cursor_visible(&self, v: bool) { self.inner.set_cursor_visible(v); }
        pub fn set_cursor_position(&self, x: f64, y: f64) { self.inner.set_cursor_position(x, y); }
        pub fn drag_window(&self) { self.inner.drag_window(); }
        pub fn request_user_attention(&self, critical: bool) { self.inner.request_user_attention(critical); }

        pub fn pre_present_notify(&self) { /* no-op on Apple */ }
    }

    impl std::fmt::Debug for Window {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Window")
                .field("id", &self.id())
                .finish()
        }
    }

    impl raw_window_handle::HasWindowHandle for Window {
        fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
            raw_window_handle::HasWindowHandle::window_handle(&self.inner)
        }
    }

    impl raw_window_handle::HasDisplayHandle for Window {
        fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
            raw_window_handle::HasDisplayHandle::display_handle(&self.inner)
        }
    }
}

// ── Application Handler ─────────────────────────────────────────────────────

pub mod application {
    use super::event::{StartCause, WindowEvent};
    use super::event_loop::ActiveEventLoop;
    use super::window::WindowId;

    /// The winit ApplicationHandler trait.
    pub trait ApplicationHandler {
        /// Called when surfaces can be created (app launched).
        fn can_create_surfaces(&mut self, event_loop: &ActiveEventLoop);

        /// Called for each window event.
        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        );

        /// Called when the event loop is about to wait.
        fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}

        /// Called when new events arrive.
        fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {}

        /// Called when the app is resumed.
        fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

        /// Called when the app is suspended.
        fn suspended(&mut self, _event_loop: &ActiveEventLoop) {}

        /// Called on memory warning.
        fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {}
    }
}

// ── Event Loop ──────────────────────────────────────────────────────────────

pub mod event_loop {
    use super::application::ApplicationHandler;
    use super::event::*;
    use super::window::{Window, WindowAttributes, WindowId};
    

    /// Control flow for the event loop.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ControlFlow {
        /// Process events as they arrive, then wait.
        Wait,
        /// Continuously poll for events (high CPU, low latency).
        Poll,
        /// Wait until the specified instant.
        WaitUntil(std::time::Instant),
    }

    impl Default for ControlFlow {
        fn default() -> Self {
            ControlFlow::Wait
        }
    }

    /// An active event loop — used to create windows and control flow.
    pub struct ActiveEventLoop {
        control_flow: std::cell::Cell<ControlFlow>,
        exit_requested: std::cell::Cell<bool>,
    }

    impl ActiveEventLoop {
        pub(crate) fn new() -> Self {
            Self {
                control_flow: std::cell::Cell::new(ControlFlow::Wait),
                exit_requested: std::cell::Cell::new(false),
            }
        }

        /// Create a window.
        pub fn create_window(&self, attrs: WindowAttributes) -> Result<Window, Box<dyn std::error::Error>> {
            let mut flags = crate::WindowFlags::empty();
            if attrs.transparent { flags |= crate::WindowFlags::TRANSPARENT; }
            if attrs.titlebar_hidden { flags |= crate::WindowFlags::TITLEBAR_HIDDEN; }
            if attrs.fullsize_content { flags |= crate::WindowFlags::FULLSIZE_CONTENT; }
            if attrs.hdr { flags |= crate::WindowFlags::HDR; }

            let native_attrs = crate::WindowAttributes {
                title: attrs.title,
                width: attrs.width,
                height: attrs.height,
                flags,
            };

            Ok(Window {
                inner: crate::Window::new(native_attrs),
            })
        }

        /// Set the control flow.
        pub fn set_control_flow(&self, cf: ControlFlow) {
            self.control_flow.set(cf);
        }

        /// Get the control flow.
        pub fn control_flow(&self) -> ControlFlow {
            self.control_flow.get()
        }

        /// Request event loop exit.
        pub fn exit(&self) {
            self.exit_requested.set(true);
        }

        pub(crate) fn exiting(&self) -> bool {
            self.exit_requested.get()
        }
    }

    impl std::fmt::Debug for ActiveEventLoop {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ActiveEventLoop").finish()
        }
    }

    /// The event loop.
    pub struct EventLoop {
        _initialized: bool,
    }

    impl EventLoop {
        /// Create a new event loop.
        pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
            crate::auto_load();
            let b = crate::fns();
            unsafe { (b.init)(super::super::event_dispatch_trampoline_compat) };
            Ok(EventLoop { _initialized: true })
        }

        /// Run the event loop with an ApplicationHandler.
        pub fn run_app<A: ApplicationHandler + 'static>(
            self,
            app: &mut A,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let event_loop = ActiveEventLoop::new();

            // Notify: can create surfaces
            app.new_events(&event_loop, StartCause::Init);
            app.can_create_surfaces(&event_loop);

            // Store handler
            COMPAT_HANDLER.with(|cell| {
                let el_ptr = &event_loop as *const ActiveEventLoop;
                let app_ptr = app as *mut A as *mut u8;

                // SAFETY: We know the event loop and app outlive the run call
                // because run() blocks until exit.
                cell.replace(Some(Box::new(move |raw_event: crate::Event| {
                    let event_loop = unsafe { &*el_ptr };
                    let app = unsafe { &mut *(app_ptr as *mut A) };

                    let wid = raw_event.window_id().map(|id| WindowId(id.0));

                    if let Some(wevent) = convert_event(&raw_event) {
                        if let Some(id) = wid {
                            app.window_event(event_loop, id, wevent);
                        }
                    }

                    match &raw_event {
                        crate::Event::Resumed => app.resumed(event_loop),
                        crate::Event::Suspended => app.suspended(event_loop),
                        _ => {}
                    }

                    if event_loop.exiting() {
                        unsafe { (crate::fns().stop_event_loop)() };
                    }
                })));
            });

            unsafe { (crate::fns().run_event_loop)() };
            Ok(())
        }

        /// Create an event loop proxy for cross-thread wake-ups.
        pub fn create_proxy(&self) -> EventLoopProxy {
            EventLoopProxy
        }
    }

    /// Event loop proxy for cross-thread communication.
    #[derive(Clone)]
    pub struct EventLoopProxy;

    impl EventLoopProxy {
        pub fn wake_up(&self) {
            // TODO: post NSEvent to wake the run loop
        }
    }

    thread_local! {
        static COMPAT_HANDLER: std::cell::RefCell<Option<Box<dyn FnMut(crate::Event)>>> =
            std::cell::RefCell::new(None);
    }

    pub(crate) fn dispatch_compat(raw: crate::Event) {
        COMPAT_HANDLER.with(|cell| {
            if let Some(handler) = cell.borrow_mut().as_mut() {
                handler(raw);
            }
        });
    }

    fn convert_event(raw: &crate::Event) -> Option<WindowEvent> {
        match raw {
            crate::Event::Resized { width, height, .. } =>
                Some(WindowEvent::SurfaceResized(super::dpi::PhysicalSize::new(*width, *height))),
            crate::Event::Moved { x, y, .. } =>
                Some(WindowEvent::Moved(super::dpi::PhysicalPosition::new(*x, *y))),
            crate::Event::CloseRequested(_) =>
                Some(WindowEvent::CloseRequested),
            crate::Event::Destroyed(_) =>
                Some(WindowEvent::Destroyed),
            crate::Event::Focused(_) =>
                Some(WindowEvent::Focused(true)),
            crate::Event::Unfocused(_) =>
                Some(WindowEvent::Focused(false)),
            crate::Event::KeyDown { keycode, .. } =>
                Some(WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        physical_key: *keycode,
                        state: ElementState::Pressed,
                        repeat: false,
                    },
                    is_synthetic: false,
                }),
            crate::Event::KeyUp { keycode, .. } =>
                Some(WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        physical_key: *keycode,
                        state: ElementState::Released,
                        repeat: false,
                    },
                    is_synthetic: false,
                }),
            crate::Event::MouseMoved { x, y, .. } =>
                Some(WindowEvent::CursorMoved {
                    position: super::dpi::PhysicalPosition::new(*x as i32, *y as i32),
                }),
            crate::Event::MouseButtonDown { button, .. } =>
                Some(WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: *button,
                }),
            crate::Event::MouseButtonUp { button, .. } =>
                Some(WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: *button,
                }),
            crate::Event::Scroll { dx, dy, .. } =>
                Some(WindowEvent::MouseWheel { delta_x: *dx, delta_y: *dy }),
            crate::Event::ScaleFactorChanged { scale, .. } =>
                Some(WindowEvent::ScaleFactorChanged { scale_factor: *scale }),
            crate::Event::RedrawRequested(_) =>
                Some(WindowEvent::RedrawRequested),
            crate::Event::ThemeChanged { theme, .. } =>
                Some(WindowEvent::ThemeChanged(match theme {
                    crate::Theme::Dark => Theme::Dark,
                    crate::Theme::Light => Theme::Light,
                })),
            crate::Event::TouchStart { x, y, .. } =>
                Some(WindowEvent::Touch {
                    phase: TouchPhase::Started,
                    location: super::dpi::PhysicalPosition::new(*x as i32, *y as i32),
                }),
            crate::Event::TouchMove { x, y, .. } =>
                Some(WindowEvent::Touch {
                    phase: TouchPhase::Moved,
                    location: super::dpi::PhysicalPosition::new(*x as i32, *y as i32),
                }),
            crate::Event::TouchEnd { x, y, .. } =>
                Some(WindowEvent::Touch {
                    phase: TouchPhase::Ended,
                    location: super::dpi::PhysicalPosition::new(*x as i32, *y as i32),
                }),
            crate::Event::Occluded { occluded, .. } =>
                Some(WindowEvent::Occluded(*occluded)),
            crate::Event::CursorEntered(_) =>
                Some(WindowEvent::CursorEntered),
            crate::Event::CursorLeft(_) =>
                Some(WindowEvent::CursorLeft),
            crate::Event::PinchGesture { delta, .. } =>
                Some(WindowEvent::PinchGesture { delta: *delta }),
            crate::Event::RotationGesture { delta, .. } =>
                Some(WindowEvent::RotationGesture { delta: *delta }),
            _ => None,
        }
    }
}
