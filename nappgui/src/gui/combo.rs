use std::{cell::RefCell, rc::Rc};

use crate::{
    draw_2d::{Color, Image},
    gui::{
        event::{TextEvent, TextFilterEvent},
        impl_control, GUID,
    },
    types::{Align, FontStyle},
    util::macros::listener,
};

use nappgui_sys::{
    combo_OnChange, combo_OnFilter, combo_add_elem, combo_align, combo_autoselect, combo_bgcolor, combo_bgcolor_focus,
    combo_clear, combo_color, combo_color_focus, combo_copy, combo_count, combo_create, combo_cut, combo_del_elem,
    combo_editable, combo_get_image, combo_get_selected, combo_get_text, combo_ins_elem, combo_list_height,
    combo_passmode, combo_paste, combo_phcolor, combo_phstyle, combo_phtext, combo_select, combo_selected,
    combo_set_elem, combo_text, combo_tooltip, combo_width,
};

#[derive(Default)]
pub(crate) struct ComboInner {
    ptr: RefCell<*mut nappgui_sys::Combo>,
    on_filter: RefCell<Option<Rc<dyn Fn(&TextEvent) -> TextFilterEvent + 'static>>>,
    on_change: RefCell<Option<Rc<dyn Fn(&TextEvent) -> bool + 'static>>>,
}

/// The combo control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Combo(GUID);

impl_control!(Combo, ComboInner);

impl Combo {
    /// Create a combo control.
    pub fn new() -> Self {
        let combo = unsafe { combo_create() };
        unsafe { Self::from_raw(combo) }
    }

    /// Set a function to filter the text while editing.
    pub fn set_on_filter_handler<F>(&self, callback: F)
    where
        F: Fn(&TextEvent) -> TextFilterEvent + 'static,
    {
        self.inner(|object| *object.on_filter.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.0, Combo, on_filter(TextEvent) -> TextFilterEvent);
        unsafe { combo_OnFilter(self.as_ptr(), listener) };
    }

    /// Set a function to be called when the text has changed.
    pub fn set_on_change_handler<F>(&self, callback: F)
    where
        F: Fn(&TextEvent) -> bool + 'static,
    {
        self.inner(|object| *object.on_change.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.as_ptr(), Combo, on_change(TextEvent) -> bool);
        unsafe { combo_OnChange(self.as_ptr(), listener) };
    }

    /// Set the default control width.
    ///
    /// # Remarks
    /// The default width of an EditBox will be 100px. This value can be modified by this function.
    pub fn set_width(&self, width: f32) {
        unsafe { combo_width(self.as_ptr(), width) };
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

    /// Activate the password mode, which will hide the typed characters.
    pub fn set_passmode(&self, passmode: bool) {
        unsafe { combo_passmode(self.as_ptr(), passmode as _) }
    }

    /// Enable or disable editing in the control.
    pub fn set_editable(&self, editable: bool) {
        unsafe { combo_editable(self.as_ptr(), editable as _) }
    }

    /// Activate or deactivate auto-selection of text.
    pub fn set_autoselect(&self, autoselect: bool) {
        unsafe { combo_autoselect(self.as_ptr(), autoselect as _) }
    }

    /// Select text.
    pub fn select(&self, start: i32, end: i32) {
        unsafe { combo_select(self.as_ptr(), start, end) }
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

    /// Copies the selected text to the clipboard.
    pub fn copy(&self) {
        unsafe { combo_copy(self.as_ptr()) }
    }

    /// Cuts the selected text, copying it to the clipboard.
    pub fn cut(&self) {
        unsafe { combo_cut(self.as_ptr()) }
    }

    /// Pastes the text from the clipboard into the caret position.
    pub fn paste(&self) {
        unsafe { combo_paste(self.as_ptr()) }
    }

    /// Add a new item to the drop-down list.
    pub fn add_element(&self, text: &str, image: Option<&Image>) {
        let text = std::ffi::CString::new(text).unwrap();
        if let Some(image) = image {
            unsafe { combo_add_elem(self.as_ptr(), text.as_ptr(), image.as_ptr()) }
        } else {
            unsafe { combo_add_elem(self.as_ptr(), text.as_ptr(), std::ptr::null()) }
        }
    }

    /// Edit an item from the drop-down list.
    pub fn set_element(&self, index: u32, text: &str, image: Option<&Image>) {
        let text = std::ffi::CString::new(text).unwrap();
        if let Some(image) = image {
            unsafe { combo_set_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr()) }
        } else {
            unsafe { combo_set_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null()) }
        }
    }

    /// Insert an item in the drop-down list.
    pub fn insert_element(&self, index: u32, text: &str, image: Option<&Image>) {
        let text = std::ffi::CString::new(text).unwrap();
        if let Some(image) = image {
            unsafe { combo_ins_elem(self.as_ptr(), index, text.as_ptr(), image.as_ptr()) }
        } else {
            unsafe { combo_ins_elem(self.as_ptr(), index, text.as_ptr(), std::ptr::null()) }
        }
    }

    /// Remove an item from the drop-down list.
    pub fn delete_element(&self, index: u32) {
        unsafe { combo_del_elem(self.as_ptr(), index) }
    }

    /// Remove all items from the dropdown list.
    pub fn clear(&self) {
        unsafe { combo_clear(self.as_ptr()) }
    }

    /// Gets the number of items in the list.
    pub fn count(&self) -> u32 {
        unsafe { combo_count(self.as_ptr()) }
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
