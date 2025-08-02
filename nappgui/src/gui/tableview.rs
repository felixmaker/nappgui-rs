use crate::{
    draw_2d::Font,
    gui::{
        control::impl_control,
        event::{EvTbDataParams, EvTbDataResult},
    },
    types::{Align, EventType},
    util::macros::callback,
};

use nappgui_sys::{
    tableview_OnData, tableview_OnHeaderClick, tableview_OnRowClick, tableview_OnSelect,
    tableview_column_freeze, tableview_column_limits, tableview_column_resizable,
    tableview_column_width, tableview_create, tableview_deselect, tableview_deselect_all,
    tableview_focus_row, tableview_font, tableview_get_focus_row, tableview_grid,
    tableview_header_align, tableview_header_clickable, tableview_header_height,
    tableview_header_resizable, tableview_header_title, tableview_header_visible,
    tableview_hkey_scroll, tableview_multisel, tableview_new_column_text, tableview_row_height,
    tableview_scroll_visible, tableview_select, tableview_selected, tableview_size,
    tableview_update, S2Df,
};

/// The table view trait.
pub trait TableViewTrait {
    /// Returns a raw pointer to the tableview object.
    fn as_ptr(&self) -> *mut nappgui_sys::TableView;

    callback! {
        /// Notifies that the selection has changed.
        on_select() => tableview_OnSelect;

        /// Notify each time a row is clicked.
        on_row_click() => tableview_OnRowClick;

        /// Notifies each time a header is clicked.
        on_header_click()  => tableview_OnHeaderClick;
    }

    /// Sets up a handler to read data from the application.
    fn on_data<F>(&self, handler: F)
    where
        F: FnMut(&EvTbDataParams) -> EvTbDataResult + 'static,
    {
        use std::ffi::c_void;

        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut Box<dyn FnMut(&EvTbDataParams) -> EvTbDataResult>;
            let f = &mut *data;
            let event = crate::core::event::Event::new(event);
            match event.type_() {
                EventType::TableNRows => {
                    let params = EvTbDataParams::TableNCols;
                    if let Ok(r) =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&params)))
                    {
                        if let EvTbDataResult::TableNCols(nrows) = r {
                            event.result(nrows);
                        }
                    }
                }
                EventType::TableCell => {
                    let params = EvTbDataParams::TableCell(event.params().unwrap());
                    if let Ok(r) =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&params)))
                    {
                        if let EvTbDataResult::TableCell(cell) = r {
                            event.result(cell);
                        }
                    }
                }
                _ => {}
            }
        }

        let cb: Box<dyn FnMut(&EvTbDataParams) -> EvTbDataResult> = Box::new(handler);
        let data: *mut Box<dyn FnMut(&EvTbDataParams) -> EvTbDataResult> =
            Box::into_raw(Box::new(cb));

        let listener = unsafe { nappgui_sys::listener_imp(data as *mut c_void, Some(shim)) };

        unsafe {
            tableview_OnData(self.as_ptr(), listener);
        }
    }

    /// Sets the general font for the entire table.
    fn font(&self, font: &Font) {
        unsafe { tableview_font(self.as_ptr(), font.inner) }
    }

    /// Sets the default size of the table control.
    ///
    /// # Remarks
    /// Corresponds to the Natural sizing of the control. By default 256x128.
    fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe { tableview_size(self.as_ptr(), size) }
    }

    /// Adds a new column to the table.
    fn new_column_text(&self) -> u32 {
        unsafe { tableview_new_column_text(self.as_ptr()) }
    }

    /// Sets the width of a column.
    fn column_width(&self, index: u32, width: f32) {
        unsafe { tableview_column_width(self.as_ptr(), index, width) }
    }

    /// Sets the size limits of a column.
    fn column_limits(&self, index: u32, min: f32, max: f32) {
        unsafe { tableview_column_limits(self.as_ptr(), index, min, max) }
    }

    /// Sets whether a column is resizable or not.
    fn column_resizable(&self, column_id: u32, resizable: bool) {
        unsafe { tableview_column_resizable(self.as_ptr(), column_id, resizable as _) }
    }

    /// Allows to freeze the first columns of the table. During horizontal movement they will remain fixed.
    fn column_freeze(&self, freeze: u32) {
        unsafe { tableview_column_freeze(self.as_ptr(), freeze) }
    }

    /// Sets the text of a column header.
    fn header_title(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { tableview_header_title(self.as_ptr(), index, text.as_ptr()) }
    }

    /// Sets the alignment of the header text.
    fn header_align(&self, index: u32, align: Align) {
        unsafe { tableview_header_align(self.as_ptr(), index, align as _) }
    }

    /// Sets whether the table header is visible or not.
    fn header_visible(&self, visible: bool) {
        unsafe { tableview_header_visible(self.as_ptr(), visible as _) }
    }

    /// Sets whether the table header can be clicked as a button.
    fn header_clickable(&self, clickable: bool) {
        unsafe { tableview_header_clickable(self.as_ptr(), clickable as _) }
    }

    /// Sets whether the header allows column resizing.
    fn header_resizable(&self, resizable: bool) {
        unsafe { tableview_header_resizable(self.as_ptr(), resizable as _) }
    }

    /// Force the height of the header.
    ///
    /// # Remarks
    /// The height of the header is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. Its use is not recommended.
    /// See Table appearance.
    fn header_height(&self, height: f32) {
        unsafe { tableview_header_height(self.as_ptr(), height) }
    }

    /// Force the height of the row.
    ///
    /// # Remarks
    /// The row height is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. its use is not recommended.
    /// See Table appearance.
    fn row_height(&self, height: f32) {
        unsafe { tableview_row_height(self.as_ptr(), height) }
    }

    /// Sets the horizontal scrolling when pressing the \[LEFT\] and \[RIGHT\] keys.
    fn horizontal_key_scroll(&self, force_column: bool, scoll: f32) {
        unsafe { tableview_hkey_scroll(self.as_ptr(), force_column as _, scoll) }
    }

    /// Sets the row selection mode.
    fn multiselect(&self, multisel: bool, preserve: bool) {
        unsafe { tableview_multisel(self.as_ptr(), multisel as _, preserve as _) }
    }

    /// Sets the drawing of the interior lines.
    fn grid(&self, hlines: bool, vlines: bool) {
        unsafe { tableview_grid(self.as_ptr(), hlines as _, vlines as _) }
    }

    /// Synchronizes the table with the data source.
    ///
    /// # Remarks
    /// See Data connection. We must call this function from the application
    /// whenever the data linked to the table changes, in order to update the view.
    fn update(&self) {
        unsafe { tableview_update(self.as_ptr()) }
    }

    /// Selects rows in the table.
    fn select(&self, row: &[u32], n: u32) {
        unsafe { tableview_select(self.as_ptr(), row.as_ptr(), n) }
    }

    /// Deselects rows in the table.
    fn deselect(&self, row: &[u32], n: u32) {
        unsafe { tableview_deselect(self.as_ptr(), row.as_ptr(), n) }
    }

    /// Deselects all rows in the table.
    fn deselect_all(&self) {
        unsafe { tableview_deselect_all(self.as_ptr()) }
    }

    /// Returns the currently selected rows.
    fn selected(&self) -> Option<Vec<u32>> {
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
    fn focus_row(&self, row: u32, align: Align) {
        unsafe { tableview_focus_row(self.as_ptr(), row, align as _) }
    }

    /// Gets the row that has keyboard focus.
    fn get_focus_row(&self) -> u32 {
        unsafe { tableview_get_focus_row(self.as_ptr()) }
    }

    /// Show or hide scroll bars.
    fn scroll_visible(&self, hscroll: bool, vscroll: bool) {
        unsafe { tableview_scroll_visible(self.as_ptr(), hscroll as _, vscroll as _) }
    }
}

/// TableViews are data views that display tabulated information arranged in rows and columns.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct TableView {
    pub(crate) inner: *mut nappgui_sys::TableView,
}

impl TableViewTrait for TableView {
    fn as_ptr(&self) -> *mut nappgui_sys::TableView {
        self.inner
    }
}

impl TableView {
    /// Create an table view control.
    pub fn new() -> Self {
        let table = unsafe { tableview_create() };
        Self { inner: table }
    }
}

impl_control!(TableView, guicontrol_tableview);
