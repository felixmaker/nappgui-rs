use std::rc::Rc;

use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_length, slider_steps, slider_tooltip, slider_value,
    slider_vertical,
};

use crate::gui::{Callback, define_object, event::SliderEvent, listener};

#[derive(Default)]
pub(crate) struct SliderProps {
    on_moved: Callback<SliderEvent>,
}

define_object!(Slider, SliderInner, Slider, SliderProps);

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
        self.inner(|inner| *inner.props.on_moved.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), SliderInner, on_moved(SliderEvent));
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
