use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use crate::{
    draw_2d::Image,
    gui::{event::EvButton, global_get, global_record},
    util::macros::callback,
};

use nappgui_sys::{
    popup_OnSelect, popup_add_elem, popup_clear, popup_count, popup_create, popup_get_image, popup_get_selected,
    popup_get_text, popup_list_height, popup_selected, popup_set_elem, popup_tooltip,
};

pub(crate) struct PopUpInner {
    ptr: NonNull<nappgui_sys::PopUp>,
}

impl PopUpInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::PopUp) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to PopUpInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::PopUp {
        self.ptr.as_ptr()
    }
}

/// The popup control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct PopUp(Weak<PopUpInner>);

impl PopUp {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::PopUp) -> Self {
        let object = global_record(ptr as _, PopUpInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::PopUp) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    /// Returns the underlying raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::PopUp {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a new popup control (PopUp button).
    pub fn new() -> Self {
        let popup = unsafe { popup_create() };
        unsafe { Self::from_raw(popup) }
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
    pub fn add_image_element(&self, text: &str, image: &Image) {
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
    pub fn set_image_element(&self, index: u32, text: &str, image: &Image) {
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

    /// Gets the icon of a popup element.
    pub fn image(&self, index: u32) -> Option<Image> {
        let image = unsafe { popup_get_image(self.as_ptr(), index) };
        if image.is_null() {
            return None;
        }
        Some(unsafe { Image::from_raw_cloned(image) })
    }
}
