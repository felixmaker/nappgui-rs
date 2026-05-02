use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{
    edit_OnChange, edit_OnFilter, edit_OnFocus, edit_align, edit_autoselect, edit_bgcolor, edit_bgcolor_focus,
    edit_color, edit_color_focus, edit_copy, edit_create, edit_cut, edit_editable, edit_font, edit_get_height,
    edit_get_text, edit_multiline, edit_passmode, edit_paste, edit_phcolor, edit_phstyle, edit_phtext, edit_select,
    edit_text, edit_tooltip, edit_vpadding,
};

use crate::{
    draw_2d::{font::Font, Color},
    gui::{
        event::{EvText, EvTextFilter},
        global_record,
    },
    types::{Align, FontStyle},
    util::macros::callback,
};

pub(crate) struct EditInner {
    ptr: NonNull<nappgui_sys::Edit>,
}

impl EditInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Edit) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to EditInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Edit {
        self.ptr.as_ptr()
    }
}

/// The edit control.
///
/// # Remarks
/// If the object is not attached to a window, it causes a memory leak.
#[repr(transparent)]
pub struct Edit(Weak<EditInner>);

impl Edit {
    /// Create a cell from a pointer.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Edit) -> Self {
        let object = global_record(ptr as _, EditInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    /// Returns a raw pointer to the cell object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Edit {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a text edit control.
    pub fn new() -> Self {
        let edit = unsafe { edit_create() };
        unsafe { Self::from_raw(edit) }
    }

    /// Create a multiline text edit control.
    pub fn new_multiline() -> Self {
        let edit = unsafe { edit_multiline() };
        unsafe { Self::from_raw(edit) }
    }

    callback! {
        /// Set a function to filter the text while editing.
        pub on_filter(EvText) -> EvTextFilter => edit_OnFilter;

        /// Set a function to detect when the text has changed.
        pub on_change(EvText) -> bool => edit_OnChange;

        /// Sets a handler for keyboard focus.
        pub on_focus(bool) => edit_OnFocus;
    }

    /// Set the edit control text.
    pub fn set_text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            edit_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the font of the edit control.
    pub fn set_font(&self, font: &Font) {
        unsafe { edit_font(self.as_ptr(), font.as_ptr()) }
    }

    /// Set text alignment.
    pub fn set_align(&self, align: Align) {
        unsafe {
            edit_align(self.as_ptr(), align as _);
        }
    }

    /// Activate the password mode, which will hide the typed characters.
    pub fn set_passmode(&self, passmode: bool) {
        unsafe {
            edit_passmode(self.as_ptr(), passmode as _);
        }
    }

    /// Enable or disable editing in the control.
    pub fn set_editable(&self, editable: bool) {
        unsafe {
            edit_editable(self.as_ptr(), editable as _);
        }
    }

    /// Activate or deactivate auto-selection of text.
    pub fn set_autoselect(&self, autoselect: bool) {
        unsafe {
            edit_autoselect(self.as_ptr(), autoselect as _);
        }
    }

    /// Select text.
    pub fn set_select(&self, start: i32, end: i32) {
        unsafe {
            edit_select(self.as_ptr(), start as _, end as _);
        }
    }

    /// Assigns a tooltip to the edit control.
    pub fn set_tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            edit_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the text color.
    ///
    /// # Remarks
    /// RGB values may not be fully portable. See Colors.
    pub fn set_color(&self, color: Color) {
        unsafe {
            edit_color(self.as_ptr(), color.inner);
        }
    }

    /// Sets the color of the text, when the control has the keyboard focus.
    ///
    /// # Remarks
    /// RGB values may not be fully portable. See Colors.
    pub fn set_color_focus(&self, color: Color) {
        unsafe {
            edit_color_focus(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color.
    pub fn set_background_color(&self, color: Color) {
        unsafe {
            edit_bgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Sets the background color, when the control has keyboard focus.
    pub fn set_background_color_focus(&self, color: Color) {
        unsafe {
            edit_bgcolor_focus(self.as_ptr(), color.inner);
        }
    }

    /// Set an explanatory text for when the control is blank (placeholder).
    pub fn set_placeholder_text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            edit_phtext(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the color of the placeholder text.
    pub fn set_placeholder_color(&self, color: Color) {
        unsafe {
            edit_phcolor(self.as_ptr(), color.inner);
        }
    }

    /// Set the font style for the placeholder.
    pub fn set_placeholder_style(&self, style: FontStyle) {
        unsafe {
            edit_phstyle(self.as_ptr(), style.to_fstyle_t());
        }
    }

    /// Set the inner vertical margin.
    ///
    /// # Remarks
    ///
    /// padding: If 0 there will be no margin between the text and the border of the control.
    /// If <0 the default margin will be set.
    pub fn set_vpadding(&self, padding: f32) {
        unsafe {
            edit_vpadding(self.as_ptr(), padding);
        }
    }

    /// Get control text.
    pub fn text(&self) -> String {
        let text = unsafe { edit_get_text(self.as_ptr()) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the current height of the control.
    pub fn height(&self) -> f32 {
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
