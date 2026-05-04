use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    ptr::NonNull,
    rc::{Rc, Weak},
};

use crate::{
    draw_2d::{Font, Image},
    gui::{event::ButtonEvent, global_get, global_record},
    types::GuiState,
    util::macros::listener,
};

use nappgui_sys::{
    button_OnClick, button_check, button_check3, button_flat, button_flatgle, button_font, button_get_font,
    button_get_height, button_get_image, button_get_image_alt, button_get_state, button_get_text, button_hpadding,
    button_image, button_image_alt, button_push, button_radio, button_state, button_text, button_text_alt,
    button_tooltip, button_vpadding, button_width,
};

pub(crate) struct ButtonInner {
    ptr: NonNull<nappgui_sys::Button>,
    on_click: RefCell<Option<Rc<dyn Fn(&ButtonEvent) + 'static>>>,
}

impl ButtonInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Button) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to ButtonInner::from_raw"),
            on_click: RefCell::new(None),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Button {
        self.ptr.as_ptr()
    }
}

/// The button control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct Button(Weak<ButtonInner>);

impl Button {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Button) -> Self {
        let object = global_record(ptr as _, ButtonInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::Button) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Button {
        self.0.upgrade().map(|button| button.as_ptr()).unwrap()
    }

    /// Create a push button, the typical [Accept], [Cancel], etc.
    pub fn new(text: &str) -> Self {
        let button = unsafe { Self::from_raw(button_push()) };
        button.set_text(text);
        button
    }

    /// Create a checkbox.
    pub fn new_check(text: &str) -> Self {
        let button = unsafe { Self::from_raw(button_check()) };
        button.set_text(text);
        button
    }

    /// Create a checkbox with three states.
    pub fn new_check3(text: &str) -> Self {
        let button = unsafe { Self::from_raw(button_check3()) };
        button.set_text(text);
        button
    }

    /// Create a radio button.
    pub fn new_radio(text: &str) -> Self {
        let button = unsafe { Self::from_raw(button_radio()) };
        button.set_text(text);
        button
    }

    /// Create a flat button, to which an image can be assigned. It is the typical toolbar button.
    pub fn new_flat(icon: &Image) -> Self {
        let button = unsafe { Self::from_raw(button_flat()) };
        button.set_image(icon);
        button
    }

    /// Create a flat button with status. The button will alternate between pressed/released each time you click on it.
    pub fn new_flatgle(icon: &Image) -> Self {
        let button = unsafe { Self::from_raw(button_flatgle()) };
        button.set_image(icon);
        button
    }

    /// Set a function for pressing the button.
    pub fn set_on_click_handler<F>(&self, callback: F)
    where
        F: Fn(&ButtonEvent) + 'static,
    {
        self.0
            .upgrade()
            .map(|button| *button.on_click.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.as_ptr(), ButtonInner, on_click(ButtonEvent));
        unsafe { button_OnClick(self.as_ptr(), listener) };
    }

    /// Set the default width of a push button.
    ///
    /// # Remarks
    /// The size of the click button is automatically calculated according to the text it contains. With this function we can
    /// set a width greater than the calculated one. It does not apply to other types of buttons (flat, check, radio).
    pub fn set_width(&self, width: f32) {
        unsafe { button_width(self.as_ptr(), width) };
    }

    /// Set the text that the button will display.
    ///
    /// # Remarks
    /// In flat buttons, the text will be displayed as tooltip.
    pub fn set_text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_text(self.as_ptr(), text.as_ptr()) };
    }

    /// Set an alternative text.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    pub fn set_text_alt(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_text_alt(self.as_ptr(), text.as_ptr()) };
    }

    /// Set the button font.
    pub fn set_font(&self, font: &Font) {
        unsafe { button_font(self.as_ptr(), font.as_ptr()) };
    }

    /// Set the icon that will show the button.
    ///
    /// # Remarks
    /// Not applicable in checkbox or radiobutton. In flat buttons, the size of the control will be adjusted to the image.
    pub fn set_image(&self, image: &Image) {
        unsafe { button_image(self.as_ptr(), image.as_ptr()) };
    }

    /// Set an alternative image for the button.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    pub fn set_image_alt(&self, image: &Image) {
        unsafe { button_image_alt(self.as_ptr(), image.as_ptr()) };
    }

    /// Set the button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    pub fn set_state(&self, state: GuiState) {
        unsafe { button_state(self.as_ptr(), state as _) };
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_tooltip(self.as_ptr(), text.as_ptr()) };
    }

    /// Sets the inner horizontal padding.
    pub fn set_hpadding(&self, padding: f32) {
        unsafe { button_hpadding(self.as_ptr(), padding) };
    }

    /// Sets the inner vertical margin.
    pub fn set_vpadding(&self, padding: f32) {
        unsafe { button_vpadding(self.as_ptr(), padding) };
    }

    /// Get button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    pub fn state(&self) -> GuiState {
        let state = unsafe { button_get_state(self.as_ptr()) };
        GuiState::try_from(state).unwrap()
    }

    /// Get the button text.
    pub fn text(&self) -> String {
        let text = unsafe { button_get_text(self.as_ptr()) };
        let text = unsafe { CStr::from_ptr(text) };
        text.to_string_lossy().into_owned()
    }

    /// Get the button font.
    pub fn font(&self) -> Font {
        let font = unsafe { button_get_font(self.as_ptr()) };
        unsafe { Font::from_raw_cloned(font as _) }
    }

    /// Gets the button icon.
    pub fn image(&self) -> Option<Image> {
        let image = unsafe { button_get_image(self.as_ptr()) };
        if image.is_null() {
            return None;
        }
        Some(unsafe { Image::from_raw_cloned(image) })
    }

    /// Gets the alternative icon for the button.
    pub fn image_alt(&self) -> Option<Image> {
        let image = unsafe { button_get_image_alt(self.as_ptr()) };
        if image.is_null() {
            return None;
        }
        Some(unsafe { Image::from_raw_cloned(image) })
    }

    /// Gets the current height of the control.
    pub fn height(&self) -> f32 {
        unsafe { button_get_height(self.as_ptr()) }
    }
}
