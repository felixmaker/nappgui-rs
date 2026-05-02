use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let layout = Layout::new(1, 3);
        let button = Button::new("Click Me!");
        let text = TextView::new();
        let panel = Panel::new();

        let menu = Menu::new();
        menu.add_item(&MenuItem::new("Edit"));
        menu.add_item(&MenuItem::new("Exit"));
        let submenu = MenuItem::new("File");
        submenu.set_submenu(&menu);

        let menu = Menu::new();
        menu.add_item(&submenu);

        layout.set_control(0, 1, &button);
        layout.set_control(0, 2, &text);
        layout.set_horizontal_size(0, 250.0);
        layout.set_vertical_size(2, 100.0);
        layout.set_margin(5.0);
        layout.set_vertical_margin(0, 5.0);
        layout.set_vertical_margin(1, 5.0);

        panel.set_layout(&layout);

        let window = Window::new(WindowFlags::default());
        window.set_panel(&panel);
        window.title("Hello, World!");
        window.origin(500.0, 200.0);
        window.on_close(|_| finish());

        window.show();

        menubar(&menu, &window);

        Self {}
    }
}

fn main() {
    osmain::<App>();
}
