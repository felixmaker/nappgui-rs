use std::sync::Arc;

use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_steps, slider_tooltip, slider_value, slider_vertical,
};

use crate::{gui::event::EvSlider, util::macros::callback};

/// The slider type
pub(crate) struct SliderInner {
    inner: *mut nappgui_sys::Slider,
}

/// Sliders are normally used to edit continuous and bounded numerical values.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone)]
pub struct Slider {
    pub(crate) inner: Arc<SliderInner>,
}

impl Slider {
    /// Create a new slider control.
    pub fn new() -> Self {
        let updown = unsafe { slider_create() };
        assert!(!updown.is_null());
        Self {
            inner: Arc::new(SliderInner { inner: updown }),
        }
    }

    /// Create a new vertical slider.
    pub fn new_vertical() -> Self {
        let updown = unsafe { slider_vertical() };
        assert!(!updown.is_null());
        Self {
            inner: Arc::new(SliderInner { inner: updown }),
        }
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

    /// Returns a raw pointer to the slider object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Slider {
        self.inner.inner
    }
}
