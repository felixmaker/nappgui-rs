use nappgui_sys::{
    panel_create, panel_custom, panel_get_layout, panel_layout, panel_scroll, panel_size, panel_update, panel_visible_layout
};

use crate::gui::{Layout, Object, WeakObject, global_object};

/// A Panel is a control within a window that groups other controls. It defines its own reference system,
/// that is, if we move a panel all its descendants will move in unison since their locations will be
/// relative to its origin. It will support other (sub)-panels as descendants, which allows to form a
/// Window Hierarchy. A Panel is a control within a window that groups other controls.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone)]
pub struct Panel(WeakObject<nappgui_sys::Panel>);

impl Panel {
    fn from_raw(panel: *mut nappgui_sys::Panel) -> Self {
        assert!(!panel.is_null());
        Self(Object::global_new(panel, crate::gui::ObjectType::Panel))
    }

    /// Create a panel.
    pub fn new() -> Self {
        let panel = unsafe { panel_create() };
        Self::from_raw(panel)
    }

    /// Create a panel with scroll bars.
    pub fn new_scroll(hscroll: bool, vscroll: bool) -> Self {
        let panel = unsafe { panel_scroll(hscroll as _, vscroll as _) };
        Self::from_raw(panel)
    }

    /// Create a fully configurable panel.
    pub fn new_custom(hscroll: bool, vscroll: bool, border: bool) -> Self {
        let panel = unsafe { panel_custom(hscroll as _, vscroll as _, border as _) };
        Self::from_raw(panel)
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
        let layout = global_object::<nappgui_sys::Layout>(layout)?;
        Some(Layout(layout))
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

    /// Returns a raw pointer to the panel object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::Panel {
        self.0.as_ptr().expect("error: object no longer able to access!")
    }
}
