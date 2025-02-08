use std::ffi::CString;

use crate::{
    draw_2d::{Color, Font},
    prelude::FStyle,
    util::macros::callback,
};

use nappgui_sys::{
    align_t, label_OnClick, label_align, label_bgcolor, label_bgcolor_over, label_color,
    label_color_over, label_create, label_font, label_multiline, label_size_text, label_style_over,
    label_text,
};

/// Label controls are used to insert small blocks of text into windows and forms. They are of uniform format,
/// that is, the font and color attributes will be applied to the entire text. In most cases the content will
/// be limited to a single line, although it is possible to show blocks that extend in several lines. The control
/// size will be adjusted to the text it contains
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

    callback! {
        /// Set the OnClick event handler.
        pub on_click(Label) => label_OnClick;
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
    pub fn style_over(&self, style: FStyle) {
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
