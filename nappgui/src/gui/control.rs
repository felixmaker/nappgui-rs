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
    /// Convert the control to a pointer to the control type.
    fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl;
    /// From a pointer to the control type, create a control.
    fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Option<Self>
    where
        Self: Sized;
    // /// The C Control type.
    // type CControlType;

    // /// Returns the underlying raw pointer.
    // fn as_ptr(&self) -> *mut Self::CControlType;
    // /// Convert the control to a pointer to the control type.
    // fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl {
    //     self.as_ptr() as *mut nappgui_sys::GuiControl
    // }
    // /// From a pointer to the control type, create a control.
    // fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Option<Self>
    // where
    //     Self: Sized;
    // /// Gets the control type.
    // fn control_type(&self) -> ControlType;
}

/// Gets the control.
pub(crate) fn control_uid(ptr: *mut nappgui_sys::GuiControl) -> Option<GUID> {
    let uid = unsafe { nappgui_sys::guicontrol_get_tag(ptr) };
    if uid == u32::MAX {
        None
    } else {
        Some(uid)
    }
}

// /// Macro to implement the `Control` trait for widget types.
// macro_rules! impl_control {
//     ($rust_type:ident, $guicontrol_func:ident) => {
//         impl Control for $rust_type {
//             type CControlType = nappgui_sys::$rust_type;

//             fn as_ptr(&self) -> *mut Self::CControlType {
//                 self.as_ptr()
//             }

//             fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Option<Self>
//             where
//                 Self: Sized,
//             {
//                 let c_ptr = unsafe { nappgui_sys::$guicontrol_func(pointer) };
//                 if c_ptr.is_null() {
//                     None
//                 } else {
//                     Some(unsafe { Self::from_ptr(c_ptr) })
//                 }
//             }

//             fn control_type(&self) -> ControlType {
//                 ControlType::$rust_type
//             }
//         }
//     };
// }

// // Import all widget types
// use crate::gui::{
//     Button, Combo, Edit, ImageView, Label, Line, ListBox, Panel, PopUp, Progress, Slider, SplitView, TableView,
//     TextView, UpDown, View, WebView,
// };

// // Implement Control for all widgets using the macro
// impl_control!(Button, guicontrol_button);
// impl_control!(Combo, guicontrol_combo);
// impl_control!(Edit, guicontrol_edit);
// impl_control!(ImageView, guicontrol_imageview);
// impl_control!(Label, guicontrol_label);
// impl_control!(Panel, guicontrol_panel);
// impl_control!(ListBox, guicontrol_listbox);
// impl_control!(PopUp, guicontrol_popup);
// impl_control!(Progress, guicontrol_progress);
// impl_control!(Slider, guicontrol_slider);
// impl_control!(SplitView, guicontrol_splitview);
// impl_control!(TableView, guicontrol_tableview);
// impl_control!(TextView, guicontrol_textview);
// impl_control!(UpDown, guicontrol_updown);
// impl_control!(View, guicontrol_view);
// impl_control!(WebView, guicontrol_webview);
// impl_control!(Line, guicontrol_line);

/// Macro to implement the `Control` trait for widget types.
macro_rules! impl_control {
    ($type:ident, $type_ex:ident) => {
        impl crate::gui::Control for $type {
            fn as_control_ptr(&self) -> *mut nappgui_sys::GuiControl {
                self.as_ptr() as _
            }

            fn from_control_ptr(pointer: *mut nappgui_sys::GuiControl) -> Option<Self>
            where
                Self: Sized,
            {
                Self::from_ptr(pointer as _)
            }
        }

        #[allow(dead_code)]
        impl $type {
            pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::$type) -> Self {
                let uid = crate::gui::global_guid();
                crate::gui::global_record(uid, crate::gui::GObject::$type($type_ex::from_raw(ptr)));
                Self(uid)
            }

            pub(crate) fn from_ptr(ptr: *mut nappgui_sys::$type) -> Option<Self> {
                let uid = crate::gui::control_uid(ptr as _)?;
                Some(Self(uid))
            }

            pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$type {
                match crate::gui::global_get(self.0).unwrap().as_ref() {
                    crate::gui::GObject::$type(obj) => obj.as_ptr(),
                    _ => panic!("$type object not found."),
                }
            }

            pub(crate) fn inner<F, R>(&self, f: F) -> Option<R>
            where
                F: FnOnce(&$type_ex) -> R,
            {
                crate::gui::global_get(self.0).and_then(|x| match x.as_ref() {
                    crate::gui::GObject::$type(object) => Some(f(object)),
                    _ => None,
                })
            }
        }

        impl $type_ex {
            pub(crate) fn from_raw(ptr: *mut nappgui_sys::$type) -> Self {
                assert!(!ptr.is_null(), "Null pointer passed to $type_ex::from_raw");
                Self {
                    ptr: RefCell::new(ptr),
                    ..Default::default()
                }
            }

            pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::$type {
                *self.ptr.borrow()
            }
        }
    };
}

pub(crate) use impl_control;

use crate::gui::GUID;
