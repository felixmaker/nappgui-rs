use crate::{
    callback,
    draw_2d::{color::Color, font::Font, image::Image},
};
use nappgui_sys::{
    listbox_OnDown, listbox_OnSelect, listbox_add_elem, listbox_check, listbox_checkbox,
    listbox_checked, listbox_clear, listbox_color, listbox_count, listbox_create, listbox_del_elem,
    listbox_font, listbox_multisel, listbox_select, listbox_selected, listbox_set_elem,
    listbox_size, listbox_text, S2Df,
};

pub struct ListBox {
    pub(crate) inner: *mut nappgui_sys::ListBox,
}

impl ListBox {
    pub(crate) fn new(ptr: *mut nappgui_sys::ListBox) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

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
            listbox_size(self.inner, size);
        }
    }

    /// Show or hide checkboxes to the left of items.
    pub fn checkbox(&self, show: bool) {
        unsafe {
            listbox_checkbox(self.inner, show as i8);
        }
    }

    /// Enable multiple selection.
    pub fn multisel(&self, enable: bool) {
        unsafe {
            listbox_multisel(self.inner, enable as i8);
        }
    }

    /// Adds a new element.
    pub fn add_elem(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_add_elem(self.inner, text.as_ptr(), image.inner);
        }
    }

    /// Edit a list item.
    pub fn set_elem(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe {
            listbox_set_elem(self.inner, index, text.as_ptr(), image.inner);
        }
    }

    /// Delete an item from the list.
    pub fn del_elem(&self, index: u32) {
        unsafe {
            listbox_del_elem(self.inner, index);
        }
    }

    /// Sets the font of the list.
    pub fn font(&self, font: &Font) {
        unsafe {
            listbox_font(self.inner, font.inner);
        }
    }

    /// Remove all items from the list.
    pub fn clear(&self) {
        unsafe {
            listbox_clear(self.inner);
        }
    }

    /// Sets the text color of an element.
    pub fn color(&self, index: u32, color: &Color) {
        unsafe {
            listbox_color(self.inner, index, color.inner);
        }
    }

    /// Select an item from the program code.
    ///
    /// # Remarks
    /// If multiple selection is not enabled, selecting one item implies de-selecting all the others.
    pub fn select(&self, index: u32, select: bool) {
        unsafe {
            listbox_select(self.inner, index, select as i8);
        }
    }

    /// Check or uncheck the checkbox of the element from the program code.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    pub fn check(&self, index: u32, check: bool) {
        unsafe {
            listbox_check(self.inner, index, check as i8);
        }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Remarks
    /// The number of elements.
    pub fn count(&self) -> u32 {
        unsafe { listbox_count(self.inner) }
    }

    /// Returns the text of an element.
    pub fn text(&self, index: u32) -> String {
        unsafe {
            let text = listbox_text(self.inner, index);
            std::ffi::CStr::from_ptr(text)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Returns whether or not an element is selected.
    pub fn selected(&self, index: u32) -> bool {
        unsafe { listbox_selected(self.inner, index) != 0 }
    }

    /// Returns whether an element is checked or not.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    pub fn checked(&self, index: u32) -> bool {
        unsafe { listbox_checked(self.inner, index) != 0 }
    }
}
