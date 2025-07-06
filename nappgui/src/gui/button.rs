use std::{ffi::CString, rc::Rc};

use crate::{
    draw_2d::{Font, Image},
    types::GuiState,
    util::macros::{callback, impl_gui_control, pub_crate_ptr_ops},
};

use nappgui_sys::{
    button_OnClick, button_check, button_check3, button_flat, button_flatgle, button_font,
    button_get_height, button_get_state, button_image, button_image_alt, button_push, button_radio,
    button_state, button_text, button_text_alt, button_tooltip, button_vpadding,
};

macro_rules! button_basic {
    ($button_type: ty) => {
        pub_crate_ptr_ops!(*mut nappgui_sys::Button);

        callback! {
            /// Set a function for pressing the button.
            pub on_click($button_type) => button_OnClick;
        }

        /// Set the text that the button will display.
        ///
        /// # Remarks
        /// In flat buttons, the text will be displayed as tooltip.
        pub fn text(&self, text: &str) {
            let text = CString::new(text).unwrap();
            unsafe { button_text(self.as_ptr(), text.as_ptr()) };
        }

        /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
        pub fn tooltip(&self, text: &str) {
            let text = CString::new(text).unwrap();
            unsafe { button_tooltip(self.as_ptr(), text.as_ptr()) }
        }

        /// Sets the inner vertical margin.
        pub fn vpadding(&self, padding: f32) {
            unsafe { button_vpadding(self.as_ptr(), padding) }
        }

        /// Set the button font.
        pub fn font(&self, font: &Font) {
            unsafe { button_font(self.as_ptr(), font.inner) }
        }

        /// Gets the current height of the control.
        pub fn get_height(&self) -> f32 {
            unsafe { button_get_height(self.as_ptr()) }
        }
    };
}

macro_rules! button_alt {
    () => {
        /// Set an alternative text.
        ///
        /// # Remarks
        /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
        pub fn text_alt(&self, text: &str) {
            let text = CString::new(text).unwrap();
            unsafe { button_text_alt(self.as_ptr(), text.as_ptr()) }
        }

        /// Set an alternative image for the button.
        ///
        /// # Remarks
        /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
        pub fn image_alt(&self, image: &Image) {
            unsafe { button_image_alt(self.as_ptr(), image.inner) }
        }
    };
}

macro_rules! button_state {
    () => {
        /// Set the button status.
        ///
        /// # Remarks
        /// Not applicable on push buttons button_push.
        pub fn state(&self, state: GuiState) {
            unsafe { button_state(self.as_ptr(), state as _) }
        }

        /// Get button status.
        ///
        /// # Remarks
        /// Not applicable on push buttons button_push.
        pub fn get_state(&self) -> GuiState {
            let state = unsafe { button_get_state(self.as_ptr()) };
            GuiState::try_from(state).unwrap()
        }
    };
}

macro_rules! button_image {
    () => {
        /// Set the icon that will show the button.
        ///
        /// # Remarks
        /// Not applicable in checkbox or radiobutton. In flat buttons, the size of the control will be adjusted to the image.
        pub fn image(&self, image: &Image) {
            unsafe { button_image(self.as_ptr(), image.inner) }
        }
    };
}

/// The push button.
#[repr(transparent)]
pub struct PushButton {
    pub(crate) inner: Rc<*mut nappgui_sys::Button>,
}

impl PushButton {
    /// Create a push button.
    pub fn new(text: &str) -> Self {
        let button = unsafe { button_push() };
        let button = Self::from_raw(button);
        button.text(text);
        button
    }

    button_basic!(PushButton);
    button_image!();
}

/// The checkbox button.
#[repr(transparent)]
pub struct CheckButton {
    pub(crate) inner: Rc<*mut nappgui_sys::Button>,
}

impl CheckButton {
    /// Create a checkbox button.
    pub fn new(text: &str) -> Self {
        let button = unsafe { button_check() };
        let button = Self::from_raw(button);
        button.text(text);
        button
    }

    button_basic!(CheckButton);
    button_state!();
}

/// The checkbox button with three states.
#[repr(transparent)]
pub struct Check3Button {
    pub(crate) inner: Rc<*mut nappgui_sys::Button>,
}

impl Check3Button {
    /// Create a checkbox button.
    pub fn new(text: &str) -> Self {
        let button = unsafe { button_check3() };
        let button = Self::from_raw(button);
        button.text(text);
        button
    }

    button_basic!(Check3Button);
    button_state!();
}

/// The radio button
#[repr(transparent)]
pub struct RadioButton {
    pub(crate) inner: Rc<*mut nappgui_sys::Button>,
}

impl RadioButton {
    /// Create a radio button.
    pub fn new(text: &str) -> Self {
        let button = unsafe { button_radio() };
        let button = Self::from_raw(button);
        button.text(text);
        button
    }

    button_basic!(RadioButton);
    button_state!();
}

/// The flat button, to which an image can be assigned. It is the typical toolbar button.
#[repr(transparent)]
pub struct FlatButton {
    pub(crate) inner: Rc<*mut nappgui_sys::Button>,
}

impl FlatButton {
    /// Create a flat button.
    pub fn new(text: &str, image: &Image) -> Self {
        let button = unsafe { button_flat() };
        let button = Self::from_raw(button);
        button.text(text);
        button.image(image);
        button
    }

    button_basic!(FlatButton);
    button_image!();
}

/// The flat button with status. The button will alternate between pressed/released each time you click on it.
#[repr(transparent)]
pub struct FlatButtonEx {
    pub(crate) inner: Rc<*mut nappgui_sys::Button>,
}

impl FlatButtonEx {
    /// Create a flat button.
    pub fn new(text: &str, image: &Image) -> Self {
        let button = unsafe { button_flatgle() };
        let button = Self::from_raw(button);
        button.text(text);
        button.image(image);
        button
    }

    button_basic!(FlatButtonEx);
    button_image!();
    button_alt!();
}

impl_gui_control!(PushButton, guicontrol_button);
impl_gui_control!(CheckButton, guicontrol_button);
impl_gui_control!(Check3Button, guicontrol_button);
impl_gui_control!(RadioButton, guicontrol_button);
impl_gui_control!(FlatButton, guicontrol_button);
impl_gui_control!(FlatButtonEx, guicontrol_button);

/// The button trait.
pub trait ButtonTrait {
    /// Get inner button pointer.
    fn as_button_ptr(&self) -> *mut nappgui_sys::Button;
}

macro_rules! impl_button {
    ($type: ty) => {
        impl ButtonTrait for $type {
            fn as_button_ptr(&self) -> *mut nappgui_sys::Button {
                *self.inner
            }
        }
    };
}

impl_button!(PushButton);
impl_button!(CheckButton);
impl_button!(Check3Button);
impl_button!(RadioButton);
impl_button!(FlatButton);
impl_button!(FlatButtonEx);
