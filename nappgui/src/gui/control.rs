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
///
/// # Usage
/// ```rust
/// impl_control!(Button, Button, guicontrol_button, ControlType::Button);
/// ```
///
/// Parameters:
/// - First: Rust widget struct name
/// - Second: C control type from nappgui_sys
/// - Third: guicontrol conversion function (e.g., guicontrol_button)
/// - Fourth: ControlType enum variant
macro_rules! impl_control {
    ($rust_type:ident, $c_type:ident, $guicontrol_func:ident, $control_type:expr) => {
        impl Control for $rust_type {
            type CControlType = nappgui_sys::$c_type;

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
                    Some(unsafe { Self::from_raw(c_ptr) })
                }
            }

            fn control_type(&self) -> ControlType {
                $control_type
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
impl_control!(Button, Button, guicontrol_button, ControlType::Button);
impl_control!(Combo, Combo, guicontrol_combo, ControlType::Combo);
impl_control!(Edit, Edit, guicontrol_edit, ControlType::Edit);
impl_control!(ImageView, ImageView, guicontrol_imageview, ControlType::ImageView);
impl_control!(Label, Label, guicontrol_label, ControlType::Label);
impl_control!(Panel, Panel, guicontrol_panel, ControlType::Panel);
impl_control!(ListBox, ListBox, guicontrol_listbox, ControlType::ListBox);
impl_control!(PopUp, PopUp, guicontrol_popup, ControlType::PopUp);
impl_control!(Progress, Progress, guicontrol_progress, ControlType::Progress);
impl_control!(Slider, Slider, guicontrol_slider, ControlType::Slider);
impl_control!(SplitView, SplitView, guicontrol_splitview, ControlType::SplitView);
impl_control!(TableView, TableView, guicontrol_tableview, ControlType::TableView);
impl_control!(TextView, TextView, guicontrol_textview, ControlType::TextView);
impl_control!(UpDown, UpDown, guicontrol_updown, ControlType::UpDown);
impl_control!(View, View, guicontrol_view, ControlType::View);
impl_control!(WebView, WebView, guicontrol_webview, ControlType::WebView);
impl_control!(Line, Line, guicontrol_line, ControlType::Line);
