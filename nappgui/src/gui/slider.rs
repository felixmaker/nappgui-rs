use std::ptr::NonNull;

use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_steps, slider_tooltip, slider_value, slider_vertical,
};

use crate::{gui::event::EvSlider, util::macros::callback};

/// Sliders are normally used to edit continuous and bounded numerical values.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct Slider(NonNull<nappgui_sys::Slider>);

impl Slider {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Slider) -> Self {
        Slider(NonNull::new(ptr).expect("Null pointer passed to Slider::from_raw"))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Slider {
        self.0.as_ptr()
    }

    /// Create a new slider control.
    pub fn new() -> Self {
        unsafe { Slider::from_raw(slider_create()) }
    }

    /// Create a new vertical slider.
    pub fn new_vertical() -> Self {
        unsafe { Slider::from_raw(slider_vertical()) }
    }

    callback! {
        /// Set an event handler for slider movement.
        pub on_moved(EvSlider) => slider_OnMoved;
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
