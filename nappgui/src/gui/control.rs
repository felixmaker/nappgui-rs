use std::sync::Arc;

use nappgui_sys::{guicontrol_get_tag, guicontrol_tag};

use super::*;

pub(crate) struct ControlInner {
    inner: *mut nappgui_sys::GuiControl,
}

/// The control object.
#[allow(unused)]
pub struct Control {
    pub(crate) inner: Arc<ControlInner>,
}

impl Control {
    /// Sets a tag for the control.
    pub fn set_tag(&self, tag: u32) {
        unsafe { guicontrol_tag(self.as_ptr(), tag) };
    }

    /// Gets a tag for the control.
    pub fn tag(&self) -> u32 {
        unsafe { guicontrol_get_tag(self.as_ptr()) }
    }

    /// Returns the raw pointer to the control.
    pub fn as_ptr(&self) -> *mut nappgui_sys::GuiControl {
        self.inner.inner
    }

    /// Create a control from raw pointer.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::GuiControl) -> Self {
        assert!(!ptr.is_null());
        Self {
            inner: Arc::new(ControlInner { inner: ptr }),
        }
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

impl_control!(Button, guicontrol_button);
impl_control!(Combo, guicontrol_combo);
impl_control!(Edit, guicontrol_edit);
impl_control!(ImageView, guicontrol_imageview);
impl_control!(Label, guicontrol_label);
impl_control!(Panel, guicontrol_panel);
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
