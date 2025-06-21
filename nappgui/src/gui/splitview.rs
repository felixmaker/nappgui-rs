use nappgui_sys::{
    splitview_get_pos, splitview_horizontal, splitview_minsize0, splitview_minsize1,
    splitview_panel, splitview_pos, splitview_splitview, splitview_textview, splitview_vertical,
    splitview_view, splitview_visible0, splitview_visible1, splitview_webview
};

use crate::{gui::WebView, prelude::SplitMode};

use super::{Panel, TextView, View};

/// The SplitView are views divided into two parts, where in each of them we place another view or
/// a panel. The dividing line is scrollable, which allows resizing both halves, dividing the total
/// size of the control between the children.
pub struct SplitView {
    pub(crate) inner: *mut nappgui_sys::SplitView,
}

impl SplitView {
    pub(crate) fn new(ptr: *mut nappgui_sys::SplitView) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a splitview with horizontal split.
    pub fn horizontal() -> Self {
        let splitview = unsafe { splitview_horizontal() };
        Self::new(splitview)
    }

    /// Create a splitview with vertical split.
    pub fn vertical() -> Self {
        let splitview = unsafe { splitview_vertical() };
        Self::new(splitview)
    }

    /// Add a custom view to the splitview.
    pub fn view(&self, view: &View, tabstop: bool) {
        unsafe { splitview_view(self.inner, view.inner, tabstop as i8) }
    }

    /// Add a text view to the splitview.
    pub fn textview(&self, view: &TextView, tabstop: bool) {
        unsafe { splitview_textview(self.inner, view.inner, tabstop as i8) }
    }

    /// Add a web view to SplitView.
    pub fn webview(&self, view: &WebView, tabstop: bool) {
        unsafe { splitview_webview(self.inner, view.inner, tabstop as i8) }
    }

    /// Add a splitview (child) to the splitview.
    pub fn splitview(&self, child: &SplitView) {
        unsafe { splitview_splitview(self.inner, child.inner) }
    }

    /// Add a panel to the splitview.
    pub fn panel(&self, panel: &Panel) {
        unsafe { splitview_panel(self.inner, panel.inner) }
    }

    /// Sets the position of the view separator.
    pub fn pos(&self, mode: SplitMode, pos: f32) {
        unsafe { splitview_pos(self.inner, mode, pos) }
    }

    /// Get the current divider position.
    pub fn get_pos(&self, mode: SplitMode) -> f32 {
        unsafe { splitview_get_pos(self.inner, mode) }
    }

    /// Show/hide the left/upper child.
    pub fn visible0(&self, visible: bool) {
        unsafe { splitview_visible0(self.inner, visible as i8) }
    }

    /// Show/hide the right/bottom child.
    pub fn visible1(&self, visible: bool) {
        unsafe { splitview_visible1(self.inner, visible as i8) }
    }

    /// Set the minimum size of the left/upper child.
    pub fn min_size0(&self, size: f32) {
        unsafe { splitview_minsize0(self.inner, size) }
    }

    /// Set the minimum size of the right/bottom child.
    pub fn min_size1(&self, size: f32) {
        unsafe { splitview_minsize1(self.inner, size) }
    }
}
