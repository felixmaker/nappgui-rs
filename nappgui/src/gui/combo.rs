use std::{
    cell::RefCell,
    ptr::NonNull,
    rc::{Rc, Weak},
};

use crate::{
    draw_2d::{Color, Image},
    gui::{
        event::{EvText, EvTextFilter},
        global_get, global_record,
    },
    types::{Align, FontStyle},
    util::macros::listener,
};

use nappgui_sys::{
    combo_OnChange, combo_OnFilter, combo_add_elem, combo_align, combo_bgcolor, combo_bgcolor_focus, combo_clear,
    combo_color, combo_color_focus, combo_count, combo_create, combo_del_elem, combo_get_image, combo_get_selected,
    combo_get_text, combo_ins_elem, combo_list_height, combo_phcolor, combo_phstyle, combo_phtext, combo_selected,
    combo_set_elem, combo_text, combo_tooltip,
};

pub(crate) struct ComboInner {
    ptr: NonNull<nappgui_sys::Combo>,
    on_filter: RefCell<Option<Rc<dyn Fn(&EvText) -> EvTextFilter + 'static>>>,
    on_change: RefCell<Option<Rc<dyn Fn(&EvText) -> bool + 'static>>>,
}

impl ComboInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Combo) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to ComboInner::from_raw"),
            on_filter: RefCell::new(None),
            on_change: RefCell::new(None),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Combo {
        self.ptr.as_ptr()
    }
}

/// The combo control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct Combo(Weak<ComboInner>);

impl Combo {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Combo) -> Self {
        let object = global_record(ptr as _, ComboInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::Combo) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Combo {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a combo control.
    pub fn new() -> Self {
        let combo = unsafe { combo_create() };
        unsafe { Self::from_raw(combo) }
    }

    /// Set a function to filter the text while editing.
    pub fn set_on_filter_handler<F>(&self, callback: F)
    where
        F: Fn(&EvText) -> EvTextFilter + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_filter.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.as_ptr(), ComboInner, on_filter(EvText) -> EvTextFilter);
        unsafe { combo_OnFilter(self.as_ptr(), listener) };
    }

    /// Set a function to be called when the text has changed.
    pub fn set_on_change_handler<F>(&self, callback: F)
    where
        F: Fn(&EvText) -> bool + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_change.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.as_ptr(), ComboInner, on_change(EvText) -> bool);
        unsafe { combo_OnChange(self.as_ptr(), listener) };
    }

    /// Set the combo edit text.
    pub fn set_text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_text(self.as_ptr(), text.as_ptr()) }
    }

    /// Set text alignment.
    pub fn set_align(&self, align: Align) {
        unsafe { combo_align(self.as_ptr(), align as _) }
    }

    /// Assign a tooltip to the control combo.
    pub fn set_tooltip(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_tooltip(self.as_ptr(), text.as_ptr()) }
    }

    /// Set the color of the combo text.
    pub fn set_color(&self, color: Color) {
        unsafe { combo_color(self.as_ptr(), color.inner) }
    }

    /// Sets the color of the text, when the control has the keyboard focus.
    pub fn set_color_focus(&self, color: Color) {
        unsafe { combo_color_focus(self.as_ptr(), color.inner) }
    }

    /// Set the background color.
    pub fn set_background_color(&self, color: Color) {
        unsafe { combo_bgcolor(self.as_ptr(), color.inner) }
    }

    /// Sets the background color when the control has keyboard focus.
    pub fn set_background_color_focus(&self, color: Color) {
        unsafe { combo_bgcolor_focus(self.as_ptr(), color.inner) }
    }

    /// Set an explanatory text for when the control is blank.
    pub fn set_placeholder_text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_phtext(self.as_ptr(), text.as_ptr()) }
    }

    /// Set the color of the placeholder text.
    pub fn set_placeholder_color(&self, color: Color) {
        unsafe { combo_phcolor(self.as_ptr(), color.inner) }
    }

    /// Set the font style for the placeholder.
    pub fn set_placeholder_style(&self, style: FontStyle) {
        unsafe { combo_phstyle(self.as_ptr(), style.to_fstyle_t()) }
    }

    /// Get the element text by index.
    pub fn element_text(&self, index: u32) -> String {
        let text = unsafe { combo_get_text(self.as_ptr(), index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the number of items in the dropdown list.
    pub fn count(&self) -> u32 {
        unsafe { combo_count(self.as_ptr()) }
    }

    /// Add a new item to the drop-down list.
    pub fn add_element(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_add_elem(self.as_ptr(), text.as_ptr(), std::ptr::null()) }
    }

    /// Add a new item with image to the drop-down list.
    pub fn add_image_element(&self, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();

        unsafe { combo_add_elem(self.as_ptr(), text.as_ptr(), image.as_ptr()) }
    }

    /// Edit an item from the drop-down list.
    pub fn set_element(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_set_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null()) }
    }

    /// Edit an item with image from the drop-down list.
    pub fn set_image_element(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_set_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr()) }
    }

    /// Insert an item in the drop-down list.
    pub fn insert_element(&self, index: u32, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_ins_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null()) }
    }

    /// Insert an item with image in the drop-down list.
    pub fn insert_image_element(&self, index: u32, text: &str, image: &Image) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { combo_ins_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr()) }
    }

    /// Remove an item from the drop-down list.
    pub fn delete_element(&self, index: u32) {
        unsafe { combo_del_elem(self.as_ptr(), index) }
    }

    /// Remove all items from the dropdown list.
    pub fn clear(&self) {
        unsafe { combo_clear(self.as_ptr()) }
    }

    /// Set the size of the drop-down list.
    pub fn set_list_height(&self, size: u32) {
        unsafe { combo_list_height(self.as_ptr(), size) }
    }

    /// Set the selected combo element.
    pub fn set_selected(&self, index: u32) {
        unsafe { combo_selected(self.as_ptr(), index) }
    }

    /// Get the selected combo item.
    pub fn selected(&self) -> u32 {
        unsafe { combo_get_selected(self.as_ptr()) }
    }

    /// Gets the text of a combo element.
    pub fn text(&self, index: u32) -> String {
        let text = unsafe { combo_get_text(self.as_ptr(), index) };
        let text = unsafe { std::ffi::CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Gets the icon of a combo element.
    pub fn image(&self, index: u32) -> Option<Image> {
        let image = unsafe { combo_get_image(self.as_ptr(), index) };
        if image.is_null() {
            return None;
        }
        Some(unsafe { Image::from_raw_cloned(image) })
    }
}
