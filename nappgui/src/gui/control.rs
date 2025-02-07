use nappgui_sys::{
    guicontrol_button, guicontrol_combo, guicontrol_edit, guicontrol_get_tag, guicontrol_imageview,
    guicontrol_label, guicontrol_listbox, guicontrol_panel, guicontrol_popup, guicontrol_progress,
    guicontrol_slider, guicontrol_splitview, guicontrol_tableview, guicontrol_tag,
    guicontrol_textview, guicontrol_updown, guicontrol_view, guicontrol_webview,
};

use super::*;

/// GuiControl is the virtual base class where common functionality is defined for all the controls 
/// and views that we use to create the user interfaces
pub struct Control {
    pub(crate) inner: *mut nappgui_sys::GuiControl,
}

impl Control {
    pub(crate) fn new(ptr: *mut nappgui_sys::GuiControl) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Convert a control to a label.
    pub fn label(&self) -> Option<Label> {
        let label = unsafe { guicontrol_label(self.inner) };
        if label.is_null() {
            None
        } else {
            Some(Label::new(label))
        }
    }

    /// Convert a control to a button.
    pub fn button(&self) -> Option<Button> {
        let button = unsafe { guicontrol_button(self.inner) };
        if button.is_null() {
            None
        } else {
            Some(Button::new(button))
        }
    }

    /// Convert a control to a popup.
    pub fn popup(&self) -> Option<PopUp> {
        let popup = unsafe { guicontrol_popup(self.inner) };
        if popup.is_null() {
            None
        } else {
            Some(PopUp::new(popup))
        }
    }

    /// Convert a control to a edit.
    pub fn edit(&self) -> Option<Edit> {
        let edit = unsafe { guicontrol_edit(self.inner) };
        if edit.is_null() {
            None
        } else {
            Some(Edit::new(edit))
        }
    }

    /// Convert a control to a combo.
    pub fn combo(&self) -> Option<Combo> {
        let combo = unsafe { guicontrol_combo(self.inner) };
        if combo.is_null() {
            None
        } else {
            Some(Combo::new(combo))
        }
    }

    /// Convert a control to a listbox.
    pub fn listbox(&self) -> Option<ListBox> {
        let listbox = unsafe { guicontrol_listbox(self.inner) };
        if listbox.is_null() {
            None
        } else {
            Some(ListBox::new(listbox))
        }
    }

    /// Convert a control to a updown.
    pub fn updown(&self) -> Option<UpDown> {
        let updown = unsafe { guicontrol_updown(self.inner) };
        if updown.is_null() {
            None
        } else {
            Some(UpDown::new(updown))
        }
    }

    /// Convert a control to a slider.
    pub fn slider(&self) -> Option<Slider> {
        let slider = unsafe { guicontrol_slider(self.inner) };
        if slider.is_null() {
            None
        } else {
            Some(Slider::new(slider))
        }
    }

    /// Convert a control to a progress.
    pub fn progress(&self) -> Option<Progress> {
        let progress = unsafe { guicontrol_progress(self.inner) };
        if progress.is_null() {
            None
        } else {
            Some(Progress::new(progress))
        }
    }

    /// Convert a control to a view.
    pub fn view(&self) -> Option<View> {
        let view = unsafe { guicontrol_view(self.inner) };
        if view.is_null() {
            None
        } else {
            Some(View::new(view))
        }
    }

    /// Convert a control to a textview.
    pub fn textview(&self) -> Option<TextView> {
        let textview = unsafe { guicontrol_textview(self.inner) };
        if textview.is_null() {
            None
        } else {
            Some(TextView::new(textview))
        }
    }

    /// Convert a control to a webview.
    pub fn webview(&self) -> Option<WebView> {
        let webview = unsafe { guicontrol_webview(self.inner) };
        if webview.is_null() {
            None
        } else {
            Some(WebView::new(webview))
        }
    }

    /// Convert a control to a webview.
    pub fn imageview(&self) -> Option<ImageView> {
        let imageview = unsafe { guicontrol_imageview(self.inner) };
        if imageview.is_null() {
            None
        } else {
            Some(ImageView::new(imageview))
        }
    }

    /// Convert a control to a imageview.
    pub fn tableview(&self) -> Option<TableView> {
        let tableview = unsafe { guicontrol_tableview(self.inner) };
        if tableview.is_null() {
            None
        } else {
            Some(TableView::new(tableview))
        }
    }

    /// Convert a control to a splitview.
    pub fn splitview(&self) -> Option<SplitView> {
        let splitview = unsafe { guicontrol_splitview(self.inner) };
        if splitview.is_null() {
            None
        } else {
            Some(SplitView::new(splitview))
        }
    }

    /// Convert a control to a panel.
    pub fn panel(&self) -> Option<Panel> {
        let panel = unsafe { guicontrol_panel(self.inner) };
        if panel.is_null() {
            None
        } else {
            Some(Panel::new(panel))
        }
    }

    /// Sets a tag for the control.
    pub fn tag(&self, tag: u32) {
        unsafe { guicontrol_tag(self.inner, tag) };
    }

    /// Gets a tag for the control.
    pub fn get_tag(&self) -> u32 {
        unsafe { guicontrol_get_tag(self.inner) }
    }
}
