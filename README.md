# nappgui-rs

Rust bindings to NAppGUI

NAppGUI is a professional SDK to build cross-platform desktop applications using The C Programming Language (ANSI C90). NAppGUI has built as a lightweight layer on top of the operating system native APIs, that allows create portable programs extremelly fast and small without any external dependency.

See [frang75/nappgui_src](https://github.com/frang75/nappgui_src) and its [home page](https://nappgui.com/) for more information.


## example

```rust
use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {
    _window: Window,
}

impl AppHandler for App {
    fn create() -> Self {
        let clicks = std::rc::Rc::new(std::cell::RefCell::new(0));

        let panel = Panel::new();
        let layout = Layout::new(1, 3);
        let label = Label::new("Hello, I'm a label");
        let button = PushButton::new("Click Me!");
        let text = TextView::new();

        layout.set(0, 0, &label);
        layout.set(0, 1, &button);
        layout.set(0, 2, &text);
        layout.hsize(0, 250.0);
        layout.vsize(2, 100.0);
        layout.margin(5.0);
        layout.vmargin(0, 5.0);
        layout.vmargin(1, 5.0);

        button.on_click(move |_button, _params| {
            text.writef(&format!("Button click {}\n", clicks.borrow()));
            *clicks.borrow_mut() += 1;
        });

        panel.layout(&layout);

        let window = Window::new(WindowFlags::default());
        window.panel(&panel);
        window.title("Hello, World!");
        window.origin(500.0, 200.0);
        window.on_close(|_window, _event| finish());

        window.show();

        Self { _window: window }
    }
}

fn main() {
    osmain::<App>();
}
```


## requirements

 - CMake: for building the NAppGui static library
 - Windows SDK: only needed when using MSVC

