use std::rc::Rc;

use crate::{
    draw_2d::Color, types::{Align, FontStyle}, util::macros::{callback, impl_gui_control, pub_crate_ptr_ops}
};

use nappgui_sys::{
    textview_OnFilter, textview_OnFocus, textview_afspace, textview_apply_all,
    textview_apply_select, textview_bfspace, textview_bgcolor, textview_clear, textview_color,
    textview_copy, textview_cpos_writef, textview_create, textview_cut, textview_del_select,
    textview_editable, textview_family, textview_fsize, textview_fstyle, textview_get_text,
    textview_halign, textview_lspacing, textview_paste, textview_pgcolor, textview_scroll_caret,
    textview_scroll_visible, textview_select, textview_show_select, textview_size, textview_units,
    textview_wrap, textview_writef, S2Df,
};

/// TextView are views designed to work with rich text blocks, where fonts, sizes and colors can be combined.
pub struct TextView {
    pub(crate) inner: Rc<*mut nappgui_sys::TextView>,
}

impl TextView {
    pub_crate_ptr_ops!(*mut nappgui_sys::TextView);

    /// Create a text view.
    pub fn create() -> Self {
        let textview = unsafe { textview_create() };
        Self::from_raw(textview)
    }

    callback! {
        /// Set a handler to filter text while editing.
        ///
        /// # Remarks
        /// It works the same way as in Edit controls. See Filter texts and GUI Events.
        pub on_filter(TextView) => textview_OnFilter;

        /// Set a handler for keyboard focus.
        pub on_focus(TextView) => textview_OnFocus;
    }

    /// Sets the default size of the view.
    pub fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe { textview_size(self.as_ptr(), size) }
    }

    /// Clears all content from view.
    pub fn clear(&self) {
        unsafe { textview_clear(self.as_ptr()) }
    }

    /// Writes text to the view, using the format of the printf.
    pub fn writef(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            textview_writef(self.as_ptr(), text.as_ptr());
        }
    }

    /// Insert text into the cursor position.
    pub fn cpos_writef(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            textview_cpos_writef(self.as_ptr(), text.as_ptr());
        }
    }

    /// Sets the text units.
    pub fn units(&self, units: u32) {
        unsafe { textview_units(self.as_ptr(), units) }
    }

    /// Sets the font family of the text ("Arial", "Times New Roman", "Helvetica", etc).
    ///
    /// # Remarks
    /// Not all families will be present on all platforms. Use font_exists_family or font_installed_families to check.
    pub fn family(&self, family: &str) {
        let family = std::ffi::CString::new(family).unwrap();
        unsafe {
            textview_family(self.as_ptr(), family.as_ptr());
        }
    }

    /// Set the text size.
    ///
    /// # Remarks
    /// The value is conditional on the units established in textview_units.
    pub fn fsize(&self, size: f32) {
        unsafe { textview_fsize(self.as_ptr(), size) }
    }

    /// Sets the text style.
    pub fn fstyle(&self, style: FontStyle) {
        unsafe {
            textview_fstyle(self.as_ptr(), style.to_fstyle_t());
        }
    }

    /// Sets the text color.
    pub fn color(&self, color: Color) {
        unsafe {
            textview_color(self.as_ptr(), color.inner);
        }
    }

    /// Sets the background color of the text.
    pub fn bgcolor(&self, color: Color) {
        unsafe {
            textview_bgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Sets the background color of the control.
    pub fn pgcolor(&self, color: Color) {
        unsafe {
            textview_pgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Sets the alignment of text in a paragraph.
    pub fn halign(&self, align: Align) {
        unsafe {
            textview_halign(self.as_ptr(), align as _);
        }
    }

    /// Sets the line spacing of the paragraph.
    pub fn lspacing(&self, spacing: f32) {
        unsafe {
            textview_lspacing(self.as_ptr(), spacing);
        }
    }

    /// Sets a vertical space before the paragraph.
    pub fn bfspace(&self, space: f32) {
        unsafe {
            textview_bfspace(self.as_ptr(), space);
        }
    }

    /// Sets a vertical space after the paragraph.
    pub fn afspace(&self, space: f32) {
        unsafe {
            textview_afspace(self.as_ptr(), space);
        }
    }

    /// Applies the character and paragraph attributes to all text in the control. If there is no
    /// text, they will be taken as the default attributes of the text added using the keyboard.
    pub fn apply_all(&self) {
        unsafe {
            textview_apply_all(self.as_ptr());
        }
    }

    /// Applies character and paragraph attributes to selected text.
    pub fn apply_select(&self) {
        unsafe {
            textview_apply_select(self.as_ptr());
        }
    }

    /// Show or hide scroll bars.
    pub fn scroll_visible(&self, horizontal: bool, vertical: bool) {
        unsafe {
            textview_scroll_visible(self.as_ptr(), horizontal as _, vertical as _);
        }
    }

    /// Sets whether or not the control text is editable.
    pub fn editable(&self, editable: bool) {
        unsafe {
            textview_editable(self.as_ptr(), editable as _);
        }
    }

    /// Select text.
    ///
    /// # Remarks
    /// It works the same way as in Edit controls. See Text selection.
    pub fn select(&self, start: i32, end: i32) {
        unsafe {
            textview_select(self.as_ptr(), start, end);
        }
    }

    /// Sets whether to show or hide the text selection when keyboard focus is lost.
    ///
    /// # Remarks
    /// When lose keyboard focus, the control will retain the text selection. This feature only
    /// affects the visibility of the selection.
    pub fn show_select(&self, show: bool) {
        unsafe {
            textview_show_select(self.as_ptr(), show as _);
        }
    }

    /// Delete the selected text.
    /// # Remarks
    /// It has an effect similar to textview_cut, but without copying the eliminated text on the
    /// clipboard. See Select text.
    pub fn del_select(&self) {
        unsafe {
            textview_del_select(self.as_ptr());
        }
    }

    /// In texts that exceed the visible part, it scrolls to the position of the caret.
    pub fn scroll_caret(&self) {
        unsafe {
            textview_scroll_caret(self.as_ptr());
        }
    }

    /// Gets the text of the control.
    pub fn get_text(&self) -> String {
        let text = unsafe { textview_get_text(self.as_ptr()) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Copies the selected text to the clipboard.
    pub fn copy(&self) {
        unsafe {
            textview_copy(self.as_ptr());
        }
    }

    /// Cuts the selected text, copying it to the clipboard.
    pub fn cut(&self) {
        unsafe {
            textview_cut(self.as_ptr());
        }
    }

    /// Pastes the text from the clipboard into the caret position.
    pub fn paste(&self) {
        unsafe {
            textview_paste(self.as_ptr());
        }
    }

    /// Turn automatic text wrapping on or off.
    pub fn wrap(&self, wrap: bool) {
        unsafe {
            textview_wrap(self.as_ptr(), wrap as _);
        }
    }
}

impl_gui_control!(TextView, guicontrol_textview);
