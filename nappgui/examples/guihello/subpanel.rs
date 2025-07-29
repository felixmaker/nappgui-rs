use nappgui::prelude::*;

pub fn subpanels() -> Panel {
    let panel1 = Panel::new();
    let panel2 = Panel::new();
    let layout1 = Layout::new(2, 2);
    let layout2 = Layout::new(2, 2);
    let label1 = Label::new("Main Panel");
    let label2 = Label::new("Sub Panel");
    let button = PushButton::new("Push Button");
    let slider = Slider::new();
    let edit = Edit::new();
    edit.text("EditBox");

    layout2.set(0, 0, label2);
    layout2.set(0, 1, button);
    layout2.set(1, 1, slider);

    layout2.horizontal_size(1, 150.0);
    layout2.horizontal_margin(0, 10.0);
    layout2.vertical_margin(0, 10.0);
    layout2.margin4(5.0, 10.0, 10.0, 10.0);

    layout2.border_color(Color::rgba(158, 158, 158, 1));
    
    panel2.layout(layout2);

    layout1.set(0, 0, label1);
    layout1.set(1, 1, edit);
    layout1.set(0, 1, panel2);

    layout1.horizontal_size(1, 150.0);
    layout1.horizontal_margin(0, 10.0);
    layout1.vertical_margin(0, 10.0);
    layout1.margin4(5.0, 10.0, 10.0, 10.0);

    panel1.layout(layout1);

    panel1
}