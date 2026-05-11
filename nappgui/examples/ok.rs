use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let layout = Layout::new(1, 3);
        let button = Button::new("Click Me!");
        let text = TextView::new();
        let panel = Panel::new();

        let global_menu = Menu::new();

        let menu = Menu::new();
        let item = MenuItem::new("Edit");
        item.on_click(|params| println!("{} is clicked!", params.text));
        menu.add_item(item);
        menu.add_item(MenuItem::new("Exit"));
        let submenu = MenuItem::new("File");
        submenu.set_submenu(menu);

        global_menu.add_item(submenu);

        let global_menu2 = Menu::new();

        let menu = Menu::new();
        let item = MenuItem::new("Edit2");
        item.on_click(|params| println!("{} is clicked!", params.text));
        menu.add_item(item);
        menu.add_item(MenuItem::new("Exit2"));
        let submenu = MenuItem::new("File2");
        submenu.set_submenu(menu);

        global_menu2.add_item(submenu);

        layout.set_control(0, 1, button);
        layout.set_control(0, 2, text);
        layout.set_horizontal_size(0, 250.0);
        layout.set_vertical_size(2, 100.0);
        layout.set_margin(5.0, 5.0, 5.0, 5.0);
        layout.set_vertical_margin(0, 5.0);
        layout.set_vertical_margin(1, 5.0);

        panel.add_layout(&layout);

        let window = Window::new(WindowFlags::default());
        window.set_panel(&panel);
        window.title("Hello, World!");
        window.set_menubar(&global_menu);
        window.set_menubar(&global_menu2);
        window.set_origin(500.0, 200.0);
        window.set_on_close_handler(|_| finish());

        window.show();

        Self {}
    }
}

fn main() {
    osmain::<App>();
}
