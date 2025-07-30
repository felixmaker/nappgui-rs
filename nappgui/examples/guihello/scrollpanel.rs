use nappgui::prelude::*;

const ROWS: u32 = 100;

pub fn scrollpanel() -> ScrollPanel {
    let panel = ScrollPanel::new(false, true);
    let layout = Layout::new(3, ROWS);
    let margin = panel.scroll_width();
    panel.size(-1.0, 400.0);
    for i in 0..ROWS {
        let label = Label::new(&format!("User {}", i + 1));
        let button = PushButton::new(&format!("Name of User {}", i + 1));
        let edit = Edit::new();
        edit.text(&format!("Edit {}", i + 1));
        layout.set(0, i, label);
        layout.set(1, i, edit);
        layout.set(2, i, button);
    }

    for i in 0..ROWS {
        layout.vertical_margin(i, 5.0);
    }

    layout.horizontal_margin(0, 10.0);
    layout.horizontal_margin(1, 10.0);
    layout.horizontal_size(1, 150.0);
    layout.margin4(0.0, margin, 0.0, 0.0);
    panel.layout(layout);
    panel
}
