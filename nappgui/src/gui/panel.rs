use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{
    panel_create, panel_custom, panel_get_layout, panel_layout, panel_scroll, panel_size, panel_update,
    panel_visible_layout,
};

use crate::gui::{global_get, global_record, Layout};

pub(crate) struct PanelInner {
    ptr: NonNull<nappgui_sys::Panel>,
}

impl PanelInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Panel) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to PanelInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Panel {
        self.ptr.as_ptr()
    }
}

/// The panel control.
///
/// # Remark
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct Panel(Weak<PanelInner>);

impl Panel {
    pub(crate) unsafe fn from_raw(panel: *mut nappgui_sys::Panel) -> Self {
        let object = global_record(panel as _, PanelInner::from_raw(panel));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(panel: *mut nappgui_sys::Panel) -> Self {
        let object = global_get(panel as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Panel {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a panel.
    pub fn new() -> Self {
        unsafe { Self::from_raw(panel_create()) }
    }

    /// Create a panel with scroll bars.
    pub fn new_scroll(hscroll: bool, vscroll: bool) -> Self {
        unsafe { Self::from_raw(panel_scroll(hscroll as _, vscroll as _)) }
    }

    /// Create a fully configurable panel.
    pub fn new_custom(hscroll: bool, vscroll: bool, border: bool) -> Self {
        unsafe { Self::from_raw(panel_custom(hscroll as _, vscroll as _, border as _)) }
    }

    /// Sets the default size of the visible area of a panel.
    pub fn set_size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { panel_size(self.as_ptr(), size) };
    }

    /// Add a layout to a panel.
    ///
    /// # Remark
    /// A panel can have multiple layouts. The first layout added is the visible layout.
    /// You may use set_visible_layout to switch visible layout.
    pub fn set_layout(&self, layout: &Layout) -> u32 {
        let result = unsafe { panel_layout(self.as_ptr(), layout.as_ptr()) };

        // let layout_child = CONTEXT.with_borrow(|ctx| ctx.get(&layout.as_id()).map(|x| x.childs.borrow().clone()));

        // if let Some(child) = layout_child {
        //     for child_id in child.iter() {
        //         global_set_owner(*child_id, self.as_id());
        //     }
        // }

        result
    }

    /// Get a layout of a panel.
    pub fn get_layout(&self, index: u32) -> Option<Layout> {
        let layout = unsafe { panel_get_layout(self.as_ptr(), index as _) };
        if layout.is_null() {
            return None;
        }
        Some(unsafe { Layout::from_raw(layout) })
    }

    /// Set the active layout inside the panel.
    ///
    /// # Remarks
    /// To make the change effective, you have to call panel_update.
    pub fn set_visible_layout(&self, index: u32) {
        unsafe { panel_visible_layout(self.as_ptr(), index as _) };
    }

    /// Update the window that contains the panel.
    ///
    /// # Remarks
    /// It is equivalent to calling window_update.
    pub fn update(&self) {
        unsafe {
            panel_update(self.as_ptr());
        };
    }
}
