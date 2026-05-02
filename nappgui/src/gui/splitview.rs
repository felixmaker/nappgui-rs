use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{
    splitview_get_pos, splitview_horizontal, splitview_minsize0, splitview_minsize1, splitview_panel, splitview_pos,
    splitview_splitview, splitview_textview, splitview_vertical, splitview_view, splitview_visible0,
    splitview_visible1, splitview_webview,
};

use crate::{
    gui::{Panel, TextView, View, WebView, global_get, global_record},
    types::SplitMode,
};

pub(crate) struct SplitViewInner {
    ptr: NonNull<nappgui_sys::SplitView>,
}

impl SplitViewInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::SplitView) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to SplitViewInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::SplitView {
        self.ptr.as_ptr()
    }
}

/// The splitview control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct SplitView(Weak<SplitViewInner>);

impl SplitView {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::SplitView) -> Self {
        let object = global_record(ptr as _, SplitViewInner::from_raw(ptr));
        SplitView(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::SplitView) -> Self {
        let object = global_get(ptr as _).unwrap();
        SplitView(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::SplitView {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

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
    /// # Remark
    /// For splitview and panel, the tabstop parameter is always set to TRUE even if tabstop is set
    /// to false.
    pub fn add<T>(&self, control: &T, tabstop: bool)
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
    pub fn set_first_child_visible(&self, visible: bool) {
        unsafe { splitview_visible0(self.as_ptr(), visible as _) }
    }

    /// Show or hide the right/bottom child.
    pub fn set_last_child_visible(&self, visible: bool) {
        unsafe { splitview_visible1(self.as_ptr(), visible as _) }
    }

    /// Set the minimum size of the left/upper child.
    pub fn set_first_child_min_size(&self, size: f32) {
        unsafe { splitview_minsize0(self.as_ptr(), size) }
    }

    /// Set the minimum size of the right/bottom child.
    pub fn set_last_child_min_size(&self, size: f32) {
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
    /// Tabstop is set to TRUE by default.
    fn insert_into_splitview(&self, split_view: &SplitView, _tabstop: bool) {
        unsafe { splitview_splitview(split_view.as_ptr(), self.as_ptr()) }
    }
}

impl SplitViewInsertChildTrait for Panel {
    /// Tabstop is set to TRUE by default.
    fn insert_into_splitview(&self, split_view: &SplitView, _tabstop: bool) {
        unsafe { splitview_panel(split_view.as_ptr(), self.as_ptr()) }
    }
}
