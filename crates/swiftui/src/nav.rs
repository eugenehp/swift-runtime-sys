//! Navigation and multi-screen support.
//!
//! ```ignore
//! #[derive(Clone, PartialEq)]
//! enum Screen { Home, Detail(i32), Settings }
//!
//! app("Nav", 400.0, 600.0, |cx| {
//!     let screen = cx.state(Screen::Home);
//!     navigator(&screen, |s| match s {
//!         Screen::Home => home(cx, &screen),
//!         Screen::Detail(id) => detail(cx, *id, &screen),
//!         Screen::Settings => settings(cx, &screen),
//!     })
//! });
//! ```

use crate::state::State;
use crate::view::View;

/// Render a view based on current navigation state.
/// The `render` function receives the current screen value and returns a view.
pub fn navigator<S: std::any::Any + Send + Clone + 'static>(
    screen: &State<S>,
    render: impl FnOnce(&S) -> View,
) -> View {
    let current = screen.get();
    render(&current)
}

/// Create a button that navigates to a different screen.
///
/// ```ignore
/// nav_button("Go to Settings", &screen, Screen::Settings)
/// nav_button("Back", &screen, Screen::Home)
/// ```
pub fn nav_button<S: std::any::Any + Send + Clone + 'static>(
    label: &str,
    screen: &State<S>,
    destination: S,
) -> View {
    crate::state::button(label, screen.set_to(destination))
}

/// Create a back button.
pub fn back_button<S: std::any::Any + Send + Clone + 'static>(
    screen: &State<S>,
    destination: S,
) -> View {
    nav_button("← Back", screen, destination)
}
