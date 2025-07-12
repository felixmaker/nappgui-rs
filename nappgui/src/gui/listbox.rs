use crate::{
    draw_2d::{Color, Font, Image},
    gui::{
        control::impl_control,
        event::{EvButton, EvMouse},
        impl_layout,
    },
    util::macros::callback,
};
use nappgui_sys::{
    listbox_OnDown, listbox_OnSelect, listbox_add_elem, listbox_check, listbox_checkbox,
    listbox_checked, listbox_clear, listbox_color, listbox_count, listbox_create, listbox_del_elem,
    listbox_font, listbox_multisel, listbox_select, listbox_selected, listbox_set_elem,
    listbox_size, listbox_text, S2Df,
};

/// The listbox trait.
pub trait ListBoxTrait {
    /// Returns a raw pointer to the list box object.
    fn as_ptr(&self) -> *mut nappgui_sys::ListBox;

    callback! {
        /// Sets a handler for a mouse button press.
        ///
        /// # Remarks
        /// This event is processed before listbox_OnSelect. In the tag field of Event the number of the
        /// element clicked will be received or UINT32_MAX if it corresponds to an empty area of the ListBox.
        /// If the event returns FALSE on event_result, the element will be prevented from being selected
        /// (TRUE by default). See GUI Events.
        on_down(EvMouse) -> bool => listbox_OnDown;

        /// Set an event handler for the selection of a new item.
        on_select(EvButton) => listbox_OnSelect;
    }

    /// Set the default size of the list.
    ///
    /// # Remarks
    /// It corresponds to Natural sizing of control Default 128x128.
    fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe {
            listbox_size(self.as_ptr(), size);
        }
    }

    /// Show or hide checkboxes to the left of items.
    fn checkbox(&self, show: bool) {
        unsafe {
            listbox_checkbox(self.as_ptr(), show as _);
        }
    }

    /// Enable multiple selection.
    fn multisel(&self, enable: bool) {
        unsafe {
            listbox_multisel(self.as_ptr(), enable as _);
        }
    }

    /// Adds a new element.
    fn add_elem(&self, text: &str, image: Option<&Image>) {
        let text = std::ffi::CString::new(text).unwrap();
        let image = match image {
            Some(value) => value.inner,
            None => std::ptr::null(),
        };
        unsafe {
            listbox_add_elem(self.as_ptr(), text.as_ptr(), image);
        }
    }

    /// Edit a list item.
    fn set_elem(&self, index: usize, text: &str, image: Option<&Image>) {
        let text = std::ffi::CString::new(text).unwrap();
        let image = match image {
            Some(value) => value.inner,
            None => std::ptr::null(),
        };
        unsafe {
            listbox_set_elem(self.as_ptr(), index as _, text.as_ptr(), image);
        }
    }

    /// Delete an item from the list.
    fn del_elem(&self, index: usize) {
        unsafe {
            listbox_del_elem(self.as_ptr(), index as _);
        }
    }

    /// Sets the font of the list.
    fn font(&self, font: &Font) {
        unsafe {
            listbox_font(self.as_ptr(), font.inner);
        }
    }

    /// Remove all items from the list.
    fn clear(&self) {
        unsafe {
            listbox_clear(self.as_ptr());
        }
    }

    /// Sets the text color of an element.
    fn color(&self, index: usize, color: Color) {
        unsafe {
            listbox_color(self.as_ptr(), index as _, color.inner);
        }
    }

    /// Select an item from the program code.
    ///
    /// # Remarks
    /// If multiple selection is not enabled, selecting one item implies de-selecting all the others.
    fn select(&self, index: usize, select: bool) {
        unsafe {
            listbox_select(self.as_ptr(), index as _, select as _);
        }
    }

    /// Check or uncheck the checkbox of the element from the program code.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    fn check(&self, index: usize, check: bool) {
        unsafe {
            listbox_check(self.as_ptr(), index as _, check as _);
        }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Remarks
    /// The number of elements.
    fn count(&self) -> usize {
        unsafe { listbox_count(self.as_ptr()) as _ }
    }

    /// Returns the text of an element.
    fn text(&self, index: usize) -> String {
        unsafe {
            let text = listbox_text(self.as_ptr(), index as _);
            std::ffi::CStr::from_ptr(text)
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Returns whether or not an element is selected.
    fn selected(&self, index: usize) -> bool {
        unsafe { listbox_selected(self.as_ptr(), index as _) != 0 }
    }

    /// Returns whether an element is checked or not.
    ///
    /// # Remarks
    /// Checking an item is independent of selecting it. Items can be marked even if checkboxes are not
    /// visible. See listbox_checkbox.
    fn checked(&self, index: usize) -> bool {
        unsafe { listbox_checked(self.as_ptr(), index as _) != 0 }
    }
}

/// The ListBox are controls that display a series of elements as a list.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ListBox {
    pub(crate) inner: *mut nappgui_sys::ListBox,
}

impl ListBoxTrait for ListBox {
    fn as_ptr(&self) -> *mut nappgui_sys::ListBox {
        self.inner
    }
}

impl ListBox {
    /// Create a new list control.
    pub fn new() -> Self {
        let listbox = unsafe { listbox_create() };
        Self { inner: listbox }
    }
}

impl_control!(ListBox, guicontrol_listbox);
impl_layout!(ListBox, ListBoxTrait, layout_listbox);
