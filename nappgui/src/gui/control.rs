use std::rc::Rc;

use nappgui_sys::{
    guicontrol_button, guicontrol_combo, guicontrol_edit, guicontrol_get_tag, guicontrol_imageview,
    guicontrol_label, guicontrol_listbox, guicontrol_panel, guicontrol_popup, guicontrol_progress,
    guicontrol_slider, guicontrol_splitview, guicontrol_tableview, guicontrol_tag,
    guicontrol_textview, guicontrol_updown, guicontrol_view, guicontrol_webview,
};

use crate::util::macros::impl_ptr;

use super::*;

/// GuiControl is the virtual base class where common functionality is defined for all the controls 
/// and views that we use to create the user interfaces
pub struct Control {
    pub(crate) inner: Rc<*mut nappgui_sys::GuiControl>,
}

impl Control {
    impl_ptr!(nappgui_sys::GuiControl);

    /// Convert a control to a label.
    pub fn label(&self) -> Option<Label> {
        let label = unsafe { guicontrol_label(self.as_ptr()) };
        if label.is_null() {
            None
        } else {
            Some(Label::from_ptr(label))
        }
    }

    /// Convert a control to a button.
    pub fn button(&self) -> Option<Button> {
        let button = unsafe { guicontrol_button(self.as_ptr()) };
        if button.is_null() {
            None
        } else {
            Some(Button::from_ptr(button))
        }
    }

    /// Convert a control to a popup.
    pub fn popup(&self) -> Option<PopUp> {
        let popup = unsafe { guicontrol_popup(self.as_ptr()) };
        if popup.is_null() {
            None
        } else {
            Some(PopUp::from_ptr(popup))
        }
    }

    /// Convert a control to a edit.
    pub fn edit(&self) -> Option<Edit> {
        let edit = unsafe { guicontrol_edit(self.as_ptr()) };
        if edit.is_null() {
            None
        } else {
            Some(Edit::from_ptr(edit))
        }
    }

    /// Convert a control to a combo.
    pub fn combo(&self) -> Option<Combo> {
        let combo = unsafe { guicontrol_combo(self.as_ptr()) };
        if combo.is_null() {
            None
        } else {
            Some(Combo::from_ptr(combo))
        }
    }

    /// Convert a control to a listbox.
    pub fn listbox(&self) -> Option<ListBox> {
        let listbox = unsafe { guicontrol_listbox(self.as_ptr()) };
        if listbox.is_null() {
            None
        } else {
            Some(ListBox::from_ptr(listbox))
        }
    }

    /// Convert a control to a updown.
    pub fn updown(&self) -> Option<UpDown> {
        let updown = unsafe { guicontrol_updown(self.as_ptr()) };
        if updown.is_null() {
            None
        } else {
            Some(UpDown::from_ptr(updown))
        }
    }

    /// Convert a control to a slider.
    pub fn slider(&self) -> Option<Slider> {
        let slider = unsafe { guicontrol_slider(self.as_ptr()) };
        if slider.is_null() {
            None
        } else {
            Some(Slider::from_ptr(slider))
        }
    }

    /// Convert a control to a progress.
    pub fn progress(&self) -> Option<Progress> {
        let progress = unsafe { guicontrol_progress(self.as_ptr()) };
        if progress.is_null() {
            None
        } else {
            Some(Progress::from_ptr(progress))
        }
    }

    /// Convert a control to a view.
    pub fn view(&self) -> Option<View> {
        let view = unsafe { guicontrol_view(self.as_ptr()) };
        if view.is_null() {
            None
        } else {
            Some(View::from_ptr(view))
        }
    }

    /// Convert a control to a textview.
    pub fn textview(&self) -> Option<TextView> {
        let textview = unsafe { guicontrol_textview(self.as_ptr()) };
        if textview.is_null() {
            None
        } else {
            Some(TextView::from_ptr(textview))
        }
    }

    /// Convert a control to a webview.
    pub fn webview(&self) -> Option<WebView> {
        let webview = unsafe { guicontrol_webview(self.as_ptr()) };
        if webview.is_null() {
            None
        } else {
            Some(WebView::from_ptr(webview))
        }
    }

    /// Convert a control to a webview.
    pub fn imageview(&self) -> Option<ImageView> {
        let imageview = unsafe { guicontrol_imageview(self.as_ptr()) };
        if imageview.is_null() {
            None
        } else {
            Some(ImageView::from_ptr(imageview))
        }
    }

    /// Convert a control to a imageview.
    pub fn tableview(&self) -> Option<TableView> {
        let tableview = unsafe { guicontrol_tableview(self.as_ptr()) };
        if tableview.is_null() {
            None
        } else {
            Some(TableView::from_ptr(tableview))
        }
    }

    /// Convert a control to a splitview.
    pub fn splitview(&self) -> Option<SplitView> {
        let splitview = unsafe { guicontrol_splitview(self.as_ptr()) };
        if splitview.is_null() {
            None
        } else {
            Some(SplitView::from_ptr(splitview))
        }
    }

    /// Convert a control to a panel.
    pub fn panel(&self) -> Option<Panel> {
        let panel = unsafe { guicontrol_panel(self.as_ptr()) };
        if panel.is_null() {
            None
        } else {
            Some(Panel::from_ptr(panel))
        }
    }

    /// Sets a tag for the control.
    pub fn tag(&self, tag: u32) {
        unsafe { guicontrol_tag(self.as_ptr(), tag) };
    }

    /// Gets a tag for the control.
    pub fn get_tag(&self) -> u32 {
        unsafe { guicontrol_get_tag(self.as_ptr()) }
    }
}
