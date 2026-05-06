use std::ffi::CString;

use nappgui_sys::{
    layout_bgcolor, layout_cell, layout_control, layout_create, layout_group, layout_halign, layout_hexpandn, layout_hmargin, layout_hsize, layout_insert_col,
    layout_insert_row, layout_margin4, layout_ncols, layout_nrows, layout_panel_replace,
    layout_remove_col, layout_remove_row, layout_show_col, layout_show_row, layout_skcolor, layout_taborder,
    layout_tabstop, layout_update, layout_valign, layout_vexpandn,
    layout_vmargin, layout_vsize,
};

use crate::{
    draw_2d::Color,
    types::{Align, GuiOrient},
};

use super::*;

/// The layout.
///
/// # Remarks
/// If the layout is not attached to a panel, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Layout(*mut nappgui_sys::Layout);

impl Layout {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Layout) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }

    /// Gets the raw pointer to the layout.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Layout {
        self.0
    }

    /// Creates a new layout.
    pub fn new(ncols: u32, nrows: u32) -> Self {
        let layout = unsafe { layout_create(ncols, nrows) };
        unsafe { Self::from_raw(layout) }
    }

    /// Gets the number of columns in the layout.
    pub fn ncols(&self) -> u32 {
        unsafe { layout_ncols(self.as_ptr()) }
    }

    /// Gets the number of rows in the layout.
    pub fn nrows(&self) -> u32 {
        unsafe { layout_nrows(self.as_ptr()) }
    }

    /// Get a layout cell.
    pub fn cell(&self, col: u32, row: u32) -> LayoutCell<'_> {
        let cell = unsafe { layout_cell(self.as_ptr(), col, row) };
        unsafe { LayoutCell::from_raw(cell) }
    }

    /// Gets the control assigned to a cell in the layout.
    pub fn control<T>(&self, col: u32, row: u32) -> Option<T>
    where
        T: Control,
    {
        let control = unsafe { layout_control(self.as_ptr(), col, row) };
        T::from_control_ptr(control)
    }

    /// Insert a control to the layout.
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    pub fn set_control<T>(&self, col: u32, row: u32, control: &T)
    where
        T: LayoutControl + Control,
    {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        control.insert_in_layout(self, col, row);
        // global_move_ownership(control.as_control_ptr() as _, self.as_ptr() as _);
    }

    /// Replaces one Panel in a layout with another.
    ///
    /// # Remarks
    /// In cell (col,row) there must previously exist a panel that will be destroyed,
    /// without the possibility of recovering it. See Replacing panels.
    ///
    /// TODO!
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    pub unsafe fn panel_replace<T>(&self, panel: &Panel, col: u32, row: u32) {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_panel_replace(self.as_ptr(), panel.as_ptr(), col, row) }
    }

    /// Insert a new column into the layout.
    ///
    /// # Remarks
    ///
    /// Empty cells are inserted that will not affect the layout of the window.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    pub fn insert_column(&self, col: u32) {
        assert!(col < self.ncols());

        unsafe { layout_insert_col(self.as_ptr(), col) };
    }

    /// Insert a new row into the layout.
    ///
    /// # Remarks
    ///
    /// Empty cells are inserted that will not affect the layout of the window.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    pub fn insert_row(&self, row: u32) {
        assert!(row < self.nrows());

        unsafe { layout_insert_row(self.as_ptr(), row) };
    }

    /// Deletes an existing column in the layout.
    ///
    /// # Remarks
    ///
    /// All cell content (controls/sub-layouts) is irreversibly deleted.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    pub fn remove_column(&self, col: u32) {
        assert!(col < self.ncols());

        unsafe { layout_remove_col(self.as_ptr(), col) };
    }

    /// Deletes an existing row in the layout.
    ///
    /// # Remarks
    /// All cell content (controls/sub-layouts) is irreversibly deleted.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    pub fn remove_row(&self, row: u32) {
        assert!(row < self.nrows());

        unsafe { layout_remove_row(self.as_ptr(), row) };
    }

    /// Set how the keyboard focus will move when you press \[TAB\].
    pub fn set_taborder(&self, taborder: GuiOrient) {
        unsafe { layout_taborder(self.as_ptr(), taborder as _) };
    }

    /// Sets whether or not a cell in the layout will receive keyboard focus when navigating
    /// with \[TAB\]-\[SHIFT\]\[TAB\].
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    pub fn set_tabstop(&self, col: u32, row: u32, tabstop: bool) {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_tabstop(self.as_ptr(), col, row, tabstop as _) };
    }

    /// Set a fixed width for a layout column.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    pub fn set_horizontal_size(&self, col: u32, width: f32) {
        assert!(col < self.ncols());

        unsafe { layout_hsize(self.as_ptr(), col, width) };
    }

    /// Set a fixed height for a layout row.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    pub fn set_vertical_size(&self, row: u32, height: f32) {
        assert!(row < self.nrows());

        unsafe { layout_vsize(self.as_ptr(), row, height) };
    }

    /// Establish an inter-column margin within the layout. It is the separation between two
    /// consecutive columns.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    pub fn set_horizontal_margin(&self, col: u32, margin: f32) {
        assert!(col < self.ncols());

        unsafe { layout_hmargin(self.as_ptr(), col, margin) };
    }

    /// Set an inter-row margin within the layout. It is the separation between two consecutive rows.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    pub fn set_vertical_margin(&self, row: u32, margin: f32) {
        assert!(row < self.nrows());

        unsafe { layout_vmargin(self.as_ptr(), row, margin) };
    }

    /// Set the column to expand horizontally.
    ///
    /// # Remarks
    ///
    /// cols: The columns to expand horizontally.
    /// weights: The weights of the columns. len(weights) = len(cols) - 1.
    pub fn set_horizontal_expand(&self, cols: &[u32], weights: &[f32]) {
        assert!(cols.len() > 0);
        assert_eq!(cols.len() - 1, weights.len());

        for col in cols {
            assert!(*col < self.ncols());
        }

        unsafe { layout_hexpandn(self.as_ptr(), cols.len() as _, cols.as_ptr(), weights.as_ptr()) };
    }

    /// Set the row that will expand vertically.
    ///
    ///
    /// rows: The rows to expand vertically.
    /// weights: The weights of the rows. len(weights) = len(rows) - 1.
    pub fn set_vertical_expand(&self, rows: &[u32], weights: &[f32]) {
        assert!(rows.len() > 0);
        assert_eq!(rows.len() - 1, weights.len());

        for row in rows {
            assert!(*row < self.nrows());
        }

        unsafe { layout_vexpandn(self.as_ptr(), rows.len() as _, rows.as_ptr(), weights.as_ptr()) };
    }

    /// Sets the horizontal alignment of a cell. It will take effect when the column is
    /// wider than the cell.
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    pub fn set_horizontal_align(&self, col: u32, row: u32, align: Align) {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_halign(self.as_ptr(), col, row, align as _) };
    }

    /// Sets the vertical alignment of a cell. It will take effect when the row is
    /// taller than the cell.
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    pub fn set_vertical_align(&self, col: u32, row: u32, align: Align) {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_valign(self.as_ptr(), col, row, align as _) };
    }

    /// Show or hide a layout column.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    pub fn set_column_visible(&self, col: u32, visible: bool) {
        assert!(col < self.ncols());

        unsafe { layout_show_col(self.as_ptr(), col, visible as _) };
    }

    /// Show or hide a layout row.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    pub fn set_row_visible(&self, row: u32, visible: bool) {
        assert!(row < self.nrows());

        unsafe { layout_show_row(self.as_ptr(), row, visible as _) };
    }

    /// Set margins for the layout border.
    pub fn set_margin(&self, top: f32, right: f32, button: f32, left: f32) {
        unsafe { layout_margin4(self.as_ptr(), top, right, button, left) };
    }

    /// Assign a background color to the layout.
    pub fn set_background_color(&self, color: Color) {
        unsafe { layout_bgcolor(self.as_ptr(), color.inner) };
    }

    /// Assign a color to the edge of the layout.
    pub fn set_border_color(&self, color: Color) {
        unsafe { layout_skcolor(self.as_ptr(), color.inner) };
    }

    /// Sets a GroupBox type decoration around the layout.
    pub fn set_group(&self, group: bool, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { layout_group(self.as_ptr(), group as _, text.as_ptr()) };
    }

    /// Update the window associated with the layout.
    pub fn update(&self) {
        unsafe { layout_update(self.as_ptr()) };
    }

    // /// Associate a type struct with a layout.
    // fn dbind_imp(&self, type_: &str, size: u16) {
    //     let type_ = CString::new(type_).unwrap();
    //     unsafe {
    //         layout_dbind_imp(self.as_ptr(), std::ptr::null_mut(), type_.as_ptr(), size);
    //     }
    // }

    // /// Associate an object with a layout to view and edit it.
    // fn dbind_obj_imp(&self, obj: *mut c_void, type_: &str) {
    //     let type_ = CString::new(type_).unwrap();

    //     unsafe {
    //         layout_dbind_obj_imp(self.as_ptr(), obj, type_.as_ptr());
    //     }
    // }

    // /// Gets the object associated with a layout.
    // fn dbind_get_obj_imp(&self, type_: &str) -> *mut c_void {
    //     let type_ = CString::new(type_).unwrap();
    //     unsafe { layout_dbind_get_obj_imp(self.as_ptr(), type_.as_ptr()) }
    // }
}

// /// A Layout is a virtual and transparent grid always linked with a Panel which serves to locate the different
// /// interface elements.
// ///
// /// # Remark
// /// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
// /// components assciated with it will be automatically released.
// #[repr(transparent)]
// #[derive(Clone, Copy, Debug)]
// pub struct Layout {
//     pub(crate) inner: *mut nappgui_sys::Layout,
// }

// impl LayoutTrait for Layout {
//     fn as_ptr(&self) -> *mut nappgui_sys::Layout {
//         self.inner
//     }
// }

// impl Layout {
//     /// Create a new layout specifying the number of columns and rows.
//     pub fn new(ncols: u32, nrows: u32) -> Self {
//         let layout = unsafe { layout_create(ncols as _, nrows as _) };
//         Self { inner: layout }
//     }
// }

// /// Associates a cell with the field of a struct.
// #[macro_export]
// macro_rules! layout_dbind {
//     ($layout: expr, $struct: ty) => {
//         nappgui::gui::Layout::dbind_imp($layout, stringify!($struct), size_of::<$struct>() as _)
//     };
// }

// /// Associate an object with a layout to view and edit it.
// #[macro_export]
// macro_rules! layout_dbind_obj {
//     ($layout: expr, $obj: expr, $type: ty) => {
//         nappgui::gui::Layout::dbind_obj_imp($layout, $obj, stringify!($type))
//     };
// }

/// Define how controls are laid out in a layout.
pub trait LayoutControl {
    /// Insert the control to the layout
    fn insert_in_layout(&self, layout: &Layout, col: u32, row: u32);
}

macro_rules! impl_layout {
    ($type: ty, $func: ident) => {
        impl crate::gui::LayoutControl for $type {
            fn insert_in_layout(&self, layout: &Layout, col: u32, row: u32) {
                unsafe {
                    nappgui_sys::$func(layout.as_ptr(), self.as_ptr(), col, row);
                }
            }
        }
    };
}

impl_layout!(Layout, layout_layout);
impl_layout!(Button, layout_button);
impl_layout!(Combo, layout_combo);
impl_layout!(Edit, layout_edit);
impl_layout!(ImageView, layout_imageview);
impl_layout!(Label, layout_label);
impl_layout!(Panel, layout_panel);
impl_layout!(ListBox, layout_listbox);
impl_layout!(PopUp, layout_popup);
impl_layout!(Progress, layout_progress);
impl_layout!(Slider, layout_slider);
impl_layout!(SplitView, layout_splitview);
impl_layout!(TableView, layout_tableview);
impl_layout!(TextView, layout_textview);
impl_layout!(UpDown, layout_updown);
impl_layout!(View, layout_view);
impl_layout!(WebView, layout_webview);
