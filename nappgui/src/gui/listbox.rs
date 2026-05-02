use std::{ffi::CStr, ptr::NonNull};

use crate::{
    draw_2d::{Color, Font, Image},
    gui::event::{EvButton, EvMouse},
    util::macros::callback,
};
use nappgui_sys::{
    listbox_OnDown, listbox_OnSelect, listbox_add_elem, listbox_check, listbox_checkbox, listbox_checked,
    listbox_clear, listbox_color, listbox_count, listbox_create, listbox_del_elem, listbox_font, listbox_get_image,
    listbox_get_selected, listbox_get_text, listbox_multisel, listbox_select, listbox_selected, listbox_set_elem,
    listbox_size, S2Df,
};

/// The ListBox are controls that display a series of elements as a list.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct ListBox(NonNull<nappgui_sys::ListBox>);

impl ListBox {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::ListBox) -> Self {
        Self(NonNull::new(ptr).unwrap())
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::ListBox {
        self.0.as_ptr()
    }

    /// Create a new list control.
    pub fn new() -> Self {
        let listbox = unsafe { listbox_create() };
        unsafe { Self::from_raw(listbox) }
    }

    callback! {
        /// Sets a handler for a mouse button press.
        ///
        /// # Remarks
        /// This event is processed before listbox_OnSelect. In the tag field of Event the number of the
        /// element clicked will be received or UINT32_MAX if it corresponds to an empty area of the ListBox.
        /// If the event returns FALSE on event_result, the element will be prevented from being selected
        /// (TRUE by default). See GUI Events.
        pub on_down(EvMouse) -> bool => listbox_OnDown;

        /// Set an event handler for the selection of a new item.
        pub on_select(EvButton) => listbox_OnSelect;
    }

    /// Set the default size of the list.
    ///
    /// # Remarks
    /// It corresponds to Natural sizing of control Default 128x128.
    pub fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe {
            listbox_size(self.as_ptr(), size);
        }
    }

    /// Show or hide checkboxes to the left of items.
    pub fn set_checkbox(&self, show: bool) {
        unsafe {
            listbox_checkbox(self.as_ptr(), show as _);
        }
    }

    /// Enable multiple selection.
    pub fn set_multiselect(&self, enable: bool) {
        unsafe {
            listbox_multisel(self.as_ptr(), enable as _);
        }
    }

    /// Adds a new element.
    pub fn add_element(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_add_elem(self.as_ptr(), text.as_ptr(), std::ptr::null());
        }
    }

    /// Adds a new element with image.
    pub fn add_image_element(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_add_elem(self.as_ptr(), text.as_ptr(), image.as_ptr());
        }
    }

    /// Edit a list item.
    pub fn set_element(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_set_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null());
        }
    }

    /// Edit a list item with image.
    pub fn set_image_element(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_set_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr());
        }
    }

    /// Delete an item from the list.
    pub fn delete_element(&self, index: u32) {
        unsafe {
            listbox_del_elem(self.as_ptr(), index);
        }
    }

    /// Sets the font of the list.
    pub fn set_font(&self, font: &Font) {
        unsafe { listbox_font(self.as_ptr(), font.as_ptr()) }
    }

    /// Remove all items from the list.
    pub fn clear(&self) {
        unsafe {
            listbox_clear(self.as_ptr());
        }
    }

    /// Sets the text color of an element.
    pub fn set_color(&self, index: u32, color: Color) {
        unsafe {
            listbox_color(self.as_ptr(), index, color.inner);
        }
    }

    /// Select an item from the program code.
    ///
    /// # Remarks
    /// If multiple selection is not enabled, selecting one item implies de-selecting all the others.
    pub fn select(&self, index: u32, select: bool) {
        unsafe {
            listbox_select(self.as_ptr(), index, select as _);
        }
    }

    /// Check or uncheck the checkbox of the element from the program code.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    pub fn check(&self, index: u32, check: bool) {
        unsafe {
            listbox_check(self.as_ptr(), index, check as _);
        }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Remarks
    /// The number of elements.
    pub fn count(&self) -> usize {
        unsafe { listbox_count(self.as_ptr()) as _ }
    }

    /// Returns whether or not an element is selected.
    pub fn is_selected(&self, index: u32) -> bool {
        unsafe { listbox_selected(self.as_ptr(), index) != 0 }
    }

    /// Returns whether an element is checked or not.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    pub fn is_checked(&self, index: u32) -> bool {
        unsafe { listbox_checked(self.as_ptr(), index) != 0 }
    }

    /// Returns the text of an element.
    pub fn text(&self, index: u32) -> String {
        let text = unsafe { listbox_get_text(self.as_ptr(), index) };
        unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() }
    }

    /// Gets the icon of an element.
    pub fn image(&self, index: u32) -> Option<Image> {
        let image = unsafe { listbox_get_image(self.as_ptr(), index) };
        if image.is_null() {
            None
        } else {
            Some(unsafe { Image::from_raw_cloned(image) })
        }
    }

    /// Gets the selected element.
    ///
    /// # Remarks
    /// This function is not valid for multiple selection lists.
    pub fn selected(&self) -> Option<u32> {
        let index = unsafe { listbox_get_selected(self.as_ptr()) };
        if index == u32::MAX {
            None
        } else {
            Some(index as _)
        }
    }
}
