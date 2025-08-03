use std::{cell::RefCell, rc::Rc};

use nappgui::prelude::*;

struct Data {
    view: Option<TextView>,
    window: Option<WeakWindow>,
}

fn hotkey_event(data: &Rc<RefCell<Data>>, key: &str) {
    let data = data.borrow();
    if let Some(view) = data.view.as_ref() {
        view.write(format!("[{}] hotkey pressed\n", key).as_str());
        view.select(-1, -1);
        view.scroll_caret();
    }
}

fn set_hotkeys(data: &Rc<RefCell<Data>>) {
    if let Some(window) = data.borrow().window.as_ref() {
        let data1 = data.clone();
        let window = window.upgrade().unwrap();
        window.hotkey(KeyCode::F1, ModifierKey::None, move || {
            hotkey_event(&data1, "F1")
        });
        let data2 = data.clone();
        window.hotkey(KeyCode::F2, ModifierKey::None, move || {
            hotkey_event(&data2, "F2")
        });
        let data3 = data.clone();
        window.hotkey(KeyCode::F3, ModifierKey::None, move || {
            hotkey_event(&data3, "F3")
        });
        let data4 = data.clone();
        window.hotkey(KeyCode::F4, ModifierKey::None, move || {
            hotkey_event(&data4, "F4")
        });
        let data5 = data.clone();
        window.hotkey(KeyCode::F5, ModifierKey::None, move || {
            hotkey_event(&data5, "F5")
        });
        let data6 = data.clone();
        window.hotkey(KeyCode::F6, ModifierKey::None, move || {
            hotkey_event(&data6, "F6")
        });
        let data7 = data.clone();
        window.hotkey(KeyCode::F7, ModifierKey::None, move || {
            hotkey_event(&data7, "F7")
        });
        let data8 = data.clone();
        window.hotkey(KeyCode::F8, ModifierKey::None, move || {
            hotkey_event(&data8, "F8")
        });
        let data9 = data.clone();
        window.hotkey(KeyCode::F9, ModifierKey::None, move || {
            hotkey_event(&data9, "F9")
        });
        let data10 = data.clone();
        window.hotkey(KeyCode::F10, ModifierKey::None, move || {
            hotkey_event(&data10, "F10")
        });
    }
}

pub fn hotkeys(window: WeakWindow) -> Panel {
    let data = Data {
        view: None,
        window: Some(window),
    };
    let data = Rc::new(RefCell::new(data));

    let panel = Panel::new();
    let layout = Layout::new(1, 5);
    let label = Label::new("Press [F1]-[F10] hotkeys");
    let edit = Edit::new();
    let view = TextView::new();
    let button1 = PushButton::new("Unset hotkeys");
    let button2 = PushButton::new("Set hotkeys");

    let data1 = data.clone();
    button1.on_click(move |_| {
        if let Some(window) = data1.borrow().window.as_ref() {
            let window = window.upgrade().unwrap();
            window.clear_hotkeys();
        }
    });

    let data2 = data.clone();
    button2.on_click(move |_| {
        set_hotkeys(&data2);
    });

    layout.set(0, 0, label);
    layout.set(0, 1, edit);
    layout.set(0, 2, view);
    layout.set(0, 3, button1);
    layout.set(0, 4, button2);

    layout.vertical_margin(0, 5.0);
    layout.vertical_margin(1, 5.0);
    layout.vertical_margin(2, 5.0);
    layout.vertical_margin(3, 5.0);
    layout.vertical_margin(4, 5.0);

    panel.layout(layout);
    data.borrow_mut().view = Some(view);

    panel
}
