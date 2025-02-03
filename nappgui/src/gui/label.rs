use std::ffi::CString;

use crate::{core::event::Event, draw_2d::{Color, Font}};

use nappgui_sys::{
    align_t, fstyle_t, label_OnClick, label_align, label_bgcolor, label_bgcolor_over, label_color,
    label_color_over, label_create, label_font, label_multiline, label_size_text, label_style_over,
    label_text, listener_imp,
};

pub struct Label {
    pub(crate) inner: *mut nappgui_sys::Label,
}

impl Label {
    pub(crate) fn new(ptr: *mut nappgui_sys::Label) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a text control.
    pub fn create() -> Label {
        let label = unsafe { label_create() };
        Self::new(label)
    }

    /// Create a multi-line text control.
    pub fn multiline() -> Label {
        let label = unsafe { label_multiline() };
        Self::new(label)
    }

    /// Set the OnClick event handler.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(&mut Label, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut Label, &Event)>, *mut nappgui_sys::Label);
            let f = &mut *(*data).0;
            let mut label = Label { inner: (*data).1 };
            // let ev = Event::new(event);
            // let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut label, &ev)));
        }

        let cb: Box<dyn FnMut(&mut Label, &Event)> = Box::new(handler);

        let data: *mut (Box<dyn FnMut(&mut Label, &Event)>, *mut nappgui_sys::Label) =
            Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            label_OnClick(
                self.inner,
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Set the text that the label will display.
    pub fn text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            label_text(self.inner, text.as_ptr());
        }
    }

    /// Set the text with which the control will be sized.
    ///
    /// # Remarks
    /// By default, a Label control will be sized to the exact size of the text it
    /// contains. See Dynamic labels.
    pub fn size_text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            label_size_text(self.inner, text.as_ptr());
        }
    }

    /// Set the text font.
    pub fn font(&self, font: &Font) {
        unsafe {
            label_font(self.inner, font.inner);
        }
    }

    /// Set the font modifiers, when the mouse is over the control.
    pub fn style_over(&self, style: fstyle_t) {
        unsafe {
            label_style_over(self.inner, (style as i32).try_into().unwrap());
        }
    }

    /// Sets the horizontal alignment of the text with respect to the size of the control.
    pub fn align(&self, align: align_t) {
        unsafe {
            label_align(self.inner, align);
        }
    }

    /// Set the text color.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn color(&self, color: Color) {
        unsafe {
            label_color(self.inner, color.inner);
        }
    }

    /// Set the color of the text, when the mouse is over the control.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn color_over(&self, color: Color) {
        unsafe {
            label_color_over(self.inner, color.inner);
        }
    }

    /// Set the background color of the text.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn bgcolor(&self, color: Color) {
        unsafe {
            label_bgcolor(self.inner, color.inner);
        }
    }

    /// Set the background color of the text, when the mouse is over the control.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn bgcolor_over(&self, color: Color) {
        unsafe {
            label_bgcolor_over(self.inner, color.inner);
        }
    }
}
