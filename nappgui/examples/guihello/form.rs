use std::{cell::RefCell, rc::Rc};

use nappgui::{
    gui::event::{EvButton, EvText, EvTextFilter},
    prelude::*,
};

struct FormData {
    window: Window,
    validate_check: Option<CheckButton>,
}

fn modal_window(edit: &mut Edit, text: &str, focus_info: &FocusInfo) -> Window {
    let message = format!("Do you want to validate the text '{}' of the EditBox '{:?}'? The focus will be moved to the 'todo!' control using the '{:?}' action.", text, edit.as_control_ptr(), focus_info.action);

    let window = Window::new(WindowFlags {
        process_escape_key: true,
        ..Default::default()
    });

    let panel = Panel::new();

    let layout1 = Layout::new(1, 2);
    let layout2 = Layout::new(3, 1);

    let label = Label::new(&message);
    label.multiline(true);

    let button1 = PushButton::new("Yes");
    let button2 = PushButton::new("No");

    let window1 = window.clone();
    button1.on_click(move |_, _| {
        window1.stop_modal(GuiClose::Accept);
    });

    let window2 = window.clone();
    button2.on_click(move |_, _| {
        window2.stop_modal(GuiClose::Cancel);
    });

    layout1.set(0, 0, &label);
    layout2.set(1, 0, &button1);
    layout2.set(2, 0, &button2);
    layout1.set(0, 1, &layout2);

    layout1.hsize(0, 250.0);
    layout1.vmargin(0, 10.0);
    layout2.hmargin(1, 5.0);
    layout2.margin4(0.0, 10.0, 0.0, 0.0);
    layout1.margin(10.0);
    layout2.hexpand(0);

    window.title("Data validation");
    panel.layout(&layout1);
    window.panel(&panel);
    window.defbutton(&button1);

    window
}

fn modal_pos(window: &Window, parent: &Window) -> Point2D {
    let pos = parent.get_origin();
    let s1 = parent.get_size();
    let s2 = window.get_size();
    let x = pos.x + (s1.width - s2.width) / 2.0;
    let y = pos.y + (s1.height - s2.height) / 2.0;
    Point2D { x, y }
}

fn validate_field(data: Rc<RefCell<FormData>>, edit: &mut Edit, text: &str) -> bool {
    let window = &data.borrow().window;

    let focus_info = window.focus_info();
    let modal = modal_window(edit, text, &focus_info);

    let pos = modal_pos(&modal, window);
    modal.origin(pos.x, pos.y);
    match modal.modal(window) {
        GuiClose::Accept => return true,
        _ => return false,
    }
}

fn numbers(data: Rc<RefCell<FormData>>, color_bg: Color) -> Layout {
    let layout = Layout::new(5, 1);
    let label = Label::new("Height (cm):");
    let edit1 = Edit::new();
    let edit2 = Edit::new();
    let updown1 = UpDown::new();
    let updown2 = UpDown::new();

    edit1.text("25");
    edit2.text("175");
    edit1.autoselect(true);
    edit1.align(Align::Right);
    edit2.align(Align::Right);

    edit1.on_filter(|_, params| -> EvTextFilter { filter_number(params) });
    edit2.on_filter(|_, params| -> EvTextFilter { filter_number(params) });

    let data1 = data.clone();
    edit1.on_change(move |edit, params| -> bool { on_edit_change(data1.clone(), edit, params) });
    let data2 = data.clone();
    edit2.on_change(move |edit, params| -> bool { on_edit_change(data2.clone(), edit, params) });

    edit1.bgcolor_focus(color_bg);
    edit2.bgcolor_focus(color_bg);

    let mut edit_1 = edit1.clone();
    updown1.on_click(move |_, params| {
        on_updown_click(&mut edit_1, params);
    });

    let mut edit_2 = edit2.clone();
    updown2.on_click(move |_, params| {
        on_updown_click(&mut edit_2, params);
    });

    updown1.tooltip("Increase/Decrease age");
    updown2.tooltip("Increase/Decrease height");

    layout.set(2, 0, &label);
    layout.set(0, 0, &edit1);
    layout.set(3, 0, &edit2);
    layout.set(1, 0, &updown1);
    layout.set(4, 0, &updown2);

    layout.hmargin(1, 10.);
    layout.hmargin(2, 10.);
    layout.hexpand2(0, 3, 0.5);

    layout
}

fn on_updown_click(edit: &mut Edit, params: &EvButton) {
    let text = edit.get_text();
    let mut n: i32 = text.parse().unwrap();
    if params.index == 0 {
        n = n + 1;
    } else {
        n = n - 1
    }
    edit.text(n.to_string().as_str());
}

fn on_edit_change(data: Rc<RefCell<FormData>>, edit: &mut Edit, params: &EvText) -> bool {
    let data_cloned = data.clone();
    if let Some(button) = &data.borrow().validate_check {
        if button.get_state() == GuiState::On {
            return validate_field(data_cloned, edit, &params.text);
        }
    };
    false
}

fn only_number(text: &str) -> String {
    let mut vec = Vec::new();
    for char in text.as_bytes() {
        if char.is_ascii_digit() {
            vec.push(*char);
        } else {
            break;
        }
    }
    String::from_utf8(vec).unwrap()
}

fn filter_number(params: &EvText) -> EvTextFilter {
    let text = only_number(&params.text);
    EvTextFilter {
        apply: true,
        text,
        cpos: 0,
    }
}

fn edits(data: Rc<RefCell<FormData>>) -> Layout {
    let colorbg = Color::rgb(112, 112, 48);

    let layout1 = Layout::new(2, 6);
    let layout2 = numbers(data.clone(), colorbg);

    let label1 = Label::new("User Name:");
    let label2 = Label::new("Password:");
    let label3 = Label::new("Address:");
    let label4 = Label::new("City:");
    let label5 = Label::new("Phone:");
    let label6 = Label::new("Age:");

    label1.color_over(Color::rgb(255, 128, 52));
    label2.color_over(Color::rgb(70, 129, 207));
    label3.color_over(Color::rgb(119, 188, 31));
    label4.style_over(FontStyle {
        is_italic: true,
        is_underlined: true,
        ..Default::default()
    });

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

    edit1.select(2, 6);
    edit2.password(true);

    let data1 = data.clone();
    edit1.on_change(move |edit, params| on_edit_change(data1.clone(), edit, params));
    let data2 = data.clone();
    edit2.on_change(move |edit, params| on_edit_change(data2.clone(), edit, params));
    let data3 = data.clone();
    edit3.on_change(move |edit, params| on_edit_change(data3.clone(), edit, params));
    let data4 = data.clone();
    edit4.on_change(move |edit, params| on_edit_change(data4.clone(), edit, params));
    let data5 = data.clone();
    edit5.on_change(move |edit, params| on_edit_change(data5.clone(), edit, params));

    edit1.bgcolor_focus(colorbg);
    edit2.bgcolor_focus(colorbg);
    edit3.bgcolor_focus(colorbg);
    edit4.bgcolor_focus(colorbg);
    edit5.bgcolor_focus(colorbg);

    layout1.set(0, 0, &label1);
    layout1.set(0, 1, &label2);
    layout1.set(0, 2, &label3);
    layout1.set(0, 3, &label4);
    layout1.set(0, 4, &label5);
    layout1.set(0, 5, &label6);

    layout1.set(1, 0, &edit1);
    layout1.set(1, 1, &edit2);
    layout1.set(1, 2, &edit3);
    layout1.set(1, 3, &edit4);
    layout1.set(1, 4, &edit5);

    layout1.set(1, 5, &layout2);

    layout1.hmargin(0, 5.);
    layout1.hexpand(1);
    layout1.vmargin(0, 5.);
    layout1.vmargin(1, 5.);
    layout1.vmargin(2, 5.);
    layout1.vmargin(3, 5.);
    layout1.vmargin(4, 5.);

    layout1
}

fn toolbar(data: Rc<RefCell<FormData>>) -> Layout {
    let layout = Layout::new(1, 1);
    let button = CheckButton::new("Field validations");
    layout.set(0, 0, &button);
    let mut data = data.borrow_mut();
    data.validate_check = Some(button);
    layout
}

fn form(data: Rc<RefCell<FormData>>) -> Layout {
    let layout1 = Layout::new(1, 3);
    let data2 = data.clone();
    let data3 = data.clone();
    let layout2 = edits(data2);
    let layout3 = toolbar(data3);
    let label = Label::new("Please fill in all the information on the form. We will use this data to send commercial mail at all hours, not caring much if it bothers you or not.");
    label.style_over(FontStyle {
        is_italic: true,
        ..Default::default()
    });
    label.multiline(true);

    layout1.set(0, 0, &layout2);
    layout1.set(0, 1, &layout3);
    layout1.set(0, 2, &label);

    layout1.hsize(0, 300.);
    layout1.halign(0, 1, Align::Left);
    layout1.vmargin(0, 10.);
    layout1.vmargin(1, 10.);
    layout1.margin(10.);
    layout1.tabstop(0, 1, false);

    layout1
}

pub fn form_basic(window: Window) -> Panel {
    let data = Rc::new(RefCell::new(FormData {
        window: window,
        validate_check: None,
    }));

    let layout = form(data);
    let panel = Panel::new();
    panel.layout(&layout);
    panel
}
