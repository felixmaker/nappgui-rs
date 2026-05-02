use std::ptr::NonNull;

use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::{gui::event::EvButton, util::macros::callback};

/// UpDown are two-part horizontally divided button controls.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct UpDown(NonNull<nappgui_sys::UpDown>);

impl UpDown {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::UpDown) -> Self {
        UpDown(NonNull::new(ptr).expect("Null pointer passed to UpDown::from_raw"))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::UpDown {
        self.0.as_ptr()
    }

    /// Create an updown control.
    pub fn new() -> Self {
        let updown = unsafe { updown_create() };
        unsafe { UpDown::from_raw(updown) }
    }

    callback! {
        /// Set an event handler for pressing the button.
        pub on_click(EvButton) => updown_OnClick;
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.as_ptr(), tooltip.as_ptr()) }
    }
}
