use std::{cell::RefCell, rc::Rc};

use nappgui_sys::{
    edit_OnChange, edit_OnFilter, edit_OnFocus, edit_align, edit_autoselect, edit_bgcolor, edit_bgcolor_focus,
    edit_color, edit_color_focus, edit_copy, edit_create, edit_cut, edit_editable, edit_font, edit_get_height,
    edit_get_text, edit_multiline, edit_passmode, edit_paste, edit_phcolor, edit_phstyle, edit_phtext, edit_select,
    edit_text, edit_tooltip, edit_vpadding,
};

use crate::{
    draw_2d::{font::Font, Color},
    gui::{
        event::{TextEvent, TextFilterEvent},
        impl_control, GUID,
    },
    types::{Align, FontStyle},
    util::macros::listener,
};

#[derive(Default)]
pub(crate) struct EditInner {
    ptr: RefCell<*mut nappgui_sys::Edit>,
    on_filter: RefCell<Option<Rc<dyn Fn(&TextEvent) -> TextFilterEvent + 'static>>>,
    on_change: RefCell<Option<Rc<dyn Fn(&TextEvent) -> bool + 'static>>>,
    on_focus: RefCell<Option<Rc<dyn Fn(&bool) + 'static>>>,
}

/// The edit control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Edit(GUID);

impl_control!(Edit, EditInner);

impl Edit {
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

    /// Set a function to filter the text while editing.
    pub fn set_on_filter_handler<F>(&self, callback: F)
    where
        F: Fn(&TextEvent) -> TextFilterEvent + 'static,
    {
        self.inner(|object| *object.on_filter.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.0, Edit, on_filter(TextEvent) -> TextFilterEvent);
        unsafe { edit_OnFilter(self.as_ptr(), listener) };
    }

    /// Set a function to detect when the text has changed.
    pub fn set_on_change_handler<F>(&self, callback: F)
    where
        F: Fn(&TextEvent) -> bool + 'static,
    {
        self.inner(|object| *object.on_change.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.0, Edit, on_change(TextEvent) -> bool);
        unsafe { edit_OnChange(self.as_ptr(), listener) };
    }

    /// Sets a handler for keyboard focus.
    pub fn set_on_focus_handler<F>(&self, callback: F)
    where
        F: Fn(&bool) + 'static,
    {
        self.inner(|object| *object.on_focus.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.0, Edit, on_focus(bool));
        unsafe { edit_OnFocus(self.as_ptr(), listener) };
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
