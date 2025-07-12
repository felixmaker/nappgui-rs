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

        layout.set(0, 0, label);
        layout.set(0, 1, button);
        layout.set(0, 2, text);
        layout.hsize(0, 250.0);
        layout.vsize(2, 100.0);
        layout.margin(5.0);
        layout.vmargin(0, 5.0);
        layout.vmargin(1, 5.0);

        button.on_click(move |_params| {
            text.writef(&format!("Button click {}\n", clicks.borrow()));
            *clicks.borrow_mut() += 1;
        });

        panel.layout(layout);

        let window = Window::new(WindowFlags::default());
        window.panel(panel);
        window.title("Hello, World!");
        window.origin(500.0, 200.0);
        window.on_close(|_| finish());

        window.show();

        Self { _window: window }
    }
}

fn main() {
    osmain::<App>();
}
