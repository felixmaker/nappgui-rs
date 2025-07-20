use nappgui::prelude::*;
use crate::res::*;

fn set_rtf(text: &TextView) {
    let (rtf, rtf_size) = gui_file(TEXTVIEW_RTF);
    let stream = Stream::from_block(rtf, rtf_size as _);
    text.rtf(&stream);
}

fn set_hard_coding(text: &TextView) {
    text.units(64);
    text.line_spacing(1.15);
    text.after_space(10.0);
    text.font_family("Arial");
    text.font_size(16.0);
    text.write("What is Lorem Ipsum?\n");
    text.font_size(11.0);
    text.write("Lorem Ipsum ");
    text.font_style(FontStyle {
        is_bold: true,
        ..Default::default()
    });
    text.write("is simply");
    text.font_style(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.write(" dummy text of the ");
    text.font_style(FontStyle {
        is_italic: true,
        ..Default::default()
    });
    text.write("printing and typesetting ");
    text.font_style(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.write("industry. ");
    text.font_size(16.0);
    text.color(Color::rgb(255, 0, 0));
    text.write("Lorem Ipsum ");
    text.font_size(11.0);
    text.write("has been the ");
    text.font_family("Courier New");
    text.font_size(14.0);
    text.write("[industry's standard] ");
    text.font_family("Arial");
    text.font_size(11.0);
    text.font_style(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    text.write("dummy text");
    text.font_style(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.write(" ever ");
    text.font_style(FontStyle {
        is_strike_out: true,
        ..Default::default()
    });
    text.write("since the 1500s");
    text.font_style(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.write(", when an ");
    text.color(Color::rgb(0, 176, 80));
    text.write("unknown printer ");
    text.write("took a galley of type and scrambled it to make a type specimen book");
    text.font_style(FontStyle {
        is_italic: true,
        ..Default::default()
    });
    text.color(Color::rgb(0, 77, 187));
    text.background_color(Color::rgb(192, 192, 192));
    text.write(". It has survived not only five centuries");
    text.font_style(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.color(Color::rgb(0, 0, 0));
    text.background_color(Color::rgb(255, 255, 255));
    text.write(
        ", but also the leap into electronic typesetting, remaining essentially unchanged.",
    );
}

pub fn textviews() -> Panel {
    let panel = Panel::new();
    let layout = Layout::new(1, 4);

    let label1 = Label::new("From RTF data");
    let label2 = Label::new("Hard coding");
    let text1 = TextView::new();
    let text2 = TextView::new();

    set_rtf(&text1);
    set_hard_coding(&text2);

    text1.size(450.0, 250.0);
    text2.size(450.0, 250.0);

    layout.set(0, 0, label1);
    layout.set(0, 1, text1);
    layout.set(0, 2, label2);
    layout.set(0, 3, text2);

    layout.vertical_margin( 0, 5.0);
    layout.vertical_margin( 1, 10.0);
    layout.vertical_margin( 2, 5.0);

    panel.layout(layout);
    panel
}
