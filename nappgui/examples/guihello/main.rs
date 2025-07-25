mod buttons;
mod editor;
mod form;
mod labels;
mod listboxes;
mod popcom;
mod seltext;
mod sliders;
mod tabstops;
mod vpadding;
mod textviews;
mod table;
mod splits;
mod modalwin;
mod flyout;

#[allow(unused)]
pub(crate) mod res {
    nappgui::include_resource!(res);
}

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

        list.add_element("Labels single line");
        list.add_element("Labels multi line");
        list.add_element("Labels mouse sensitive");
        list.add_element("Buttons");
        list.add_element("PopUp Combo");
        list.add_element("ListBoxes");
        list.add_element("Form");
        list.add_element("Text select");
        list.add_element("Text editor");
        list.add_element("Sliders");
        list.add_element("Vertical padding");
        list.add_element("Tabstops");
        list.add_element("TextViews");
        list.add_element("TableView");
        list.add_element("SplitViews");
        list.add_element("Modal Windows");
        list.add_element("Flyout Windows");
        list.add_element("Hotkeys");
        list.add_element("Data Binding");
        list.add_element("Struct Binding");
        list.add_element("Basic Layout");
        list.add_element("SubLayouts");
        list.add_element("Subpanels");
        list.add_element("Multi-Layouts");
        list.add_element("Scroll panel");
        list.add_element("Dynamic layouts");
        list.add_element("Dynamic menus");
        list.add_element("IP Input");
        list.add_element("Font x-scale");
        list.add_element("Reduce components");
        list.select(0, true);

        let window = self.window.as_weak();
        let layout = self.layout.clone();
        list.on_select(move |params| {
            set_panel(&window, &layout, params.index);
            window.update();
        });

        self.layout.set(0, 0, list);
        set_panel(&self.window.as_weak(), &self.layout, 0);

        panel.layout(self.layout);
        self.layout.vertical_align(0, 0, Align::Left);
        self.layout.vertical_align(1, 0, Align::Left);
        self.layout.margin(10f32);
        self.layout.horizontal_margin(0, 10f32);

        panel
    }
}

fn set_panel(window: &WeakWindow, layout: &Layout, index: u32) {
    let mut defbutton: Option<PushButton> = None;

    let panel = match index {
        0 => labels::labels_single_line(),
        1 => labels::labels_multi_line(),
        2 => labels::labels_mouse_over(),
        3 => buttons::buttons_basics(&mut defbutton),
        4 => popcom::popup_combo(),
        5 => listboxes::listboxes(),
        6 => form::form_basic(window.clone()),
        7 => seltext::seltext(window.clone()),
        8 => editor::editor(),
        9 => sliders::sliders(),
        10 => vpadding::vpadding(),
        11 => tabstops::tabstops(),
        12 => textviews::textviews(),
        13 => table::table_view(),
        14 => splits::split_panel(),
        15 => modalwin::modal_windows(window.clone()),
        16 => flyout::flyout_window(window.clone()),
        _ => {
            return;
        }
    };

    layout.panel_replace(panel, 1, 0);

    if let Some(defbutton) = &mut defbutton {
        window.default_button(defbutton);
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
    layout.set(0, 0, label);
    layout.margin(10f32);
    panel.layout(layout);
    window.panel(panel);
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
        _ => {}
    }
    true
}

impl AppHandler for App {
    fn create() -> Self {
        gui_respack(res::res_respack);
        gui_language("");

        let app = App::new();
        let panel = app.panel();
        app.window.panel(panel);
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
