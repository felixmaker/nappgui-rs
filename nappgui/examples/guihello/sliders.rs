use nappgui::prelude::*;

pub fn sliders() -> Panel {
    let panel = Panel::new();

    let layout1 = Layout::new(2, 1);
    let layout2 = Layout::new(1, 8);

    let label1 = Label::new("Slider");
    let label2 = Label::new("Slider  (discrete 6 steps)");
    let label3 = Label::new("Progress Bar");
    let label4 = Label::new("Progress Undefined");

    let progress1 = Progress::new();
    let progress2 = Progress::new();

    let slider1 = Slider::new();
    let slider2 = Slider::new();
    let slider3 = Slider::new_vertical();

    slider2.steps(6);
    slider1.on_moved(move |params| {
        progress1.value(params.pos);
    });

    progress2.undefined(true);

    slider1.tooltip("Horizontal Slider");
    slider2.tooltip("Horizontal Discrete Slider");
    slider3.tooltip("Vertical Slider");

    layout2.set(0, 0, label1);
    layout2.set(0, 2, label2);
    layout2.set(0, 4, label3);
    layout2.set(0, 6, label4);

    layout2.set(0, 1, slider1);
    layout2.set(0, 3, slider2);

    layout1.set(1, 0, slider3);
    layout2.set(0, 5, progress1);
    layout2.set(0, 7, progress2);

    layout2.hsize(0, 300.0);
    layout1.set(0, 0, layout2);
    layout2.vmargin(0, 5.0);
    layout2.vmargin(1, 5.0);
    layout2.vmargin(2, 5.0);
    layout2.vmargin(3, 5.0);
    layout2.vmargin(4, 5.0);
    layout2.vmargin(5, 5.0);
    layout2.vmargin(6, 5.0);
    layout2.hmargin(0, 10.0);

    panel.layout(layout1);

    panel
}
