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
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            body: "New note...".into(),
            pinned: false,
            tag: Tag::Personal,
        }
    }
    pub fn samples() -> Vec<Self> {
        vec![
            Self {
                title: "Swift Runtime".into(),
                body: "490+ symbols, arm64 asm thunks, ABI structs.".into(),
                pinned: true,
                tag: Tag::Work,
            },
            Self {
                title: "Grocery List".into(),
                body: "Milk, eggs, bread, cheese, coffee".into(),
                pinned: false,
                tag: Tag::Personal,
            },
            Self {
                title: "Architecture".into(),
                body: "cx.state() for reactive. Compose views from functions.".into(),
                pinned: true,
                tag: Tag::Ideas,
            },
            Self {
                title: "Meeting Notes".into(),
                body: "Q4 roadmap. Action: finalize API, write docs.".into(),
                pinned: false,
                tag: Tag::Work,
            },
        ]
    }
}
