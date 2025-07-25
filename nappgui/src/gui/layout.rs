use std::ffi::{c_void, CString};

use nappgui_sys::{
    layout_bgcolor, layout_cell, layout_control, layout_create, layout_dbind_get_obj_imp,
    layout_dbind_imp, layout_dbind_obj_imp, layout_halign, layout_hexpand, layout_hexpand2,
    layout_hexpand3, layout_hmargin, layout_hsize, layout_insert_col, layout_insert_row,
    layout_margin, layout_margin2, layout_margin4, layout_ncols, layout_nrows,
    layout_panel_replace, layout_remove_col, layout_remove_row, layout_show_col, layout_show_row,
    layout_skcolor, layout_taborder, layout_tabstop, layout_update, layout_valign, layout_vexpand,
    layout_vexpand2, layout_vexpand3, layout_vmargin, layout_vsize,
};

use crate::{
    draw_2d::Color,
    types::{Align, GuiOrient},
};

use super::*;

/// The layout trait.
pub trait LayoutTrait {
    /// Returns a raw pointer to the layout object.
    fn as_ptr(&self) -> *mut nappgui_sys::Layout;

    /// Get a layout cell.
    fn cell(&self, col: u32, row: u32) -> Cell {
        Cell {
            inner: unsafe { layout_cell(self.as_ptr(), col, row) },
        }
    }

    /// Gets the control assigned to a cell in the layout.
    fn get<T>(&self, col: u32, row: u32) -> Option<T>
    where
        T: ControlTrait,
    {
        let control = unsafe { layout_control(self.as_ptr(), col, row) };
        T::from_control_ptr(control)
    }

    /// Insert a control to the layout.
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    fn set<T>(&self, col: u32, row: u32, control: T)
    where
        T: ControlLayoutTrait,
        Self: Sized + Copy,
    {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        control.insert_in_layout(*self, col, row);
    }

    /// Replaces one Panel in a layout with another.
    ///
    /// # Remarks
    /// In cell (col,row) there must previously exist a panel that will be destroyed,
    /// without the possibility of recovering it. See Replacing panels.
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    fn panel_replace<T>(&self, panel: T, col: u32, row: u32)
    where
        T: PanelTrait,
    {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_panel_replace(self.as_ptr(), panel.as_ptr(), col, row) };
    }

    /// Gets the number of columns in the layout.
    fn ncols(&self) -> u32 {
        unsafe { layout_ncols(self.as_ptr()) }
    }

    /// Gets the number of rows in the layout.
    fn nrows(&self) -> u32 {
        unsafe { layout_nrows(self.as_ptr()) }
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
    fn insert_column(&self, col: u32) {
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
    fn insert_row(&self, row: u32) {
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
    fn remove_column(&self, col: u32) {
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
    fn remove_row(&self, row: u32) {
        assert!(row < self.nrows());

        unsafe { layout_remove_row(self.as_ptr(), row) };
    }

    /// Set how the keyboard focus will move when you press \[TAB\].
    fn taborder(&self, taborder: GuiOrient) {
        unsafe { layout_taborder(self.as_ptr(), taborder as _) };
    }

    /// Sets whether or not a cell in the layout will receive keyboard focus when navigating
    /// with \[TAB\]-\[SHIFT\]\[TAB\].
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    fn tabstop(&self, col: u32, row: u32, tabstop: bool) {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_tabstop(self.as_ptr(), col, row, tabstop as _) };
    }

    /// Set a fixed width for a layout column.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    fn horizontal_size(&self, col: u32, width: f32) {
        assert!(col < self.ncols());

        unsafe { layout_hsize(self.as_ptr(), col, width) };
    }

    /// Set a fixed height for a layout row.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    fn vertical_size(&self, row: u32, height: f32) {
        assert!(row < self.nrows());

        unsafe { layout_vsize(self.as_ptr(), row, height) };
    }

    /// Establish an inter-column margin within the layout. It is the separation between two
    /// consecutive columns.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    fn horizontal_margin(&self, col: u32, margin: f32) {
        assert!(col < self.ncols());

        unsafe { layout_hmargin(self.as_ptr(), col, margin) };
    }

    /// Set an inter-row margin within the layout. It is the separation between two consecutive rows.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    fn vertical_margin(&self, row: u32, margin: f32) {
        assert!(row < self.nrows());

        unsafe { layout_vmargin(self.as_ptr(), row, margin) };
    }

    /// Set the column to expand horizontally.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    fn horizontal_expand(&self, col: u32) {
        assert!(col < self.ncols());

        unsafe { layout_hexpand(self.as_ptr(), col) };
    }

    /// Set the two columns that will expand horizontally.
    ///
    /// # Remarks
    /// The expansion of col2 = 1 - exp.
    ///
    /// # Panics
    ///
    /// Panics if col1 or col2 is out of bounds.
    fn horizontal_expand2(&self, col1: u32, col2: u32, exp: f32) {
        assert!(col1 < self.ncols());
        assert!(col2 < self.ncols());

        unsafe { layout_hexpand2(self.as_ptr(), col1 as _, col2 as _, exp) };
    }

    /// Set the three columns that will expand horizontally.
    ///
    /// # Remarks
    /// exp1 + exp2 < = 1. The expansion of col3 = 1 - exp1 - exp2.
    ///
    /// # Panics
    ///
    /// Panics if col1 or col2 or col3 is out of bounds.
    fn horizontal_expand3(&self, col1: u32, col2: u32, col3: u32, exp1: f32, exp2: f32) {
        assert!(col1 < self.ncols());
        assert!(col2 < self.ncols());
        assert!(col3 < self.ncols());

        unsafe { layout_hexpand3(self.as_ptr(), col1 as _, col2 as _, col3 as _, exp1, exp2) };
    }

    /// Set the row that will expand vertically.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    fn vertical_expand(&self, row: u32) {
        assert!(row < self.nrows());

        unsafe { layout_vexpand(self.as_ptr(), row) };
    }

    /// Set the two rows that will expand vertically.
    ///
    /// # Remarks
    /// The expansion of row2 = 1 - exp.
    ///
    /// # Panics
    ///
    /// Panics if row1 or row2 is out of bounds.
    fn vertical_expand2(&self, row1: u32, row2: u32, exp: f32) {
        assert!(row1 < self.nrows());
        assert!(row2 < self.nrows());

        unsafe { layout_vexpand2(self.as_ptr(), row1 as _, row2 as _, exp) };
    }

    /// Set the three rows that will expand vertically.
    ///
    /// # Remarks
    /// exp1 + exp2 < = 1. The expansion of row3 = 1 - exp1 - exp2.
    ///
    /// # Panics
    ///
    /// Panics if row1 or row2 or row3 is out of bounds.
    fn vertical_expand3(&self, row1: u32, row2: u32, row3: u32, exp1: f32, exp2: f32) {
        assert!(row1 < self.nrows());
        assert!(row2 < self.nrows());
        assert!(row3 < self.nrows());

        unsafe { layout_vexpand3(self.as_ptr(), row1 as _, row2 as _, row3 as _, exp1, exp2) };
    }

    /// Sets the horizontal alignment of a cell. It will take effect when the column is
    /// wider than the cell.
    ///
    /// # Panics
    ///
    /// Panics if col or row is out of bounds.
    fn horizontal_align(&self, col: u32, row: u32, align: Align) {
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
    fn vertical_align(&self, col: u32, row: u32, align: Align) {
        assert!(col < self.ncols());
        assert!(row < self.nrows());

        unsafe { layout_valign(self.as_ptr(), col, row, align as _) };
    }

    /// Show or hide a layout column.
    ///
    /// # Panics
    ///
    /// Panics if col is out of bounds.
    fn show_column(&self, col: u32, visible: bool) {
        assert!(col < self.ncols());

        unsafe { layout_show_col(self.as_ptr(), col, visible as _) };
    }

    /// Show or hide a layout row.
    ///
    /// # Panics
    ///
    /// Panics if row is out of bounds.
    fn show_row(&self, row: u32, visible: bool) {
        assert!(row < self.nrows());

        unsafe { layout_show_row(self.as_ptr(), row, visible as _) };
    }

    /// Set a uniform margin for the layout border.
    fn margin(&self, margin: f32) {
        unsafe { layout_margin(self.as_ptr(), margin) };
    }

    /// Set a horizontal and vertical margin for the layout edge.
    fn margin2(&self, mtb: f32, mlr: f32) {
        unsafe { layout_margin2(self.as_ptr(), mtb, mlr) };
    }

    /// Set margins for the layout border.
    fn margin4(&self, mt: f32, mb: f32, ml: f32, mr: f32) {
        unsafe { layout_margin4(self.as_ptr(), mt, mb, ml, mr) };
    }

    /// Assign a background color to the layout.
    fn background_color(&self, color: Color) {
        unsafe { layout_bgcolor(self.as_ptr(), color.inner) };
    }

    /// Assign a color to the edge of the layout.
    fn skin_color(&self, color: Color) {
        unsafe { layout_skcolor(self.as_ptr(), color.inner) };
    }

    /// Update the window associated with the layout.
    fn update(&self) {
        unsafe { layout_update(self.as_ptr()) };
    }

    /// Associate a type struct with a layout.
    fn dbind_imp(&self, type_: &str, size: u16) {
        let type_ = CString::new(type_).unwrap();
        unsafe {
            layout_dbind_imp(self.as_ptr(), std::ptr::null_mut(), type_.as_ptr(), size);
        }
    }

    /// Associate an object with a layout to view and edit it.
    fn dbind_obj_imp(&self, obj: *mut c_void, type_: &str) {
        let type_ = CString::new(type_).unwrap();

        unsafe {
            layout_dbind_obj_imp(self.as_ptr(), obj, type_.as_ptr());
        }
    }

    /// Gets the object associated with a layout.
    fn dbind_get_obj_imp(&self, type_: &str) -> *mut c_void {
        let type_ = CString::new(type_).unwrap();
        unsafe { layout_dbind_get_obj_imp(self.as_ptr(), type_.as_ptr()) }
    }
}

/// A Layout is a virtual and transparent grid always linked with a Panel which serves to locate the different
/// interface elements.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub(crate) inner: *mut nappgui_sys::Layout,
}

impl LayoutTrait for Layout {
    fn as_ptr(&self) -> *mut nappgui_sys::Layout {
        self.inner
    }
}

impl Layout {
    /// Create a new layout specifying the number of columns and rows.
    pub fn new(ncols: u32, nrows: u32) -> Self {
        let layout = unsafe { layout_create(ncols as _, nrows as _) };
        Self { inner: layout }
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

/// Define how controls are laid out in a layout.
pub trait ControlLayoutTrait {
    /// Insert the control to the layout
    fn insert_in_layout<T>(&self, layout: T, col: u32, row: u32)
    where
        T: LayoutTrait + Sized;
}

macro_rules! impl_layout {
    ($type: ty, $trait: ident, $func: ident) => {
        impl crate::gui::ControlLayoutTrait for $type
        where
            $type: $trait,
        {
            fn insert_in_layout<T>(&self, layout: T, col: u32, row: u32)
            where
                T: crate::gui::LayoutTrait,
            {
                unsafe {
                    nappgui_sys::$func(layout.as_ptr(), self.as_ptr(), col, row);
                }
            }
        }
    };
}

pub(crate) use impl_layout;

impl_layout!(Layout, LayoutTrait, layout_layout);
