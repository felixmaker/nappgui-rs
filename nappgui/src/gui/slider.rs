use std::rc::Rc;

use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_steps, slider_tooltip, slider_value,
    slider_vertical,
};

use crate::util::macros::{callback, pub_crate_ptr_ops};

/// Sliders are normally used to edit continuous and bounded numerical values.
pub struct Slider {
    pub(crate) inner: Rc<*mut nappgui_sys::Slider>,
}

impl Slider {
    pub_crate_ptr_ops!(*mut nappgui_sys::Slider);

    /// Create a new slider control.
    pub fn create() -> Self {
        let updown = unsafe { slider_create() };
        Self::new(updown)
    }

    /// Create a new vertical slider.
    pub fn create_vertical() -> Self {
        let updown = unsafe { slider_vertical() };
        Self::new(updown)
    }

    callback! {
        /// Set an event handler for slider movement.
        pub on_moved(Slider) => slider_OnMoved;
    }

    /// Set a tooltip for the slider. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            slider_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Changes the slider from continuous range to discrete intervals.
    pub fn steps(&self, steps: u32) {
        unsafe {
            slider_steps(self.as_ptr(), steps);
        }
    }

    /// Set the slider position.
    pub fn value(&self, value: f32) {
        unsafe {
            slider_value(self.as_ptr(), value);
        }
    }

    /// Get the slider position.
    pub fn get_value(&self) -> f32 {
        unsafe { slider_get_value(self.as_ptr()) }
    }
}
