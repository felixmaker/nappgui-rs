use nappgui_sys::{progress_create, progress_undefined, progress_value};

pub struct Progress {
    pub(crate) inner: *mut nappgui_sys::Progress,
}

impl Progress {
    pub(crate) fn new(ptr: *mut nappgui_sys::Progress) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a progress bar.
    pub fn create() -> Self {
        let progress = unsafe { progress_create() };
        Self::new(progress)
    }

    /// Set the progress bar as undefined.
    pub fn undefined(&self, running: bool) {
        unsafe { progress_undefined(self.inner, running as i8) };
    }

    /// Set the progress position.
    pub fn value(&self, value: f32) {
        unsafe { progress_value(self.inner, value) };
    }
}
