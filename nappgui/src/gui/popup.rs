use std::rc::Rc;

use crate::{
    draw_2d::image::Image,
    util::macros::{callback, impl_gui_control, pub_crate_ptr_ops},
};

use nappgui_sys::{
    popup_OnSelect, popup_add_elem, popup_clear, popup_count, popup_create, popup_get_selected,
    popup_get_text, popup_list_height, popup_selected, popup_set_elem, popup_tooltip,
};

/// PopUps are buttons that have a drop-down menu associated with them. Apparently they
/// look like pushbuttons that when pressed show a list of options. In Hello PopUp and PopUp!
/// you have an example of use.
pub struct PopUp {
    pub(crate) inner: Rc<*mut nappgui_sys::PopUp>,
}

impl PopUp {
    pub_crate_ptr_ops!(*mut nappgui_sys::PopUp);

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
            popup_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Add a new item to the popup list.
    pub fn add_elem(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_add_elem(self.as_ptr(), text.as_ptr(), image.inner);
        }
    }

    /// Edit an item from the drop-down list.
    pub fn set_elem(&self, index: usize, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_set_elem(self.as_ptr(), index as _, text.as_ptr(), image.inner);
        }
    }

    /// Remove all items from the dropdown list.
    pub fn clear(&self) {
        unsafe {
            popup_clear(self.as_ptr());
        }
    }

    /// Gets the number of items in the list.
    pub fn count(&self) -> usize {
        unsafe { popup_count(self.as_ptr()) as _ }
    }

    /// Set the size of the drop-down list.
    pub fn list_height(&self, elems: usize) {
        unsafe {
            popup_list_height(self.as_ptr(), elems as _);
        }
    }

    /// Set the selected popup element.
    pub fn selected(&self, index: usize) {
        unsafe {
            popup_selected(self.as_ptr(), index as _);
        }
    }

    /// Get the selected popup item.
    pub fn get_selected(&self) -> usize {
        unsafe { popup_get_selected(self.as_ptr()) as _ }
    }

    /// Gets the text of a popup element.
    pub fn get_text(&self, index: usize) -> String {
        let text = unsafe { popup_get_text(self.as_ptr(), index as _) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }
}

impl_gui_control!(PopUp, guicontrol_popup);
