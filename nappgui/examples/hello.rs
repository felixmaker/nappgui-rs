use nappgui::prelude::*;
use nappgui::osapp::*;

struct App {
    _window: Window,
}

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

        let window = Window::create(WindowFlags::default());
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
