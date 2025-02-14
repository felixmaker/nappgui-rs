use std::rc::Rc;

use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::util::macros::{callback, impl_ptr};

/// UpDown are two-part horizontally divided button controls.
pub struct UpDown {
    pub(crate) inner: Rc<*mut nappgui_sys::UpDown>,
}

impl UpDown {
    impl_ptr!(nappgui_sys::UpDown);

    /// Create an updown control.
    pub fn create() -> Self {
        let updown = unsafe { updown_create() };
        Self::new(updown)
    }

    callback! {
        /// Set an event handler for pressing the button.
        pub on_click(UpDown) => updown_OnClick;
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.as_ptr(), tooltip.as_ptr()) }
    }
}
