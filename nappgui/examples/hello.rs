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
        let label = Label::new();
        let button = Button::new();
        let text = TextView::new();

        layout.set_control(0, 0, label);
        layout.set_control(0, 1, button);
        layout.set_control(0, 2, text);
        layout.set_horizontal_size(0, 250.0);
        layout.set_vertical_size(2, 100.0);
        layout.set_margin(5.0, 5.0, 5.0, 5.0);
        layout.set_vertical_margin(0, 5.0);
        layout.set_vertical_margin(1, 5.0);

        panel.add_layout(layout);

        let window = Window::new(WindowFlags::default());
        window.set_panel(panel);

        let button2 = Button::new();
        button2.set_text("Click Me2!");

        button.set_on_click_handler(move |_params| {
            text.write(&format!("Button click {}\n", clicks.borrow()));
            *clicks.borrow_mut() += 1;

            let frame = window.control_frame(button2);
            
            println!("{:?}", frame.position.x);
            println!("{:?}", frame.position.y);
            println!("{:?}", frame.size.width);
            println!("{:?}", frame.size.height);
        });

        window.set_title("Hello, World!");
        window.set_origin(500.0, 200.0);
        window.set_on_close_handler(|_| finish());

        window.show();

        Self { _window: window }
    }
}

fn main() {
    osmain::<App>();
}
