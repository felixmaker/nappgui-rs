use std::{cell::RefCell, rc::Rc};

use nappgui::prelude::*;

struct PopUpData {
    popup: Option<PopUp>,
}

fn popups(layout: &Layout, data: &Rc<RefCell<PopUpData>>) {
    let label1 = Label::new("Language:");
    let label2 = Label::new("Color:");
    let popup1 = PopUp::new();
    let popup2 = PopUp::new();

    // todo! gui_image
    popup1.add_elem("English", None);
    popup1.add_elem("Español", None);
    popup1.add_elem("Portugues", None);
    popup1.add_elem("Italiana", None);
    popup1.add_elem("Tiếng Việt", None);
    popup1.add_elem("России", None);
    popup1.add_elem("日本語", None);
    popup2.add_elem("Red", None);
    popup2.add_elem("Blue", None);
    popup2.add_elem("Green", None);
    popup2.add_elem("Yellow", None);
    popup2.add_elem("Black", None);
    popup2.add_elem("White", None);

    popup1.list_height(10);
    popup2.list_height(10);

    layout.set(0, 0, &label1);
    layout.set(0, 1, &label2);
    layout.set(1, 0, &popup1);
    layout.set(1, 1, &popup2);

    data.borrow_mut().popup = Some(popup1);
}

pub fn combos(layout: &Layout) {
    let label1 = Label::new("Search:");
    let label2 = Label::new("Folder:");

    let combo1 = Combo::new();
    let combo2 = Combo::new();

    combo1.add_elem("Search", None);
    combo1.add_elem("Disk", None);
    combo1.add_elem("Edit", None);

    combo2.add_elem("/home/fran/Desktop", None);
    combo2.add_elem("/usr/include", None);
    combo2.add_elem("/mnt/volume1", None);
    combo2.add_elem("/etc/html/log.txt", None);

    layout.set(2, 0, &label1);
    layout.set(2, 1, &label2);
    layout.set(3, 0, &combo1);
    layout.set(3, 1, &combo2);
}

pub fn popup_combo() -> Panel {
    let data = Rc::new(RefCell::new(PopUpData { popup: None }));

    let panel = Panel::new();
    let layout = Layout::new(4, 4);
    let button1 = PushButton::new("Add elem to PopUp");
    let button2 = PushButton::new("Clear PopUp");

    let data1 = data.clone();
    button1.on_click(move |_| {
        if let Some(popup) = &data1.borrow_mut().popup {
            popup.add_elem("Español", None);
        }
    });

    let data2 = data.clone();
    button2.on_click(move |_| {
        if let Some(popup) = &data2.borrow_mut().popup {
            popup.clear();
        }
    });

    popups(&layout, &data);
    combos(&layout);

    layout.set(1, 2, &button1);
    layout.set(1, 3, &button2);

    layout.margin(10.);
    layout.vmargin(0, 10.);
    layout.vmargin(1, 10.);
    layout.vmargin(2, 5.);
    layout.hmargin(0, 5.);
    layout.hmargin(1, 10.);
    layout.hmargin(2, 5.);

    layout.hsize(1, 150.);
    layout.hsize(3, 150.);

    panel.layout(&layout);
    panel
}
