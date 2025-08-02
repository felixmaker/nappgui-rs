use std::{cell::RefCell, rc::Rc};

use crate::res::*;
use nappgui::prelude::*;

struct PopUpData {
    popup: Option<PopUp>,
}

fn popups(layout: &Layout, data: &Rc<RefCell<PopUpData>>) {
    let label1 = Label::new("Language:");
    let label2 = Label::new("Color:");
    let popup1 = PopUp::new();
    let popup2 = PopUp::new();

    popup1.add_image_element("English", gui_image(UKING_PNG));
    popup1.add_image_element("Español", gui_image(SPAIN_PNG));
    popup1.add_image_element("Portugues", gui_image(PORTUGAL_PNG));
    popup1.add_image_element("Italiana", gui_image(ITALY_PNG));
    popup1.add_image_element("Tiếng Việt", gui_image(VIETNAM_PNG));
    popup1.add_image_element("России", gui_image(RUSSIA_PNG));
    popup1.add_image_element("日本語", gui_image(JAPAN_PNG));
    popup2.add_image_element("Red", gui_image(RED_PNG));
    popup2.add_image_element("Blue", gui_image(BLUE_PNG));
    popup2.add_image_element("Green", gui_image(GREEN_PNG));
    popup2.add_image_element("Yellow", gui_image(YELLOW_PNG));
    popup2.add_image_element("Black", gui_image(BLACK_PNG));
    popup2.add_image_element("White", gui_image(WHITE_PNG));

    popup1.list_height(10);
    popup2.list_height(10);

    layout.set(0, 0, label1);
    layout.set(0, 1, label2);
    layout.set(1, 0, popup1);
    layout.set(1, 1, popup2);

    data.borrow_mut().popup = Some(popup1);
}

pub fn combos(layout: &Layout) {
    let label1 = Label::new("Search:");
    let label2 = Label::new("Folder:");

    let combo1 = Combo::new();
    let combo2 = Combo::new();

    combo1.add_element("Search");
    combo1.add_element("Disk");
    combo1.add_element("Edit");

    combo2.add_element("/home/fran/Desktop");
    combo2.add_element("/usr/include");
    combo2.add_element("/mnt/volume1");
    combo2.add_element("/etc/html/log.txt");

    layout.set(2, 0, label1);
    layout.set(2, 1, label2);
    layout.set(3, 0, combo1);
    layout.set(3, 1, combo2);
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
            popup.add_element("Español");
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

    layout.set(1, 2, button1);
    layout.set(1, 3, button2);

    layout.margin(10.);
    layout.vertical_margin(0, 10.);
    layout.vertical_margin(1, 10.);
    layout.vertical_margin(2, 5.);
    layout.horizontal_margin(0, 5.);
    layout.horizontal_margin(1, 10.);
    layout.horizontal_margin(2, 5.);

    layout.horizontal_size(1, 150.);
    layout.horizontal_size(3, 150.);

    panel.layout(layout);
    panel
}
