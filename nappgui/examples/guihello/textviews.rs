use nappgui::prelude::*;
use crate::res::*;

fn set_rtf(text: &TextView) {
    let (rtf, rtf_size) = gui_file(TEXTVIEW_RTF);
    let stream = Stream::from_block(rtf, rtf_size as _);
    text.rtf(&stream);
}

fn set_hard_coding(text: &TextView) {
    text.units(64);
    text.lspacing(1.15);
    text.afspace(10.0);
    text.family("Arial");
    text.fsize(16.0);
    text.writef("What is Lorem Ipsum?\n");
    text.fsize(11.0);
    text.writef("Lorem Ipsum ");
    text.fstyle(FontStyle {
        is_bold: true,
        ..Default::default()
    });
    text.writef("is simply");
    text.fstyle(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.writef(" dummy text of the ");
    text.fstyle(FontStyle {
        is_italic: true,
        ..Default::default()
    });
    text.writef("printing and typesetting ");
    text.fstyle(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.writef("industry. ");
    text.fsize(16.0);
    text.color(Color::rgb(255, 0, 0));
    text.writef("Lorem Ipsum ");
    text.fsize(11.0);
    text.writef("has been the ");
    text.family("Courier New");
    text.fsize(14.0);
    text.writef("[industry's standard] ");
    text.family("Arial");
    text.fsize(11.0);
    text.fstyle(FontStyle {
        is_underlined: true,
        ..Default::default()
    });
    text.writef("dummy text");
    text.fstyle(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.writef(" ever ");
    text.fstyle(FontStyle {
        is_strike_out: true,
        ..Default::default()
    });
    text.writef("since the 1500s");
    text.fstyle(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.writef(", when an ");
    text.color(Color::rgb(0, 176, 80));
    text.writef("unknown printer ");
    text.writef("took a galley of type and scrambled it to make a type specimen book");
    text.fstyle(FontStyle {
        is_italic: true,
        ..Default::default()
    });
    text.color(Color::rgb(0, 77, 187));
    text.bgcolor(Color::rgb(192, 192, 192));
    text.writef(". It has survived not only five centuries");
    text.fstyle(FontStyle {
        is_normal: true,
        ..Default::default()
    });
    text.color(Color::rgb(0, 0, 0));
    text.bgcolor(Color::rgb(255, 255, 255));
    text.writef(
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

    layout.vmargin( 0, 5.0);
    layout.vmargin( 1, 10.0);
    layout.vmargin( 2, 5.0);

    panel.layout(layout);
    panel
}
