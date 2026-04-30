use nappgui_sys::{guicontrol_get_tag, guicontrol_tag};

/// Control type.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ControlType {
    Button = 0,
    Combo,
    Edit,
    ImageView,
    Label,
    Panel,
    ListBox,
    PopUp,
    Progress,
    Slider,
    SplitView,
    TableView,
    TextView,
    UpDown,
    View,
    WebView,
}

pub(crate) trait Control {
    type CControlType;

    fn control_type(&self) -> ControlType;
    fn as_ptr(&self) -> *mut Self::CControlType;
    unsafe fn from_raw(pointer: *mut Self::CControlType) -> Self
    where
        Self: Sized;
}

/// Control trait for all controls in NAppGUI.
pub trait AsControl {
    /// Convert the control to a pointer to the control type.
    fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl;
    /// From a pointer to the control type, create a control.
    unsafe fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Self
    where
        Self: Sized;
    /// Gets the control type.
    fn control_type(&self) -> ControlType;
    /// Gets the tag of the control.
    fn tag(&self) -> u32 {
        unsafe { guicontrol_get_tag(self.as_control_ptr()) }
    }
    /// Sets a tag for the control.
    fn set_tag(&mut self, tag: u32) {
        unsafe { guicontrol_tag(self.as_control_ptr(), tag) };
    }
}

impl<T> AsControl for T
where
    T: Control,
{
    fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl {
        self.as_ptr() as *mut nappgui_sys::GuiControl
    }

    fn control_type(&self) -> ControlType {
        Control::control_type(self)
    }

    unsafe fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Self
    where
        Self: Sized,
    {
        Self::from_raw(pointer as *mut <T as Control>::CControlType)
    }
}
