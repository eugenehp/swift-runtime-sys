//! Thread-local SwiftUI context.

use crate::views::SwiftUI;
use std::cell::RefCell;

thread_local! {
    static UI: RefCell<Option<SwiftUI>> = RefCell::new(None);
}

/// Initialize the thread-local SwiftUI context.
pub fn init(helper_path: &str) {
    let ui = SwiftUI::load(helper_path).expect("Failed to load SwiftUI helper");
    UI.with(|cell| {
        *cell.borrow_mut() = Some(ui);
    });
}

/// Run a closure with access to the SwiftUI context.
pub(crate) fn with_ui<R>(f: impl FnOnce(&SwiftUI) -> R) -> R {
    UI.with(|cell| {
        let borrow = cell.borrow();
        let ui = borrow
            .as_ref()
            .expect("SwiftUI not initialized. Call swiftui::init() first.");
        f(ui)
    })
}

/// Check if the context is initialized.
pub fn _is_initialized() -> bool {
    UI.with(|cell| cell.borrow().is_some())
}
