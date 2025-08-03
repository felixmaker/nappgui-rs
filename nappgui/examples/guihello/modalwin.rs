use std::cell::RefCell;
use std::rc::Rc;

use nappgui::prelude::*;

#[derive(Default)]
struct Data {
    type_: u32,
    label: Option<Label>,
    window: Option<WeakWindow>,
}

fn window_title(type_: u32) -> String {
    match type_ {
        0 => "Modal Window without [Return] nor [Esc]".to_string(),
        1 => "Modal Window with [Return]".to_string(),
        2 => "Modal Window with [Esc]".to_string(),
        3 => "Modal Window with [Return] and [Esc]".to_string(),
        _ => "".to_string(),
    }
}

fn window_flags(type_: u32) -> WindowFlags {
    let mut flags = WindowFlags::default();
    match type_ {
        1 => flags.process_return_key = true,
        2 => flags.process_escape_key = true,
        3 => {
            flags.process_return_key = true;
            flags.process_escape_key = true
        }
        _ => {}
    }
    flags
}

fn close_layout(window: &WeakWindow) -> Layout {
    let layout = Layout::new(1, 4);
    let button1 = PushButton::new("Close modal with 10 value");
    let button2 = PushButton::new("Close modal with 20 value");
    let button3 = PushButton::new("Close modal with 30 value");
    let button4 = PushButton::new("Close modal with 40 value");

    let window1 = window.clone();
    button1.on_click(move |_| {
        window1.upgrade().unwrap().stop_modal(GuiClose::Custom(10));
    });

    let window2 = window.clone();
    button2.on_click(move |_| {
        window2.upgrade().unwrap().stop_modal(GuiClose::Custom(20));
    });

    let window3 = window.clone();
    button3.on_click(move |_| {
        window3.upgrade().unwrap().stop_modal(GuiClose::Custom(30));
    });

    let window4 = window.clone();
    button4.on_click(move |_| {
        window4.upgrade().unwrap().stop_modal(GuiClose::Custom(40));
    });

    layout.set(0, 0, button1);
    layout.set(0, 1, button2);
    layout.set(0, 2, button3);
    layout.set(0, 3, button4);

    layout.vertical_margin(0, 5.0);
    layout.vertical_margin(1, 5.0);
    layout.vertical_margin(2, 5.0);

    layout
}

fn modal_window(data: &Rc<RefCell<Data>>) {
    if let Data {
        type_,
        label: Some(label),
        window: Some(parent),
    } = &*data.borrow()
    {
        let parent = parent.upgrade().unwrap();
        let pos = parent.get_origin();
        let window = Window::new(window_flags(*type_));
        let panel = Panel::new();
        let layout1 = Layout::new(2, 1);
        let data = Rc::new(RefCell::new(Data::default()));
        let layout2 = modal_layout(&data);
        let layout3 = close_layout(&window.as_weak());
        layout1.set(0, 0, layout2);
        layout1.set(1, 0, layout3);
        layout1.horizontal_margin(0, 10.0);
        layout1.vertical_align(1, 0, Align::Left);
        layout1.margin(10.0);
        panel.layout(layout1); // Order TODO
        window.panel(panel); // Order TODO
        window.title(&window_title(*type_));
        window.origin(pos.x + 20.0, pos.y + 20.0);
        let result = window.modal(parent);
        let text = match result {
            GuiClose::Cancel => format!("Modal stop: [Esc] ({})", u32::from(result)),
            GuiClose::Accept => format!("Modal stop: [Return] ({})", u32::from(result)),
            GuiClose::Close => format!("Modal stop: [X] ({})", u32::from(result)),
            GuiClose::Deactivate => format!("Modal stop: {}", u32::from(result)),
            GuiClose::Custom(v) => format!("Modal stop: {}", v),
        };
        label.text(&text);
    }
}

fn modal_layout(data: &Rc<RefCell<Data>>) -> Layout {
    let layout = Layout::new(1, 5);
    let button1 = PushButton::new("Modal Window without [Return] nor [Esc]");
    let button2 = PushButton::new("Modal Window with [Return]");
    let button3 = PushButton::new("Modal Window with [Esc]");
    let button4 = PushButton::new("Modal Window with [Return] and [Esc]");
    let label = Label::new("Modal stop: --");

    let data1 = data.clone();
    button1.on_click(move |_| {
        modal_window(&data1);
    });

    layout.set(0, 0, button1);
    layout.set(0, 1, button2);
    layout.set(0, 2, button3);
    layout.set(0, 3, button4);
    layout.set(0, 4, label);
    layout.horizontal_align(0, 4, Align::Justify); // make label shown full.
    
    layout.vertical_margin(0, 5.0);
    layout.vertical_margin(1, 5.0);
    layout.vertical_margin(2, 5.0);
    layout.vertical_margin(3, 20.0);

    data.borrow_mut().label = Some(label);

    layout
}

pub fn modal_windows(parent: WeakWindow) -> Panel {
    let data = Rc::new(RefCell::new(Data {
        window: Some(parent),
        ..Default::default()
    }));
    let panel = Panel::new();
    let layout = modal_layout(&data);
    panel.layout(layout);
    panel
}
