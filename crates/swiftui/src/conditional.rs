//! Conditional and iterative view construction.

use crate::dsl;
use crate::view::View;

/// Conditionally include a view. Returns empty spacer if false.
///
/// ```ignore
/// vstack![
///     text("always shown"),
///     when(show_details, || text("details")),
///     when_else(is_premium, || text("Premium"), || text("Free")),
/// ]
/// ```
pub fn when(condition: bool, view: impl FnOnce() -> View) -> View {
    if condition {
        view()
    } else {
        empty()
    }
}

/// Conditionally show one of two views.
pub fn when_else(
    condition: bool,
    if_true: impl FnOnce() -> View,
    if_false: impl FnOnce() -> View,
) -> View {
    if condition {
        if_true()
    } else {
        if_false()
    }
}

/// An empty zero-size view.
pub fn empty() -> View {
    // A spacer with minLength 0 effectively takes no space
    dsl::spacer()
}

/// Create views from an iterator.
///
/// ```ignore
/// let items = vec!["Apple", "Banana", "Cherry"];
/// vstack![
///     for_each(&items, |item| text(item)),
/// ]
/// ```
pub fn for_each<T>(items: &[T], view_fn: impl Fn(&T) -> View) -> View {
    let views: Vec<View> = items.iter().map(|item| view_fn(item)).collect();
    dsl::vstack(views)
}

/// Create views from an iterator with index.
pub fn for_each_enumerated<T>(items: &[T], view_fn: impl Fn(usize, &T) -> View) -> View {
    let views: Vec<View> = items
        .iter()
        .enumerate()
        .map(|(i, item)| view_fn(i, item))
        .collect();
    dsl::vstack(views)
}

/// Create an hstack from an iterator.
pub fn for_each_h<T>(items: &[T], view_fn: impl Fn(&T) -> View) -> View {
    let views: Vec<View> = items.iter().map(|item| view_fn(item)).collect();
    dsl::hstack(views)
}
