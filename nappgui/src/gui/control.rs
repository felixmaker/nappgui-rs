use crate::gui::*;

/// The control object.
#[repr(transparent)]
pub struct Control {
    inner: *mut nappgui_sys::GuiControl,
}

impl Control {
    pub(crate) fn from_raw(inner: *mut nappgui_sys::GuiControl) -> Self {
        assert!(!inner.is_null());
        Self { inner }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::GuiControl {
        self.inner
    }
}

macro_rules! impl_control {
    ($type:ident, $func:ident, $nappgui_func:ident) => {
        impl AsRef<Control> for $type {
            fn as_ref(&self) -> &Control {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl Control {
            #[doc = concat!("Converts the control to a ", stringify!($type))]
            pub fn $func(&self) -> Option<$type> {
                let control = unsafe { nappgui_sys::$nappgui_func(self.as_ptr()) };
                if control.is_null() {
                    None
                } else {
                    Some($type::from_raw(control))
                }
            }
        }
    };
}

impl_control!(Button, as_button, guicontrol_button);
impl_control!(Combo, as_combo, guicontrol_combo);
impl_control!(Edit, as_edit, guicontrol_edit);
impl_control!(ImageView, as_imageview, guicontrol_imageview);
impl_control!(Label, as_label, guicontrol_label);
impl_control!(Panel, as_panel, guicontrol_panel);
impl_control!(ListBox, as_listbox, guicontrol_listbox);
impl_control!(PopUp, as_popup, guicontrol_popup);
impl_control!(Progress, as_progress, guicontrol_progress);
impl_control!(Slider, as_slider, guicontrol_slider);
impl_control!(SplitView, as_splitview, guicontrol_splitview);
impl_control!(TableView, as_tableview, guicontrol_tableview);
impl_control!(TextView, as_textview, guicontrol_textview);
impl_control!(UpDown, as_updown, guicontrol_updown);
impl_control!(View, as_view, guicontrol_view);
impl_control!(WebView, as_webview, guicontrol_webview);
impl_control!(Line, as_line, guicontrol_line);
