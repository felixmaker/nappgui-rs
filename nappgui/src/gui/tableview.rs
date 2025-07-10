use std::rc::Rc;

use crate::{
    draw_2d::Font, gui::{control::impl_control, impl_layout}, types::Align, util::macros::{callback, pub_crate_ptr_ops}
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

/// TableViews are data views that display tabulated information arranged in rows and columns.
#[derive(Clone)]
pub struct TableView {
    pub(crate) inner: Rc<*mut nappgui_sys::TableView>,
}

impl TableView {
    pub_crate_ptr_ops!(*mut nappgui_sys::TableView);

    /// Create an table view control.
    pub fn new() -> Self {
        let table = unsafe { tableview_create() };
        Self::from_raw(table)
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
        unsafe { tableview_font(self.as_ptr(), font.inner) }
    }

    /// Sets the default size of the table control.
    ///
    /// # Remarks
    /// Corresponds to the Natural sizing of the control. By default 256x128.
    pub fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe { tableview_size(self.as_ptr(), size) }
    }

    /// Adds a new column to the table.
    pub fn new_column_text(&self) -> usize {
        unsafe { tableview_new_column_text(self.as_ptr()) as _ }
    }

    /// Sets the width of a column.
    pub fn column_width(&self, index: usize, width: f32) {
        unsafe { tableview_column_width(self.as_ptr(), index as _, width) }
    }

    /// Sets the size limits of a column.
    pub fn column_limits(&self, index: usize, min: f32, max: f32) {
        unsafe { tableview_column_limits(self.as_ptr(), index as _, min, max) }
    }

    /// Sets whether a column is resizable or not.
    pub fn column_resizable(&self, column_id: usize, resizable: bool) {
        unsafe { tableview_column_resizable(self.as_ptr(), column_id as _, resizable as _) }
    }

    /// Allows to freeze the first columns of the table. During horizontal movement they will remain fixed.
    pub fn column_freeze(&self, freeze: usize) {
        unsafe { tableview_column_freeze(self.as_ptr(), freeze as _) }
    }

    /// Sets the text of a column header.
    pub fn header_title(&self, index: usize, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { tableview_header_title(self.as_ptr(), index as _, text.as_ptr()) }
    }

    /// Sets the alignment of the header text.
    pub fn header_align(&self, index: usize, align: Align) {
        unsafe { tableview_header_align(self.as_ptr(), index as _, align as _) }
    }

    /// Sets whether the table header is visible or not.
    pub fn header_visible(&self, visible: bool) {
        unsafe { tableview_header_visible(self.as_ptr(), visible as _) }
    }

    /// Sets whether the table header can be clicked as a button.
    pub fn header_clickable(&self, clickable: bool) {
        unsafe { tableview_header_clickable(self.as_ptr(), clickable as _) }
    }

    /// Sets whether the header allows column resizing.
    pub fn header_resizable(&self, resizable: bool) {
        unsafe { tableview_header_resizable(self.as_ptr(), resizable as _) }
    }

    /// Force the height of the header.
    ///
    /// # Remarks
    /// The height of the header is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. Its use is not recommended.
    /// See Table appearance.
    pub fn header_height(&self, height: f32) {
        unsafe { tableview_header_height(self.as_ptr(), height) }
    }

    /// Force the height of the row.
    ///
    /// # Remarks
    /// The row height is automatically calculated from the content. Forcing
    /// this value may cause the table to not display correctly. its use is not recommended.
    /// See Table appearance.
    pub fn row_height(&self, height: f32) {
        unsafe { tableview_row_height(self.as_ptr(), height) }
    }

    /// Sets the horizontal scrolling when pressing the \[LEFT\] and \[RIGHT\] keys.
    pub fn hkey_scroll(&self, force_column: bool, scoll: f32) {
        unsafe { tableview_hkey_scroll(self.as_ptr(), force_column as _, scoll) }
    }

    /// Sets the row selection mode.
    pub fn multisel(&self, multisel: bool, preserve: bool) {
        unsafe { tableview_multisel(self.as_ptr(), multisel as _, preserve as _) }
    }

    /// Sets the drawing of the interior lines.
    pub fn grid(&self, hlines: bool, vlines: bool) {
        unsafe { tableview_grid(self.as_ptr(), hlines as _, vlines as _) }
    }

    /// Synchronizes the table with the data source.
    ///
    /// # Remarks
    /// See Data connection. We must call this function from the application
    /// whenever the data linked to the table changes, in order to update the view.
    pub fn update(&self) {
        unsafe { tableview_update(self.as_ptr()) }
    }

    /// Selects rows in the table.
    pub fn select(&self, row: &[u32], n: usize) {
        unsafe { tableview_select(self.as_ptr(), row.as_ptr(), n as _) }
    }

    /// Deselects rows in the table.
    pub fn deselect(&self, row: &[u32], n: usize) {
        unsafe { tableview_deselect(self.as_ptr(), row.as_ptr(), n as _) }
    }

    /// Deselects all rows in the table.
    pub fn deselect_all(&self) {
        unsafe { tableview_deselect_all(self.as_ptr()) }
    }

    /// Returns the currently selected rows.
    pub fn selected(&self) -> Option<Vec<usize>> {
        let result = unsafe { tableview_selected(self.as_ptr()) };

        if result.is_null() {
            return None;
        }

        let result = unsafe { *result };

        if result.content.is_null() {
            return None;
        }

        let content = unsafe { *result.content };

        let elem = &content.elem;

        Some(elem.iter().map(|&x| x as _).collect())
    }

    /// Set keyboard focus to a specific row.
    ///
    /// # Remarks
    /// Setting keyboard focus to a row only has effects on navigation, but does not involve
    /// selecting the row. The table is automatically scrolled so that the row is visible.
    /// In this case, align indicates where the vertical scroll is adjusted (up, down or centered).
    pub fn focus_row(&self, row: usize, align: Align) {
        unsafe { tableview_focus_row(self.as_ptr(), row as _, align as _) }
    }

    /// Gets the row that has keyboard focus.
    pub fn get_focus_row(&self) -> usize {
        unsafe { tableview_get_focus_row(self.as_ptr()) as _ }
    }

    /// Show or hide scroll bars.
    pub fn scroll_visible(&self, hscroll: bool, vscroll: bool) {
        unsafe { tableview_scroll_visible(self.as_ptr(), hscroll as _, vscroll as _) }
    }
}

impl_control!(TableView, guicontrol_tableview);
impl_layout!(TableView, layout_tableview);
