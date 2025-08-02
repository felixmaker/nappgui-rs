use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_steps, slider_tooltip, slider_value,
    slider_vertical,
};

use crate::{gui::event::EvSlider, util::macros::callback};

/// The slider trait.
pub trait SliderTrait {
    /// Returns a raw pointer to the slider object.
    fn as_ptr(&self) -> *mut nappgui_sys::Slider;

    callback! {
        /// Set an event handler for slider movement.
         on_moved(EvSlider) => slider_OnMoved;
    }

    /// Set a tooltip for the slider. It is a small explanatory text that will appear when the mouse is over the control.
    fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            slider_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Changes the slider from continuous range to discrete intervals.
    fn steps(&self, steps: u32) {
        unsafe {
            slider_steps(self.as_ptr(), steps);
        }
    }

    /// Set the slider position.
    fn value(&self, value: f32) {
        unsafe {
            slider_value(self.as_ptr(), value);
        }
    }

    /// Get the slider position.
    fn get_value(&self) -> f32 {
        unsafe { slider_get_value(self.as_ptr()) }
    }
}

/// Sliders are normally used to edit continuous and bounded numerical values.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Slider {
    pub(crate) inner: *mut nappgui_sys::Slider,
}

impl SliderTrait for Slider {
    fn as_ptr(&self) -> *mut nappgui_sys::Slider {
        self.inner
    }
}

impl Slider {
    /// Create a new slider control.
    pub fn new() -> Self {
        let updown = unsafe { slider_create() };
        Self { inner: updown }
    }

    /// Create a new vertical slider.
    pub fn new_vertical() -> Self {
        let updown = unsafe { slider_vertical() };
        Self { inner: updown }
    }
}
