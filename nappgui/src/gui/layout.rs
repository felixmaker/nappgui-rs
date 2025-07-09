use std::{
    ffi::{c_void, CString},
    rc::Rc,
};

use nappgui_sys::{
    layout_bgcolor, layout_button, layout_cell, layout_combo, layout_control, layout_create,
    layout_dbind_get_obj_imp, layout_dbind_imp, layout_dbind_obj_imp, layout_edit, layout_halign,
    layout_hexpand, layout_hexpand2, layout_hexpand3, layout_hmargin, layout_hsize,
    layout_imageview, layout_insert_col, layout_insert_row, layout_label, layout_layout,
    layout_listbox, layout_margin, layout_margin2, layout_margin4, layout_ncols, layout_nrows,
    layout_panel, layout_panel_replace, layout_popup, layout_progress, layout_remove_col,
    layout_remove_row, layout_show_col, layout_show_row, layout_skcolor, layout_slider,
    layout_splitview, layout_tableview, layout_taborder, layout_tabstop, layout_textview,
    layout_update, layout_updown, layout_valign, layout_vexpand, layout_vexpand2, layout_vexpand3,
    layout_view, layout_vmargin, layout_vsize, layout_webview,
};

use crate::{
    draw_2d::Color,
    types::{Align, GuiOrient},
    util::macros::pub_crate_ptr_ops,
};

use super::*;

/// A Layout is a virtual and transparent grid always linked with a Panel which serves to locate the different
/// interface elements.
#[derive(Clone)]
pub struct Layout {
    pub(crate) inner: Rc<*mut nappgui_sys::Layout>,
}

impl Layout {
    pub_crate_ptr_ops!(*mut nappgui_sys::Layout);

    /// Create a new layout specifying the number of columns and rows.
    pub fn new(rows: usize, cols: usize) -> Self {
        let layout = unsafe { layout_create(rows as _, cols as _) };
        Self::from_raw(layout)
    }

    /// Get a layout cell.
    pub fn cell(&self, col: usize, row: usize) -> Cell {
        unsafe { Cell::from_raw_no_drop(layout_cell(self.as_ptr(), col as _, row as _)) }
    }

    /// Gets the control assigned to a cell in the layout.
    pub fn control<T>(&self, col: usize, row: usize) -> Option<T>
    where
        T: ControlTrait,
    {
        let control = unsafe { layout_control(self.as_ptr(), col as _, row as _) };
        T::from_control_ptr(control)
    }

    /// Insert a Label control in a layout.
    pub fn label(&self, label: &Label, col: usize, row: usize) {
        unsafe { layout_label(self.as_ptr(), label.as_ptr(), col as _, row as _) };
    }

    /// Insert a Button control in a layout.
    pub fn button<T>(&self, button: &T, col: usize, row: usize)
    where
        T: ButtonTrait,
    {
        unsafe { layout_button(self.as_ptr(), button.as_button_ptr(), col as _, row as _) };
    }

    /// Insert a PopUp control in a layout.
    pub fn popup(&self, popup: &PopUp, col: usize, row: usize) {
        unsafe { layout_popup(self.as_ptr(), popup.as_ptr(), col as _, row as _) };
    }

    /// Insert an Edit control in a layout.
    pub fn edit(&self, edit: &Edit, col: usize, row: usize) {
        unsafe { layout_edit(self.as_ptr(), edit.as_ptr(), col as _, row as _) };
    }

    /// Insert a Combo control in a layout.
    pub fn combo(&self, combo: &Combo, col: usize, row: usize) {
        unsafe { layout_combo(self.as_ptr(), combo.as_ptr(), col as _, row as _) };
    }

    /// Insert a ListBox control in a layout.
    pub fn listbox(&self, listbox: &ListBox, col: usize, row: usize) {
        unsafe { layout_listbox(self.as_ptr(), listbox.as_ptr(), col as _, row as _) };
    }

    /// Insert an UpDown control in a layout.
    pub fn updown(&self, updown: &UpDown, col: usize, row: usize) {
        unsafe { layout_updown(self.as_ptr(), updown.as_ptr(), col as _, row as _) };
    }

    /// Insert an Slider control in a layout.
    pub fn slider(&self, slider: &Slider, col: usize, row: usize) {
        unsafe { layout_slider(self.as_ptr(), slider.as_ptr(), col as _, row as _) };
    }

    /// Insert a Progress control in a layout.
    pub fn progress(&self, progress: &Progress, col: usize, row: usize) {
        unsafe { layout_progress(self.as_ptr(), progress.as_ptr(), col as _, row as _) };
    }

    /// Insert View in a layout.
    pub fn view(&self, view: &View, col: usize, row: usize) {
        unsafe { layout_view(self.as_ptr(), view.as_ptr(), col as _, row as _) };
    }

    /// Insert a TextView control in a layout.
    pub fn textview(&self, textview: &TextView, col: usize, row: usize) {
        unsafe { layout_textview(self.as_ptr(), textview.as_ptr(), col as _, row as _) };
    }

    /// Insert a WebView control in a layout.
    pub fn webview(&self, webview: &WebView, col: usize, row: usize) {
        unsafe { layout_webview(self.as_ptr(), webview.as_ptr(), col as _, row as _) };
    }

    /// Insert an ImageView control in a layout.
    pub fn imageview(&self, imageview: &ImageView, col: usize, row: usize) {
        unsafe { layout_imageview(self.as_ptr(), imageview.as_ptr(), col as _, row as _) };
    }

    /// Insert an TableView control in a layout.
    pub fn tableview(&self, tableview: &TableView, col: usize, row: usize) {
        unsafe { layout_tableview(self.as_ptr(), tableview.as_ptr(), col as _, row as _) };
    }

    /// Insert an SplitView control in a layout.
    pub fn splitview(&self, splitview: &SplitView, col: usize, row: usize) {
        unsafe { layout_splitview(self.as_ptr(), splitview.as_ptr(), col as _, row as _) };
    }

    /// Insert a Panel control in a layout.
    pub fn panel(&self, panel: &Panel, col: usize, row: usize) {
        unsafe { layout_panel(self.as_ptr(), panel.as_ptr(), col as _, row as _) };
    }

    /// Replaces one Panel in a layout with another.
    ///
    /// # Remarks
    /// In cell (col,row) there must previously exist a panel that will be destroyed,
    /// without the possibility of recovering it. See Replacing panels.
    pub fn panel_replace(&self, panel: &Panel, col: usize, row: usize) {
        unsafe { layout_panel_replace(self.as_ptr(), panel.as_ptr(), col as _, row as _) };
    }

    /// Insert a layout into a cell in another layout.
    pub fn layout(&self, layout: &Layout, col: usize, row: usize) {
        unsafe { layout_layout(self.as_ptr(), layout.as_ptr(), col as _, row as _) };
    }

    /// Gets the number of columns in the layout.
    pub fn ncols(&self) -> usize {
        unsafe { layout_ncols(self.as_ptr()) as _ }
    }

    /// Gets the number of rows in the layout.
    pub fn nrows(&self) -> usize {
        unsafe { layout_nrows(self.as_ptr()) as _ }
    }

    /// Insert a new column into the layout.
    ///
    /// # Remarks
    /// Empty cells are inserted that will not affect the layout of the window.
    pub fn insert_col(&self, col: usize) {
        unsafe { layout_insert_col(self.as_ptr(), col as _) };
    }

    /// Insert a new row into the layout.
    ///
    /// # Remarks
    /// Empty cells are inserted that will not affect the layout of the window.
    pub fn insert_row(&self, row: usize) {
        unsafe { layout_insert_row(self.as_ptr(), row as _) };
    }

    /// Deletes an existing column in the layout.
    ///
    /// # Remarks
    /// All cell content (controls/sub-layouts) is irreversibly deleted.
    pub fn remove_col(&self, col: usize) {
        unsafe { layout_remove_col(self.as_ptr(), col as _) };
    }

    /// Deletes an existing row in the layout.
    ///
    /// # Remarks
    /// All cell content (controls/sub-layouts) is irreversibly deleted.
    pub fn remove_row(&self, row: usize) {
        unsafe { layout_remove_row(self.as_ptr(), row as _) };
    }

    /// Set how the keyboard focus will move when you press \[TAB\].
    pub fn taborder(&self, taborder: GuiOrient) {
        unsafe { layout_taborder(self.as_ptr(), taborder as _) };
    }

    /// Sets whether or not a cell in the layout will receive keyboard focus when navigating
    /// with \[TAB\]-\[SHIFT\]\[TAB\].
    pub fn tabstop(&self, col: u32, row: u32, tabstop: bool) {
        unsafe { layout_tabstop(self.as_ptr(), col, row, tabstop as _) };
    }

    /// Set a fixed width for a layout column.
    pub fn hsize(&self, col: usize, width: f32) {
        unsafe { layout_hsize(self.as_ptr(), col as _, width) };
    }

    /// Set a fixed height for a layout row.
    pub fn vsize(&self, row: usize, height: f32) {
        unsafe { layout_vsize(self.as_ptr(), row as _, height) };
    }

    /// Establish an inter-column margin within the layout. It is the separation between two
    /// consecutive columns.
    pub fn hmargin(&self, col: usize, margin: f32) {
        unsafe { layout_hmargin(self.as_ptr(), col as _, margin) };
    }

    /// Set an inter-row margin within the layout. It is the separation between two consecutive rows.
    pub fn vmargin(&self, row: usize, margin: f32) {
        unsafe { layout_vmargin(self.as_ptr(), row as _, margin) };
    }

    /// Set the column to expand horizontally.
    pub fn hexpand(&self, col: usize) {
        unsafe { layout_hexpand(self.as_ptr(), col as _) };
    }

    /// Set the two columns that will expand horizontally.
    ///
    /// # Remarks
    /// The expansion of col2 = 1 - exp.
    pub fn hexpand2(&self, col1: usize, col2: usize, exp: f32) {
        unsafe { layout_hexpand2(self.as_ptr(), col1 as _, col2 as _, exp) };
    }

    /// Set the three columns that will expand horizontally.
    ///
    /// # Remarks
    /// exp1 + exp2 < = 1. The expansion of col3 = 1 - exp1 - exp2.
    pub fn hexpand3(&self, col1: usize, col2: usize, col3: usize, exp1: f32, exp2: f32) {
        unsafe { layout_hexpand3(self.as_ptr(), col1 as _, col2 as _, col3 as _, exp1, exp2) };
    }

    /// Set the row that will expand vertically.
    pub fn vexpand(&self, row: usize) {
        unsafe { layout_vexpand(self.as_ptr(), row as _) };
    }

    /// Set the two rows that will expand vertically.
    ///
    /// # Remarks
    /// The expansion of row2 = 1 - exp.
    pub fn vexpand2(&self, row1: usize, row2: usize, exp: f32) {
        unsafe { layout_vexpand2(self.as_ptr(), row1 as _, row2 as _, exp) };
    }

    /// Set the three rows that will expand vertically.
    ///
    /// # Remarks
    /// exp1 + exp2 < = 1. The expansion of row3 = 1 - exp1 - exp2.
    pub fn vexpand3(&self, row1: usize, row2: usize, row3: usize, exp1: f32, exp2: f32) {
        unsafe { layout_vexpand3(self.as_ptr(), row1 as _, row2 as _, row3 as _, exp1, exp2) };
    }

    /// Sets the horizontal alignment of a cell. It will take effect when the column is
    /// wider than the cell.
    pub fn halign(&self, col: usize, row: usize, align: Align) {
        unsafe { layout_halign(self.as_ptr(), col as _, row as _, align as _) };
    }

    /// Sets the vertical alignment of a cell. It will take effect when the row is
    /// taller than the cell.
    pub fn valign(&self, col: usize, row: usize, align: Align) {
        unsafe { layout_valign(self.as_ptr(), col as _, row as _, align as _) };
    }

    /// Show or hide a layout column.
    pub fn show_col(&self, col: usize, visible: bool) {
        unsafe { layout_show_col(self.as_ptr(), col as _, visible as _) };
    }

    /// Show or hide a layout row.
    pub fn show_row(&self, row: usize, visible: bool) {
        unsafe { layout_show_row(self.as_ptr(), row as _, visible as _) };
    }

    /// Set a uniform margin for the layout border.
    pub fn margin(&self, margin: f32) {
        unsafe { layout_margin(self.as_ptr(), margin) };
    }

    /// Set a horizontal and vertical margin for the layout edge.
    pub fn margin2(&self, mtb: f32, mlr: f32) {
        unsafe { layout_margin2(self.as_ptr(), mtb, mlr) };
    }

    /// Set margins for the layout border.
    pub fn margin4(&self, mt: f32, mb: f32, ml: f32, mr: f32) {
        unsafe { layout_margin4(self.as_ptr(), mt, mb, ml, mr) };
    }

    /// Assign a background color to the layout.
    pub fn bgcolor(&self, color: Color) {
        unsafe { layout_bgcolor(self.as_ptr(), color.inner) };
    }

    /// Assign a color to the edge of the layout.
    pub fn skcolor(&self, color: Color) {
        unsafe { layout_skcolor(self.as_ptr(), color.inner) };
    }

    /// Update the window associated with the layout.
    pub fn update(&self) {
        unsafe { layout_update(self.as_ptr()) };
    }

    /// Associate a type struct with a layout.
    pub fn dbind_imp(&self, type_: &str, size: u16) {
        let type_ = CString::new(type_).unwrap();
        unsafe {
            layout_dbind_imp(self.as_ptr(), std::ptr::null_mut(), type_.as_ptr(), size);
        }
    }

    /// Associate an object with a layout to view and edit it.
    pub fn dbind_obj_imp(&self, obj: *mut c_void, type_: &str) {
        let type_ = CString::new(type_).unwrap();

        unsafe {
            layout_dbind_obj_imp(self.as_ptr(), obj, type_.as_ptr());
        }
    }

    /// Gets the object associated with a layout.
    pub fn dbind_get_obj_imp(&self, type_: &str) -> *mut c_void {
        let type_ = CString::new(type_).unwrap();
        unsafe { layout_dbind_get_obj_imp(self.as_ptr(), type_.as_ptr()) }
    }
}

/// Associates a cell with the field of a struct.
#[macro_export]
macro_rules! layout_dbind {
    ($layout: expr, $struct: ty) => {
        nappgui::gui::Layout::dbind_imp($layout, stringify!($struct), size_of::<$struct>() as _)
    };
}

/// Associate an object with a layout to view and edit it.
#[macro_export]
macro_rules! layout_dbind_obj {
    ($layout: expr, $obj: expr, $type: ty) => {
        nappgui::gui::Layout::dbind_obj_imp($layout, $obj, stringify!($type))
    };
}
