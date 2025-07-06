use std::{ffi::CString, rc::Rc};

use nappgui_sys::{
    cell_button, cell_combo, cell_control, cell_dbind_imp, cell_edit, cell_empty, cell_enabled,
    cell_imageview, cell_label, cell_layout, cell_listbox, cell_padding, cell_padding2,
    cell_padding4, cell_panel, cell_popup, cell_progress, cell_slider, cell_splitview,
    cell_tableview, cell_textview, cell_updown, cell_view, cell_visible, cell_webview,
};

use crate::util::macros::pub_crate_ptr_ops;

use super::*;

/// Cells are the inner elements of a Layout and will house a control or a sublayout.
pub struct Cell {
    pub(crate) inner: Rc<*mut nappgui_sys::Cell>,
}

impl Cell {
    pub_crate_ptr_ops!(*mut nappgui_sys::Cell);

    /// Check if the cell is empty.
    pub fn empty(&self) -> bool {
        unsafe { cell_empty(self.as_ptr()) != 0 }
    }

    /// Get control of the inside of the cell.
    pub fn control(&self) -> Option<Control> {
        let ptr = unsafe { cell_control(self.as_ptr()) };
        unsafe { Control::new_option_no_drop(ptr) }
    }

    /// Get the label inside the cell.
    pub fn label(&self) -> Option<Label> {
        let ptr = unsafe { cell_label(self.as_ptr()) };
        unsafe { Label::new_option_no_drop(ptr) }
    }

    /// Get the button inside the cell.
    pub fn button(&self) -> Option<Button> {
        let ptr = unsafe { cell_button(self.as_ptr()) };
        unsafe { Button::new_option_no_drop(ptr) }
    }

    /// Get the popup inside the cell.
    pub fn popup(&self) -> Option<PopUp> {
        let ptr = unsafe { cell_popup(self.as_ptr()) };
        unsafe { PopUp::new_option_no_drop(ptr) }
    }

    /// Get the edit inside the cell.
    pub fn edit(&self) -> Option<Edit> {
        let ptr = unsafe { cell_edit(self.as_ptr()) };
        unsafe { Edit::new_option_no_drop(ptr) }
    }

    /// Get the combo inside the cell.
    pub fn combo(&self) -> Option<Combo> {
        let ptr = unsafe { cell_combo(self.as_ptr()) };
        unsafe { Combo::new_option_no_drop(ptr) }
    }

    /// Get the listbox inside the cell.
    pub fn listbox(&self) -> Option<ListBox> {
        let ptr = unsafe { cell_listbox(self.as_ptr()) };
        unsafe { ListBox::new_option_no_drop(ptr) }
    }

    /// Get the updown inside the cell.
    pub fn updown(&self) -> Option<UpDown> {
        let ptr = unsafe { cell_updown(self.as_ptr()) };
        unsafe { UpDown::new_option_no_drop(ptr) }
    }

    /// Get the slider inside the cell.
    pub fn slider(&self) -> Option<Slider> {
        let ptr = unsafe { cell_slider(self.as_ptr()) };
        unsafe { Slider::new_option_no_drop(ptr) }
    }

    /// Get the progress inside the cell.
    pub fn progress(&self) -> Option<Progress> {
        let ptr = unsafe { cell_progress(self.as_ptr()) };
        unsafe { Progress::new_option_no_drop(ptr) }
    }

    /// Get the view inside the cell.
    pub fn view(&self) -> Option<View> {
        let ptr = unsafe { cell_view(self.as_ptr()) };
        unsafe { View::new_option_no_drop(ptr) }
    }

    /// Get the textview inside the cell.
    pub fn textview(&self) -> Option<TextView> {
        let ptr = unsafe { cell_textview(self.as_ptr()) };
        unsafe { TextView::new_option_no_drop(ptr) }
    }

    /// Get the webview inside the cell.
    pub fn webview(&self) -> Option<WebView> {
        let ptr = unsafe { cell_webview(self.as_ptr()) };
        unsafe { WebView::new_option_no_drop(ptr) }
    }

    /// Get the imageview inside the cell.
    pub fn imageview(&self) -> Option<ImageView> {
        let ptr = unsafe { cell_imageview(self.as_ptr()) };
        unsafe { ImageView::new_option_no_drop(ptr) }
    }

    /// Get the tableview inside the cell.
    pub fn tableview(&self) -> Option<TableView> {
        let ptr = unsafe { cell_tableview(self.as_ptr()) };
        unsafe { TableView::new_option_no_drop(ptr) }
    }

    /// Get the splitview inside the cell.
    pub fn splitview(&self) -> Option<SplitView> {
        let ptr = unsafe { cell_splitview(self.as_ptr()) };
        unsafe { SplitView::new_option_no_drop(ptr) }
    }

    /// Get the panel inside the cell.
    pub fn panel(&self) -> Option<Panel> {
        let ptr = unsafe { cell_panel(self.as_ptr()) };
        unsafe { Panel::new_option_no_drop(ptr) }
    }

    /// Get the layout inside the cell.
    pub fn layout(&self) -> Option<Layout> {
        let ptr = unsafe { cell_layout(self.as_ptr()) };
        unsafe { Layout::new_option_no_drop(ptr) }
    }

    /// Activate or deactivate a cell.
    pub fn enabled(&self, enabled: bool) {
        unsafe { cell_enabled(self.as_ptr(), enabled as i8) }
    }

    /// Show or hide a cell.
    ///
    /// # Remarks
    /// If the cell contains a sublayout, the command will affect all controls recursively.
    pub fn visible(&self, visible: bool) {
        unsafe { cell_visible(self.as_ptr(), visible as i8) }
    }

    /// Set an inner margin.
    pub fn padding(&self, pall: f32) {
        unsafe {
            cell_padding(self.as_ptr(), pall);
        }
    }

    /// Set an inner margin.
    pub fn padding2(&self, pleft: f32, pright: f32) {
        unsafe {
            cell_padding2(self.as_ptr(), pleft, pright);
        }
    }

    /// Set an inner margin.
    pub fn padding4(&self, pleft: f32, ptop: f32, pright: f32, pbottom: f32) {
        unsafe {
            cell_padding4(self.as_ptr(), pleft, ptop, pright, pbottom);
        }
    }

    /// Associates a cell with the field of a struct.
    pub fn dbind_imp(
        &self,
        type_: &str,
        size: u16,
        mname: &str,
        mtype: &str,
        moffset: u16,
        msize: u16,
    ) {
        let type_ = CString::new(type_).unwrap();
        let mname = CString::new(mname).unwrap();
        let mtype = CString::new(mtype).unwrap();
        unsafe {
            cell_dbind_imp(
                self.as_ptr(),
                type_.as_ptr(),
                size,
                mname.as_ptr(),
                mtype.as_ptr(),
                moffset,
                msize,
            );
        }
    }
}

/// Associates a cell with the field of a struct.
#[macro_export]
macro_rules! cell_dbind {
    ($cell: expr, $struct: ty, $field: ident, $field_type: ty, $bind_type: literal) => {
        nappgui::gui::Cell::dbind_imp(
            $cell,
            stringify!($struct),
            size_of::<$struct>() as _,
            stringify!($field),
            $bind_type,
            offset_of!($struct, $field) as _,
            size_of::<$field_type>() as _,
        )
    };
}
