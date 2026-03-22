use crate::model::Tag;
use swiftui::prelude::*;
use swiftui::{txt, vstack};

pub fn tag_badge(tag: &Tag) -> View {
    txt!("{}", tag.label())
        .size(10.0)
        .foreground(tag.color())
        .padding(4.0)
        .bg(rgba(tag.color().r, tag.color().g, tag.color().b, 0.15))
        .rounded(6.0)
}

pub fn pin_icon(pinned: bool) -> View {
    view_if(pinned, || txt!("📌").size(12.0), || txt!("").size(12.0))
}

pub fn section_header(title: &str) -> View {
    txt!("{title}").size(11.0).foreground(GRAY).padding(4.0)
}

pub fn empty_state(icon: &str, message: &str) -> View {
    vstack![
        spacer(),
        txt!("{icon}").size(48.0),
        txt!("{message}").style(Subtitle),
        spacer()
    ]
}
