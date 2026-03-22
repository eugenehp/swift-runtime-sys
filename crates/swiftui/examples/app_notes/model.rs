//! Data model for the notes app.

#[derive(Clone, Debug)]
pub struct Note {
    pub title: String,
    pub body: String,
    pub pinned: bool,
    pub tag: Tag,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Work,
    Personal,
    Ideas,
}

impl Tag {
    pub fn label(&self) -> &str {
        match self {
            Tag::Work => "💼 Work",
            Tag::Personal => "🏠 Personal",
            Tag::Ideas => "💡 Ideas",
        }
    }

    pub fn color(&self) -> swiftui::dsl::Color {
        use swiftui::dsl::Color;
        match self {
            Tag::Work => Color::BLUE,
            Tag::Personal => Color::GREEN,
            Tag::Ideas => Color::YELLOW,
        }
    }
}

impl Note {
    pub fn samples() -> Vec<Note> {
        vec![
            Note {
                title: "Swift Runtime Bindings".into(),
                body: "Complete FFI bindings to the Swift runtime with 490+ symbols, arm64 asm thunks, and ABI struct layouts.".into(),
                pinned: true,
                tag: Tag::Work,
            },
            Note {
                title: "Grocery List".into(),
                body: "Milk, eggs, bread, cheese, apples, coffee beans".into(),
                pinned: false,
                tag: Tag::Personal,
            },
            Note {
                title: "App Architecture".into(),
                body: "Use cx.state() for reactive state. Compose views from small functions. Keep model separate from views.".into(),
                pinned: true,
                tag: Tag::Ideas,
            },
            Note {
                title: "Meeting Notes".into(),
                body: "Discussed Q4 roadmap. Action items: finalize API, write docs, prepare demo.".into(),
                pinned: false,
                tag: Tag::Work,
            },
            Note {
                title: "Rust Tricks".into(),
                body: "Use .bind() for button callbacks. Use for_each() for lists. Use when() for conditionals.".into(),
                pinned: false,
                tag: Tag::Ideas,
            },
        ]
    }
}
