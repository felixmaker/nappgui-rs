use std::ptr::NonNull;

use nappgui_sys::{line_horizontal, line_length, line_vertical};

/// The line widget.
///
/// Lines are simple horizontal or vertical dividers used to visually separate UI elements.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components associated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Line(NonNull<nappgui_sys::Line>);

impl Line {
    /// Creates a `Line` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be non-null and point to a valid `Line` instance
    /// managed by the C library.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Line) -> Self {
        Line(NonNull::new(ptr).expect("Null pointer passed to Line::from_raw"))
    }

    /// Returns the underlying raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Line {
        self.0.as_ptr()
    }

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