use std::rc::Rc;

use crate::{
    draw_2d::Font,
    gui::{
        define_object,
        event::{TableDataParams, TableDataResult},
        listener, Callback, VoidCallback,
    },
    types::{Align, EventType, TablePositionEvent},
};

use nappgui_sys::{
    tableview_OnData, tableview_OnHeaderClick, tableview_OnRowClick, tableview_OnSelect, tableview_add_column_text,
    tableview_column_align, tableview_column_freeze, tableview_column_icon, tableview_column_limits,
    tableview_column_resizable, tableview_column_width, tableview_create, tableview_del_column, tableview_deselect,
    tableview_deselect_all, tableview_focus_row, tableview_font, tableview_get_focus_row, tableview_grid,
    tableview_header_align, tableview_header_clickable, tableview_header_height, tableview_header_resizable,
    tableview_header_title, tableview_header_visible, tableview_hkey_scroll, tableview_multisel, tableview_row_height,
    tableview_scroll_visible, tableview_select, tableview_selected, tableview_size, tableview_update, S2Df,
};

#[derive(Default)]
pub(crate) struct TableViewProps {
    on_select: VoidCallback,
    on_row_click: VoidCallback,
    on_header_click: VoidCallback,
    on_data: Callback<TableDataParams, TableDataResult>,
}

define_object!(TableView, TableViewInner, TableView, TableViewProps);

impl TableView {
    /// Create an table view control.
    pub fn new() -> Self {
        unsafe { TableView::from_raw(tableview_create()) }
    }

    /// Notifies that the selection has changed.
    pub fn set_on_select_handler<F>(&self, handler: F)
    where
        F: Fn() + 'static,
    {
        self.inner(|inner| *inner.props.on_select.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), TableViewInner, on_select());
        unsafe { tableview_OnSelect(self.as_ptr(), listener) }
    }

    /// Notify each time a row is clicked.
    pub fn set_on_row_click_handler<F>(&self, handler: F)
    where
        F: Fn() + 'static,
    {
        self.inner(|inner| *inner.props.on_row_click.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), TableViewInner, on_row_click());
        unsafe { tableview_OnRowClick(self.as_ptr(), listener) }
    }

    /// Notify each time a header is clicked.
    pub fn set_on_header_click_handler<F>(&self, handler: F)
    where
        F: Fn() + 'static,
    {
        self.inner(|inner| *inner.props.on_header_click.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), TableViewInner, on_header_click());
        unsafe { tableview_OnHeaderClick(self.as_ptr(), listener) }
    }

    /// Sets up a handler to read data from the application.
    pub fn set_on_data_handler<F>(&self, handler: F)
    where
        F: Fn(&TableDataParams) -> TableDataResult + 'static,
    {
        self.inner(|inner| *inner.props.on_data.borrow_mut() = Some(Rc::new(handler)));

        let listener = {
            use std::ffi::c_void;
            extern "C" fn shim(obj: *mut c_void, event: *mut nappgui_sys::Event) {
                if let Some(Some(f)) =
                    crate::gui::global_object(obj as _, |inner: &TableViewInner| inner.props.on_data.borrow().clone())
                {
                    let event = crate::core::event::Event::new(event);
                    let params = match event.type_() {
                        EventType::TableNRows => TableDataParams::TableNCols,
                        EventType::TableCell => {
                            TableDataParams::TableCell(unsafe { event.params::<TablePositionEvent>() })
                        }
                        _ => {
                            return;
                        }
                    };
                    if let Ok(r) =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&(params as TableDataParams))))
                    {
                        match r {
                            TableDataResult::TableNCols(n) => unsafe { event.result(n) },
                            TableDataResult::TableCell(table_cell_event) => unsafe { event.result(table_cell_event) },
                        }
                    }
                }
            }
            let listener = unsafe { nappgui_sys::listener_imp(self.0 as _, Some(shim)) };
            listener
        };

        unsafe { tableview_OnData(self.as_ptr(), listener) }
    }

    /// Sets the general font for the entire table.
    pub fn set_font(&mut self, font: &Font) {
        unsafe { tableview_font(self.as_ptr(), font.as_ptr()) }
    }

    /// Sets the default size of the table control.
    ///
    /// # Remarks
    /// Corresponds to the Natural sizing of the control. By default 256x128.
    pub fn set_size(&mut self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe { tableview_size(self.as_ptr(), size) }
    }

    /// Adds a new column to the table.
    pub fn add_column(&mut self) -> u32 {
        unsafe { tableview_add_column_text(self.as_ptr()) }
    }

    /// Deletes a column from the table.
    pub fn remove_column(&mut self, index: u32) {
        unsafe { tableview_del_column(self.as_ptr(), index) }
    }

    /// Indicates that a column will display an icon.
    pub fn set_column_icon(&mut self, index: u32, icon_height: f32, hmargin: f32) {
        unsafe { tableview_column_icon(self.as_ptr(), index, icon_height, hmargin) }
    }

    /// Sets the width of a column.
    pub fn set_column_width(&mut self, index: u32, width: f32) {
        unsafe { tableview_column_width(self.as_ptr(), index, width) }
    }

    /// Sets the size limits of a column.
    pub fn set_column_limits(&mut self, index: u32, min: f32, max: f32) {
        unsafe { tableview_column_limits(self.as_ptr(), index, min, max) }
    }

    /// Sets the default text alignment for the column data.
    pub fn set_column_align(&mut self, index: u32, align: Align) {
        unsafe { tableview_column_align(self.as_ptr(), index, align as _) }
    }

    /// Sets whether a column is resizable or not.
    pub fn set_column_resizable(&mut self, column_id: u32, resizable: bool) {
        unsafe { tableview_column_resizable(self.as_ptr(), column_id, resizable as _) }
    }

    /// Allows to freeze the first columns of the table. During horizontal movement they will remain fixed.
    pub fn set_column_freeze(&mut self, freeze: u32) {
        unsafe { tableview_column_freeze(self.as_ptr(), freeze) }
    }

    /// Sets the text of a column header.
    pub fn set_header_title(&mut self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { tableview_header_title(self.as_ptr(), index, text.as_ptr()) }
    }

    /// Sets the alignment of the header text.
    pub fn set_header_align(&mut self, index: u32, align: Align) {
        unsafe { tableview_header_align(self.as_ptr(), index, align as _) }
    }

    /// Sets whether the table header is visible or not.
    pub fn set_header_visible(&mut self, visible: bool) {
        unsafe { tableview_header_visible(self.as_ptr(), visible as _) }
    }

    /// Sets whether the table header can be clicked as a button.
    pub fn set_header_clickable(&mut self, clickable: bool) {
        unsafe { tableview_header_clickable(self.as_ptr(), clickable as _) }
    }

    /// Sets whether the header allows column resizing.
    pub fn set_header_resizable(&mut self, resizable: bool) {
        unsafe { tableview_header_resizable(self.as_ptr(), resizable as _) }
    }

    /// Force the height of the header.
    ///
    /// # Remarks
    /// The height of the header is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. Its use is not recommended.
    /// See Table appearance.
    pub fn set_header_height(&mut self, height: f32) {
        unsafe { tableview_header_height(self.as_ptr(), height) }
    }

    /// Force the height of the row.
    ///
    /// # Remarks
    /// The row height is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. its use is not recommended.
    /// See Table appearance.
    pub fn set_row_height(&mut self, height: f32) {
        unsafe { tableview_row_height(self.as_ptr(), height) }
    }

    /// Sets the horizontal scrolling when pressing the \[LEFT\] and \[RIGHT\] keys.
    pub fn set_horizontal_key_scroll(&mut self, force_column: bool, scoll: f32) {
        unsafe { tableview_hkey_scroll(self.as_ptr(), force_column as _, scoll) }
    }

    /// Sets the row selection mode.
    pub fn set_multiselect(&mut self, multisel: bool, preserve: bool) {
        unsafe { tableview_multisel(self.as_ptr(), multisel as _, preserve as _) }
    }

    /// Sets the drawing of the interior lines.
    pub fn set_grid(&mut self, hlines: bool, vlines: bool) {
        unsafe { tableview_grid(self.as_ptr(), hlines as _, vlines as _) }
    }

    /// Synchronizes the table with the data source.
    ///
    /// # Remarks
    /// See Data connection. We must call this function from the application
    /// whenever the data linked to the table changes, in order to update the view.
    pub fn update(&mut self) {
        unsafe { tableview_update(self.as_ptr()) }
    }

    /// Selects rows in the table.
    pub fn select(&self, row: &[u32], n: u32) {
        unsafe { tableview_select(self.as_ptr(), row.as_ptr(), n) }
    }

    /// Deselects rows in the table.
    pub fn deselect(&self, row: &[u32], n: u32) {
        unsafe { tableview_deselect(self.as_ptr(), row.as_ptr(), n) }
    }

    /// Deselects all rows in the table.
    pub fn deselect_all(&self) {
        unsafe { tableview_deselect_all(self.as_ptr()) }
    }

    /// Returns the currently selected rows.
    pub fn selected(&self) -> Option<Vec<u32>> {
        let result = unsafe { tableview_selected(self.as_ptr()) };

        if result.is_null() {
            return None;
        }

        let result = unsafe { *result };

        if result.size == 0 || result.content.is_null() {
            return None;
        }

        let content = unsafe { *result.content };

        let elem = &content.elem;

        Some(elem[..result.size as usize].to_vec())
    }

    /// Set keyboard focus to a specific row.
    ///
    /// # Remarks
    /// Setting keyboard focus to a row only has effects on navigation, but does not involve
    /// selecting the row. The table is automatically scrolled so that the row is visible.
    /// In this case, align indicates where the vertical scroll is adjusted (up, down or centered).
    pub fn set_focus_row(&mut self, row: u32, align: Align) {
        unsafe { tableview_focus_row(self.as_ptr(), row, align as _) }
    }

    /// Gets the row that has keyboard focus.
    pub fn focus_row(&self) -> u32 {
        unsafe { tableview_get_focus_row(self.as_ptr()) }
    }

    /// Show or hide scroll bars.
    pub fn set_scroll_visible(&self, hscroll: bool, vscroll: bool) {
        unsafe { tableview_scroll_visible(self.as_ptr(), hscroll as _, vscroll as _) }
    }
}
