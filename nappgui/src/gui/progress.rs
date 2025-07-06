use std::rc::Rc;

use nappgui_sys::{progress_create, progress_undefined, progress_value};

use crate::util::macros::{impl_gui_control, pub_crate_ptr_ops};

/// Progress bars are passive controls that show the remaining time to complete a certain task.
pub struct Progress {
    pub(crate) inner: Rc<*mut nappgui_sys::Progress>,
}

impl Progress {
    pub_crate_ptr_ops!(*mut nappgui_sys::Progress);

    /// Create a progress bar.
    pub fn create() -> Self {
        let progress = unsafe { progress_create() };
        Self::new(progress)
    }

    /// Set the progress bar as undefined.
    pub fn undefined(&self, running: bool) {
        unsafe { progress_undefined(self.as_ptr(), running as _) };
    }

    /// Set the progress position.
    pub fn value(&self, value: f32) {
        unsafe { progress_value(self.as_ptr(), value) };
    }
}

impl_gui_control!(Progress, guicontrol_progress);
