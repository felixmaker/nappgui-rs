use std::rc::Rc;

use nappgui_sys::{
    webview_OnFocus, webview_back, webview_create, webview_forward, webview_navigate, webview_size,
};

use crate::{gui::impl_layout, util::macros::{callback, impl_gui_control, pub_crate_ptr_ops}};

/// A WebView control will allow us to embed Web content in our application. It will behave in the same way
/// as other view controls such as View or TextView in terms of layout or resizing, displaying a fully
/// functional browser in its client area.
#[derive(Clone)]
pub struct WebView {
    pub(crate) inner: Rc<*mut nappgui_sys::WebView>,
}

impl WebView {
    pub_crate_ptr_ops!(*mut nappgui_sys::WebView);

    /// Create a Web View.
    pub fn new() -> Self {
        let webview = unsafe { webview_create() };
        Self::from_raw(webview)
    }

    callback! {
        /// Sets a handler for keyboard focus.
        pub on_focus(WebView, bool) => webview_OnFocus;
    }

    /// Sets the default size of the view.
    pub fn size(&self, width: f32, height: f32) {
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

impl_gui_control!(WebView, guicontrol_webview);
impl_layout!(WebView, layout_webview);
