mod activity;
mod cards;
mod header;
mod sidebar;

pub use activity::*;
pub use cards::*;
pub use header::*;
pub use sidebar::*;

use swiftui::prelude::*;

pub fn overview(cx: &Cx) -> View {
    let refresh_count = cx.state(0i32);

    vstack![
        page_header("Overview", "Dashboard summary", &refresh_count,),
        divider(),
        // Stats row
        hstack![
            stat_card("Users", "12,847", "+12%", Color::BLUE),
            stat_card("Revenue", "$48.2K", "+8%", Color::GREEN),
            stat_card("Orders", "1,284", "-3%", Color::YELLOW),
            stat_card("Growth", "23%", "+5%", Color::PURPLE),
        ],
        divider(),
        // Bottom half: projects + activity side by side
        hstack![
            project_list().frame(-1.0, -1.0),
            divider(),
            recent_activity().frame(250.0, -1.0),
        ],
        spacer(),
    ]
    .padding(16.0)
}

pub fn projects(cx: &Cx) -> View {
    let filter = cx.state(0i32); // 0=all, 1=active, 2=completed

    vstack![
        text("Projects").style(StylePreset::Title),
        hstack![
            button("All", filter.set_to(0)),
            button("Active", filter.set_to(1)),
            button("Completed", filter.set_to(2)),
        ],
        divider(),
        project_list(),
        spacer(),
    ]
    .padding(16.0)
}

pub fn settings_page(cx: &Cx) -> View {
    let dark_mode = cx.state(true);
    let notifications = cx.state(true);
    let auto_save = cx.state(false);

    vstack![
        text("Settings").style(StylePreset::Title),
        divider(),
        vstack![
            toggle("Dark Mode", dark_mode.get()),
            toggle("Notifications", notifications.get()),
            toggle("Auto-Save", auto_save.get()),
            slider(0.7, 0.0, 1.0),
            text("Font Size").style(StylePreset::Caption),
        ]
        .style(StylePreset::CardDark),
        spacer(),
    ]
    .padding(16.0)
}
