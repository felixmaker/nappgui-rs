use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{
    slider_OnMoved, slider_create, slider_get_value, slider_steps, slider_tooltip, slider_value, slider_vertical,
};

use crate::{
    gui::{event::EvSlider, global_get, global_record},
    util::macros::callback,
};

pub(crate) struct SliderInner {
    ptr: NonNull<nappgui_sys::Slider>,
}

impl SliderInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Slider) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to SliderInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Slider {
        self.ptr.as_ptr()
    }
}

/// The slider control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct Slider(Weak<SliderInner>);

impl Slider {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Slider) -> Self {
        let object = global_record(ptr as _, SliderInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::Slider) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Slider {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
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
