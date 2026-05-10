use std::{marker::PhantomData, ptr::NonNull};

use nappgui_sys::{cell_control, cell_empty, cell_enabled, cell_padding, cell_padding2, cell_padding4, cell_visible};

use crate::gui::{control_uid, Control, Layout};

/// Represents a cell in a layout.
#[repr(transparent)]
pub struct LayoutCell<'a> {
    ptr: NonNull<nappgui_sys::Cell>,
    _marker: PhantomData<&'a Layout>,
}

impl<'a> LayoutCell<'a> {
    /// Create a cell from a pointer.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Cell) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to LayoutCell::from_raw"),
            _marker: PhantomData,
        }
    }

    /// Returns a raw pointer to the cell object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Cell {
        self.ptr.as_ptr()
    }

    /// Check if the cell is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { cell_empty(self.as_ptr()) != 0 }
    }

    /// Get control of the inside of the cell.
    ///
    /// # Remarks
    /// If the cell is empty or contains a sublayout, this function will return `None`.
    pub fn control<T>(&self) -> Option<T>
    where
        T: Control,
    {
        let ptr = unsafe { cell_control(self.as_ptr()) };
        let _uid = control_uid(ptr)?;
        // global_get(uid)
        todo!()
    }

    /// Activate or deactivate a cell.
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { cell_enabled(self.as_ptr(), enabled as _) }
    }

    /// Show or hide a cell.
    ///
    /// # Remarks
    /// If the cell contains a sublayout, the command will affect all controls recursively.
    pub fn set_visible(&self, visible: bool) {
        unsafe { cell_visible(self.as_ptr(), visible as _) }
    }

    /// Set an inner margin.
    pub fn set_padding(&self, pall: f32) {
        unsafe { cell_padding(self.as_ptr(), pall) }
    }

    /// Set an inner margin.
    pub fn set_padding2(&self, pleft: f32, pright: f32) {
        unsafe { cell_padding2(self.as_ptr(), pleft, pright) }
    }

    /// Set an inner margin.
    pub fn set_padding4(&self, pleft: f32, ptop: f32, pright: f32, pbottom: f32) {
        unsafe { cell_padding4(self.as_ptr(), pleft, ptop, pright, pbottom) }
    }
}
