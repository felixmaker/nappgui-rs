use std::rc::Rc;

use nappgui_sys::{updown_OnClick, updown_create, updown_tooltip};

use crate::gui::{define_object, event::ButtonEvent, listener, Callback};

#[derive(Default)]
pub(crate) struct UpDownProps {
    on_click: Callback<ButtonEvent>,
}

define_object!(UpDown, UpDownInner, UpDown, UpDownProps);

impl UpDown {
    /// Create an updown control.
    pub fn new() -> Self {
        let updown = unsafe { updown_create() };
        UpDown::from_raw(updown)
    }

    /// Set an event handler for pressing the button.
    pub fn set_on_click_handler<F>(&self, handler: F)
    where
        F: Fn(&ButtonEvent) + 'static,
    {
        self.inner(|inner| *inner.props.on_click.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), UpDownInner, on_click(ButtonEvent));
        unsafe { updown_OnClick(self.as_ptr(), listener) }
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, tooltip: &str) {
        let tooltip = std::ffi::CString::new(tooltip).unwrap();
        unsafe { updown_tooltip(self.as_ptr(), tooltip.as_ptr()) }
    }
}
