use std::{cell::RefCell, rc::Rc};

use crate::res::*;
use nappgui::prelude::*;

#[derive(Default, Debug)]
struct SelData {
    window: Option<WeakWindow>,
    edit_range: Option<Edit>,
    caps: Option<CheckButton>,
    text: Option<TextView>,
    info_text: Option<TextView>,
}

fn text_controls(data: Rc<RefCell<SelData>>) -> Layout {
    let layout = Layout::new(7, 1);
    let button1 = FlatButton::new("Copy", &gui_image(COPY_PNG));
    let button2 = FlatButton::new("Paste", &gui_image(PASTE_PNG));
    let button3 = FlatButton::new("Cut", &gui_image(CUT_PNG));
    let button4 = PushButton::new("Select Text");
    let button5 = CheckButton::new("Caps");
    let button6 = CheckButton::new("Wrap");
    let edit = Edit::new();
    edit.text("2,6");

    let data1 = data.clone();
    button1.on_click(move |_| {
        if let Some(window) = &data1.borrow().window {
            if let Some(edit) = window.get_focus::<Edit>() {
                edit.copy();
            } else if let Some(textview) = window.get_focus::<TextView>() {
                textview.copy();
            }
        }
    });

    let data2 = data.clone();
    button2.on_click(move |_| {
        if let Some(window) = &data2.borrow().window {
            if let Some(edit) = window.get_focus::<Edit>() {
                edit.paste();
            } else if let Some(textview) = window.get_focus::<TextView>() {
                textview.paste();
            }
        }
    });

    let data3 = data.clone();
    button3.on_click(move |_| {
        if let Some(window) = &data3.borrow().window {
            if let Some(edit) = window.get_focus::<Edit>() {
                edit.cut();
            } else if let Some(textview) = window.get_focus::<TextView>() {
                textview.cut();
            }
        }
    });

    let data4 = data.clone();
    button4.on_click(move |_| {
        let data = data4.borrow();
        if let Some(window) = &data.window {
            if let Some(range) = &data.edit_range {
                let text = range.get_text();
                let group: Vec<usize> = text
                    .trim()
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap_or(0))
                    .collect();
                let left = *group.get(0).unwrap_or(&0);
                let right = *group.get(1).unwrap_or(&1);

                if let Some(edit) = window.get_focus::<Edit>() {
                    edit.select(left as _, right as _);
                } else if let Some(textview) = window.get_focus::<TextView>() {
                    textview.select(left as _, right as _);
                }
            }
        }
    });

    let data6 = data.clone();
    button6.on_click(move |params| {
        if let Some(text) = &data6.borrow().text {
            text.wrap(if params.state == GuiState::On {
                true
            } else {
                false
            });
        }
    });

    button6.state(GuiState::On);

    layout.set(0, 0, button1);
    layout.set(1, 0, button2);
    layout.set(2, 0, button3);
    layout.set(3, 0, edit);
    layout.set(4, 0, button4);
    layout.set(5, 0, button5);
    layout.set(6, 0, button6);

    layout.horizontal_size(3, 40.0);
    layout.horizontal_margin(0, 5.0);
    layout.horizontal_margin(1, 5.0);
    layout.horizontal_margin(2, 5.0);
    layout.horizontal_margin(3, 5.0);
    layout.horizontal_margin(4, 5.0);
    layout.horizontal_margin(5, 5.0);

    (*data.borrow_mut()).edit_range = Some(edit);
    (*data.borrow_mut()).caps = Some(button5);

    layout
}

fn textview_controls(data: Rc<RefCell<SelData>>) -> Layout {
    let layout = Layout::new(5, 1);
    let label = Label::new("TextView");

    let button1 = FlatButton::new("Edit", &gui_image(EDIT16_PNG)); 
    let button2 = FlatButton::new("Cursor", &gui_image(CURSOR16_PNG));
    let button3 = FlatButton::new("Error", &gui_image(ERROR16_PNG));

    let data1 = data.clone();
    button1.on_click(move |_| {
        if let Some(text) = &data1.borrow().text {
            text.write("add");
        }
    });

    let data2 = data.clone();
    button2.on_click(move |_| {
        if let Some(text) = &data2.borrow().text {
            text.write("ins");
        }
    });

    let data3 = data.clone();
    button3.on_click(move |_| {
        if let Some(text) = &data3.borrow().text {
            text.delete_select();
        }
    });

    button1.tooltip("Add text at the end of TextView");
    button2.tooltip("Insert text at cursor position in TextView");
    button3.tooltip("Delete the selected text, without copy into clipboard");

    layout.set(0, 0, label);
    layout.set(2, 0, button1);
    layout.set(3, 0, button2);
    layout.set(4, 0, button3);

    layout.horizontal_expand(1);

    layout
}

fn layout(data: Rc<RefCell<SelData>>) -> Layout {
    let layout1 = Layout::new(1, 11);
    let layout2 = textview_controls(data.clone());
    let layout3 = text_controls(data.clone());

    let label1 = Label::new("EditBox");
    let label2 = Label::new("EditBox (multiline)");
    let label3 = Label::new("EditBox (multiline with user-height)");
    let label5 = Label::new("Info");

    let edit1 = Edit::new();
    let edit2 = Edit::new_multiline();
    let edit3 = Edit::new_multiline();

    edit1.text("This is a text in the EditBox control");
    edit2.text("This is a text in the multiline EditBox control");
    edit3.text("This is a text in the multiline EditBox control");

    let text1 = TextView::new();
    let text2 = TextView::new();

    // todo! edit_min_height
    // todo! callback on edit

    layout1.set(0, 0, label1);
    layout1.set(0, 1, edit1);
    layout1.set(0, 2, label2);
    layout1.set(0, 3, edit2);
    layout1.set(0, 4, label3);
    layout1.set(0, 5, edit3);
    layout1.set(0, 6, layout2);
    layout1.set(0, 7, text1);
    layout1.set(0, 8, layout3);
    layout1.set(0, 9, label5);
    layout1.set(0, 10, text2);

    layout1.tabstop(0, 6, false);
    layout1.tabstop(0, 7, true);
    layout1.tabstop(0, 8, false);

    layout1.horizontal_align(0, 8, Align::Left);
    layout1.vertical_margin(1, 5.0);
    layout1.vertical_margin(3, 5.0);
    layout1.vertical_margin(5, 5.0);
    layout1.vertical_margin(7, 5.0);

    (*data.borrow_mut()).text = Some(text1);
    (*data.borrow_mut()).info_text = Some(text2);

    layout1
}

pub fn seltext(window: WeakWindow) -> Panel {
    let mut data = SelData::default();
    data.window = Some(window);
    let data = Rc::new(RefCell::new(data));
    let layout = layout(data);
    let panel = Panel::new();
    panel.layout(layout);
    panel
}
