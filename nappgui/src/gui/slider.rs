use nappgui_sys::{
    listener_imp, slider_OnMoved, slider_create, slider_get_value, slider_steps, slider_tooltip,
    slider_value, slider_vertical,
};

use crate::core::event::Event;

pub struct Slider {
    pub(crate) inner: *mut nappgui_sys::Slider,
}

impl Slider {
    pub(crate) fn new(ptr: *mut nappgui_sys::Slider) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new slider control.
    pub fn create() -> Self {
        let updown = unsafe { nappgui_sys::slider_create() };
        Self::new(updown)
    }

    /// Create a new vertical slider.
    pub fn create_vertical() -> Self {
        let updown = unsafe { nappgui_sys::slider_vertical() };
        Self::new(updown)
    }

    /// Set an event handler for slider movement.
    pub fn on_moved<F>(&self, handler: F)
    where
        F: FnMut(&mut Slider, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (
                Box<dyn FnMut(&mut Slider, &Event)>,
                *mut nappgui_sys::Slider,
            );
            let f = &mut *(*data).0;
            let mut obj = Slider { inner: (*data).1 };
            let ev = Event::new(event as _);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &ev)));
        }

        let cb: Box<dyn FnMut(&mut Slider, &Event)> = Box::new(handler);

        let data: *mut (
            Box<dyn FnMut(&mut Slider, &Event)>,
            *mut nappgui_sys::Slider,
        ) = Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            slider_OnMoved(
                self.inner,
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Set a tooltip for the slider. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            slider_tooltip(self.inner, text.as_ptr());
        }
    }

    /// Changes the slider from continuous range to discrete intervals.
    pub fn steps(&self, steps: u32) {
        unsafe {
            slider_steps(self.inner, steps);
        }
    }

    /// Set the slider position.
    pub fn value(&self, value: f32) {
        unsafe {
            slider_value(self.inner, value);
        }
    }

    /// Get the slider position.
    pub fn get_value(&self) -> f32 {
        unsafe { slider_get_value(self.inner) }
    }
}
