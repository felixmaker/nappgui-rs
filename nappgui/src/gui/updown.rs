use std::{
    cell::RefCell,
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::{
    gui::{event::ButtonEvent, global_get, global_record},
    util::macros::listener,
};

pub(crate) struct UpDownInner {
    ptr: NonNull<nappgui_sys::UpDown>,
    on_click: RefCell<Option<Rc<dyn Fn(&ButtonEvent) + 'static>>>,
}

impl UpDownInner {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::UpDown) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to UpDownInner::from_raw"),
            on_click: RefCell::new(None),
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

    /// Set an event handler for pressing the button.
    pub fn set_on_click_handler<F>(&self, handler: F)
    where
        F: Fn(&ButtonEvent) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_click.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), UpDownInner, on_click(ButtonEvent));
        unsafe { updown_OnClick(self.as_ptr(), listener) }
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.as_ptr(), tooltip.as_ptr()) }
    }
}
