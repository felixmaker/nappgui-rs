# nappgui-rs

Rust bindings to NAppGUI

NAppGUI is a professional SDK to build cross-platform desktop applications using The C Programming Language (ANSI C90). NAppGUI has built as a lightweight layer on top of the operating system native APIs, that allows create portable programs extremelly fast and small without any external dependency.

See [frang75/nappgui_src](https://github.com/frang75/nappgui_src) and its [home page](https://nappgui.com/) for more information.


## Example

```rust
use nappgui::gui::*;
use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let clicks = std::rc::Rc::new(std::cell::RefCell::new(0));

        let panel = Panel::create();
        let layout = Layout::create(1, 3);
        let label = Label::create();
        let button = Button::push();
        let text = TextView::create();

        label.text("Hello!, I'm a label");
        button.text("Click Me!");

        layout.label(&label, 0, 0);
        layout.button(&button, 0, 1);
        layout.textview(&text, 0, 2);
        layout.hsize(0, 250.0);
        layout.vsize(2, 100.0);
        layout.margin(5.0);
        layout.vmargin(0, 5.0);
        layout.vmargin(1, 5.0);

        button.on_click(move |_button, _event| {
            text.writef(&format!("Button click {}\n", clicks.borrow()));
            *clicks.borrow_mut() += 1;
        });

        panel.layout(&layout);

        let window = Window::create(WindowFlag::ekWINDOW_STD);
        window.panel(&panel);
        window.title("Hello, World!");
        window.origin(V2Df { x: 500.0, y: 200.0 });
        window.on_close(|_window, _event| finish());

        window.show();

        App {}
    }
}

fn main() {
    osmain::<App>();
}
```


## Requirements

 - Git: for some patches on Windows build
 - CMake: for building the NAppGui static library
 - Windows SDK: only needed when using MSVC

