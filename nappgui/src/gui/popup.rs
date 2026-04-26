use std::sync::Arc;

use crate::{draw_2d::Image, gui::event::EvButton, util::macros::callback};

use nappgui_sys::{
    popup_OnSelect, popup_add_elem, popup_clear, popup_count, popup_create, popup_get_selected, popup_get_text,
    popup_list_height, popup_selected, popup_set_elem, popup_tooltip,
};

/// The popup type.
pub(crate) struct PopupInner {
    inner: *mut nappgui_sys::PopUp,
}

/// PopUps are buttons that have a drop-down menu associated with them. Apparently they
/// look like pushbuttons that when pressed show a list of options. In Hello PopUp and PopUp!
/// you have an example of use.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone)]
pub struct PopUp {
    pub(crate) inner: Arc<PopupInner>,
}

impl PopUp {
    /// Create a new popup control (PopUp button).
    pub fn new() -> Self {
        let popup = unsafe { popup_create() };
        assert!(!popup.is_null());
        Self {
            inner: Arc::new(PopupInner { inner: popup }),
        }
    }

    callback! {
        /// Set an event handler for the selection of a new item.
        pub on_select(EvButton) => popup_OnSelect;
    }

    /// Assign a tooltip to the popup control.
    pub fn set_tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Add a new item to the popup list.
    pub fn add_element(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_add_elem(self.as_ptr(), text.as_ptr(), std::ptr::null());
        }
    }

    /// Add a new item with image to the popup list.
    pub fn add_image_element(&self, text: &str, image: &Image)
    {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_add_elem(self.as_ptr(), text.as_ptr(), image.as_ptr());
        }
    }

    /// Edit an item from the drop-down list.
    pub fn set_element(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_set_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null());
        }
    }

    /// Edit an item with image from the drop-down list.
    pub fn set_image_element(&self, index: u32, text: &str, image: &Image)
    {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_set_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr());
        }
    }

    /// Remove all items from the dropdown list.
    pub fn clear(&self) {
        unsafe {
            popup_clear(self.as_ptr());
        }
    }

    /// Gets the number of items in the list.
    pub fn count(&self) -> u32 {
        unsafe { popup_count(self.as_ptr()) }
    }

    /// Set the size of the drop-down list.
    pub fn set_list_height(&self, elems: u32) {
        unsafe {
            popup_list_height(self.as_ptr(), elems);
        }
    }

    /// Set the selected popup element.
    pub fn set_selected(&self, index: u32) {
        unsafe {
            popup_selected(self.as_ptr(), index);
        }
    }

    /// Get the selected popup item.
    pub fn selected(&self) -> u32 {
        unsafe { popup_get_selected(self.as_ptr()) }
    }

    /// Gets the text of a popup element.
    pub fn text(&self, index: u32) -> String {
        let text = unsafe { popup_get_text(self.as_ptr(), index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Returns a raw pointer to the popup object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::PopUp {
        self.inner.inner
    }
}
