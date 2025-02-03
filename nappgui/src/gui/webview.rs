use nappgui_sys::{
    webview_OnFocus, webview_back, webview_create, webview_forward, webview_navigate, webview_size,
};

pub struct WebView {
    pub(crate) inner: *mut nappgui_sys::WebView,
}

impl WebView {
    pub(crate) fn new(ptr: *mut nappgui_sys::WebView) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a Web View.
    pub fn create() -> Self {
        let webview = unsafe { webview_create() };
        Self::new(webview)
    }

    /// Sets a handler for keyboard focus.
    pub fn on_focus<F>(&self, handler: F)
    where
        F: FnMut(&mut WebView, bool) + 'static,
    {
        todo!();
    }

    /// Sets the default size of the view.
    pub fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { webview_size(self.inner, size) }
    }

    /// Loads a URL in the web view.
    pub fn navigate(&self, url: &str) {
        let url = std::ffi::CString::new(url).unwrap();
        unsafe { webview_navigate(self.inner, url.as_ptr()) }
    }

    /// Go back to the previous page in the browser stack.
    pub fn back(&self) {
        unsafe { webview_back(self.inner) }
    }

    /// Moves to the next page in the browser stack.
    pub fn forward(&self) {
        unsafe { webview_forward(self.inner) }
    }
}
