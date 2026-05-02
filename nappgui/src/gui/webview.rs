use std::{
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{webview_OnFocus, webview_back, webview_create, webview_forward, webview_navigate, webview_size};

use crate::{
    gui::{global_get, global_record},
    util::macros::callback,
};

pub(crate) struct WebViewInner {
    ptr: NonNull<nappgui_sys::WebView>,
}

impl WebViewInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::WebView) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to WebViewInner::from_raw"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::WebView {
        self.ptr.as_ptr()
    }
}

/// The web view control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct WebView(Weak<WebViewInner>);

impl WebView {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::WebView) -> Self {
        let object = global_record(ptr as _, WebViewInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::WebView) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::WebView {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a Web View.
    pub fn new() -> Self {
        unsafe { Self::from_raw(webview_create()) }
    }

    callback! {
        /// Sets a handler for keyboard focus.
        pub on_focus(bool) => webview_OnFocus;
    }

    /// Sets the default size of the view.
    pub fn set_size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { webview_size(self.as_ptr(), size) }
    }

    /// Loads a URL in the web view.
    pub fn navigate(&self, url: &str) {
        let url = std::ffi::CString::new(url).unwrap();
        unsafe { webview_navigate(self.as_ptr(), url.as_ptr()) }
    }

    /// Go back to the previous page in the browser stack.
    pub fn back(&self) {
        unsafe { webview_back(self.as_ptr()) }
    }

    /// Moves to the next page in the browser stack.
    pub fn forward(&self) {
        unsafe { webview_forward(self.as_ptr()) }
    }
}
