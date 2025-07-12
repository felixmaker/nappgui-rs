use nappgui_sys::{
    splitview_get_pos, splitview_horizontal, splitview_minsize0, splitview_minsize1,
    splitview_panel, splitview_pos, splitview_splitview, splitview_textview, splitview_vertical,
    splitview_view, splitview_visible0, splitview_visible1, splitview_webview,
};

use crate::{
    gui::{control::impl_control, impl_layout, PanelTrait, TextViewTrait, ViewTrait, WebViewTrait},
    types::SplitMode,
};

/// The splitview trait.
pub trait SplitViewTrait {
    /// Returns a raw pointer to the splitview object.
    fn as_ptr(&self) -> *mut nappgui_sys::SplitView;

    /// Add a custom view to the splitview.
    fn view<T>(&self, view: T, tabstop: bool)
    where
        T: ViewTrait,
    {
        unsafe { splitview_view(self.as_ptr(), view.as_ptr(), tabstop as _) }
    }

    /// Add a text view to the splitview.
    fn textview<T>(&self, view: T, tabstop: bool)
    where
        T: TextViewTrait,
    {
        unsafe { splitview_textview(self.as_ptr(), view.as_ptr(), tabstop as _) }
    }

    /// Add a web view to SplitView.
    fn webview<T>(&self, view: T, tabstop: bool)
    where
        T: WebViewTrait,
    {
        unsafe { splitview_webview(self.as_ptr(), view.as_ptr(), tabstop as _) }
    }

    /// Add a splitview (child) to the splitview.
    fn splitview<T>(&self, child: T)
    where
        T: SplitViewTrait,
    {
        unsafe { splitview_splitview(self.as_ptr(), child.as_ptr()) }
    }

    /// Add a panel to the splitview.
    fn panel<T>(&self, panel: T)
    where
        T: PanelTrait,
    {
        unsafe { splitview_panel(self.as_ptr(), panel.as_ptr()) }
    }

    /// Sets the position of the view separator.
    fn pos(&self, mode: SplitMode, pos: f32) {
        unsafe { splitview_pos(self.as_ptr(), mode as _, pos) }
    }

    /// Get the current divider position.
    fn get_pos(&self, mode: SplitMode) -> f32 {
        unsafe { splitview_get_pos(self.as_ptr(), mode as _) }
    }

    /// Show/hide the left/upper child.
    fn visible0(&self, visible: bool) {
        unsafe { splitview_visible0(self.as_ptr(), visible as _) }
    }

    /// Show/hide the right/bottom child.
    fn visible1(&self, visible: bool) {
        unsafe { splitview_visible1(self.as_ptr(), visible as _) }
    }

    /// Set the minimum size of the left/upper child.
    fn min_size0(&self, size: f32) {
        unsafe { splitview_minsize0(self.as_ptr(), size) }
    }

    /// Set the minimum size of the right/bottom child.
    fn min_size1(&self, size: f32) {
        unsafe { splitview_minsize1(self.as_ptr(), size) }
    }
}

/// The SplitView are views divided into two parts, where in each of them we place another view or
/// a panel. The dividing line is scrollable, which allows resizing both halves, dividing the total
/// size of the control between the children.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct SplitView {
    pub(crate) inner: *mut nappgui_sys::SplitView,
}

impl SplitViewTrait for SplitView {
    fn as_ptr(&self) -> *mut nappgui_sys::SplitView {
        self.inner
    }
}

impl SplitView {
    /// Create a splitview with horizontal split.
    pub fn new_horizontal() -> Self {
        let splitview = unsafe { splitview_horizontal() };
        Self { inner: splitview }
    }

    /// Create a splitview with vertical split.
    pub fn new_vertical() -> Self {
        let splitview = unsafe { splitview_vertical() };
        Self { inner: splitview }
    }
}

impl_control!(SplitView, guicontrol_splitview);
impl_layout!(SplitView, SplitViewTrait, layout_splitview);
