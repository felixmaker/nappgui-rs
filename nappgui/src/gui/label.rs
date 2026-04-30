use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use crate::{
    draw_2d::{Color, Font},
    gui::event::EvMouse,
    types::{Align, Ellipsis, FontStyle},
    util::macros::callback,
};

use nappgui_sys::{
    label_OnClick, label_align, label_bgcolor, label_bgcolor_over, label_color, label_color_over, label_create,
    label_font, label_get_font, label_get_text, label_multiline, label_size_text, label_style_over, label_text,
    label_trim,
};

/// Label controls are used to insert small blocks of text into windows and forms. They are of uniform format,
/// that is, the font and color attributes will be applied to the entire text. In most cases the content will
/// be limited to a single line, although it is possible to show blocks that extend in several lines. The control
/// size will be adjusted to the text it contains
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone)]
pub struct Label(NonNull<nappgui_sys::Label>);

impl Label {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Label) -> Self {
        Self(NonNull::new(ptr).expect("Null pointer passed to Label::from_raw"))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Label {
        self.0.as_ptr()
    }

    /// Create a text control.
    pub fn new(text: &str) -> Label {
        let label = unsafe { label_create() };
        let label = unsafe { Label::from_raw(label) };
        label.set_text(text);
        label
    }

    callback! {
        /// Set the OnClick event handler.
        pub on_click(EvMouse) => label_OnClick;
    }

    /// Set the text that the label will display.
    pub fn set_text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            label_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the text with which the control will be sized.
    ///
    /// # Remarks
    /// By default, a Label control will be sized to the exact size of the text it
    /// contains. See Dynamic labels.
    pub fn set_text_sized(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            label_size_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the text font.
    pub fn set_font(&self, font: &Font) {
        unsafe { label_font(self.as_ptr(), font.as_ptr()) }
    }

    /// Set the font modifiers, when the mouse is over the control.
    pub fn set_style_over(&self, style: FontStyle) {
        unsafe {
            label_style_over(self.as_ptr(), style.to_fstyle_t() as _);
        }
    }

    /// Set the label multi-lined.
    pub fn set_multiline(&self, multiline: bool) {
        unsafe { label_multiline(self.as_ptr(), multiline as _) };
    }

    /// Sets the horizontal alignment of the text with respect to the size of the control.
    pub fn set_align(&self, align: Align) {
        unsafe {
            label_align(self.as_ptr(), align as _);
        }
    }

    /// Set the text color.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn set_color(&self, color: Color) {
        unsafe {
            label_color(self.as_ptr(), color.inner);
        }
    }

    /// Set the color of the text, when the mouse is over the control.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn set_color_over(&self, color: Color) {
        unsafe {
            label_color_over(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color of the text.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn set_background_color(&self, color: Color) {
        unsafe {
            label_bgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color of the text, when the mouse is over the control.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    pub fn set_background_color_over(&self, color: Color) {
        unsafe {
            label_bgcolor_over(self.as_ptr(), color.inner);
        }
    }

    /// Sets how the text will be clipped when it does not fit inside the control entirety.
    ///
    /// #Remarks
    /// This does not apply if it is a multiline label.
    pub fn trim(&self, ellipsis: Ellipsis) {
        unsafe { label_trim(self.as_ptr(), ellipsis as _) };
    }

    /// Get the label text.
    pub fn text(&self) -> String {
        let text = unsafe { label_get_text(self.as_ptr()) };
        let text = unsafe { CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Get the font associated with control.
    pub fn font(&self) -> Font {
        let font = unsafe { label_get_font(self.as_ptr()) };
        unsafe { Font::from_raw_cloned(font as _) }
    }
}
