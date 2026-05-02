use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::{
    gui::{event::EvButton, global_get, global_record},
    util::macros::callback,
};

pub(crate) struct UpDownInner {
    ptr: NonNull<nappgui_sys::UpDown>,
}

impl UpDownInner {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::UpDown) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to UpDownInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::UpDown {
        self.ptr.as_ptr()
    }
}

/// The updown control.
///
/// # Remark
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct UpDown(Weak<UpDownInner>);

impl UpDown {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::UpDown) -> Self {
        let object = global_record(ptr as _, UpDownInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::UpDown) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::UpDown {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
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
