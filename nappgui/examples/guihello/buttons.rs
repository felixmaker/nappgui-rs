use nappgui::prelude::*;

use crate::res::*;

fn flatbuttons() -> Layout {
    let layout = Layout::new(6, 1);
    let button1 = FlatButton::new("Open File", gui_image(FOLDER24_PNG));
    let button2 = FlatButton::new("Save File", gui_image(DISK24_PNG));
    let button3 = FlatButton::new("Search File", gui_image(SEARCH24_PNG));
    let button4 = FlatButton::new("Edit File", gui_image(EDIT24_PNG));
    let button5 = FlatButton::new("Add File", gui_image(PLUS24_PNG));
    let button6 = FlatButton::new("Delete File", gui_image(ERROR24_PNG));

    layout.set(0, 0, button1);
    layout.set(1, 0, button2);
    layout.set(2, 0, button3);
    layout.set(3, 0, button4);
    layout.set(4, 0, button5);
    layout.set(5, 0, button6);

    layout
}

fn radios() -> Layout {
    let layout = Layout::new(1, 4);

    let radio1 = RadioButton::new("&Wireframe");
    let radio2 = RadioButton::new("&Shaded");
    let radio3 = RadioButton::new("&Realistic");
    let radio4 = RadioButton::new("&V-Ray");

    radio1.state(GuiState::On);

    layout.set(0, 0, radio1);
    layout.set(0, 1, radio2);
    layout.set(0, 2, radio3);
    layout.set(0, 3, radio4);

    layout.margin(5f32);
    layout.vertical_margin(0, 3f32);
    layout.vertical_margin(1, 3f32);
    layout.vertical_margin(2, 3f32);

    layout
}

fn checks() -> Layout {
    let layout = Layout::new(1, 4);

    let check1 = CheckButton::new("&Lines");
    let check2 = CheckButton::new("M&eshes");
    let check3 = CheckButton::new("M&aterials");
    let check4 = CheckButton::new("L&ights");

    check1.state(GuiState::On);
    check2.state(GuiState::Off);
    check3.state(GuiState::Off);
    check4.state(GuiState::On);

    layout.set(0, 0, check1);
    layout.set(0, 1, check2);
    layout.set(0, 2, check3);
    layout.set(0, 3, check4);

    layout.margin(5f32);
    layout.vertical_margin(0, 3f32);
    layout.vertical_margin(1, 3f32);
    layout.vertical_margin(2, 3f32);
    layout
}

fn pushes(defbutton: &mut Option<PushButton>) -> Layout {
    let layout = Layout::new(4, 1);
    let push1 = PushButton::new("Re&try");
    let push2 = PushButton::new("&Cancel");
    let push3 = PushButton::new("&Ok");

    push1.image(gui_image(RETRY_PNG));

    layout.set(0, 0, push1);
    layout.set(2, 0, push2);
    layout.set(3, 0, push3);

    layout.horizontal_margin(2, 5f32);
    layout.horizontal_expand(1);

    defbutton.replace(push1);

    layout
}

fn buttons(defbutton: &mut Option<PushButton>) -> Layout {
    let layout = Layout::new(1, 3);
    let layout1 = flatbuttons();
    let layout2 = Layout::new(2, 2);
    let layout3 = radios();
    let layout4 = checks();
    let layout5 = pushes(defbutton);

    let check1 = CheckButton::new("Enable 3&D Render");
    let check2 = Check3Button::new("Enable &Preview Settings");

    check1.state(GuiState::On);
    check2.state(GuiState::Mixed);

    layout.set(0, 0, layout1);

    layout2.set(0, 0, check1);
    layout2.set(0, 1, layout3);
    layout2.set(1, 0, check2);
    layout2.set(1, 1, layout4);

    layout.set(0, 1, layout2);
    layout.set(0, 2, layout5);

    layout.horizontal_align(0, 0, Align::Left);
    layout2.margin(5f32);
    layout2.horizontal_margin(0, 10f32);
    layout5.margin(5f32);

    layout
}

pub fn buttons_basics(defbutton: &mut Option<PushButton>) -> Panel {
    let layout = buttons(defbutton);
    let panel = Panel::new();
    panel.layout(layout);
    panel
}
