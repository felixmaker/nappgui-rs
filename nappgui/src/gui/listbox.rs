use std::rc::Rc;

use crate::{
    draw_2d::{Color, Font, Image},
    util::macros::{callback, impl_gui_control, pub_crate_ptr_ops},
};
use nappgui_sys::{
    listbox_OnDown, listbox_OnSelect, listbox_add_elem, listbox_check, listbox_checkbox,
    listbox_checked, listbox_clear, listbox_color, listbox_count, listbox_create, listbox_del_elem,
    listbox_font, listbox_multisel, listbox_select, listbox_selected, listbox_set_elem,
    listbox_size, listbox_text, S2Df,
};

/// The ListBox are controls that display a series of elements as a list.
pub struct ListBox {
    pub(crate) inner: Rc<*mut nappgui_sys::ListBox>,
}

impl ListBox {
    pub_crate_ptr_ops!(*mut nappgui_sys::ListBox);

    /// Create a new list control.
    pub fn create() -> Self {
        let listbox = unsafe { listbox_create() };
        Self::new(listbox)
    }

    callback! {
        /// Sets a handler for a mouse button press.
        ///
        /// # Remarks
        /// This event is processed before listbox_OnSelect. In the tag field of Event the number of the
        /// element clicked will be received or UINT32_MAX if it corresponds to an empty area of the ListBox.
        /// If the event returns FALSE on event_result, the element will be prevented from being selected
        /// (TRUE by default). See GUI Events.
        pub on_down(ListBox) => listbox_OnDown;

        /// Set an event handler for the selection of a new item.
        pub on_select(ListBox) => listbox_OnSelect;
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
    pub fn checkbox(&self, show: bool) {
        unsafe {
            listbox_checkbox(self.as_ptr(), show as i8);
        }
    }

    /// Enable multiple selection.
    pub fn multisel(&self, enable: bool) {
        unsafe {
            listbox_multisel(self.as_ptr(), enable as i8);
        }
    }

    /// Adds a new element.
    pub fn add_elem(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_add_elem(self.as_ptr(), text.as_ptr(), image.inner);
        }
    }

    /// Edit a list item.
    pub fn set_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_set_elem(self.as_ptr(), index, text.as_ptr(), image.inner);
        }
    }

    /// Delete an item from the list.
    pub fn del_elem(&self, index: u32) {
        unsafe {
            listbox_del_elem(self.as_ptr(), index);
        }
    }

    /// Sets the font of the list.
    pub fn font(&self, font: &Font) {
        unsafe {
            listbox_font(self.as_ptr(), font.inner);
        }
    }

    /// Remove all items from the list.
    pub fn clear(&self) {
        unsafe {
            listbox_clear(self.as_ptr());
        }
    }

    /// Sets the text color of an element.
    pub fn color(&self, index: u32, color: &Color) {
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
            listbox_select(self.as_ptr(), index, select as i8);
        }
    }

    /// Check or uncheck the checkbox of the element from the program code.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    pub fn check(&self, index: u32, check: bool) {
        unsafe {
            listbox_check(self.as_ptr(), index, check as i8);
        }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Remarks
    /// The number of elements.
    pub fn count(&self) -> u32 {
        unsafe { listbox_count(self.as_ptr()) }
    }

    /// Returns the text of an element.
    pub fn text(&self, index: u32) -> String {
        unsafe {
            let text = listbox_text(self.as_ptr(), index);
            std::ffi::CStr::from_ptr(text)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Returns whether or not an element is selected.
    pub fn selected(&self, index: u32) -> bool {
        unsafe { listbox_selected(self.as_ptr(), index) != 0 }
    }

    /// Returns whether an element is checked or not.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    pub fn checked(&self, index: u32) -> bool {
        unsafe { listbox_checked(self.as_ptr(), index) != 0 }
    }
}

impl_gui_control!(ListBox, guicontrol_listbox);
