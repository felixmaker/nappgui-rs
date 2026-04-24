use std::{
    sync::Arc,
};

use nappgui_sys::{
    panel_create, panel_custom, panel_get_layout, panel_layout, panel_scroll, panel_size, panel_update,
    panel_visible_layout,
};

use crate::gui::Layout;

pub(crate) struct PanelInner {
    pub(crate) inner: *mut nappgui_sys::Panel,
}

/// A Panel is a control within a window that groups other controls. It defines its own reference system,
/// that is, if we move a panel all its descendants will move in unison since their locations will be
/// relative to its origin. It will support other (sub)-panels as descendants, which allows to form a
/// Window Hierarchy. A Panel is a control within a window that groups other controls.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct Panel {
    pub(crate) inner: Arc<PanelInner>,
}

impl Panel {
    /// Create a panel.
    pub fn new() -> Self {
        let panel = unsafe { panel_create() };
        assert!(!panel.is_null());
        Self {
            inner: Arc::new(PanelInner { inner: panel }),
        }
    }

    /// Create a panel with scroll bars.
    pub fn new_scroll(hscroll: bool, vscroll: bool) -> Self {
        let panel = unsafe { panel_scroll(hscroll as _, vscroll as _) };
        assert!(!panel.is_null());
        Self {
            inner: Arc::new(PanelInner { inner: panel }),
        }
    }

    /// Create a fully configurable panel.
    pub fn new_custom(hscroll: bool, vscroll: bool, border: bool) -> Self {
        let panel = unsafe { panel_custom(hscroll as _, vscroll as _, border as _) };
        assert!(!panel.is_null());
        Self {
            inner: Arc::new(PanelInner { inner: panel }),
        }
    }

    /// Sets the default size of the visible area of a panel.
    pub fn set_size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            panel_size(self.as_ptr(), size);
        }
    }

    /// Add a layout to a panel.
    ///
    /// # Remark
    /// A panel can have multiple layouts. The first layout added is the visible layout.
    /// You may use set_visible_layout to switch visible layout.
    pub fn push_layout(&self, layout: &Layout) {
        unsafe {
            panel_layout(self.as_ptr(), layout.as_ptr());
        }
    }

    /// Get a layout of a panel.
    pub fn get_layout(&self, index: u32) -> Option<&Layout> {
        let layout = unsafe { panel_get_layout(self.as_ptr(), index as _) };
        if layout.is_null() {
            None
        } else {
            unsafe { std::mem::transmute(layout) }
        }
    }

    /// Set the active layout inside the panel.
    ///
    /// # Remarks
    /// To make the change effective, you have to call panel_update.
    pub fn set_visible_layout(&self, index: u32) {
        unsafe {
            panel_visible_layout(self.as_ptr(), index as _);
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

    /// Returns a raw pointer to the panel object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Panel {
        self.inner.inner
    }
}
