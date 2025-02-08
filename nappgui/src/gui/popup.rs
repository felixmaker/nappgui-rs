use crate::{draw_2d::image::Image, util::macros::callback};

use nappgui_sys::{
    popup_OnSelect, popup_add_elem, popup_clear, popup_count, popup_create, popup_get_selected,
    popup_get_text, popup_list_height, popup_selected, popup_set_elem, popup_tooltip,
};

/// PopUps are buttons that have a drop-down menu associated with them. Apparently they
/// look like pushbuttons that when pressed show a list of options. In Hello PopUp and PopUp!
/// you have an example of use.
pub struct PopUp {
    pub(crate) inner: *mut nappgui_sys::PopUp,
}

impl PopUp {
    pub(crate) fn new(ptr: *mut nappgui_sys::PopUp) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new popup control (PopUp button).
    pub fn create() -> Self {
        let popup = unsafe { popup_create() };
        Self::new(popup)
    }

    callback! {
        /// Set an event handler for the selection of a new item.
        pub on_select(PopUp) => popup_OnSelect;
    }

    /// Assign a tooltip to the popup control.
    pub fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_tooltip(self.inner, text.as_ptr());
        }
    }

    /// Add a new item to the popup list.
    pub fn add_elem(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_add_elem(self.inner, text.as_ptr(), image.inner);
        }
    }

    /// Edit an item from the drop-down list.
    pub fn set_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_set_elem(self.inner, index, text.as_ptr(), image.inner);
        }
    }

    /// Remove all items from the dropdown list.
    pub fn clear(&self) {
        unsafe {
            popup_clear(self.inner);
        }
    }

    /// Gets the number of items in the list.
    pub fn count(&self) -> u32 {
        unsafe { popup_count(self.inner) }
    }

    /// Set the size of the drop-down list.
    pub fn list_height(&self, elems: u32) {
        unsafe {
            popup_list_height(self.inner, elems);
        }
    }

    /// Set the selected popup element.
    pub fn selected(&self, index: u32) {
        unsafe {
            popup_selected(self.inner, index);
        }
    }

    /// Get the selected popup item.
    pub fn get_selected(&self) -> u32 {
        unsafe { popup_get_selected(self.inner) }
    }

    /// Gets the text of a popup element.
    pub fn get_text(&self, index: u32) -> String {
        let text = unsafe { popup_get_text(self.inner, index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }
}
