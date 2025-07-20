use std::ffi::CString;

use crate::{
    draw_2d::{Font, ImageTrait},
    gui::{event::EvButton, impl_layout},
    types::GuiState,
    util::macros::callback,
};

use nappgui_sys::{
    button_OnClick, button_check, button_check3, button_flat, button_flatgle, button_font,
    button_get_height, button_get_state, button_image, button_image_alt, button_push, button_radio,
    button_state, button_text, button_text_alt, button_tooltip, button_vpadding,
};

/// The button trait.
pub trait ButtonTrait {
    /// Returns a raw pointer to the button object.
    fn as_ptr(&self) -> *mut nappgui_sys::Button;

    callback! {
        /// Set a function for pressing the button.
        on_click(EvButton) => button_OnClick;
    }

    /// Set the text that the button will display.
    ///
    /// # Remarks
    /// In flat buttons, the text will be displayed as tooltip.
    fn text(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_text(self.as_ptr(), text.as_ptr()) };
    }

    /// Set a tooltip for the button. It is a small explanatory text that will appear when the mouse is over the control.
    fn tooltip(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_tooltip(self.as_ptr(), text.as_ptr()) }
    }

    /// Sets the inner vertical margin.
    fn vertical_padding(&self, padding: f32) {
        unsafe { button_vpadding(self.as_ptr(), padding) }
    }

    /// Set the button font.
    fn font(&self, font: &Font) {
        unsafe { button_font(self.as_ptr(), font.inner) }
    }

    /// Gets the current height of the control.
    fn get_height(&self) -> f32 {
        unsafe { button_get_height(self.as_ptr()) }
    }
}

/// The button alt trait.
pub trait ButtonAltTrait: ButtonTrait {
    /// Set an alternative text.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    fn text_alt(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { button_text_alt(self.as_ptr(), text.as_ptr()) }
    }

    /// Set an alternative image for the button.
    ///
    /// # Remarks
    /// Only applicable on flat buttons with status button_flatgle. It will be displayed when the button is in ekGUI_ON status.
    fn image_alt<T>(&self, image: &T)
    where
        T: ImageTrait,
    {
        unsafe { button_image_alt(self.as_ptr(), image.as_ptr()) }
    }
}

/// The button state trait.
pub trait ButtonStateTrait: ButtonTrait {
    /// Set the button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    fn state(&self, state: GuiState) {
        unsafe { button_state(self.as_ptr(), state as _) }
    }

    /// Get button status.
    ///
    /// # Remarks
    /// Not applicable on push buttons button_push.
    fn get_state(&self) -> GuiState {
        let state = unsafe { button_get_state(self.as_ptr()) };
        GuiState::try_from(state).unwrap()
    }
}

/// The button state trait.
pub trait ButtonImageTrait: ButtonTrait {
    /// Set the icon that will show the button.
    ///
    /// # Remarks
    /// Not applicable in checkbox or radiobutton. In flat buttons, the size of the control will be adjusted to the image.
    fn image<T>(&self, image: &T)
    where
        T: ImageTrait,
    {
        unsafe { button_image(self.as_ptr(), image.as_ptr()) }
    }
}

/// The push button. (no ownership on rust side)
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct PushButton {
    pub(crate) inner: *mut nappgui_sys::Button,
}

/// The checkbox button. (no ownership on rust side)
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct CheckButton {
    pub(crate) inner: *mut nappgui_sys::Button,
}

/// The checkbox button with three states. (no ownership on rust side)
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Check3Button {
    pub(crate) inner: *mut nappgui_sys::Button,
}

/// The radio button. (no ownership on rust side)
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct RadioButton {
    pub(crate) inner: *mut nappgui_sys::Button,
}

/// The flat button, to which an image can be assigned. It is the typical toolbar button. (no ownership on rust side)
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct FlatButton {
    pub(crate) inner: *mut nappgui_sys::Button,
}

impl FlatButton {
    /// Create a flat button.
    pub fn new<T>(text: &str, image: &T) -> Self
    where
        T: ImageTrait,
    {
        let button = Self {
            inner: unsafe { button_flat() },
        };
        button.text(text);
        button.image(image);
        button
    }
}

/// The flat button with status. The button will alternate between pressed/released each time you click on it.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct FlatButtonEx {
    pub(crate) inner: *mut nappgui_sys::Button,
}

impl FlatButtonEx {
    /// Create a flat button.
    pub fn new<T>(text: &str, image: &T) -> Self
    where
        T: ImageTrait,
    {
        let button = Self {
            inner: unsafe { button_flatgle() },
        };
        button.text(text);
        button.image(image);
        button
    }
}

macro_rules! impl_button {
    ($type: ty $(, $trait: ty)*) => {
        impl ButtonTrait for $type {
            fn as_ptr(&self) -> *mut nappgui_sys::Button {
                self.inner
            }
        }

        $(
            impl $trait for $type {}
        )*
    };
}

macro_rules! impl_button_new {
    ($type: ty, $func: ident) => {
        impl $type {
            /// Create a button.
            pub fn new(text: &str) -> Self {
                let button = Self {
                    inner: unsafe { $func() },
                };
                button.text(text);
                button
            }
        }
    };
}

impl_button!(PushButton, ButtonImageTrait);
impl_button!(CheckButton, ButtonStateTrait);
impl_button!(Check3Button, ButtonStateTrait);
impl_button!(RadioButton, ButtonStateTrait);
impl_button!(FlatButton, ButtonImageTrait);
impl_button!(FlatButtonEx, ButtonImageTrait, ButtonAltTrait);

impl_button_new!(PushButton, button_push);
impl_button_new!(CheckButton, button_check);
impl_button_new!(Check3Button, button_check3);
impl_button_new!(RadioButton, button_radio);

impl_layout!(PushButton, ButtonTrait, layout_button);
impl_layout!(CheckButton, ButtonTrait, layout_button);
impl_layout!(Check3Button, ButtonTrait, layout_button);
impl_layout!(RadioButton, ButtonTrait, layout_button);
impl_layout!(FlatButton, ButtonTrait, layout_button);
impl_layout!(FlatButtonEx, ButtonTrait, layout_button);
