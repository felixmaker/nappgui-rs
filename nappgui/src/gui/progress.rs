use nappgui_sys::{progress_create, progress_undefined, progress_value, progress_width};

use crate::gui::define_object;

#[derive(Default)]
pub(crate) struct ProgressProps {}

define_object!(Progress, ProgressInner, Progress, ProgressProps);

impl Progress {
    /// Create a progress bar.
    pub fn new() -> Self {
        let progress = unsafe { progress_create() };
        Self::from_raw(progress)
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
