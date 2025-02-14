use std::rc::Rc;

use nappgui_sys::{
    splitview_horizontal, splitview_panel, splitview_pos, splitview_size, splitview_split,
    splitview_text, splitview_vertical, splitview_view, S2Df,
};

use crate::util::macros::impl_ptr;

use super::{Panel, TextView, View};

/// The SplitView are views divided into two parts, where in each of them we place another view or 
/// a panel. The dividing line is scrollable, which allows resizing both halves, dividing the total 
/// size of the control between the children.
pub struct SplitView {
    pub(crate) inner: Rc<*mut nappgui_sys::SplitView>,
}

impl SplitView {
    impl_ptr!(nappgui_sys::SplitView);

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

    /// Sets the default size of the view.
    pub fn size(&self, width: f32, height: f32) {
        let size = S2Df { width, height };
        unsafe { splitview_size(self.as_ptr(), size) }
    }

    /// Add a custom view to the splitview.
    pub fn view(&self, view: &View, tabstop: bool) {
        unsafe { splitview_view(self.as_ptr(), view.as_ptr(), tabstop as i8) }
    }

    /// Add a text view to the splitview.
    pub fn text(&self, view: &TextView, tabstop: bool) {
        unsafe { splitview_text(self.as_ptr(), view.as_ptr(), tabstop as i8) }
    }

    /// Add a splitview (child) to the splitview.
    pub fn split(&self, child: &SplitView) {
        unsafe { splitview_split(self.as_ptr(), child.as_ptr()) }
    }

    /// Add a panel to the splitview.
    pub fn panel(&self, panel: &Panel) {
        unsafe { splitview_panel(self.as_ptr(), panel.as_ptr()) }
    }

    /// Sets the position of the view separator.
    pub fn pos(&self, pos: f32) {
        unsafe { splitview_pos(self.as_ptr(), pos) }
    }
}
