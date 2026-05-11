use nappgui::osapp::*;
use nappgui::prelude::*;
use nappgui_macros::DataBind;

#[derive(DataBind)]
struct Data {
    text: NappguiString,
    boolean: NappguiBoolean,
}

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        Data::dbind_register().expect("DBind failed");
        
        let panel = Panel::new();

        let layout = Layout::new(1, 10);
        let text1 = Edit::new();
        let text2 = Edit::new();
        let check1 = Button::new_check("Check 1");
        let check2 = Button::new_check("Check 2");

        layout.set_control(0, 0, text1);
        layout.set_control(0, 1, text2);
        layout.set_control(0, 2, check1);
        layout.set_control(0, 3, check2);

        layout.dbind("Data");
        layout.dbind_cell(0, 0, "text");
        layout.dbind_cell(0, 1, "text");
        layout.dbind_cell(0, 2, "boolean");
        layout.dbind_cell(0, 3, "boolean");

        panel.add_layout(&layout);

        let window = Window::new(WindowFlags::default());
        window.set_panel(&panel);
        window.title("Hello, World!");
        window.set_origin(500.0, 200.0);
        window.set_client_size(200.0, 100.0);
        window.set_on_close_handler(|_| finish());

        window.show();

        Self {}
    }
}

fn main() {
    osmain::<App>();
}
