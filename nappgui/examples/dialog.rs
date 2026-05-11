use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let layout = Layout::new(1, 4);
        let button_select_dir = Button::new("Select Dir");
        let button_open_file = Button::new("Open File");
        let button_save_file = Button::new("Save File");
        let button_color = Button::new("Color");

        let panel = Panel::new();

        layout.set_margin(5.0, 5.0, 5.0, 5.0);
        layout.set_control(0, 0, button_select_dir);
        layout.set_control(0, 1, button_open_file);
        layout.set_control(0, 2, button_save_file);
        layout.set_control(0, 3, button_color);

        panel.add_layout(layout);

        let window = Window::new(WindowFlags::default());
        window.set_panel(panel);
        window.title("Dialogs");
        window.set_origin(500.0, 200.0);
        window.set_client_size(200.0, 150.0);
        window.set_on_close_handler(|_| finish());

        let window1 = window.clone();
        button_select_dir.set_on_click_handler(move |_| {
            let dir = window1.launch_select_dir_dialog("Select Dir:", "");
            println!("{:?}", dir);
        });

        let window1 = window.clone();
        button_open_file.set_on_click_handler(move |_| {
            let file = window1.launch_open_file_dialog("Open File:", &["txt"], "", "");
            println!("{:?}", file);
        });

        let window1 = window.clone();
        button_save_file.set_on_click_handler(move |_| {
            let file = window1.launch_save_file_dialog("Save File:", &["txt"], "", "");
            println!("{:?}", file);
        });

        let window1 = window.clone();
        button_color.set_on_click_handler(move |_| {
            let origin = window1.origin();
            window1.launch_color_dialog(
                "Choose Color:",
                origin.x,
                origin.y,
                Align::Center,
                Align::Center,
                Color::blue(0),
                &[],
                |color| println!("{:?}", color),
            );
        });

        window.show();

        Self {}
    }
}

fn main() {
    osmain::<App>();
}
