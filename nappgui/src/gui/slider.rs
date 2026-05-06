use std::{cell::RefCell, rc::Rc};

use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_length, slider_steps, slider_tooltip, slider_value,
    slider_vertical,
};

use crate::{
    gui::{event::SliderEvent, impl_control, GUID},
    util::macros::listener,
};

#[derive(Default)]
pub(crate) struct SliderInner {
    ptr: RefCell<*mut nappgui_sys::Slider>,
    on_moved: RefCell<Option<Rc<dyn Fn(&SliderEvent) + 'static>>>,
}

/// The slider control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Slider(GUID);

impl_control!(Slider, SliderInner);

impl Slider {
    /// Create a new slider control.
    pub fn new() -> Self {
        unsafe { Slider::from_raw(slider_create()) }
    }

    /// Create a new vertical slider.
    pub fn new_vertical() -> Self {
        unsafe { Slider::from_raw(slider_vertical()) }
    }

    /// Set the natural length of control. By default 100px.
    pub fn set_length(&self, length: f32) {
        unsafe { slider_length(self.as_ptr(), length) }
    }

    /// Set an event handler for slider movement.
    pub fn set_on_moved_handler<F>(&self, handler: F)
    where
        F: Fn(&SliderEvent) + 'static,
    {
        self.inner(|inner| *inner.on_moved.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.0, Slider, on_moved(SliderEvent));
        unsafe { slider_OnMoved(self.as_ptr(), listener) }
    }

    /// Set a tooltip for the slider. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            slider_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Changes the slider from continuous range to discrete intervals.
    pub fn set_steps(&self, steps: u32) {
        unsafe {
            slider_steps(self.as_ptr(), steps);
        }
    }

    /// Set the slider position.
    pub fn set_value(&self, value: f32) {
        unsafe {
            slider_value(self.as_ptr(), value);
        }
    }

    /// Get the slider position.
    pub fn value(&self) -> f32 {
        unsafe { slider_get_value(self.as_ptr()) }
    }
}
