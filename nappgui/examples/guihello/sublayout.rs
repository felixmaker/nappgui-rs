use nappgui::prelude::*;

fn updown_layout() -> Layout {
    let layout = Layout::new(2, 1);
    let label = Label::new("UpDown");
    let updown = UpDown::new();
    layout.set(0, 0, label);
    layout.set(1, 0, updown);
    layout.horizontal_expand(0);
    layout
}

fn left_grid_layout() -> Layout {
    let layout = Layout::new(2, 4);
    let updown_layout = updown_layout();
    let label = Label::new("Hello!, I'm a label.");
    let button1 = PushButton::new("Push Button");
    let button2 = PushButton::new("Check Button");
    let slider = Slider::new();
    let popup = PopUp::new();
    let edit = Edit::new();
    let progress = Progress::new();
    popup.add_element("Option 1");
    popup.add_element("Option 2");
    popup.add_element("Option 3");
    popup.add_element("Option 4");
    progress.undefined(true);
    layout.set(0, 0, label);
    layout.set(0, 1, button1);
    layout.set(0, 2, button2);
    layout.set(0, 3, slider);
    layout.set(1, 0, popup);
    layout.set(1, 1, edit);
    layout.set(1, 2, updown_layout);
    layout.set(1, 3, progress);
    layout.horizontal_size(0, 150.0);
    layout.horizontal_size(1, 150.0);
    layout.horizontal_margin(0, 5.0);
    layout.vertical_margin(0, 5.0);
    layout.vertical_margin(2, 5.0);
    layout.vertical_margin(1, 5.0);
    layout
}

fn left_layout() -> Layout {
    let layout = Layout::new(1, 2);
    let left_grid_layout = left_grid_layout();
    let button = PushButton::new("Clear");
    layout.set(0, 0, left_grid_layout);
    layout.set(0, 1, button);
    layout.vertical_margin(0, 5.0);
    layout
}

fn top_layout() -> Layout {
    let layout = Layout::new(2, 1);
    let left_layout = left_layout();
    let view = TextView::new();
    layout.set(0, 0, left_layout);
    layout.set(1, 0, view);
    layout.horizontal_size(1, 230.0);
    layout.horizontal_margin(0, 5.0);
    layout
}

fn bottom_layout() -> Layout {
    let layout = Layout::new(6, 1);
    let label1 = Label::new("Select 1");
    let label2 = Label::new("Select 2");
    let label3 = Label::new("Select 3");
    let label4 = Label::new("Select 4");
    let label5 = Label::new("Select 5");
    let label6 = Label::new("Select 6");
    label1.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    label2.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    label3.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    label4.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    label5.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    label6.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    layout.set(0, 0, label1);
    layout.set(1, 0, label2);
    layout.set(2, 0, label3);
    layout.set(3, 0, label4);
    layout.set(4, 0, label5);
    layout.set(5, 0, label6);
    layout
}

fn main_layout() -> Layout {
    let layout = Layout::new(1, 2);
    let layout1 = top_layout();
    let layout2 = bottom_layout();
    layout.set(0, 0, layout1);
    layout.set(0, 1, layout2);
    layout.margin(5.0);
    layout.vertical_margin(0, 5.0);
    layout
}

pub fn sublayouts() -> Panel {
    let panel = Panel::new();
    let layout = main_layout();
    panel.layout(layout);
    panel
}
