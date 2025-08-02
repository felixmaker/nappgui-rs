use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::{gui::event::EvButton, util::macros::callback};

/// The updown trait.
pub trait UpDownTrait {
    /// Returns a raw pointer to the updown object.
    fn as_ptr(&self) -> *mut nappgui_sys::UpDown;

    callback! {
        /// Set an event handler for pressing the button.
         on_click(EvButton) => updown_OnClick;
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    fn tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.as_ptr(), tooltip.as_ptr()) }
    }
}

/// UpDown are two-part horizontally divided button controls.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct UpDown {
    pub(crate) inner: *mut nappgui_sys::UpDown,
}

impl UpDownTrait for UpDown {
    fn as_ptr(&self) -> *mut nappgui_sys::UpDown {
        self.inner
    }
}

impl UpDown {
    /// Create an updown control.
    pub fn new() -> Self {
        let updown = unsafe { updown_create() };
        Self { inner: updown }
    }
}
