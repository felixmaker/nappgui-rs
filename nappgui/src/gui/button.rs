use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use crate::{
    draw_2d::{Font, Image, ImageTrait},
    gui::event::EvButton,
    types::{ButtonStyle, GuiState},
    util::macros::callback,
};

use nappgui_sys::{
    button_OnClick, button_check, button_check3, button_flat, button_flatgle, button_font, button_get_font,
    button_get_height, button_get_state, button_get_text, button_hpadding, button_image, button_image_alt, button_push,
    button_radio, button_state, button_text, button_text_alt, button_tooltip, button_vpadding, button_width,
};

pub(crate) struct ButtonInner {
    inner: *mut nappgui_sys::Button,
    style: ButtonStyle,
}

/// The button.
///
/// # Remarks
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct Button {
    inner: Arc<ButtonInner>,
}

impl Button {
    /// Create a push button, the typical [Accept], [Cancel], etc.
    pub fn new_push() -> Self {
        let button = unsafe { button_push() };
        unsafe { Self::from_raw(button, ButtonStyle::Push) }
    }

    /// Create a checkbox.
    pub fn new_check2() -> Self {
        let button = unsafe { button_check() };
        unsafe { Self::from_raw(button, ButtonStyle::Check2) }
    }

    /// Create a checkbox with three states.
    pub fn new_check3() -> Self {
        let button = unsafe { button_check3() };
        unsafe { Self::from_raw(button, ButtonStyle::Check3) }
    }

    /// Create a radio button.
    pub fn new_radio() -> Self {
        let button = unsafe { button_radio() };
        unsafe { Self::from_raw(button, ButtonStyle::Radio) }
    }

    /// Create a flat button, to which an image can be assigned. It is the typical toolbar button.
    pub fn new_flat() -> Self {
        let button = unsafe { button_flat() };
        unsafe { Self::from_raw(button, ButtonStyle::Flat) }
    }

    /// Create a flat button with status. The button will alternate between pressed/released each time you click on it.
    pub fn new_flatgle() -> Self {
        let button = unsafe { button_flatgle() };
        unsafe { Self::from_raw(button, ButtonStyle::Flatgle) }
    }

    callback! {
        /// Set a function for pressing the button.
        pub on_click(EvButton) => button_OnClick;
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
        if self.style() == ButtonStyle::Flatgle {
            let text = CString::new(text).unwrap();
            unsafe { button_text_alt(self.as_ptr(), text.as_ptr()) }
        }
    }

    /// Set the button font.
    pub fn set_font(&self, font: &Font) {
        unsafe { button_font(self.as_ptr(), font.inner) }
    }

    /// Set the icon that will show the button.
    ///
    /// # Remarks
    /// Not applicable in checkbox or radiobutton. In flat buttons, the size of the control will be adjusted to the image.
    pub fn set_image<T>(&self, image: &T)
    where
        T: ImageTrait,
    {
        unsafe { button_image(self.as_ptr(), image.as_ptr()) }
    }

    /// Set an alternative image for the button.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    pub fn set_image_alt<T>(&self, image: &T)
    where
        T: ImageTrait,
    {
        unsafe { button_image_alt(self.as_ptr(), image.as_ptr()) }
    }

    /// Set the button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    pub fn set_state(&self, state: GuiState) {
        unsafe { button_state(self.as_ptr(), state as _) }
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_tooltip(self.as_ptr(), text.as_ptr()) }
    }

    /// Sets the inner horizontal padding.
    pub fn set_hpadding(&self, padding: f32) {
        unsafe { button_hpadding(self.as_ptr(), padding) }
    }

    /// Sets the inner vertical margin.
    pub fn set_vpadding(&self, padding: f32) {
        unsafe { button_vpadding(self.as_ptr(), padding) }
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
        let txt = unsafe {
            let txt = button_get_text(self.as_ptr());
            CStr::from_ptr(txt)
        };
        txt.to_string_lossy().into_owned()
    }

    /// Get the button font.
    pub fn font(&self) -> Font {
        let font = unsafe { button_get_font(self.as_ptr()) };
        Font { inner: font as _ }
    }

    /// Gets the button icon.
    pub fn image(&self) -> Option<Image> {
        todo!()
    }

    /// Gets the alternative icon for the button.
    pub fn image_alt(&self) -> Option<Image> {
        todo!()
    }

    /// Gets the current height of the control.
    pub fn height(&self) -> f32 {
        unsafe { button_get_height(self.as_ptr()) }
    }

    /// Returns a raw pointer to the button object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Button {
        self.inner.inner
    }

    /// Get the button style.
    pub fn style(&self) -> ButtonStyle {
        self.inner.style
    }

    /// Create a button from a poniter.
    ///
    /// # Panics
    /// If the button is null, the func panic
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Button, style: ButtonStyle) -> Self {
        assert!(!ptr.is_null());
        Self {
            inner: Arc::new(ButtonInner { inner: ptr, style }),
        }
    }
}
