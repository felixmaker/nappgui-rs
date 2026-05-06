use std::{cell::RefCell, rc::Rc};

use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::{
    gui::{event::ButtonEvent, impl_control, GUID},
    util::macros::listener,
};

#[derive(Default)]
pub(crate) struct UpDownInner {
    ptr: RefCell<*mut nappgui_sys::UpDown>,
    on_click: RefCell<Option<Rc<dyn Fn(&ButtonEvent) + 'static>>>,
}

/// The updown control.
///
/// # Remark
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct UpDown(GUID);

impl_control!(UpDown, UpDownInner);

impl UpDown {
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
        self.inner(|inner| *inner.on_click.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.0, UpDown, on_click(ButtonEvent));
        unsafe { updown_OnClick(self.as_ptr(), listener) }
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.as_ptr(), tooltip.as_ptr()) }
    }
}
