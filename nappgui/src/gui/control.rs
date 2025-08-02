use nappgui_sys::{guicontrol_get_tag, guicontrol_tag};

use super::*;

/// The control object.
pub struct Control {
    pub(crate) inner: *mut nappgui_sys::GuiControl,
}

/// Gets the base object from a derived control.
pub trait ControlTrait {
    /// Returns the raw pointer to the control.
    fn as_ptr(&self) -> *mut nappgui_sys::GuiControl;

    /// Sets a tag for the control.
    fn tag(&self, tag: u32)
    where
        Self: Sized,
    {
        unsafe { guicontrol_tag(self.as_ptr(), tag) };
    }

    /// Gets a tag for the control.
    fn get_tag(&self) -> u32
    where
        Self: Sized,
    {
        unsafe { guicontrol_get_tag(self.as_ptr()) }
    }
}

impl ControlTrait for Control {
    fn as_ptr(&self) -> *mut nappgui_sys::GuiControl {
        self.inner
    }
}

impl AsRef<Control> for Control {
    fn as_ref(&self) -> &Control {
        self
    }
}

macro_rules! impl_control {
    ($type: ty, $func: ident) => {
        impl AsRef<Control> for $type {
            fn as_ref(&self) -> &Control {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl TryFrom<&Control> for &$type {
            type Error = crate::error::NappguiError;

            fn try_from(value: &Control) -> Result<Self, Self::Error> {
                let control = unsafe { nappgui_sys::$func(value.as_ptr()) };
                if control.is_null() {
                    Err(crate::error::NappguiError::Internal(
                        crate::error::NappguiErrorKind::WrongTypeTransmute,
                    ))
                } else {
                    Ok(unsafe { std::mem::transmute(value) })
                }
            }
        }
    };
}

impl_control!(PushButton, guicontrol_button);
impl_control!(CheckButton, guicontrol_button);
impl_control!(Check3Button, guicontrol_button);
impl_control!(RadioButton, guicontrol_button);
impl_control!(FlatButton, guicontrol_button);
impl_control!(FlatButtonEx, guicontrol_button);
impl_control!(Combo, guicontrol_combo);
impl_control!(Edit, guicontrol_edit);
impl_control!(ImageView, guicontrol_imageview);
impl_control!(Label, guicontrol_label);
impl_control!(Panel, guicontrol_panel);
impl_control!(ScrollPanel, guicontrol_panel);
impl_control!(ListBox, guicontrol_listbox);
impl_control!(PopUp, guicontrol_popup);
impl_control!(Progress, guicontrol_progress);
impl_control!(Slider, guicontrol_slider);
impl_control!(SplitView, guicontrol_splitview);
impl_control!(TableView, guicontrol_tableview);
impl_control!(TextView, guicontrol_textview);
impl_control!(UpDown, guicontrol_updown);
impl_control!(View, guicontrol_view);
impl_control!(WebView, guicontrol_webview);
