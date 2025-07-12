mod buttons;
mod form;
mod labels;
mod listboxes;
mod popcom;

use nappgui::gui::event::*;
use nappgui::osapp::*;
use nappgui::prelude::*;

struct App {
    window: Window,
    layout: Layout,
}

impl App {
    fn new() -> Self {
        let window = Window::new(WindowFlags {
            process_escape_key: true,
            process_return_key: true,
            ..Default::default()
        });

        let layout = Layout::new(2, 1);
        Self { window, layout }
    }

    fn panel(&self) -> Panel {
        let panel = Panel::new();
        let list = ListBox::new();
        list.size(180f32, 256f32);

        list.add_elem("Labels single line", None);
        list.add_elem("Labels multi line", None);
        list.add_elem("Labels mouse sensitive", None);
        list.add_elem("Buttons", None);
        list.add_elem("PopUp Combo", None);
        list.add_elem("ListBoxes", None);
        list.add_elem("Form", None);
        list.add_elem("Text select", None);
        list.add_elem("Text editor", None);
        list.add_elem("Sliders", None);
        list.add_elem("Vertical padding", None);
        list.add_elem("Tabstops", None);
        list.add_elem("TextViews", None);
        list.add_elem("TableView", None);
        list.add_elem("SplitViews", None);
        list.add_elem("Modal Windows", None);
        list.add_elem("Flyout Windows", None);
        list.add_elem("Hotkeys", None);
        list.add_elem("Data Binding", None);
        list.add_elem("Struct Binding", None);
        list.add_elem("Basic Layout", None);
        list.add_elem("SubLayouts", None);
        list.add_elem("Subpanels", None);
        list.add_elem("Multi-Layouts", None);
        list.add_elem("Scroll panel", None);
        list.add_elem("Dynamic layouts", None);
        list.add_elem("Dynamic menus", None);
        list.add_elem("IP Input", None);
        list.add_elem("Font x-scale", None);
        list.add_elem("Reduce components", None);

        list.select(0, true);

        let window = self.window.as_weak();
        let layout = self.layout.clone();
        list.on_select(move |params| {
            set_panel(&window, &layout, params.index);
            window.update();
        });

        self.layout.set(0, 0, &list);
        set_panel(&self.window.as_weak(), &self.layout, 0);

        panel.layout(&self.layout);
        self.layout.valign(0, 0, Align::Left);
        self.layout.valign(1, 0, Align::Left);
        self.layout.margin(10f32);
        self.layout.hmargin(0, 10f32);

        panel
    }
}

fn set_panel(window: &WeakWindow, layout: &Layout, index: usize) {
    let mut defbutton: Option<PushButton> = None;

    let panel = match index {
        0 => labels::labels_single_line(),
        1 => labels::labels_multi_line(),
        2 => labels::labels_mouse_over(),
        3 => buttons::buttons_basics(&mut defbutton),
        4 => popcom::popup_combo(),
        5 => listboxes::listboxes(),
        6 => form::form_basic(window.clone()),
        _ => {
            return;
        }
    };

    layout.panel_replace(&panel, 1, 0);

    if let Some(defbutton) = &mut defbutton {
        window.defbutton(defbutton);
    }
}

fn i_modal_window<T>(parent: &T, message: &str)
where
    T: WindowTrait,
{
    let panel = Panel::new();
    let layout = Layout::new(1, 1);
    let label = Label::new(message);
    let window = Window::new(WindowFlags {
        process_escape_key: true,
        process_return_key: true,
        ..Default::default()
    });
    layout.set(0, 0, &label);
    layout.margin(10f32);
    panel.layout(&layout);
    window.panel(&panel);
    window.title("Message");

    let mut pos = parent.get_origin();
    let s1 = parent.get_size();
    let s2 = window.get_size();
    pos.x = (s1.width - s2.width) / 2f32;
    pos.y = (s1.height - s2.height) / 2f32;
    window.origin(pos.x, pos.y);

    window.modal(parent);
}

fn i_on_close<T>(window: &T, params: &EvWinClose) -> bool
where
    T: WindowTrait,
{
    match params.origin {
        GuiClose::Cancel => {
            i_modal_window(
                window,
                "Pressed [ESC] key. Press [ESC] again or [RETURN] to exit.",
            );
            finish();
        }
        GuiClose::Accept => {
            i_modal_window(
                window,
                "Pressed [RETURN] key. Press [ESC] or [RETURN] again to continue.",
            );
            return false;
        }
        GuiClose::Close => {
            finish();
        }
        GuiClose::Deactivate => {}
    }
    true
}

impl AppHandler for App {
    fn create() -> Self {
        let app = App::new();
        let panel = app.panel();
        app.window.panel(&panel);
        app.window.title("NAppGUI GUI Basics");

        app.window.origin(500f32, 200f32);
        let window = app.window.as_weak();
        app.window
            .on_close(move |params| i_on_close(&window, params));
        app.window.show();
        app
    }
}

fn main() {
    osmain::<App>();
}
