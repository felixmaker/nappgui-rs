use nappgui::prelude::*;

fn multilayout_panel() -> Panel {
    let panel = Panel::new();
    let layout1 = Layout::new(2, 5);
    let layout2 = Layout::new(1, 10);
    let label1 = Label::new("User Name:");
    let label2 = Label::new("Password:");
    let label3 = Label::new("Address:");
    let label4 = Label::new("City:");
    let label5 = Label::new("Phone:");
    let edit1 = Edit::new();
    let edit2 = Edit::new();
    let edit3 = Edit::new();
    let edit4 = Edit::new();
    let edit5 = Edit::new();

    edit1.text("Amanda Callister");
    edit2.text("aQwe56nhjJk");
    edit3.text("35, Tuam Road");
    edit4.text("Galway - Ireland");
    edit5.text("+35 654 333 000");
    edit2.passmode(true);

    layout1.set(0, 0, label1);
    layout1.set(0, 1, label2);
    layout1.set(0, 2, label3);
    layout1.set(0, 3, label4);
    layout1.set(0, 4, label5);

    layout1.set(1, 0, edit1);
    layout1.set(1, 1, edit2);
    layout1.set(1, 2, edit3);
    layout1.set(1, 3, edit4);
    layout1.set(1, 4, edit5);

    layout1.horizontal_size(1, 300.0);
    layout1.horizontal_margin(0, 5.0);
    layout1.vertical_margin(0, 5.0);
    layout1.vertical_margin(1, 5.0);
    layout1.vertical_margin(2, 5.0);
    layout1.vertical_margin(3, 5.0);

    layout2.set(0, 0, label1);
    layout2.set(0, 2, label2);
    layout2.set(0, 4, label3);
    layout2.set(0, 6, label4);
    layout2.set(0, 8, label5);
    layout2.set(0, 1, edit1);
    layout2.set(0, 3, edit2);
    layout2.set(0, 5, edit3);
    layout2.set(0, 7, edit4);
    layout2.set(0, 9, edit5);
    layout2.horizontal_size(0, 200.0);
    layout2.vertical_margin(1, 5.0);
    layout2.vertical_margin(3, 5.0);
    layout2.vertical_margin(5, 5.0);
    layout2.vertical_margin(7, 5.0);

    panel.layout(layout1);
    panel.layout(layout2);
    panel
}

pub fn multilayouts() -> Panel {
    let panel1 = Panel::new();
    let panel2 = multilayout_panel();
    let button1 = RadioButton::new("Layout 1");
    button1.state(GuiState::On);
    let button2 = RadioButton::new("Layout 2");
    let layout1 = Layout::new(1, 2);
    let layout2 = Layout::new(2, 1);

    let panel = panel2;
    button1.on_click(move |params| {
        panel.visible_layout(params.index);
        panel.update();
    });

    layout2.set(0, 0, button1);
    layout2.set(1, 0, button2);
    layout1.set(0, 0, layout2);
    layout1.set(0, 1, panel2);

    layout1.vertical_margin(0, 10.0);
    layout2.horizontal_margin(0, 10.0);
    layout1.horizontal_align(0, 0, Align::Left);

    panel1.layout(layout1);
    panel1
}