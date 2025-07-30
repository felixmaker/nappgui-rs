use nappgui_sys::{
    panel_create, panel_custom, panel_get_layout, panel_layout, panel_scroll, panel_scroll_height,
    panel_scroll_width, panel_size, panel_update, panel_visible_layout,
};

use crate::gui::{control::impl_control, impl_layout, Layout, LayoutTrait};

/// The panel trait.
pub trait PanelTrait {
    /// Returns a raw pointer to the panel object.
    fn as_ptr(&self) -> *mut nappgui_sys::Panel;

    /// Sets the default size of the visible area of a panel.
    fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            panel_size(self.as_ptr(), size);
        }
    }

    /// Add a layout to a panel.
    fn layout<T>(&self, layout: T)
    where
        T: LayoutTrait,
    {
        unsafe {
            panel_layout(self.as_ptr(), layout.as_ptr());
        }
    }

    /// Get a layout of a panel.
    fn get_layout(&self, index: u32) -> Option<Layout> {
        let layout = unsafe { panel_get_layout(self.as_ptr(), index as _) };
        if layout.is_null() {
            None
        } else {
            Some(Layout { inner: layout })
        }
    }

    /// Set the active layout inside the panel.
    ///
    /// # Remarks
    /// To make the change effective, you have to call panel_update.
    fn visible_layout(&self, index: u32) {
        unsafe {
            panel_visible_layout(self.as_ptr(), index as _);
        }
    }

    /// Update the window that contains the panel.
    ///
    /// # Remarks
    /// It is equivalent to calling window_update.
    fn update(&self) {
        unsafe {
            panel_update(self.as_ptr());
        }
    }
}

/// The scroll panel trait.
pub trait ScrollPanelTrait: PanelTrait {
    /// Gets the width of the scroll bar of the associated panel.
    ///
    /// # Remarks
    /// Useful if we want to take into account the size of the scroll bars
    /// when setting the margins of the Layout.
    fn scroll_width(&self) -> f32 {
        unsafe { panel_scroll_width(self.as_ptr()) }
    }

    /// Gets the height of the scroll bar.
    fn scroll_height(&self) -> f32 {
        unsafe { panel_scroll_height(self.as_ptr()) }
    }
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
#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub(crate) inner: *mut nappgui_sys::Panel,
}

impl PanelTrait for Panel {
    fn as_ptr(&self) -> *mut nappgui_sys::Panel {
        self.inner
    }
}

/// The scroll panel.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ScrollPanel {
    pub(crate) inner: *mut nappgui_sys::Panel,
}

impl ScrollPanel {
    /// Create a panel with scroll bars.
    pub fn new(hscroll: bool, vscroll: bool) -> Self {
        let panel = unsafe { panel_scroll(hscroll as _, vscroll as _) };
        Self { inner: panel }
    }

    /// Create a fully configurable panel.
    pub fn new_custom(hscroll: bool, vscroll: bool, border: bool) -> Self {
        let panel = unsafe { panel_custom(hscroll as _, vscroll as _, border as _) };
        Self { inner: panel }
    }
}

impl From<ScrollPanel> for Panel {
    fn from(sp: ScrollPanel) -> Self {
        Self { inner: sp.inner }
    }
}

impl PanelTrait for ScrollPanel {
    fn as_ptr(&self) -> *mut nappgui_sys::Panel {
        self.inner
    }
}

impl ScrollPanelTrait for ScrollPanel {}

impl Panel {
    /// Create a panel.
    pub fn new() -> Self {
        let panel = unsafe { panel_create() };
        Self { inner: panel }
    }
}

impl_control!(Panel, guicontrol_panel);
impl_control!(ScrollPanel, guicontrol_panel);
impl_layout!(Panel, PanelTrait, layout_panel);
impl_layout!(ScrollPanel, PanelTrait, layout_panel);
