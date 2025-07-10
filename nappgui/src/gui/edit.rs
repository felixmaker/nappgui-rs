use std::rc::Rc;

use nappgui_sys::{
    edit_OnChange, edit_OnFilter, edit_OnFocus, edit_align, edit_autoselect, edit_bgcolor,
    edit_bgcolor_focus, edit_color, edit_color_focus, edit_copy, edit_create, edit_cut,
    edit_editable, edit_font, edit_get_height, edit_get_text, edit_multiline, edit_passmode,
    edit_paste, edit_phcolor, edit_phstyle, edit_phtext, edit_select, edit_text, edit_tooltip,
    edit_vpadding,
};

use crate::{
    draw_2d::{font::Font, Color},
    gui::{
        event::{EvText, EvTextFilter},
        impl_layout,
    },
    types::{Align, FontStyle},
    util::macros::{callback, impl_gui_control, pub_crate_ptr_ops},
};

/// EditBox are small text boxes with editing capabilities. Like the Label they are of uniform format:
/// The typeface and colors will affect the entire text
#[derive(Clone)]
pub struct Edit {
    pub(crate) inner: Rc<*mut nappgui_sys::Edit>,
}

impl Edit {
    pub_crate_ptr_ops!(*mut nappgui_sys::Edit);

    /// Create a text edit control.
    pub fn new() -> Self {
        let edit = unsafe { edit_create() };
        Self::from_raw(edit)
    }

    /// Create a multiline text edit control.
    pub fn new_multiline() -> Self {
        let edit = unsafe { edit_multiline() };
        Self::from_raw(edit)
    }

    callback! {
        /// Set a function to filter the text while editing.
        pub on_filter(Edit, EvText) -> EvTextFilter => edit_OnFilter;

        /// Set a function to detect when the text has changed.
        pub on_change(Edit, EvText) -> bool => edit_OnChange;

        /// Sets a handler for keyboard focus.
        pub on_focus(Edit, bool) => edit_OnFocus;
    }

    /// Set the edit control text.
    pub fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            edit_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the font of the edit control.
    pub fn font(&self, font: &Font) {
        unsafe {
            edit_font(self.as_ptr(), font.inner);
        }
    }

    /// Set text alignment.
    pub fn align(&self, align: Align) {
        unsafe {
            edit_align(self.as_ptr(), align as _);
        }
    }

    /// Activate the password mode, which will hide the typed characters.
    pub fn password(&self, passmode: bool) {
        unsafe {
            edit_passmode(self.as_ptr(), passmode as _);
        }
    }

    /// Enable or disable editing in the control.
    pub fn editable(&self, editable: bool) {
        unsafe {
            edit_editable(self.as_ptr(), editable as _);
        }
    }

    /// Activate or deactivate auto-selection of text.
    pub fn autoselect(&self, autoselect: bool) {
        unsafe {
            edit_autoselect(self.as_ptr(), autoselect as _);
        }
    }

    /// Select text.
    pub fn select(&self, start: usize, end: usize) {
        unsafe {
            edit_select(self.as_ptr(), start as _, end as _);
        }
    }

    /// Assigns a tooltip to the edit control.
    pub fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            edit_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the text color.
    ///
    /// # Remarks
    /// RGB values may not be fully portable. See Colors.
    pub fn color(&self, color: Color) {
        unsafe {
            edit_color(self.as_ptr(), color.inner);
        }
    }

    /// Sets the color of the text, when the control has the keyboard focus.
    ///
    /// # Remarks
    /// RGB values may not be fully portable. See Colors.
    pub fn color_focus(&self, color: Color) {
        unsafe {
            edit_color_focus(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color.
    pub fn bgcolor(&self, color: Color) {
        unsafe {
            edit_bgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Sets the background color, when the control has keyboard focus.
    pub fn bgcolor_focus(&self, color: Color) {
        unsafe {
            edit_bgcolor_focus(self.as_ptr(), color.inner);
        }
    }

    /// Set an explanatory text for when the control is blank (placeholder).
    pub fn phtext(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            edit_phtext(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the color of the placeholder text.
    pub fn phcolor(&self, color: Color) {
        unsafe {
            edit_phcolor(self.as_ptr(), color.inner);
        }
    }

    /// Set the font style for the placeholder.
    pub fn phstyle(&self, style: FontStyle) {
        unsafe {
            edit_phstyle(self.as_ptr(), style.to_fstyle_t());
        }
    }

    /// Sets the inner vertical margin.
    pub fn vpadding(&self, padding: f32) {
        unsafe {
            edit_vpadding(self.as_ptr(), padding);
        }
    }

    /// Get control text.
    pub fn get_text(&self) -> String {
        let text = unsafe { edit_get_text(self.as_ptr()) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the current height of the control.
    pub fn get_height(&self) -> f32 {
        unsafe { edit_get_height(self.as_ptr()) }
    }

    /// Copies the selected text to the clipboard.
    pub fn copy(&self) {
        unsafe {
            edit_copy(self.as_ptr());
        }
    }

    /// Cuts the selected text, copying it to the clipboard.
    pub fn cut(&self) {
        unsafe {
            edit_cut(self.as_ptr());
        }
    }

    /// Pastes the text from the clipboard into the caret position.
    pub fn paste(&self) {
        unsafe {
            edit_paste(self.as_ptr());
        }
    }
}

impl_gui_control!(Edit, guicontrol_edit);
impl_layout!(Edit, layout_edit);
