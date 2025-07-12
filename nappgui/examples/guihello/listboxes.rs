use nappgui::prelude::*;

fn simple_listbox() -> ListBox {
    let listbox = ListBox::new();
    listbox.size(150.0, 200.0);
    listbox.add_elem("Item 1", None);
    listbox.add_elem("Item 2", None);
    listbox.add_elem("Item 3", None);
    listbox.add_elem("Item 4", None);

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

    layout.set(0, 0, label1);
    layout.set(1, 0, label2);
    layout.set(2, 0, label3);

    layout.set(0, 1, listbox1);

    layout.hmargin(0, 10.);
    layout.hmargin(1, 10.);
    layout.vmargin(0, 5.);

    panel.layout(layout);
    panel
}
