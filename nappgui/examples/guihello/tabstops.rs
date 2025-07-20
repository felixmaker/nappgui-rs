use std::{cell::RefCell, rc::Rc};

use nappgui::prelude::*;

#[derive(Default)]
struct Data {
    button: [Option<PushButton>; 2],
    popup: [Option<PopUp>; 2],
    updown: [Option<UpDown>; 2],
    view: [Option<View>; 2],
    text: [Option<TextView>; 2],
    focus: [bool; 2],
}

fn i_layout(data: &Rc<RefCell<Data>>, tabstop: usize) -> Layout {
    let layout = Layout::new(1, 2);
    let layout1 = Layout::new(6, 1);
    let layout2 = Layout::new(4, 1);

    let button = PushButton::new("This is a button");
    let data1 = data.clone();
    button.on_click(move |_| {
        if let Some(text) = data1.borrow().text[tabstop] {
            text.writef("Button click\n");
        }
    });

    let popup = PopUp::new();
    popup.add_element("Option 1");
    popup.add_element("Option 2");

    let data2 = data.clone();
    popup.on_select(move |params| {
        if let Some(text) = data2.borrow().text[tabstop] {
            text.writef(&format!("Popup select {}\n", params.index));
        }
    });

    let edit = Edit::new();
    edit.text("VALID_TEXT");
    edit.on_change(move |params| params.text == "VALID_TEXT");

    let combo = Combo::new();
    combo.text("ComboBox");
    let slider = Slider::new();
    let updown = UpDown::new();

    layout1.set(0, 0, button);
    layout1.set(1, 0, popup);
    layout1.set(2, 0, edit);
    layout1.set(3, 0, combo);
    layout1.set(4, 0, slider);
    layout1.set(5, 0, updown);

    let view = View::new_custom(false, true);
    let data1 = data.clone();
    view.on_draw(move |params| {
        let focused = data1.borrow().focus[tabstop];
        let color = if focused {
            Color::red(255)
        } else {
            Color::blue(230)
        };
        params.ctx.clear(color);
    });

    let data1 = data.clone();
    view.on_focus(move |focus| {
        data1.borrow_mut().focus[tabstop] = *focus;
    });

    let text = TextView::new();
    text.writef("Hello text!\n");

    let table = TableView::new();
    let list = ListBox::new();
    list.add_element("Elem 1");
    list.add_element("Elem 2");

    table.on_data(|params| match params {
        EvTbDataParams::TableNCols => {
            return EvTbDataResult::TableNCols(10);
        }
        EvTbDataParams::TableCell(pos) => {
            let cell = EvTbCell {
                text: format!("Cell {} {}", pos.row, pos.col),
                align: Align::Left,
            };
            return EvTbDataResult::TableCell(cell);
        }
    });

    table.new_column_text();
    table.new_column_text();
    table.column_width(0, 100.0);
    table.header_title(0, "Column 0");
    table.column_width(1, 100.0);
    table.header_title(1, "Column 1");
    table.update();

    layout2.set(0, 0, view);
    layout2.set(1, 0, text);
    layout2.set(2, 0, list);
    layout2.set(3, 0, table);

    layout.set(0, 0, layout1);
    layout.set(0, 1, layout2);

    layout.halign(0, 0, Align::Left);
    layout.halign(0, 1, Align::Left);

    layout1.hmargin(0, 10.0);
    layout1.hmargin(1, 10.0);
    layout1.hmargin(2, 10.0);
    layout1.hmargin(3, 10.0);
    layout1.hmargin(4, 10.0);

    layout2.hmargin(0, 10.0);
    layout2.hmargin(1, 10.0);
    layout2.hmargin(2, 10.0);

    layout.vmargin(0, 10.0);

    data.borrow_mut().button[tabstop] = Some(button);
    data.borrow_mut().popup[tabstop] = Some(popup);
    data.borrow_mut().text[tabstop] = Some(text);
    data.borrow_mut().view[tabstop] = Some(view);
    data.borrow_mut().updown[tabstop] = Some(updown);

    layout
}

pub fn tabstops() -> Panel {
    let data = Rc::new(RefCell::new(Data::default()));
    let panel = Panel::new();
    let layout = Layout::new(1, 4);
    let label1 = Label::new("Controls in the TabList");
    let label2 = Label::new("Controls NOT in the TabList");

    let layout1 = i_layout(&data, 0);
    let layout2 = i_layout(&data, 1);

    layout.set(0, 0, label1);
    layout.set(0, 1, layout1);
    layout.set(0, 2, label2);
    layout.set(0, 3, layout2);

    layout.vmargin(0, 10.0);
    layout.vmargin(1, 10.0);
    layout.vmargin(2, 10.0);

    panel.layout(layout);
    panel
}
