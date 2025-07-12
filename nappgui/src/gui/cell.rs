use std::ffi::CString;

use nappgui_sys::{
    cell_control, cell_dbind_imp, cell_empty, cell_enabled, cell_padding, cell_padding2,
    cell_padding4, cell_visible,
};

use crate::gui::control::ControlTrait;

/// The cell trait.
pub trait CellTrait {
    /// Returns a raw pointer to the cell object.
    fn as_ptr(&self) -> *mut nappgui_sys::Cell;

    /// Check if the cell is empty.
    fn empty(&self) -> bool {
        unsafe { cell_empty(self.as_ptr()) != 0 }
    }

    /// Get control of the inside of the cell.
    fn control<T>(&self) -> Option<T>
    where
        T: ControlTrait,
    {
        let ptr = unsafe { cell_control(self.as_ptr()) };
        T::from_control_ptr(ptr)
    }

    /// Activate or deactivate a cell.
    fn enabled(&self, enabled: bool) {
        unsafe { cell_enabled(self.as_ptr(), enabled as _) }
    }

    /// Show or hide a cell.
    ///
    /// # Remarks
    /// If the cell contains a sublayout, the command will affect all controls recursively.
    fn visible(&self, visible: bool) {
        unsafe { cell_visible(self.as_ptr(), visible as _) }
    }

    /// Set an inner margin.
    fn padding(&self, pall: f32) {
        unsafe {
            cell_padding(self.as_ptr(), pall);
        }
    }

    /// Set an inner margin.
    fn padding2(&self, pleft: f32, pright: f32) {
        unsafe {
            cell_padding2(self.as_ptr(), pleft, pright);
        }
    }

    /// Set an inner margin.
    fn padding4(&self, pleft: f32, ptop: f32, pright: f32, pbottom: f32) {
        unsafe {
            cell_padding4(self.as_ptr(), pleft, ptop, pright, pbottom);
        }
    }

    /// Associates a cell with the field of a struct.
    fn dbind_imp(
        &self,
        type_: &str,
        size: u16,
        mname: &str,
        mtype: &str,
        moffset: u16,
        msize: u16,
    ) {
        let type_ = CString::new(type_).unwrap();
        let mname = CString::new(mname).unwrap();
        let mtype = CString::new(mtype).unwrap();
        unsafe {
            cell_dbind_imp(
                self.as_ptr(),
                type_.as_ptr(),
                size,
                mname.as_ptr(),
                mtype.as_ptr(),
                moffset,
                msize,
            );
        }
    }
}

/// Cells are the inner elements of a Layout and will house a control or a sublayout.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub(crate) inner: *mut nappgui_sys::Cell,
}

impl CellTrait for Cell {
    fn as_ptr(&self) -> *mut nappgui_sys::Cell {
        self.inner
    }
}

/// Associates a cell with the field of a struct.
#[macro_export]
macro_rules! cell_dbind {
    ($cell: expr, $struct: ty, $field: ident, $field_type: ty, $bind_type: literal) => {
        nappgui::gui::Cell::dbind_imp(
            $cell,
            stringify!($struct),
            size_of::<$struct>() as _,
            stringify!($field),
            $bind_type,
            offset_of!($struct, $field) as _,
            size_of::<$field_type>() as _,
        )
    };
}
