use std::{cell::RefCell, rc::Rc};

use nappgui::prelude::*;

#[derive(Default)]
struct Data {
    parent: Option<WeakWindow>,
    flywin: Option<Window>,
    text: Option<TextView>,
    edit: Option<Edit>,
    align: u32,
}

fn align_layout(data: &Rc<RefCell<Data>>) -> Layout {
    let layout = Layout::new(4, 1);
    let button1 = RadioButton::new("Left-top");
    let button2 = RadioButton::new("Left-bottom");
    let button3 = RadioButton::new("Right-top");
    let button4 = RadioButton::new("Right-bottom");

    let data = data.clone();
    button1.on_click(move |params| {
        data.borrow_mut().align = params.index;
    });
    layout.set(0, 0, button1);
    layout.set(1, 0, button2);
    layout.set(2, 0, button3);
    layout.set(3, 0, button4);
    layout.horizontal_margin(0, 5.0);
    layout.horizontal_margin(1, 5.0);
    layout.horizontal_margin(2, 5.0);
    button1.state(GuiState::On);
    layout
}

fn controls_layout(data: &Rc<RefCell<Data>>) -> Layout {
    let layout = Layout::new(5, 1);
    let button1 = PushButton::new("Push Button");
    let button2 = PushButton::new("...");
    let data1 = data.clone();
    button2.on_click(move |_| {
        if let Data {
            parent: Some(parent),
            edit: Some(edit),
            flywin: Some(flywin),
            align,
            ..
        } = &*data1.borrow()
        {
            let parent = parent.upgrade().unwrap();
            let frame = parent.control_frame(edit);
            let mut pos = parent.client_to_screen(frame.position.x, frame.position.y);
            let size = flywin.get_size();

            match *align {
                0 => pos.y += frame.size.height,
                1 => pos.y -= size.height,
                2 => {
                    pos.x -= size.width - frame.size.width;
                    pos.y += frame.size.height;
                }
                3 => {
                    pos.x -= size.width - frame.size.width;
                    pos.y -= size.height;
                }
                _ => {}
            }

            flywin.origin(pos.x, pos.y);
            flywin.overlay(parent);
        }
    });
    let slider = Slider::new();
    let edit = Edit::new();
    layout.set(0, 0, button1);
    layout.set(1, 0, slider);
    layout.set(3, 0, edit);
    layout.set(4, 0, button2);
    layout.horizontal_margin(0, 5.0);
    layout.horizontal_margin(1, 5.0);
    layout.horizontal_expand(2);
    data.borrow_mut().edit = Some(edit);
    layout
}

fn listbox_layout(data: &Rc<RefCell<Data>>) -> Layout {
    let layout = Layout::new(2, 1);
    let list = ListBox::new();
    let text = TextView::new();
    list.add_element("Item 1");
    list.add_element("Item 2");
    list.add_element("Item 3");
    list.add_element("Item 4");
    let data1 = data.clone();
    list.on_down(move |params| {
        if params.button == GuiMouse::Right && params.tag != u32::MAX {
            let menu = Menu::new();
            for i in 0..4 {
                let item = MenuItem::new();
                item.text(&format!("Item {} Option {}", params.tag + 1, i + 1));
                item.on_click(|_| {});
                menu.add_item(item);
            }

            let pos = gui_mouse_postion();
            if let Data {
                parent: Some(window),
                ..
            } = &*data1.borrow()
            {
                let window = window.upgrade().unwrap();
                menu.launch(window, pos.x, pos.y);
            }
        }
        true
    });
    layout.set(0, 0, list);
    layout.set(1, 0, text);
    layout.horizontal_margin(0, 20.0);
    data.borrow_mut().text = Some(text);
    layout
}

fn layout(data: &Rc<RefCell<Data>>) -> Layout {
    let layout = Layout::new(1, 5);
    let layout2 = align_layout(data);
    let layout3 = controls_layout(data);
    let layout4 = listbox_layout(data);
    let label1 = Label::new("A flyout window will be show when you press ... button\nPress [ESC] or [RETURN] to close the flyout window");
    let label2 = Label::new("A popup menu will be show when right click in ListBox");
    layout.set(0, 0, label1);
    layout.set(0, 1, layout2);
    layout.set(0, 2, layout3);
    layout.set(0, 3, label2);
    layout.set(0, 4, layout4);
    layout.vertical_margin(0, 10.0);
    layout.vertical_margin(1, 5.0);
    layout.vertical_margin(2, 5.0);
    layout.vertical_margin(3, 5.0);
    label1.multiline(true);
    layout
}

fn create_flywin() -> Window {
    let nrows = 4;
    let layout = Layout::new(2, nrows);
    let panel = Panel::new();
    let window = Window::new(WindowFlags {
        process_return_key: true,
        process_escape_key: true,
        has_outer_border: false,
        has_title_bar: false,
        has_maximize_button: false,
        has_minimize_button: false,
        has_close_button: false,
        has_resizable_borders: false,
        avoid_hiding_modal: false,
    });

    for i in 0..nrows {
        let label = Label::new(&format!("Flyout control {}", i));
        let slider = Slider::new();
        layout.set(0, i, label);
        layout.set(1, i, slider);

        if i < nrows - 1 {
            layout.vertical_margin(i, 5.0);
        }
    }

    layout.horizontal_margin(0, 5.0);
    layout.margin(10.0);
    layout.border_color(Color::red(204));
    panel.layout(layout);
    window.panel(panel);
    window
}

pub fn flyout_window(window: WeakWindow) -> Panel {
    let data = Data {
        parent: Some(window),
        flywin: Some(create_flywin()),
        ..Default::default()
    };
    let data = Rc::new(RefCell::new(data));
    let panel = Panel::new();
    let layout = layout(&data);
    panel.layout(layout);
    panel
}
