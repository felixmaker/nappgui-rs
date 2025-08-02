use crate::{
    draw_2d::{Color, ImageTrait},
    gui::{
        control::impl_control,
        event::{EvText, EvTextFilter},
    },
    types::{Align, FontStyle},
    util::macros::callback,
};

use nappgui_sys::{
    combo_OnChange, combo_OnFilter, combo_add_elem, combo_align, combo_bgcolor,
    combo_bgcolor_focus, combo_color, combo_color_focus, combo_count, combo_create, combo_del_elem,
    combo_duplicates, combo_get_text, combo_ins_elem, combo_phcolor, combo_phstyle, combo_phtext,
    combo_set_elem, combo_text, combo_tooltip,
};

/// The combo trait.
pub trait ComboTrait {
    /// Returns a raw pointer to the combo object.
    fn as_ptr(&self) -> *mut nappgui_sys::Combo;

    callback! {
        /// Set a function to filter the text while editing.
        on_filter(EvText) -> EvTextFilter => combo_OnFilter;

        /// Set a function to be called when the text has changed.
        ///
        /// # Remarks
        /// This event will also be launched when you select an item from the list, a sign that the text has changed
        /// in the edit box. See Validate texts and GUI Events.
        on_change(EvText) -> bool => combo_OnChange;
    }

    /// Set the combo edit text.
    fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_text(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set text alignment.
    fn align(&self, align: Align) {
        unsafe {
            combo_align(self.as_ptr(), align as _);
        }
    }

    /// Assign a tooltip to the control combo.
    fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the color of the combo text.
    fn color(&self, color: Color) {
        unsafe {
            combo_color(self.as_ptr(), color.inner);
        }
    }

    /// Sets the color of the text, when the control has the keyboard focus.
    fn color_focus(&self, color: Color) {
        unsafe {
            combo_color_focus(self.as_ptr(), color.inner);
        }
    }

    /// Set the background color.
    fn background_color(&self, color: Color) {
        unsafe {
            combo_bgcolor(self.as_ptr(), color.inner);
        }
    }

    /// Sets the background color when the control has keyboard focus.
    fn background_color_focus(&self, color: Color) {
        unsafe {
            combo_bgcolor_focus(self.as_ptr(), color.inner);
        }
    }

    /// Set an explanatory text for when the control is blank.
    fn placeholder_text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_phtext(self.as_ptr(), text.as_ptr());
        }
    }

    /// Set the color of the placeholder text.
    fn placeholder_color(&self, color: Color) {
        unsafe {
            combo_phcolor(self.as_ptr(), color.inner);
        }
    }

    /// Set the font style for the placeholder.
    fn placeholder_style(&self, style: FontStyle) {
        unsafe {
            combo_phstyle(self.as_ptr(), style.to_fstyle_t());
        }
    }

    /// Get control text.
    fn get_text(&self, index: u32) -> String {
        let text = unsafe { combo_get_text(self.as_ptr(), index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the number of items in the dropdown list.
    fn count(&self) -> u32 {
        unsafe { combo_count(self.as_ptr()) }
    }

    /// Add a new item to the drop-down list.
    fn add_element(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_add_elem(self.as_ptr(), text.as_ptr(), std::ptr::null());
        }
    }

    /// Add a new item with image to the drop-down list.
    fn add_image_element<T>(&self, text: &str, image: &T)
    where
        T: ImageTrait,
    {
        let text = std::ffi::CString::new(text).unwrap();

        unsafe {
            combo_add_elem(self.as_ptr(), text.as_ptr(), image.as_ptr());
        }
    }

    /// Edit an item from the drop-down list.
    fn set_element(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_set_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null());
        }
    }

    /// Edit an item with image from the drop-down list.
    fn set_image_element<T>(&self, index: u32, text: &str, image: &T)
    where
        T: ImageTrait,
    {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_set_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr());
        }
    }

    /// Insert an item in the drop-down list.
    fn insert_element(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_ins_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null());
        }
    }

    /// Insert an item with image in the drop-down list.
    fn insert_image_element<T>(&self, index: u32, text: &str, image: &T)
    where
        T: ImageTrait,
    {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            combo_ins_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr());
        }
    }

    /// Remove an item from the drop-down list.
    fn delete_element(&self, index: u32) {
        unsafe {
            combo_del_elem(self.as_ptr(), index);
        }
    }

    /// Prevents duplicate texts from the drop-down list.
    fn duplicates(&self, duplicates: bool) {
        unsafe {
            combo_duplicates(self.as_ptr(), duplicates as _);
        }
    }
}

/// ComboBox are text editing boxes with drop-down list.
#[derive(Clone, Copy, Debug)]
pub struct Combo {
    pub(crate) inner: *mut nappgui_sys::Combo,
}

impl ComboTrait for Combo {
    fn as_ptr(&self) -> *mut nappgui_sys::Combo {
        self.inner
    }
}

impl Combo {
    /// Create a combo control.
    pub fn new() -> Self {
        let combo = Self {
            inner: unsafe { combo_create() },
        };
        combo
    }
}

impl_control!(Combo, guicontrol_combo);
