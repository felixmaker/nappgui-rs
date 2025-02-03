use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

pub struct UpDown {
    pub(crate) inner: *mut nappgui_sys::UpDown,
}

impl UpDown {
    pub(crate) fn new(ptr: *mut nappgui_sys::UpDown) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create an updown control.
    pub fn create() -> Self {
        let updown = unsafe { updown_create() };
        Self::new(updown)
    }

    /// Set an event handler for pressing the button.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(&mut UpDown) + 'static,
    {
        todo!();
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.inner, tooltip.as_ptr()) }
    }
}
