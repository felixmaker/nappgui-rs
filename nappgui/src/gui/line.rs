use std::cell::RefCell;

use nappgui_sys::{line_horizontal, line_length, line_vertical};

use crate::gui::{impl_control, GUID};

#[derive(Default)]
pub(crate) struct LineInner {
    ptr: RefCell<*mut nappgui_sys::Line>,
}

/// The line control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct Line(GUID);

impl_control!(Line, LineInner);

impl Line {
    /// Create a horizontal separator.
    pub fn new_horizontal() -> Self {
        unsafe { Line::from_raw(line_horizontal()) }
    }

    /// Create a vertical separator.
    pub fn new_vertical() -> Self {
        unsafe { Line::from_raw(line_vertical()) }
    }

    /// Sets the natural length of the line. By default 100px.
    pub fn set_length(&self, length: f32) {
        unsafe {
            line_length(self.as_ptr(), length);
        }
    }
}
