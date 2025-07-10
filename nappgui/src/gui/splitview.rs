use std::rc::Rc;

use nappgui_sys::{
    splitview_get_pos, splitview_horizontal, splitview_minsize0, splitview_minsize1,
    splitview_panel, splitview_pos, splitview_splitview, splitview_textview, splitview_vertical,
    splitview_view, splitview_visible0, splitview_visible1, splitview_webview,
};

use crate::{
    gui::{control::impl_control, impl_layout, WebView},
    types::SplitMode,
    util::macros::pub_crate_ptr_ops,
};

use super::{Panel, TextView, View};

/// The SplitView are views divided into two parts, where in each of them we place another view or
/// a panel. The dividing line is scrollable, which allows resizing both halves, dividing the total
/// size of the control between the children.
#[derive(Clone)]
pub struct SplitView {
    pub(crate) inner: Rc<*mut nappgui_sys::SplitView>,
}

impl SplitView {
    pub_crate_ptr_ops!(*mut nappgui_sys::SplitView);

    /// Create a splitview with horizontal split.
    pub fn new_horizontal() -> Self {
        let splitview = unsafe { splitview_horizontal() };
        Self::from_raw(splitview)
    }

    /// Create a splitview with vertical split.
    pub fn new_vertical() -> Self {
        let splitview = unsafe { splitview_vertical() };
        Self::from_raw(splitview)
    }

    /// Add a custom view to the splitview.
    pub fn view(&self, view: &View, tabstop: bool) {
        unsafe { splitview_view(self.as_ptr(), view.as_ptr(), tabstop as _) }
    }

    /// Add a text view to the splitview.
    pub fn textview(&self, view: &TextView, tabstop: bool) {
        unsafe { splitview_textview(self.as_ptr(), view.as_ptr(), tabstop as _) }
    }

    /// Add a web view to SplitView.
    pub fn webview(&self, view: &WebView, tabstop: bool) {
        unsafe { splitview_webview(self.as_ptr(), view.as_ptr(), tabstop as _) }
    }

    /// Add a splitview (child) to the splitview.
    pub fn splitview(&self, child: &SplitView) {
        unsafe { splitview_splitview(self.as_ptr(), child.as_ptr()) }
    }

    /// Add a panel to the splitview.
    pub fn panel(&self, panel: &Panel) {
        unsafe { splitview_panel(self.as_ptr(), panel.as_ptr()) }
    }

    /// Sets the position of the view separator.
    pub fn pos(&self, mode: SplitMode, pos: f32) {
        unsafe { splitview_pos(self.as_ptr(), mode as _, pos) }
    }

    /// Get the current divider position.
    pub fn get_pos(&self, mode: SplitMode) -> f32 {
        unsafe { splitview_get_pos(self.as_ptr(), mode as _) }
    }

    /// Show/hide the left/upper child.
    pub fn visible0(&self, visible: bool) {
        unsafe { splitview_visible0(self.as_ptr(), visible as _) }
    }

    /// Show/hide the right/bottom child.
    pub fn visible1(&self, visible: bool) {
        unsafe { splitview_visible1(self.as_ptr(), visible as _) }
    }

    /// Set the minimum size of the left/upper child.
    pub fn min_size0(&self, size: f32) {
        unsafe { splitview_minsize0(self.as_ptr(), size) }
    }

    /// Set the minimum size of the right/bottom child.
    pub fn min_size1(&self, size: f32) {
        unsafe { splitview_minsize1(self.as_ptr(), size) }
    }
}

impl_control!(SplitView, guicontrol_splitview);
impl_layout!(SplitView, layout_splitview);
