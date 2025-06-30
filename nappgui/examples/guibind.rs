use std::mem::offset_of;

use nappgui::cell_dbind;
use nappgui::core::NappguiString;
use nappgui::dbind;
use nappgui::dbind_enum;
use nappgui::dbind_increment;
use nappgui::dbind_range;
use nappgui::draw_2d::Font;
use nappgui::gui::*;
use nappgui::layout_dbind;
use nappgui::layout_dbind_obj;
use nappgui::osapp::*;
use nappgui::prelude::*;

#[repr(C)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Black,
    White,
}

#[repr(C)]
struct BasicTypes {
    bool_val: bool,
    uint16_val: u16,
    real32_val: f32,
    enum_val: Color,
    enum3_val: GuiState,
    str_val: NappguiString,
}

static I_NUM_CONTROLS: usize = 9;

fn i_data_bind() {
    let _ = dbind_enum!(GuiState, ekGUI_OFF, "");
    let _ = dbind_enum!(GuiState, ekGUI_ON, "");
    let _ = dbind_enum!(GuiState, ekGUI_MIXED, "");

    let _ = dbind_enum!(Color, Red, "Red");
    let _ = dbind_enum!(Color, Blue, "Blue");
    let _ = dbind_enum!(Color, Green, "Green");
    let _ = dbind_enum!(Color, Yellow, "Yellow");
    let _ = dbind_enum!(Color, Magenta, "Magenta");
    let _ = dbind_enum!(Color, Cyan, "Cyan");
    let _ = dbind_enum!(Color, Black, "Black");
    let _ = dbind_enum!(Color, White, "White");

    let _ = dbind!(BasicTypes, bool_val, bool, "bool_t");
    let _ = dbind!(BasicTypes, uint16_val, u16, "uint16_t");
    let _ = dbind!(BasicTypes, real32_val, f32, "real32_t");
    let _ = dbind!(BasicTypes, enum_val, Color, "Color");
    let _ = dbind!(BasicTypes, enum3_val, GuiState, "GuiState");
    let _ = dbind!(BasicTypes, str_val, NappguiString, "String *");

    dbind_range!(BasicTypes, real32_val, -50.0f32, 50.0f32);
    dbind_increment!(BasicTypes, real32_val, 10.0f32);
}

fn i_radio_layout() -> Layout {
    let layout = Layout::create(1, 6);
    for i in 0..6 {
        let radio = Button::radio();
        radio.text(&format!("Radio {}", i));
        layout.button(&radio, 0, i);
    }

    layout
}

fn i_title_labels(layout: &Layout) {
    let mut font = Font::system(Font::regular_size(), FStyle::ekFBOLD as _);
    let strs = [
        "Label", "EditBox", "Check", "Check3", "Radio", "PopUp", "ListBox", "Slider", "UpDown",
    ];
    for i in 0..I_NUM_CONTROLS {
        let label = Label::create();
        label.text(strs[i]);
        label.font(&font);
        layout.label(&label, 0, i as _);
    }

    layout.hmargin(0, 10.0);
    font.destroy();
}

fn i_value_labels(layout: &Layout) {
    for i in 0..I_NUM_CONTROLS {
        let label = Label::create();
        label.align(Align::ekCENTER as _);
        layout.label(&label, 2, i as _);
        layout.halign(2, i as _, Align::ekJUSTIFY as _);
    }

    layout.hsize(2, 80.0);
    layout.hmargin(0, 10.0);
    for i in 0..(I_NUM_CONTROLS - 1) {
        layout.vmargin(i as _, 5.0);
    }

    cell_dbind!(
        &layout.cell(2, 0),
        BasicTypes,
        str_val,
        NappguiString,
        "String *"
    );
    cell_dbind!(
        &layout.cell(2, 1),
        BasicTypes,
        str_val,
        NappguiString,
        "String *"
    );
    cell_dbind!(&layout.cell(2, 2), BasicTypes, bool_val, bool, "bool_t");
    cell_dbind!(
        &layout.cell(2, 3),
        BasicTypes,
        enum3_val,
        GuiState,
        "GuiState"
    );
    cell_dbind!(&layout.cell(2, 4), BasicTypes, uint16_val, u16, "uint16_t");
    cell_dbind!(&layout.cell(2, 5), BasicTypes, real32_val, f32, "real32_t");
    cell_dbind!(&layout.cell(2, 6), BasicTypes, enum_val, Color, "Color");
    cell_dbind!(&layout.cell(2, 7), BasicTypes, real32_val, f32, "real32_t");
    cell_dbind!(&layout.cell(2, 8), BasicTypes, real32_val, f32, "real32_t");
}

fn i_layout() -> Layout {
    let layout = Layout::create(3, 9);
    let label = Label::create();
    let edit = Edit::create();
    let check = Button::check();
    let check3 = Button::check3();
    let radio = i_radio_layout();
    let pop = PopUp::create();
    let list = ListBox::create();
    let slider = Slider::create();
    let updown = UpDown::create();
    layout.label(&label, 1, 0);
    layout.edit(&edit, 1, 1);
    layout.button(&check, 1, 2);
    layout.button(&check3, 1, 3);
    layout.layout(&radio, 1, 4);
    layout.popup(&pop, 1, 5);
    layout.listbox(&list, 1, 6);
    layout.slider(&slider, 1, 7);
    layout.updown(&updown, 1, 8);

    cell_dbind!(
        &layout.cell(1, 0),
        BasicTypes,
        str_val,
        NappguiString,
        "String *"
    );
    cell_dbind!(
        &layout.cell(1, 1),
        BasicTypes,
        str_val,
        NappguiString,
        "String *"
    );
    cell_dbind!(&layout.cell(1, 2), BasicTypes, bool_val, bool, "bool_t");
    cell_dbind!(
        &layout.cell(1, 3),
        BasicTypes,
        enum3_val,
        GuiState,
        "GuiState"
    );
    cell_dbind!(&layout.cell(1, 4), BasicTypes, uint16_val, u16, "uint16_t");
    cell_dbind!(&layout.cell(1, 5), BasicTypes, real32_val, f32, "real32_t");
    cell_dbind!(&layout.cell(1, 6), BasicTypes, enum_val, Color, "Color");
    cell_dbind!(&layout.cell(1, 7), BasicTypes, real32_val, f32, "real32_t");
    cell_dbind!(&layout.cell(1, 8), BasicTypes, real32_val, f32, "real32_t");

    i_title_labels(&layout);
    i_value_labels(&layout);
    layout
}

struct App {
    _window: Window,
}

impl AppHandler for App {
    fn create() -> Self {
        i_data_bind();

        let layout = i_layout();
        let panel = Panel::create();

        let data = BasicTypes {
            bool_val: false,
            uint16_val: 0,
            real32_val: 0.0,
            enum_val: Color::White,
            enum3_val: GuiState::ekGUI_MIXED,
            str_val: NappguiString::new("Text"),
        };

        // for simple example...
        let data = Box::into_raw(Box::new(data));

        panel.layout(&layout);

        layout_dbind!(&layout, BasicTypes);
        layout_dbind_obj!(&layout, data as _, BasicTypes);

        let window = Window::create(WindowFlag::ekWINDOW_STD);
        window.panel(&panel);
        window.title("Hello, World!");
        window.origin(V2Df { x: 500.0, y: 200.0 });
        window.on_close(|_window, _event| finish());

        window.show();

        App {
            _window: window,
        }
    }
}

fn main() {
    osmain::<App>();
}
