use nappgui::prelude::*;

static LOREM: &str = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";

fn left_panel() -> Panel {
    const N: usize = 32;
    let panel = Panel::new_scroll(false, true);
    let layout = Layout::new(2, N);
    let rmargin = panel.scroll_width();

    for i in 0..N {
        let label = Label::new(&format!("Value {}", i));
        let edit = Edit::new();
        edit.text(&format!("Edit here value {}", i));
        layout.set(0, i, label);
        layout.set(1, i, edit);
    }

    for i in 0..N - 1 {
        layout.vmargin(i, 3.0);
    }

    layout.hmargin(0, 5.0);
    layout.margin4(0.0, rmargin, 0.0, 0.0);
    layout.hexpand(1);
    panel.layout(layout);

    panel
}

pub fn split_panel() -> Panel {
    let panel1 = Panel::new();
    let panel2 = left_panel();

    let layout = Layout::new(1, 1);
    let split1 = SplitView::new_vertical();
    let split2 = SplitView::new_vertical();
    let text = TextView::new();
    let view = View::new();

    text.writef(LOREM);
    view.size(400.0, 200.0);
    text.size(400.0, 200.0);

    split2.view(view, false); // todo 2d draw
    split2.textview(text, false);
    split1.panel(panel2);
    split1.splitview(split2);

    layout.set(0, 0, split1);
    panel1.layout(layout);

    panel1
}
