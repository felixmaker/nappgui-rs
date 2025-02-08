use crate::{callback, draw_2d::Font, prelude::Align};

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

/// TableViews are data views that display tabulated information arranged in rows and columns.
pub struct TableView {
    pub(crate) inner: *mut nappgui_sys::TableView,
}

impl TableView {
    pub(crate) fn new(ptr: *mut nappgui_sys::TableView) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create an table view control.
    pub fn create() -> Self {
        let table = unsafe { tableview_create() };
        Self::new(table)
    }

    callback! {
        /// Sets up a handler to read data from the application.
        pub on_data(TableView) => tableview_OnData;

        /// Notifies that the selection has changed.
        pub on_select(TableView) => tableview_OnSelect;

        /// Notify each time a row is clicked.
        pub on_row_click(TableView) => tableview_OnRowClick;

        /// Notifies each time a header is clicked.
        pub on_header_click(TableView)  => tableview_OnHeaderClick;
    }

    /// Sets the general font for the entire table.
    pub fn font(&self, font: &Font) {
        unsafe { tableview_font(self.inner, font.inner) }
    }

    /// Sets the default size of the table control.
    ///
    /// # Remarks
    /// Corresponds to the Natural sizing of the control. By default 256x128.
    pub fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe { tableview_size(self.inner, size) }
    }

    /// Adds a new column to the table.
    pub fn new_column_text(&self) -> u32 {
        unsafe { tableview_new_column_text(self.inner) }
    }

    /// Sets the width of a column.
    pub fn column_width(&self, index: u32, width: f32) {
        unsafe { tableview_column_width(self.inner, index, width) }
    }

    /// Sets the size limits of a column.
    pub fn column_limits(&self, index: u32, min: f32, max: f32) {
        unsafe { tableview_column_limits(self.inner, index, min, max) }
    }

    /// Sets whether a column is resizable or not.
    pub fn column_resizable(&self, column_id: u32, resizable: bool) {
        unsafe { tableview_column_resizable(self.inner, column_id, resizable as i8) }
    }

    /// Allows to freeze the first columns of the table. During horizontal movement they will remain fixed.
    pub fn column_freeze(&self, freeze: u32) {
        unsafe { tableview_column_freeze(self.inner, freeze) }
    }

    /// Sets the text of a column header.
    pub fn header_title(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { tableview_header_title(self.inner, index, text.as_ptr()) }
    }

    /// Sets the alignment of the header text.
    pub fn header_align(&self, index: u32, align: Align) {
        unsafe { tableview_header_align(self.inner, index, align) }
    }

    /// Sets whether the table header is visible or not.
    pub fn header_visible(&self, visible: bool) {
        unsafe { tableview_header_visible(self.inner, visible as i8) }
    }

    /// Sets whether the table header can be clicked as a button.
    pub fn header_clickable(&self, clickable: bool) {
        unsafe { tableview_header_clickable(self.inner, clickable as i8) }
    }

    /// Sets whether the header allows column resizing.
    pub fn header_resizable(&self, resizable: bool) {
        unsafe { tableview_header_resizable(self.inner, resizable as i8) }
    }

    /// Force the height of the header.
    ///
    /// # Remarks
    /// The height of the header is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. Its use is not recommended.
    /// See Table appearance.
    pub fn header_height(&self, height: f32) {
        unsafe { tableview_header_height(self.inner, height) }
    }

    /// Force the height of the row.
    ///
    /// # Remarks
    /// The row height is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. its use is not recommended.
    /// See Table appearance.
    pub fn row_height(&self, height: f32) {
        unsafe { tableview_row_height(self.inner, height) }
    }

    /// Sets the horizontal scrolling when pressing the \[LEFT\] and \[RIGHT\] keys.
    pub fn hkey_scroll(&self, force_column: bool, scoll: f32) {
        unsafe { tableview_hkey_scroll(self.inner, force_column as i8, scoll) }
    }

    /// Sets the row selection mode.
    pub fn multisel(&self, multisel: bool, preserve: bool) {
        unsafe { tableview_multisel(self.inner, multisel as i8, preserve as i8) }
    }

    /// Sets the drawing of the interior lines.
    pub fn grid(&self, hlines: bool, vlines: bool) {
        unsafe { tableview_grid(self.inner, hlines as i8, vlines as i8) }
    }

    /// Synchronizes the table with the data source.
    ///
    /// # Remarks
    /// See Data connection. We must call this function from the application
    /// whenever the data linked to the table changes, in order to update the view.
    pub fn update(&self) {
        unsafe { tableview_update(self.inner) }
    }

    /// Selects rows in the table.
    pub fn select(&self, row: &[u32], n: u32) {
        unsafe { tableview_select(self.inner, row.as_ptr(), n) }
    }

    /// Deselects rows in the table.
    pub fn deselect(&self, row: &[u32], n: u32) {
        unsafe { tableview_deselect(self.inner, row.as_ptr(), n) }
    }

    /// Deselects all rows in the table.
    pub fn deselect_all(&self) {
        unsafe { tableview_deselect_all(self.inner) }
    }

    /// Returns the currently selected rows.
    pub fn selected(&self) -> Option<Vec<u32>> {
        let result = unsafe { tableview_selected(self.inner) };

        if result.is_null() {
            return None;
        }

        let result = unsafe { *result };

        if result.content.is_null() {
            return None;
        }

        let content = unsafe { *result.content };

        let elem = &content.elem;

        Some(elem.to_vec())
    }

    /// Set keyboard focus to a specific row.
    ///
    /// # Remarks
    /// Setting keyboard focus to a row only has effects on navigation, but does not involve
    /// selecting the row. The table is automatically scrolled so that the row is visible.
    /// In this case, align indicates where the vertical scroll is adjusted (up, down or centered).
    pub fn focus_row(&self, row: u32, align: Align) {
        unsafe { tableview_focus_row(self.inner, row, align) }
    }

    /// Gets the row that has keyboard focus.
    pub fn get_focus_row(&self) -> u32 {
        unsafe { tableview_get_focus_row(self.inner) }
    }

    /// Show or hide scroll bars.
    pub fn scroll_visible(&self, hscroll: bool, vscroll: bool) {
        unsafe { tableview_scroll_visible(self.inner, hscroll as i8, vscroll as i8) }
    }
}
