use nappgui::prelude::*;

fn font_popup() -> PopUp {
    let popup = PopUp::new();
    for font in Font::installed_families() {
        popup.add_element(&font);
    }
    popup
}

fn font_size() -> PopUp {
    let popup = PopUp::new();
    for size in 10..=30 {
        popup.add_element(&size.to_string());
    }
    popup
}

fn font_color() -> PopUp {
    let popup = PopUp::new();
    popup.add_element("Default");
    popup.add_element("Black");
    popup.add_element("White");
    popup.add_element("Red");
    popup.add_element("Green");
    popup.add_element("Blue");
    popup.add_element("Yellow");
    popup.add_element("Cyan");
    popup.add_element("Magenta");
    popup
}

fn font_style() -> Layout {
    let layout = Layout::new(1, 4);
    let button1 = CheckButton::new("Bold");
    let button2 = CheckButton::new("Italic");
    let button3 = CheckButton::new("Underline");
    let button4 = CheckButton::new("Strikeout");

    layout.set(0, 0, button1);
    layout.set(0, 1, button2);
    layout.set(0, 2, button3);
    layout.set(0, 3, button4);
    return layout;
}

fn text_controls() -> Layout {
    let layout = Layout::new(5, 2);
    let layout2 = font_style();
    let label1 = Label::new("Font family");
    let label2 = Label::new("Font size");
    let label3 = Label::new("Font style");
    let label4 = Label::new("Font color");
    let label5 = Label::new("Back color");
    let popup1 = font_popup();
    let popup2 = font_size();
    let popup3 = font_color();
    let popup4 = font_color();
    layout.set(0, 0, label1);
    layout.set(1, 0, label2);
    layout.set(2, 0, label3);
    layout.set(3, 0, label4);
    layout.set(4, 0, label5);
    layout.set(0, 1, popup1);
    layout.set(1, 1, popup2);
    layout.set(2, 1, layout2);
    layout.set(3, 1, popup3);
    layout.set(4, 1, popup4);
    layout.hmargin(0, 5.0);
    layout.hmargin(1, 5.0);
    layout.hmargin(2, 5.0);
    layout.hmargin(3, 5.0);
    layout.vmargin(0, 5.0);
    layout.valign(0, 1, Align::Left);
    layout.valign(1, 1, Align::Left);
    layout.valign(2, 1, Align::Left);
    layout.valign(3, 1, Align::Left);
    layout.valign(4, 1, Align::Left);
    layout
}

fn apply_buttons() -> Layout {
    let layout = Layout::new(5, 1);
    let label = Label::new("Apply format");
    let button1 = PushButton::new("Insert text");
    let button2 = PushButton::new("Add text");
    let button3 = PushButton::new("All text");
    let button4 = PushButton::new("Selected text");

    layout.set(0, 0, button1);
    layout.set(1, 0, button2);
    layout.set(2, 0, label);
    layout.set(3, 0, button3);
    layout.set(4, 0, button4);

    layout.hmargin(0, 5.0);
    layout.hmargin(1, 10.0);
    layout.hmargin(2, 5.0);
    layout.hmargin(3, 5.0);

    layout
}

fn layout() -> Layout {
    let layout = Layout::new(1, 3);
    let layout2 = text_controls();
    let layout3 = apply_buttons();

    let text = TextView::new();
    text.editable(true);
    text.show_select(true);

    layout.set(0, 0, layout2);
    layout.set(0, 1, text);
    layout.set(0, 2, layout3);

    layout.halign(0, 2, Align::Right);
    layout.vmargin(0, 10.0);
    layout.vmargin(1, 10.0);
    layout.tabstop(0, 1, true);

    layout
}

pub fn editor() -> Panel {
    let panel = Panel::new();
    let layout = layout();
    panel.layout(layout);
    panel
}
