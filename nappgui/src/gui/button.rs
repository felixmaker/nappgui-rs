use std::ffi::CString;

use crate::{
    callback,
    draw_2d::{font::Font, image::Image},
};

use nappgui_sys::{
    button_OnClick, button_check, button_check3, button_flat, button_flatgle, button_font,
    button_get_height, button_get_state, button_get_tag, button_image, button_image_alt,
    button_push, button_radio, button_state, button_tag, button_text, button_text_alt,
    button_tooltip, button_vpadding, gui_state_t,
};

pub struct Button {
    pub(crate) inner: *mut nappgui_sys::Button,
}

impl Button {
    pub(crate) fn new(ptr: *mut nappgui_sys::Button) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a push button, the typical [Accept], [Cancel], etc.
    pub fn push() -> Self {
        let button = unsafe { button_push() };
        Self::new(button)
    }

    /// Create a checkbox.
    pub fn check() -> Self {
        let button = unsafe { button_check() };
        Self::new(button)
    }

    /// Create a checkbox with three states.
    pub fn check3() -> Self {
        let button = unsafe { button_check3() };
        Self::new(button)
    }

    /// Create a radio button.
    pub fn radio() -> Self {
        let button = unsafe { button_radio() };
        Self::new(button)
    }

    /// Create a flat button, to which an image can be assigned. It is the typical toolbar button.
    pub fn flat() -> Self {
        let button = unsafe { button_flat() };
        Self::new(button)
    }

    /// Create a flat button with status. The button will alternate between pressed/released each time you click on it.
    pub fn flatgle() -> Self {
        let button = unsafe { button_flatgle() };
        Self::new(button)
    }

    callback! {
        /// Set a function for pressing the button.
        pub on_click(Button) => button_OnClick;
    }

    /// Set the text that the button will display.
    ///
    /// # Remarks
    /// In flat buttons, the text will be displayed as tooltip.
    pub fn text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_text(self.inner, text.as_ptr()) };
    }

    /// Set an alternative text.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    pub fn text_alt(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_text_alt(self.inner, text.as_ptr()) }
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn tooltip(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_tooltip(self.inner, text.as_ptr()) }
    }

    /// Set the button font.
    pub fn font(&self, font: &Font) {
        unsafe { button_font(self.inner, font.inner) }
    }

    /// Set the icon that will show the button.
    ///
    /// # Remarks
    /// Not applicable in checkbox or radiobutton. In flat buttons, the size of the control will be adjusted to the image.
    pub fn image(&self, image: &Image) {
        unsafe { button_image(self.inner, image.inner) }
    }

    /// Set an alternative image for the button.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    pub fn image_alt(&self, image: &Image) {
        unsafe { button_image_alt(self.inner, image.inner) }
    }

    /// Set the button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    pub fn state(&self, state: gui_state_t) {
        unsafe { button_state(self.inner, state) }
    }

    /// Sets a numeric tag for the button.
    pub fn tag(&self, tag: u32) {
        unsafe { button_tag(self.inner, tag) }
    }

    /// Sets the inner vertical margin.
    pub fn vpadding(&self, padding: f32) {
        unsafe { button_vpadding(self.inner, padding) }
    }

    /// Get button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    pub fn get_state(&self) -> gui_state_t {
        unsafe { button_get_state(self.inner) }
    }

    /// Gets the button's tag.
    pub fn get_tag(&self) -> u32 {
        unsafe { button_get_tag(self.inner) }
    }

    /// Gets the current height of the control.
    pub fn get_height(&self) -> f32 {
        unsafe { button_get_height(self.inner) }
    }
}
