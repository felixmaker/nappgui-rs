use std::ptr::NonNull;

use nappgui_sys::{progress_create, progress_undefined, progress_value};

/// Progress bars are passive controls that show the remaining time to complete a certain task.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct Progress(NonNull<nappgui_sys::Progress>);

impl Progress {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Progress) -> Self {
        Self(NonNull::new(ptr).expect("Null pointer passed to Progress::from_raw"))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Progress {
        self.0.as_ptr()
    }

    /// Create a progress bar.
    pub fn new() -> Self {
        let progress = unsafe { progress_create() };
        unsafe { Self::from_raw(progress) }
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
