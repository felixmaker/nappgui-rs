use std::cell::Cell;
use std::rc::Rc;

use nappgui::osapp::*;
use nappgui::prelude::*;

nappgui_ui!(
    r#"
    <UI mod="ui">
    <HelloWindow inherits="Window" title="Hello, World!" origin="500.0,200.0">
    <Panel>
        <layout rows="3" >
            <cell for="label" column="0" row="0" />
            <cell for="button" column="0" row="1" />
            <cell for="textview" column="0" row="2" />
        </layout>
        <label name="label" text="Hello, world!" />
        <button name="button" text="Click me!" on-click="button_click"/>
        <textview name="textview" write="click" />
    </Panel>
    </HelloWindow>
    </UI>
"#
);

struct App {}

impl AppHandler for App {
    fn create() -> Self {
        let click = Rc::new(Cell::new(0));
        let window = HelloWindow::new();
        window.setter_button_click(move |_| {
            click.set(click.get() + 1);
            window.setter_click(&format!("Click: {}\n", click.get()));
        });
        window.set_on_close_handler(|_| finish());
        window.show();
        Self {}
    }
}

fn main() {
    osmain::<App>();
}
