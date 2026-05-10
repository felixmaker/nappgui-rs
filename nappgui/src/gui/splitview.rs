use nappgui_sys::{
    splitview_get_pos, splitview_horizontal, splitview_minsize0, splitview_minsize1, splitview_panel, splitview_pos,
    splitview_splitview, splitview_tableview, splitview_textview, splitview_vertical, splitview_view,
    splitview_visible0, splitview_visible1, splitview_webview,
};

use crate::{
    gui::{define_object, Panel, TableView, TextView, View, WebView},
    types::SplitMode,
};

#[derive(Default)]
pub(crate) struct SplitViewProps {}

define_object!(SplitView, SplitViewInner, SplitView, SplitViewProps);

impl SplitView {
    /// Create a splitview with horizontal split.
    pub fn new_horizontal() -> Self {
        unsafe { SplitView::from_raw(splitview_horizontal()) }
    }

    /// Create a splitview with vertical split.
    pub fn new_vertical() -> Self {
        unsafe { SplitView::from_raw(splitview_vertical()) }
    }

    /// Add a view or panel into the splitview as its split child.
    ///
    /// # Remarks
    /// For splitview and panel, the tabstop parameter is always set to TRUE even if tabstop is set
    /// to false.
    pub fn add_control<T>(&self, control: &T, tabstop: bool)
    where
        T: SplitViewInsertChildTrait,
    {
        control.insert_into_splitview(self, tabstop);
    }

    /// Sets the position of the view separator.
    pub fn set_position(&self, mode: SplitMode, pos: f32) {
        unsafe { splitview_pos(self.as_ptr(), mode as _, pos) }
    }

    /// Get the current divider position.
    pub fn position(&self, mode: SplitMode) -> f32 {
        unsafe { splitview_get_pos(self.as_ptr(), mode as _) }
    }

    /// Show or hide the left/upper child.
    pub fn set_first_visible(&self, visible: bool) {
        unsafe { splitview_visible0(self.as_ptr(), visible as _) }
    }

    /// Show or hide the right/bottom child.
    pub fn set_last_visible(&self, visible: bool) {
        unsafe { splitview_visible1(self.as_ptr(), visible as _) }
    }

    /// Set the minimum size of the left/upper child.
    pub fn set_first_min_size(&self, size: f32) {
        unsafe { splitview_minsize0(self.as_ptr(), size) }
    }

    /// Set the minimum size of the right/bottom child.
    pub fn set_last_min_size(&self, size: f32) {
        unsafe { splitview_minsize1(self.as_ptr(), size) }
    }
}

/// Define how a control insert into splitview.
pub trait SplitViewInsertChildTrait {
    /// Insert into split view as its child.
    fn insert_into_splitview(&self, split_view: &SplitView, tabstop: bool);
}

impl SplitViewInsertChildTrait for View {
    fn insert_into_splitview(&self, split_view: &SplitView, tabstop: bool) {
        unsafe { splitview_view(split_view.as_ptr(), self.as_ptr(), tabstop as _) }
    }
}

impl SplitViewInsertChildTrait for TextView {
    fn insert_into_splitview(&self, split_view: &SplitView, tabstop: bool) {
        unsafe { splitview_textview(split_view.as_ptr(), self.as_ptr(), tabstop as _) }
    }
}

impl SplitViewInsertChildTrait for WebView {
    fn insert_into_splitview(&self, split_view: &SplitView, tabstop: bool) {
        unsafe { splitview_webview(split_view.as_ptr(), self.as_ptr(), tabstop as _) }
    }
}

impl SplitViewInsertChildTrait for SplitView {
    fn insert_into_splitview(&self, split_view: &SplitView, _tabstop: bool) {
        unsafe { splitview_splitview(split_view.as_ptr(), self.as_ptr()) }
    }
}

impl SplitViewInsertChildTrait for Panel {
    fn insert_into_splitview(&self, split_view: &SplitView, _tabstop: bool) {
        unsafe { splitview_panel(split_view.as_ptr(), self.as_ptr()) }
    }
}

impl SplitViewInsertChildTrait for TableView {
    fn insert_into_splitview(&self, split_view: &SplitView, tabstop: bool) {
        unsafe { splitview_tableview(split_view.as_ptr(), self.as_ptr(), tabstop as _) }
    }
}
