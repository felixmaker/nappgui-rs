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
    Line,
}

/// Control trait for all controls in NAppGUI.
pub trait Control {
    /// The C Control type.
    type CControlType;

    /// Returns the underlying raw pointer.
    fn as_ptr(&self) -> *mut Self::CControlType;
    /// Convert the control to a pointer to the control type.
    fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl {
        self.as_ptr() as *mut nappgui_sys::GuiControl
    }
    /// From a pointer to the control type, create a control.
    fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Option<Self>
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

/// Macro to implement the `Control` trait for widget types.
macro_rules! impl_control {
    ($rust_type:ident, $guicontrol_func:ident) => {
        impl Control for $rust_type {
            type CControlType = nappgui_sys::$rust_type;

            fn as_ptr(&self) -> *mut Self::CControlType {
                self.as_ptr()
            }

            fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Option<Self>
            where
                Self: Sized,
            {
                let c_ptr = unsafe { nappgui_sys::$guicontrol_func(pointer) };
                if c_ptr.is_null() {
                    None
                } else {
                    Some(unsafe { Self::from_ptr(c_ptr) })
                }
            }

            fn control_type(&self) -> ControlType {
                ControlType::$rust_type
            }
        }
    };
}

// Import all widget types
use crate::gui::{
    Button, Combo, Edit, ImageView, Label, Line, ListBox, Panel, PopUp, Progress, Slider, SplitView, TableView,
    TextView, UpDown, View, WebView,
};

// Implement Control for all widgets using the macro
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
impl_control!(Line, guicontrol_line);
