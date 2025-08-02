use std::ffi::CString;

use crate::{
    draw_2d::{Color, Font},
    gui::event::EvMouse,
    types::{Align, FontStyle},
    util::macros::callback,
};

use nappgui_sys::{
    label_OnClick, label_align, label_bgcolor, label_bgcolor_over, label_color, label_color_over,
    label_create, label_font, label_multiline, label_size_text, label_style_over, label_text,
};

/// The label trait.
pub trait LabelTrait {
    /// Returns a raw pointer to the label object.
    fn as_ptr(&self) -> *mut nappgui_sys::Label;

    callback! {
        /// Set the OnClick event handler.
        on_click(EvMouse) => label_OnClick;
    }

    /// Set the text that the label will display.
    fn text(&self, text: &str) {
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
    fn size_text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            label_size_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the text font.
    fn font(&self, font: &Font) {
        unsafe {
            label_font(self.as_ptr(), font.inner);
        }
    }

    /// Set the font modifiers, when the mouse is over the control.
    fn style_over(&self, style: FontStyle) {
        unsafe {
            label_style_over(self.as_ptr(), style.to_fstyle_t() as _);
        }
    }

    /// Set the label multi-lined.
    fn multiline(&self, multiline: bool) {
        unsafe { label_multiline(self.as_ptr(), multiline as _) };
    }

    /// Sets the horizontal alignment of the text with respect to the size of the control.
    fn align(&self, align: Align) {
        unsafe {
            label_align(self.as_ptr(), align as _);
        }
    }

    /// Set the text color.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    fn color(&self, color: Color) {
        unsafe {
            label_color(self.as_ptr(), color.inner);
        }
    }

    /// Set the color of the text, when the mouse is over the control.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    fn color_over(&self, color: Color) {
        unsafe {
            label_color_over(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color of the text.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    fn background_color(&self, color: Color) {
        unsafe {
            label_bgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color of the text, when the mouse is over the control.
    ///
    /// # Remarks
    /// RGB values may not be fully portable.
    fn background_color_over(&self, color: Color) {
        unsafe {
            label_bgcolor_over(self.as_ptr(), color.inner);
        }
    }
}

/// Label controls are used to insert small blocks of text into windows and forms. They are of uniform format,
/// that is, the font and color attributes will be applied to the entire text. In most cases the content will
/// be limited to a single line, although it is possible to show blocks that extend in several lines. The control
/// size will be adjusted to the text it contains
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Label {
    pub(crate) inner: *mut nappgui_sys::Label,
}

impl LabelTrait for Label {
    /// Returns a raw pointer to the label object.
    fn as_ptr(&self) -> *mut nappgui_sys::Label {
        self.inner
    }
}

impl Default for Label {
    fn default() -> Self {
        let label = unsafe { label_create() };
        Self { inner: label }
    }
}

impl Label {
    /// Create a text control.
    pub fn new(text: &str) -> Label {
        let label = Label::default();
        label.text(text);
        label
    }
}
