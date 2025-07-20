use std::{cell::RefCell, rc::Rc};

use nappgui::prelude::*;

#[derive(Default)]
struct Data {
    table: Option<TableView>,
    text: Option<TextView>,
}

fn table_control_layout(data: &Rc<RefCell<Data>>) -> Layout {
    let layout = Layout::new(1, 6);
    let layout2 = Layout::new(3, 1);

    let button1 = RadioButton::new("Single select");
    let data1 = data.clone();
    button1.on_click(move |_| {
        if let Some(table) = &data1.borrow_mut().table {
            table.multisel(false, false);
        }
    });
    let button2 = RadioButton::new("Multi select");
    let data2 = data.clone();
    button2.on_click(move |_| {
        if let Some(table) = &data2.borrow_mut().table {
            table.multisel(true, false);
        }
    });

    let button3 = RadioButton::new("Preserve select");
    let data3 = data.clone();
    button3.on_click(move |_| {
        if let Some(table) = &data3.borrow_mut().table {
            table.multisel(true, true);
        }
    });

    let button4 = CheckButton::new("Resizable headers");
    let data4 = data.clone();
    button4.on_click(move |params| {
        if let Some(table) = &data4.borrow_mut().table {
            table.header_resizable(params.state == GuiState::On);
        }
    });

    let button5 = CheckButton::new("Clickable headers");
    let data5 = data.clone();
    button5.on_click(move |params| {
        if let Some(table) = &data5.borrow_mut().table {
            table.header_clickable(params.state == GuiState::On);
        }
    });

    let button6 = CheckButton::new("Freeze 0 and 1 columns");
    let data6 = data.clone();
    button6.on_click(move |params| {
        if let Some(table) = &data6.borrow_mut().table {
            let freeze = if params.state == GuiState::On {
                1
            } else {
                u32::MAX
            };
            table.column_freeze(freeze as _);
        }
    });

    let button7 = CheckButton::new("Draw grid lines");
    let data7 = data.clone();
    button7.on_click(move |params| {
        if let Some(table) = &data7.borrow_mut().table {
            table.grid(params.state == GuiState::On, params.state == GuiState::On);
        }
    });

    let button8 = PushButton::new("Print selected rows");
    let data8 = data.clone();
    button8.on_click(move |_| {
        if let Data {
            table: Some(table),
            text: Some(text),
        } = &*data8.borrow_mut()
        {
            if let Some(rows) = table.selected() {
                text.writef(&format!(
                    "Selected rows: {}\n",
                    rows.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }
        }
    });

    button1.state(GuiState::On);
    button4.state(GuiState::On);
    button5.state(GuiState::On);
    button6.state(GuiState::On);
    button7.state(GuiState::On);

    layout2.set(0, 0, button1);
    layout2.set(1, 0, button2);
    layout2.set(2, 0, button3);

    layout.set(0, 0, layout2);
    layout.set(0, 1, button4);
    layout.set(0, 2, button5);
    layout.set(0, 3, button6);
    layout.set(0, 4, button7);
    layout.set(0, 5, button8);

    layout2.hmargin(0, 5.0);
    layout2.hmargin(1, 5.0);
    layout.vmargin(0, 5.0);
    layout.vmargin(1, 5.0);
    layout.vmargin(2, 5.0);
    layout.vmargin(3, 5.0);
    layout.vmargin(4, 5.0);

    layout
}

pub fn table_view() -> Panel {
    let data = Rc::new(RefCell::new(Data::default()));
    let panel = Panel::new();
    let layout = Layout::new(1, 3);
    let layout2 = table_control_layout(&data);
    let table = TableView::new();
    let text = TextView::new();

    data.borrow_mut().table = Some(table);
    data.borrow_mut().text = Some(text);

    let table_title = [
        "Name",
        "Address",
        "City",
        "Age",
        "Value",
        "Extra\nData 1",
        "Extra\nData 2",
        "Extra\nData 3",
        "Extra\nData 4",
    ];

    let table_width = [100.0, 105.0, 50.0, 50.0, 170.0, 200.0, 200.0, 200.0, 200.0];

    for i in 0..=8 {
        table.new_column_text();
        table.header_title(i, table_title[i]);
        table.column_width(i, table_width[i]);
    }

    table.on_data(|params| match params {
        EvTbDataParams::TableNCols => 100.into(),
        EvTbDataParams::TableCell(pos) => match pos.col {
            0 => EvTbCell {
                text: format!("Name {}", pos.row),
                align: Align::Left,
            }
            .into(),
            1 => EvTbCell {
                text: format!("Address {}", pos.row),
                align: Align::Left,
            }
            .into(),
            2 => EvTbCell {
                text: format!("City {}", pos.row),
                align: Align::Left,
            }
            .into(),
            3 => EvTbCell {
                text: format!("{}", pos.row),
                align: Align::Right,
            }
            .into(),
            4 => EvTbCell {
                text: format!("{}", pos.row),
                align: Align::Left,
            }
            .into(),
            5 => EvTbCell {
                text: format!("Extra Data 1 {}", pos.row),
                align: Align::Left,
            }
            .into(),
            6 => EvTbCell {
                text: format!("Extra Data 2 {}", pos.row),
                align: Align::Left,
            }
            .into(),
            7 => EvTbCell {
                text: format!("Extra Data 3 {}", pos.row),
                align: Align::Left,
            }
            .into(),
            8 => EvTbCell {
                text: format!("Extra Data 4 {}", pos.row),
                align: Align::Left,
            }
            .into(),
            _ => EvTbCell {
                text: format!("-- {}", pos.row),
                align: Align::Left,
            }
            .into(),
        },
    });

    table.size(500.0, 300.0);
    table.multisel(false, false);
    table.header_visible(true);
    table.grid(true, true);
    table.update();

    layout.set(0, 0, layout2);
    layout.set(0, 1, table);
    layout.set(0, 2, text);
    layout.vmargin(0, 5.0);
    layout.vmargin(1, 5.0);

    panel.layout(layout);
    panel
}
