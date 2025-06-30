use std::rc::Rc;

use crate::{draw_2d::Image, prelude::Align, util::macros::{callback, pub_crate_ptr_ops}};

use nappgui_sys::{
    combo_OnChange, combo_OnFilter, combo_add_elem, combo_align, combo_bgcolor,
    combo_bgcolor_focus, combo_color, combo_color_focus, combo_count, combo_create, combo_del_elem,
    combo_duplicates, combo_get_text, combo_ins_elem, combo_phcolor, combo_phstyle, combo_phtext,
    combo_set_elem, combo_text, combo_tooltip,
};

/// ComboBox are text editing boxes with drop-down list.
pub struct Combo {
    pub(crate) inner: Rc<*mut nappgui_sys::Combo>,
}

impl Combo {
    pub_crate_ptr_ops!(*mut nappgui_sys::Combo, Rc<*mut nappgui_sys::Combo>);

    /// Create a combo control.
    pub fn create() -> Self {
        let combo = unsafe { combo_create() };
        Self::new(combo)
    }

    callback! {
        /// Set a function to filter the text while editing.
        pub on_filter(Combo) => combo_OnFilter;

        /// Set a function to be called when the text has changed.
        ///
        /// # Remarks
        /// This event will also be launched when you select an item from the list, a sign that the text has changed
        /// in the edit box. See Validate texts and GUI Events.
        pub on_change(Combo) => combo_OnChange;
    }

    /// Set the combo edit text.
    pub fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set text alignment.
    pub fn align(&self, align: Align) {
        unsafe {
            combo_align(self.as_ptr(), align);
        }
    }

    /// Assign a tooltip to the control combo.
    pub fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the color of the combo text.
    pub fn color(&self, color: u32) {
        unsafe {
            combo_color(self.as_ptr(), color);
        }
    }

    /// Sets the color of the text, when the control has the keyboard focus.
    pub fn color_focus(&self, color: u32) {
        unsafe {
            combo_color_focus(self.as_ptr(), color);
        }
    }

    /// Set the background color.
    pub fn bgcolor(&self, color: u32) {
        unsafe {
            combo_bgcolor(self.as_ptr(), color);
        }
    }

    /// Sets the background color when the control has keyboard focus.
    pub fn bgcolor_focus(&self, color: u32) {
        unsafe {
            combo_bgcolor_focus(self.as_ptr(), color);
        }
    }

    /// Set an explanatory text for when the control is blank.
    pub fn phtext(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_phtext(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the color of the placeholder text.
    pub fn phcolor(&self, color: u32) {
        unsafe {
            combo_phcolor(self.as_ptr(), color);
        }
    }

    /// Set the font style for the placeholder.
    pub fn phstyle(&self, style: u32) {
        unsafe {
            combo_phstyle(self.as_ptr(), style);
        }
    }

    /// Get control text.
    pub fn get_text(&self, index: u32) -> String {
        let text = unsafe { combo_get_text(self.as_ptr(), index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the number of items in the dropdown list.
    pub fn count(&self) -> u32 {
        unsafe { combo_count(self.as_ptr()) }
    }

    /// Add a new item to the drop-down list.
    pub fn add_elem(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_add_elem(self.as_ptr(), text.as_ptr(), image.inner);
        }
    }

    /// Edit an item from the drop-down list.
    pub fn set_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_set_elem(self.as_ptr(), index, text.as_ptr(), image.inner);
        }
    }

    /// Insert an item in the drop-down list.
    pub fn ins_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_ins_elem(self.as_ptr(), index, text.as_ptr(), image.inner);
        }
    }

    /// Remove an item from the drop-down list.
    pub fn del_elem(&self, index: u32) {
        unsafe {
            combo_del_elem(self.as_ptr(), index);
        }
    }

    /// Prevents duplicate texts from the drop-down list.
    pub fn duplicates(&self, duplicates: bool) {
        unsafe {
            combo_duplicates(self.as_ptr(), duplicates as i8);
        }
    }
}
