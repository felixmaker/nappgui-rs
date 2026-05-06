use std::cell::RefCell;

use nappgui_sys::{progress_create, progress_undefined, progress_value, progress_width};

use crate::gui::{impl_control, GUID};

#[derive(Default)]
pub(crate) struct ProgressInner {
    ptr: RefCell<*mut nappgui_sys::Progress>,
}

/// The progress bar control.
///
/// # Remark
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Progress(GUID);

impl_control!(Progress, ProgressInner);

impl Progress {
    /// Create a progress bar.
    pub fn new() -> Self {
        let progress = unsafe { progress_create() };
        unsafe { Self::from_raw(progress) }
    }

    /// Set the natural width of control. By default 100px.
    pub fn set_width(&self, width: f32) {
        unsafe { progress_width(self.as_ptr(), width) };
    }

    /// Set the progress bar as undefined.
    pub fn set_undefined(&self, running: bool) {
        unsafe { progress_undefined(self.as_ptr(), running as _) };
    }

    /// Set the progress position.
    pub fn set_value(&self, value: f32) {
        unsafe { progress_value(self.as_ptr(), value) };
    }
}
