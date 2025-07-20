use crate::res::*;
use nappgui::prelude::*;

fn full_listbox() -> ListBox {
    let listbox = ListBox::new();
    listbox.size(150.0, 200.0);
    listbox.add_image_element("Sales presentation", &gui_image(POWERPOINT_PNG));
    listbox.add_image_element("Balance 2017", &gui_image(POWERPOINT_PNG));
    listbox.add_image_element("The Last of Us Analysis", &gui_image(POWERPOINT_PNG));
    listbox.add_image_element("Phone list", &gui_image(ACCESS_PNG));
    listbox.add_image_element("Customer database", &gui_image(ACCESS_PNG));
    listbox.add_image_element("My first book", &gui_image(WORD_PNG));
    listbox.add_image_element("Letter to April", &gui_image(WORD_PNG));
    listbox.add_image_element("Cookbook Recipes", &gui_image(WORD_PNG));
    listbox.add_image_element("Dog playing piano", &gui_image(JPG_PNG));
    listbox.add_image_element("Hollidays 2019", &gui_image(JPG_PNG));
    listbox.add_image_element("Amanda's party", &gui_image(JPG_PNG));
    listbox.add_image_element("Flying", &gui_image(JPG_PNG));
    listbox.add_image_element("The C Programing Language", &gui_image(PDF_PNG));
    listbox.add_image_element("Graphics Programing with GDI+", &gui_image(PDF_PNG));
    listbox.add_image_element("Personal finances", &gui_image(EXCEL_PNG));
    listbox.add_image_element("Stocks 2017", &gui_image(EXCEL_PNG));
    listbox.add_image_element("Website Dashboard", &gui_image(EXCEL_PNG));
    listbox.add_image_element("Open Issues", &gui_image(DOCUMENT_PNG));
    listbox.add_image_element("TODO List", &gui_image(DOCUMENT_PNG));
    listbox.select(0, true);
    listbox
}

fn image_listbox() -> ListBox {
    let listbox = ListBox::new();
    listbox.size(150.0, 200.0);
    listbox.add_image_element("Spain", &gui_image(SPAIN_PNG));
    listbox.add_image_element("Italy", &gui_image(ITALY_PNG));
    listbox.add_image_element("United Kingdom", &gui_image(UKING_PNG));
    listbox.add_image_element("Vietnam", &gui_image(VIETNAM_PNG));
    listbox.add_image_element("Russia", &gui_image(RUSSIA_PNG));
    listbox.add_image_element("Portugal", &gui_image(PORTUGAL_PNG));
    listbox.add_image_element("Japan", &gui_image(JAPAN_PNG));
    listbox.add_image_element("Disk", &gui_image(DISK16_PNG));
    listbox.add_image_element("Edit", &gui_image(EDIT16_PNG));
    listbox.add_image_element("Folder", &gui_image(FOLDER16_PNG));
    listbox.add_image_element("Restore", &gui_image(RESTORE16_PNG));
    listbox.add_image_element("Search", &gui_image(SEARCH16_PNG));
    listbox.add_image_element("Error", &gui_image(ERROR16_PNG));
    listbox.select(0, true);
    listbox
}

fn simple_listbox() -> ListBox {
    let listbox = ListBox::new();
    listbox.size(150.0, 200.0);
    listbox.add_element("Item 1");
    listbox.add_element("Item 2");
    listbox.add_element("Item 3");
    listbox.add_element("Item 4");

    listbox.color(0, Color::red(255));
    listbox.color(1, Color::green(255));
    listbox.color(2, Color::blue(255));

    listbox.select(0, true);
    listbox
}

pub fn listboxes() -> Panel {
    let panel = Panel::new();
    let layout = Layout::new(3, 2);

    let label1 = Label::new("Simple ListBox");
    let label2 = Label::new("With Images");
    let label3 = Label::new("Checks and Multiselect");

    let listbox1 = simple_listbox();
    let listbox2 = image_listbox();
    let listbox3 = full_listbox();

    layout.set(0, 0, label1);
    layout.set(1, 0, label2);
    layout.set(2, 0, label3);

    layout.set(0, 1, listbox1);
    layout.set(1, 1, listbox2);
    layout.set(2, 1, listbox3);

    layout.horizontal_margin(0, 10.);
    layout.horizontal_margin(1, 10.);
    layout.vertical_margin(0, 5.);

    panel.layout(layout);
    panel
}
