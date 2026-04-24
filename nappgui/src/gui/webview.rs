use std::sync::Arc;

use nappgui_sys::{webview_OnFocus, webview_back, webview_create, webview_forward, webview_navigate, webview_size};

use crate::util::macros::callback;

/// The webview type.
pub(crate) struct WebViewInner {
    pub(crate) inner: *mut nappgui_sys::WebView,
}

/// A WebView control will allow us to embed Web content in our application. It will behave in the same way
/// as other view controls such as View or TextView in terms of layout or resizing, displaying a fully
/// functional browser in its client area.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
pub struct WebView {
    pub(crate) inner: Arc<WebViewInner>,
}

impl WebView {
    /// Create a Web View.
    pub fn new() -> Self {
        let webview = unsafe { webview_create() };
        assert!(!webview.is_null());
        Self {
            inner: Arc::new(WebViewInner { inner: webview }),
        }
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

    /// Returns a raw pointer to the webview object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::WebView {
        self.inner.inner
    }
}
