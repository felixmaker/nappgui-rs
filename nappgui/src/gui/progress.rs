use nappgui_sys::{progress_create, progress_undefined, progress_value};

/// The progress trait.
pub trait ProgressTrait {
    /// Returns a raw pointer to the progress object.
    fn as_ptr(&self) -> *mut nappgui_sys::Progress;

    /// Set the progress bar as undefined.
    fn undefined(&self, running: bool) {
        unsafe { progress_undefined(self.as_ptr(), running as _) };
    }

    /// Set the progress position.
    fn value(&self, value: f32) {
        unsafe { progress_value(self.as_ptr(), value) };
    }
}

/// Progress bars are passive controls that show the remaining time to complete a certain task.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Progress {
    pub(crate) inner: *mut nappgui_sys::Progress,
}

impl ProgressTrait for Progress {
    fn as_ptr(&self) -> *mut nappgui_sys::Progress {
        self.inner
    }
}

impl Progress {
    /// Create a progress bar.
    pub fn new() -> Self {
        let progress = unsafe { progress_create() };
        Self { inner: progress }
    }
}
