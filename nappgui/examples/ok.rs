use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let layout = Layout::new(1, 3);
        let button = Button::new_push();
        button.set_text("Click Me!");
        let text = TextView::new();
        let panel = Panel::new();

        // layout.set_control(0, 0, &label);
        layout.set_control(0, 1, &button);
        layout.set_control(0, 2, &text);
        layout.set_horizontal_size(0, 250.0);
        layout.set_vertical_size(2, 100.0);
        layout.set_margin(5.0);
        layout.set_vertical_margin(0, 5.0);
        layout.set_vertical_margin(1, 5.0);

        panel.set_layout(&layout);

        let layout2 = Layout::new(1, 3);

        let label = Label::new("Heeloo");
        layout2.set_control(0, 0, &label);
        layout2.set_control(0, 1, &button);
        layout2.set_control(0, 2, &text);
        layout2.set_horizontal_size(0, 250.0);
        layout2.set_vertical_size(2, 100.0);
        layout2.set_margin(5.0);
        layout2.set_vertical_margin(0, 5.0);
        layout2.set_vertical_margin(1, 5.0);

        // let index1 = panel.push_layout(&layout);
        let index2 = panel.set_layout(&layout2);
        println!("0>>>{}", index2);
        panel.set_visible_layout(1);

        let window = Window::new(WindowFlags::default());
        window.set_panel(&panel);
        window.title("Hello, World!");
        window.origin(500.0, 200.0);
        window.on_close(|_| finish());

        window.show();

        Self {}
    }
}

fn main() {
    osmain::<App>();
}
