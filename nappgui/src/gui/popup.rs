use crate::{
    draw_2d::ImageTrait,
    gui::{control::impl_control, event::EvButton, impl_layout},
    util::macros::callback,
};

use nappgui_sys::{
    popup_OnSelect, popup_add_elem, popup_clear, popup_count, popup_create, popup_get_selected,
    popup_get_text, popup_list_height, popup_selected, popup_set_elem, popup_tooltip,
};

/// The popup trait
pub trait PopUpTrait {
    /// Returns a raw pointer to the popup object.
    fn as_ptr(&self) -> *mut nappgui_sys::PopUp;

    callback! {
        /// Set an event handler for the selection of a new item.
         on_select(EvButton) => popup_OnSelect;
    }

    /// Assign a tooltip to the popup control.
    fn tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_tooltip(self.as_ptr(), text.as_ptr());
        }
    }

    /// Add a new item to the popup list.
    fn add_element(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_add_elem(self.as_ptr(), text.as_ptr(), std::ptr::null());
        }
    }

    /// Add a new item with image to the popup list.
    fn add_image_element<T>(&self, text: &str, image: &T)
    where
        T: ImageTrait,
    {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_add_elem(self.as_ptr(), text.as_ptr(), image.as_ptr());
        }
    }

    /// Edit an item from the drop-down list.
    fn set_element(&self, index: usize, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_set_elem(self.as_ptr(), index as _, text.as_ptr(), std::ptr::null());
        }
    }

    /// Edit an item with image from the drop-down list.
    fn set_image_element<T>(&self, index: usize, text: &str, image: &T)
    where
        T: ImageTrait,
    {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            popup_set_elem(self.as_ptr(), index as _, text.as_ptr(), image.as_ptr());
        }
    }

    /// Remove all items from the dropdown list.
    fn clear(&self) {
        unsafe {
            popup_clear(self.as_ptr());
        }
    }

    /// Gets the number of items in the list.
    fn count(&self) -> usize {
        unsafe { popup_count(self.as_ptr()) as _ }
    }

    /// Set the size of the drop-down list.
    fn list_height(&self, elems: usize) {
        unsafe {
            popup_list_height(self.as_ptr(), elems as _);
        }
    }

    /// Set the selected popup element.
    fn selected(&self, index: usize) {
        unsafe {
            popup_selected(self.as_ptr(), index as _);
        }
    }

    /// Get the selected popup item.
    fn get_selected(&self) -> usize {
        unsafe { popup_get_selected(self.as_ptr()) as _ }
    }

    /// Gets the text of a popup element.
    fn get_text(&self, index: usize) -> String {
        let text = unsafe { popup_get_text(self.as_ptr(), index as _) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }
}

/// PopUps are buttons that have a drop-down menu associated with them. Apparently they
/// look like pushbuttons that when pressed show a list of options. In Hello PopUp and PopUp!
/// you have an example of use.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct PopUp {
    pub(crate) inner: *mut nappgui_sys::PopUp,
}

impl PopUpTrait for PopUp {
    fn as_ptr(&self) -> *mut nappgui_sys::PopUp {
        self.inner
    }
}

impl PopUp {
    /// Create a new popup control (PopUp button).
    pub fn new() -> Self {
        let popup = unsafe { popup_create() };
        Self { inner: popup }
    }
}

impl_control!(PopUp, guicontrol_popup);
impl_layout!(PopUp, PopUpTrait, layout_popup);
