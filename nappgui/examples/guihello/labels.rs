use nappgui::prelude::*;

const LABEL_01: &str = "Hello.";
const LABEL_02: &str = "Hello, I'm a Label.";
const LABEL_03: &str = "Hello, I'm a Label, longer than first.";
const LABEL_04: &str = "Hello, I'm a Label, longer than first and longer than second.";
const LABEL_05: &str =
    "Hello, I'm a Label, longer than first, longer than second and longer than third.";
const LABEL_06: &str = "Hello, I'm a Label, longer than first, longer than second, longer than third and longer than fourth.";
const LABEL_07: &str = "Mouse sensitive label";

fn width_popup(layout: &Layout) -> PopUp {
    let popup = PopUp::new();
    let layout = layout.clone();
    popup.add_element("Natural");
    popup.add_element("100px");
    popup.add_element("200px");
    popup.add_element("300px");
    popup.add_element("400px");
    popup.on_select(move |params| {
        let width = match params.index {
            0 => 0,
            1 => 100,
            2 => 200,
            3 => 300,
            4 => 400,
            _ => 0,
        };
        layout.hsize(0, width as _);
        layout.update();
    });
    popup
}

fn labels(multiline: bool) -> Panel {
    let panel = Panel::new();
    let layout = Layout::new(1, 7);
    let popup = width_popup(&layout);
    let label1 = Label::new(LABEL_01);
    let label2 = Label::new(LABEL_02);
    let label3 = Label::new(LABEL_03);
    let label4 = Label::new(LABEL_04);
    let label5 = Label::new(LABEL_05);
    let label6 = Label::new(LABEL_06);
    label1.multiline(multiline);
    label2.multiline(multiline);
    label3.multiline(multiline);
    label4.multiline(multiline);
    label5.multiline(multiline);
    label6.multiline(multiline);

    layout.set(0, 0, popup);
    layout.set(0, 1, label1);
    layout.set(0, 2, label2);
    layout.set(0, 3, label3);
    layout.set(0, 4, label4);
    layout.set(0, 5, label5);
    layout.set(0, 6, label6);

    layout.vmargin(0, 5f32);
    panel.layout(layout);
    panel
}

pub fn labels_single_line() -> Panel {
    labels(false)
}

pub fn labels_multi_line() -> Panel {
    labels(true)
}

pub fn labels_mouse_over() -> Panel {
    let panel = Panel::new();
    let layout = Layout::new(1, 5);
    let font_style = FontStyle {
        is_normal: true,
        is_pixels: true,
        ..Default::default()
    };
    let font = Font::system(20f32, font_style);

    let label1 = Label::new(LABEL_07);
    let label2 = Label::new(LABEL_07);
    let label3 = Label::new(LABEL_07);
    let label4 = Label::new(LABEL_07);
    let label5 = Label::new(LABEL_07);

    label1.font(&font);
    label2.font(&font);
    label3.font(&font);
    label4.font(&font);
    label5.font(&font);

    let color_red = Color::red(255);
    let color_blue = Color::blue(255);

    label1.color_over(color_red);
    label2.color_over(color_red);
    label3.color_over(color_red);
    label4.color_over(color_red);
    label5.color_over(color_red);

    label1.style_over(FontStyle {
        is_bold: true,
        ..Default::default()
    });
    label2.style_over(FontStyle {
        is_italic: true,
        ..Default::default()
    });
    label3.style_over(FontStyle {
        is_strike_out: true,
        ..Default::default()
    });
    label4.style_over(FontStyle {
        is_underlined: true,
        ..Default::default()
    });

    label5.bgcolor_over(color_blue);

    layout.set(0, 0, label1);
    layout.set(0, 1, label2);
    layout.set(0, 2, label3);
    layout.set(0, 3, label4);
    layout.set(0, 4, label5);

    panel.layout(layout);
    panel
}
