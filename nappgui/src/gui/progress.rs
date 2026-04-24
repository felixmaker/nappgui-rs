use std::sync::Arc;

use nappgui_sys::{progress_create, progress_undefined, progress_value};

/// The progress type.
pub(crate) struct ProgressInner {
    inner: *mut nappgui_sys::Progress,
}

/// Progress bars are passive controls that show the remaining time to complete a certain task.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct Progress {
    pub(crate) inner: Arc<ProgressInner>,
}

impl Progress {
    /// Create a progress bar.
    pub fn new() -> Self {
        let progress = unsafe { progress_create() };
        assert!(!progress.is_null());
        Self {
            inner: Arc::new(ProgressInner { inner: progress }),
        }
    }

    /// Set the progress bar as undefined.
    pub fn set_undefined(&self, running: bool) {
        unsafe { progress_undefined(self.as_ptr(), running as _) };
    }

    /// Set the progress position.
    pub fn set_value(&self, value: f32) {
        unsafe { progress_value(self.as_ptr(), value) };
    }

    /// Returns a raw pointer to the progress object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Progress {
        self.inner.inner
    }
}
