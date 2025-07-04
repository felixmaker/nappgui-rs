use std::rc::Rc;

use nappgui_sys::{
    panel_create, panel_custom, panel_get_layout, panel_layout, panel_scroll, panel_scroll_height,
    panel_scroll_width, panel_size, panel_update, panel_visible_layout,
};

use crate::util::macros::pub_crate_ptr_ops;

use super::Layout;
/// A Panel is a control within a window that groups other controls. It defines its own reference system, 
/// that is, if we move a panel all its descendants will move in unison since their locations will be 
/// relative to its origin. It will support other (sub)-panels as descendants, which allows to form a 
/// Window Hierarchy. A Panel is a control within a window that groups other controls.
pub struct Panel {
    pub(crate) inner: Rc<*mut nappgui_sys::Panel>,
}

impl Panel {
    pub_crate_ptr_ops!(*mut nappgui_sys::Panel);

    /// Create a panel.
    pub fn create() -> Self {
        let panel = unsafe { panel_create() };
        Self::new(panel)
    }

    /// Create a panel with scroll bars.
    pub fn scroll(hscroll: bool, vscroll: bool) -> Self {
        let panel = unsafe { panel_scroll(hscroll as i8, vscroll as i8) };
        Self::new(panel)
    }

    /// Create a fully configurable panel.
    pub fn custom(hscroll: bool, vscroll: bool, border: bool) -> Self {
        let panel = unsafe { panel_custom(hscroll as i8, vscroll as i8, border as i8) };
        Self::new(panel)
    }

    /// Sets the default size of the visible area of a panel.
    pub fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            panel_size(self.as_ptr(), size);
        }
    }

    /// Add a layout to a panel.
    pub fn layout(&self, layout: &Layout) {
        unsafe {
            panel_layout(self.as_ptr(), layout.as_ptr());
        }
    }

    /// Get a layout of a panel.
    pub fn get_layout(&self, index: u32) -> Option<Layout> {
        let layout = unsafe { panel_get_layout(self.as_ptr(), index) };
        if layout.is_null() {
            None
        } else {
            Some(Layout::new(layout))
        }
    }

    /// Set the active layout inside the panel.
    ///
    /// # Remarks
    /// To make the change effective, you have to call panel_update.
    pub fn visible_layout(&self, index: u32) {
        unsafe {
            panel_visible_layout(self.as_ptr(), index);
        }
    }

    /// Update the window that contains the panel.
    ///
    /// # Remarks
    /// It is equivalent to calling window_update.
    pub fn update(&self) {
        unsafe {
            panel_update(self.as_ptr());
        }
    }

    /// Gets the width of the scroll bar of the associated panel.
    ///
    /// # Remarks
    /// Only valid if the panel has been created with panel_scroll. Useful if we want to take into
    /// account the size of the scroll bars when setting the margins of the Layout.
    pub fn scroll_width(&self) -> f32 {
        unsafe { panel_scroll_width(self.as_ptr()) }
    }

    /// Gets the height of the scroll bar.
    pub fn scroll_height(&self) -> f32 {
        unsafe { panel_scroll_height(self.as_ptr()) }
    }
}
