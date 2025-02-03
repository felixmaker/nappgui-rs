use nappgui_sys::{
    cell_button, cell_combo, cell_control, cell_edit, cell_empty, cell_enabled, cell_imageview,
    cell_label, cell_layout, cell_listbox, cell_padding, cell_padding2, cell_padding4, cell_panel,
    cell_popup, cell_progress, cell_slider, cell_splitview, cell_tableview, cell_textview,
    cell_updown, cell_view, cell_visible, cell_webview,
};

use super::*;

pub struct Cell {
    pub(crate) inner: *mut nappgui_sys::Cell,
}

impl Cell {
    #[allow(unused)]
    pub(crate) fn new(ptr: *mut nappgui_sys::Cell) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Check if the cell is empty.
    pub fn empty(&self) -> bool {
        unsafe { cell_empty(self.inner) != 0 }
    }

    /// Get control of the inside of the cell.
    pub fn control(&self) -> Option<Control> {
        let ptr = unsafe { cell_control(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Control::new(ptr))
        }
    }

    /// Get the label inside the cell.
    pub fn label(&self) -> Option<Label> {
        let ptr = unsafe { cell_label(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Label::new(ptr))
        }
    }

    /// Get the button inside the cell.
    pub fn button(&self) -> Option<Button> {
        let ptr = unsafe { cell_button(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Button::new(ptr))
        }
    }

    /// Get the popup inside the cell.
    pub fn popup(&self) -> Option<PopUp> {
        let ptr = unsafe { cell_popup(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(PopUp::new(ptr))
        }
    }

    /// Get the edit inside the cell.
    pub fn edit(&self) -> Option<Edit> {
        let ptr = unsafe { cell_edit(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Edit::new(ptr))
        }
    }

    /// Get the combo inside the cell.
    pub fn combo(&self) -> Option<Combo> {
        let ptr = unsafe { cell_combo(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Combo::new(ptr))
        }
    }

    /// Get the listbox inside the cell.
    pub fn listbox(&self) -> Option<ListBox> {
        let ptr = unsafe { cell_listbox(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(ListBox::new(ptr))
        }
    }

    /// Get the updown inside the cell.
    pub fn updown(&self) -> Option<UpDown> {
        let ptr = unsafe { cell_updown(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(UpDown::new(ptr))
        }
    }

    /// Get the slider inside the cell.
    pub fn slider(&self) -> Option<Slider> {
        let ptr = unsafe { cell_slider(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Slider::new(ptr))
        }
    }

    /// Get the progress inside the cell.
    pub fn progress(&self) -> Option<Progress> {
        let ptr = unsafe { cell_progress(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Progress::new(ptr))
        }
    }

    /// Get the view inside the cell.
    pub fn view(&self) -> Option<View> {
        let ptr = unsafe { cell_view(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(View::new(ptr))
        }
    }

    /// Get the textview inside the cell.
    pub fn textview(&self) -> Option<TextView> {
        let ptr = unsafe { cell_textview(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(TextView::new(ptr))
        }
    }

    /// Get the webview inside the cell.
    pub fn webview(&self) -> Option<WebView> {
        let ptr = unsafe { cell_webview(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(WebView::new(ptr))
        }
    }

    /// Get the imageview inside the cell.
    pub fn imageview(&self) -> Option<ImageView> {
        let ptr = unsafe { cell_imageview(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(ImageView::new(ptr))
        }
    }

    /// Get the tableview inside the cell.
    pub fn tableview(&self) -> Option<TableView> {
        let ptr = unsafe { cell_tableview(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(TableView::new(ptr))
        }
    }

    /// Get the splitview inside the cell.
    pub fn splitview(&self) -> Option<SplitView> {
        let ptr = unsafe { cell_splitview(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(SplitView::new(ptr))
        }
    }

    /// Get the panel inside the cell.
    pub fn panel(&self) -> Option<Panel> {
        let ptr = unsafe { cell_panel(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Panel::new(ptr))
        }
    }

    /// Get the layout inside the cell.
    pub fn layout(&self) -> Option<Layout> {
        let ptr = unsafe { cell_layout(self.inner) };
        if ptr.is_null() {
            None
        } else {
            Some(Layout::new(ptr))
        }
    }

    /// Activate or deactivate a cell.
    pub fn enabled(&self, enabled: bool) {
        unsafe { cell_enabled(self.inner, enabled as i8) }
    }

    /// Show or hide a cell.
    ///
    /// # Remarks
    /// If the cell contains a sublayout, the command will affect all controls recursively.
    pub fn visible(&self, visible: bool) {
        unsafe { cell_visible(self.inner, visible as i8) }
    }

    /// Set an inner margin.
    pub fn padding(&self, pall: f32) {
        unsafe {
            cell_padding(self.inner, pall);
        }
    }

    /// Set an inner margin.
    pub fn padding2(&self, pleft: f32, pright: f32) {
        unsafe {
            cell_padding2(self.inner, pleft, pright);
        }
    }

    /// Set an inner margin.
    pub fn padding4(&self, pleft: f32, ptop: f32, pright: f32, pbottom: f32) {
        unsafe {
            cell_padding4(self.inner, pleft, ptop, pright, pbottom);
        }
    }
}
