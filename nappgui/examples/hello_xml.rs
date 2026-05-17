use nappgui::osapp::*;
use nappgui::prelude::*;

nappgui_ui!(
    r#"
<HelloWindow inherits="Window" name="window" title="Hello, World!" origin="500.0,200.0">
    <panel>
        <layout rows="3">
            <cell for="label" column="0" row="0" />
            <cell for="button" column="0" row="1" />
            <cell for="textview" column="0" row="2" />
        </layout>
        <label name="label" text="Hello, world!" />
        <button name="button" text="Click me!"/>
        <textview name="textview" />
    </panel>
</HelloWindow>
"#
);

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let clicks = std::rc::Rc::new(std::cell::RefCell::new(0));

        let window = HelloWindow::new();
        window.window.set_on_close_handler(|_| finish());
        window.button.set_on_click_handler(move |_| {
            window.textview.write(&format!("Button click {}\n", clicks.borrow()));
            *clicks.borrow_mut() += 1;
        });
        window.window.show();
        Self {}
    }
}

fn main() {
    osmain::<App>();
}
