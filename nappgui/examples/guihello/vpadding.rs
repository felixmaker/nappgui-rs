use crate::res::*;
use nappgui::prelude::*;

pub fn vpadding() -> Panel {
    let panel = Panel::new();
    let layout = Layout::new(1, 6);

    let bg = Color::rgb(196, 247, 247);

    let edit1 = Edit::new();
    let edit2 = Edit::new();
    let edit3 = Edit::new();

    edit1.text("Edit with default padding");
    edit2.text("Edit with zero padding");
    edit3.text("Edit with high padding");

    edit1.bgcolor(bg);
    edit2.bgcolor(bg);
    edit3.bgcolor(bg);

    let button1 = PushButton::new("Button with default padding");
    let button2 = PushButton::new("Button with zero padding");
    let button3 = PushButton::new("Button with high padding");

    let font = Font::system(20.0, FontStyle::default());
    button2.font(&font);

    button1.image(&gui_image(FOLDER24_PNG));
    button2.image(&gui_image(FOLDER24_PNG));
    button3.image(&gui_image(FOLDER24_PNG));

    edit2.vpadding(0.0);
    edit3.vpadding(20.0);
    button2.vpadding(0.0);
    button3.vpadding(20.0);
    layout.set(0, 0, edit1);
    layout.set(0, 1, edit2);
    layout.set(0, 2, edit3);
    layout.set(0, 3, button1);
    layout.set(0, 4, button2);
    layout.set(0, 5, button3);
    layout.hsize(0, 250.0);
    layout.vmargin(0, 5.0);
    layout.vmargin(1, 5.0);
    layout.vmargin(2, 5.0);
    layout.vmargin(3, 5.0);
    layout.vmargin(4, 5.0);

    panel.layout(layout);
    panel
}
