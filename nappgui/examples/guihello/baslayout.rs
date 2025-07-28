use nappgui::prelude::*;

pub fn basic_layout() -> Panel {
    let panel = Panel::new();
    let layout = Layout::new(2, 5);
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
    
    edit1.text( "Amanda Callister");
    edit2.text( "aQwe56nhjJk");
    edit3.text( "35, Tuam Road");
    edit4.text( "Galway - Ireland");
    edit5.text( "+35 654 333 000");

    edit2.passmode(true);

    layout.set(0, 0, label1);
    layout.set(0, 1, label2);
    layout.set(0, 2, label3);
    layout.set(0, 3, label4);
    layout.set(0, 4, label5);
    layout.set(1, 0, edit1);
    layout.set(1, 1, edit2);
    layout.set(1, 2, edit3);
    layout.set(1, 3, edit4);
    layout.set(1, 4, edit5);

    panel.layout(layout);
   
    panel
}