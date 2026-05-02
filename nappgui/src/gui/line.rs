use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{line_horizontal, line_length, line_vertical};

use crate::gui::global_record;

pub(crate) struct LineInner {
    ptr: NonNull<nappgui_sys::Line>,
}

impl LineInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Line) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to LineInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Line {
        self.ptr.as_ptr()
    }
}

/// The line control.
///
/// # Remarks
/// If the object is not attached to a window, it causes a memory leak.
#[repr(transparent)]
pub struct Line(Weak<LineInner>);

impl Line {
    /// Creates a `Line` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be non-null and point to a valid `Line` instance
    /// managed by the C library.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Line) -> Self {
        let object = global_record(ptr as _, LineInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    /// Returns the underlying raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Line {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
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
