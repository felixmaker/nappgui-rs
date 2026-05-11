use std::cell::{Cell, RefCell};

use nappgui_sys::{
    panel_create, panel_custom, panel_layout, panel_scroll, panel_scroll_size, panel_size, panel_update,
    panel_viewport, panel_visible_layout,
};

use crate::gui::{define_object, Layout};

#[derive(Default)]
pub(crate) struct PanelProps {
    scroll: Cell<bool>,
    layouts: RefCell<Vec<Layout>>,
}

define_object!(Panel, PanelInner, Panel, PanelProps);

impl Panel {
    /// Create a panel.
    pub fn new() -> Self {
        unsafe { Self::from_raw(panel_create()) }
    }

    /// Create a panel with scroll bars.
    pub fn new_scroll(hscroll: bool, vscroll: bool) -> Self {
        let panel = unsafe { Self::from_raw(panel_scroll(hscroll as _, vscroll as _)) };
        panel.inner(|inner| inner.props.scroll.set(hscroll || vscroll));
        panel
    }

    /// Create a fully configurable panel.
    pub fn new_custom(hscroll: bool, vscroll: bool, border: bool) -> Self {
        let panel = unsafe { Self::from_raw(panel_custom(hscroll as _, vscroll as _, border as _)) };
        panel.inner(|inner| inner.props.scroll.set(hscroll || vscroll));
        panel
    }

    /// Sets the default size of the visible area of a panel.
    pub fn set_size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { panel_size(self.as_ptr(), size) }
    }

    /// Add a layout to a panel.
    ///
    /// # Remarks
    /// A panel can have multiple layouts. The first layout added is the visible layout.
    /// You may use set_visible_layout to switch visible layout.
    pub fn add_layout(&self, layout: Layout) -> u32 {
        let result = unsafe { panel_layout(self.as_ptr(), layout.as_ptr()) };
        self.inner(|inner| inner.props.layouts.borrow_mut().push(layout));
        result
    }

    /// Get a layout of a panel.
    pub fn layout(&self, index: u32) -> Option<Layout> {
        self.inner(|inner| inner.props.layouts.borrow().get(index as usize).cloned())?
    }

    /// Set the active layout inside the panel.
    ///
    /// # Remarks
    /// To make the change effective, you have to call panel_update.
    pub fn set_visible_layout(&self, index: u32) {
        unsafe { panel_visible_layout(self.as_ptr(), index as _) }
    }

    /// Update the window that contains the panel.
    ///
    /// # Remarks
    /// It is equivalent to calling window_update.
    pub fn update(&self) {
        unsafe { panel_update(self.as_ptr()) }
    }

    /// Gets the measurements of the scroll bars.
    ///
    /// # Remarks
    /// If the panel does not have scroll bars, it will return None.
    pub fn scroll_size(&self) -> Option<(f32, f32)> {
        let scroll = self.inner(|inner| inner.props.scroll.get())?;
        if scroll {
            return None;
        }
        let mut width = 0f32;
        let mut height = 0f32;
        unsafe { panel_scroll_size(self.as_ptr(), &mut width, &mut height) }
        Some((width, height))
    }

    /// Gets the dimensions of the visible area of the panel.
    ///
    /// # Remarks
    /// It returns the position and size of the visible area of the panel.
    /// If the panel does not have scroll bars, it will return ((0.0, 0.0), (0.0, 0.0)).
    pub fn viewport(&self) -> ((f32, f32), (f32, f32)) {
        let mut position = nappgui_sys::V2Df { x: 0.0, y: 0.0 };
        let mut size = nappgui_sys::S2Df {
            width: 0.0,
            height: 0.0,
        };
        unsafe { panel_viewport(self.as_ptr(), &mut position, &mut size) };
        ((position.x, position.y), (size.width, size.height))
    }
}
