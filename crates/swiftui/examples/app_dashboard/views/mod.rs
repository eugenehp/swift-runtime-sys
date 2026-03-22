mod activity;
mod cards;
mod header;
mod sidebar;

pub use activity::*;
pub use cards::*;
pub use header::*;
pub use sidebar::*;

use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

pub fn overview(cx: &Cx) -> View {
    let refresh = cx.state(0i32);
    vstack![
        page_header("Overview", "Dashboard summary", &refresh),
        divider(),
        hstack![
            stat_card("Users", "12,847", "+12%", BLUE),
            stat_card("Revenue", "$48.2K", "+8%", GREEN),
            stat_card("Orders", "1,284", "-3%", YELLOW),
            stat_card("Growth", "23%", "+5%", PURPLE),
        ],
        divider(),
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
    let filter = cx.state(0i32);
    vstack![
        txt!("Projects").style(Title),
        hstack![
            button("All", filter.set_to(0)),
            button("Active", filter.set_to(1)),
            button("Done", filter.set_to(2)),
        ],
        divider(),
        project_list(),
        spacer(),
    ]
    .padding(16.0)
}

pub fn settings_page(cx: &Cx) -> View {
    let dark = cx.state(true);
    let notifs = cx.state(true);
    vstack![
        txt!("Settings").style(Title),
        divider(),
        vstack![
            toggle("Dark Mode", dark.get()),
            toggle("Notifications", notifs.get()),
            slider(0.7, 0.0, 1.0),
            txt!("Font Size").style(Caption),
        ]
        .style(CardDark),
        spacer(),
    ]
    .padding(16.0)
}
