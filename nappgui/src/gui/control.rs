use nappgui_sys::{guicontrol_get_tag, guicontrol_tag};

/// Gets the base object from a derived control.
pub trait ControlTrait {
    /// Sets a tag for the control.
    fn tag(&self, tag: u32)
    where
        Self: Sized,
    {
        unsafe { guicontrol_tag(self.as_control_ptr(), tag) };
    }

    /// Gets a tag for the control.
    fn get_tag(&self) -> u32
    where
        Self: Sized,
    {
        unsafe { guicontrol_get_tag(self.as_control_ptr()) }
    }

    /// Returns the pointer to the control.
    fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl
    where
        Self: Sized;

    /// Returns the control from the pointer.
    fn from_control_ptr(ptr: *mut nappgui_sys::GuiControl) -> Option<Self>
    where
        Self: Sized,
        Option<Self>: Sized;
}

macro_rules! impl_control {
    ($type: ty, $func: ident) => {
        impl crate::gui::ControlTrait for $type {
            fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl {
                self.inner as _
            }

            fn from_control_ptr(ptr: *mut nappgui_sys::GuiControl) -> Option<Self> {
                let result = unsafe { nappgui_sys::$func(ptr) };
                if !result.is_null() {
                    Some(Self { inner: result })
                } else {
                    None
                }
            }
        }
    };
}

pub(crate) use impl_control;
