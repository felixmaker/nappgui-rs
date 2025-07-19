use std::{cell::RefCell, rc::Rc};

use nappgui::prelude::*;

#[derive(Default)]
struct EditData {
    text: Option<TextView>,
    family_popup: Option<PopUp>,
    size_popup: Option<PopUp>,
    color_popup: Option<PopUp>, // todo!
    back_popup: Option<PopUp>, // todo!
    bold_check: Option<CheckButton>,
    italic_check: Option<CheckButton>,
    under_check: Option<CheckButton>,
    strike_check: Option<CheckButton>,
}

fn set_params(data: &Rc<RefCell<EditData>>) {
    let data = data.borrow();

    if let EditData {
        text: Some(text),
        family_popup: Some(family_popup),
        size_popup: Some(size_popup),
        color_popup: Some(_),
        back_popup: Some(_),
        bold_check: Some(bold_check),
        italic_check: Some(italic_check),
        under_check: Some(under_check),
        strike_check: Some(strike_check),
    } = &*data
    {
        let ffamily = family_popup.get_text(family_popup.get_selected());
        let size: f32 = size_popup
            .get_text(size_popup.get_selected())
            .parse()
            .unwrap();

        let fstyle = FontStyle {
            is_bold: bold_check.get_state() == GuiState::On,
            is_italic: italic_check.get_state() == GuiState::On,
            is_strike_out: strike_check.get_state() == GuiState::On,
            is_underlined: under_check.get_state() == GuiState::On,
            ..Default::default()
        };

        text.family(&ffamily);
        text.fsize(size);
        text.fstyle(fstyle);
    }
}

fn font_popup(data: &Rc<RefCell<EditData>>) -> PopUp {
    let popup = PopUp::new();
    for font in Font::installed_families() {
        popup.add_element(&font);
    }
    popup.list_height(20);

    let data = data.clone();
    popup.on_select(move |_| {
        set_params(&data);
    });

    popup
}

fn font_size(data: &Rc<RefCell<EditData>>) -> PopUp {
    let popup = PopUp::new();
    for size in 10..=30 {
        popup.add_element(&size.to_string());
    }

    let data = data.clone();
    popup.on_select(move |_| {
        set_params(&data);
    });

    popup
}

fn font_color(data: &Rc<RefCell<EditData>>) -> PopUp {
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
    let data = data.clone();
    popup.on_select(move |_| {
        set_params(&data);
    });
    popup
}

fn font_style(data: &Rc<RefCell<EditData>>) -> Layout {
    let layout = Layout::new(1, 4);
    let button1 = CheckButton::new("Bold");
    let button2 = CheckButton::new("Italic");
    let button3 = CheckButton::new("Underline");
    let button4 = CheckButton::new("Strikeout");

    layout.set(0, 0, button1);
    layout.set(0, 1, button2);
    layout.set(0, 2, button3);
    layout.set(0, 3, button4);

    let data1 = data.clone();
    button1.on_click(move |_| {
        set_params(&data1);
    });

    let data2 = data.clone();
    button2.on_click(move |_| {
        set_params(&data2);
    });

    let data3 = data.clone();
    button3.on_click(move |_| {
        set_params(&data3);
    });

    let data4 = data.clone();
    button4.on_click(move |_| {
        set_params(&data4);
    });

    data.borrow_mut().bold_check = Some(button1);
    data.borrow_mut().italic_check = Some(button2);
    data.borrow_mut().under_check = Some(button3);
    data.borrow_mut().strike_check = Some(button4);

    return layout;
}

fn text_controls(data: &Rc<RefCell<EditData>>) -> Layout {
    let layout = Layout::new(5, 2);
    let layout2 = font_style(data);
    let label1 = Label::new("Font family");
    let label2 = Label::new("Font size");
    let label3 = Label::new("Font style");
    let label4 = Label::new("Font color");
    let label5 = Label::new("Back color");
    let popup1 = font_popup(data);
    let popup2 = font_size(data);
    let popup3 = font_color(data);
    let popup4 = font_color(data);
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

    data.borrow_mut().family_popup = Some(popup1);
    data.borrow_mut().size_popup = Some(popup2);
    data.borrow_mut().color_popup = Some(popup3);
    data.borrow_mut().back_popup = Some(popup4);

    layout
}

fn apply_buttons(data: &Rc<RefCell<EditData>>) -> Layout {
    let layout = Layout::new(5, 1);
    let label = Label::new("Apply format");
    let button1 = PushButton::new("Insert text");
    let button2 = PushButton::new("Add text");
    let button3 = PushButton::new("All text");
    let button4 = PushButton::new("Selected text");

    let data1 = data.clone();
    button1.on_click(move |_| {
        if let Some(text) = &data1.borrow().text {
            text.cpos_writef("ins");
        }
    });

    let data2 = data.clone();
    button2.on_click(move |_| {
        if let Some(text) = &data2.borrow().text {
            text.writef("add");
        }
    });

    let data3 = data.clone();
    button3.on_click(move |_| {
        if let Some(text) = &data3.borrow().text {
            text.apply_all();
        }
    });

    let data4 = data.clone();
    button4.on_click(move |_| {
        if let Some(text) = &data4.borrow().text {
            text.apply_select();
        }
    });

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

fn layout(data: &Rc<RefCell<EditData>>) -> Layout {
    let layout = Layout::new(1, 3);
    let layout2 = text_controls(&data);
    let layout3 = apply_buttons(&data);

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

    data.borrow_mut().text = Some(text);

    layout
}

pub fn editor() -> Panel {
    let data = Rc::new(RefCell::new(EditData::default()));

    let panel = Panel::new();
    let layout = layout(&data);
    set_params(&data);
    panel.layout(layout);
    panel
}
