use std::rc::Rc;

use nappgui_sys::{webview_OnFocus, webview_back, webview_create, webview_forward, webview_navigate, webview_size};

use crate::gui::{Callback, define_object, listener};

#[derive(Default)]
pub(crate) struct WebViewProps {
    on_focus: Callback<bool>,
}

define_object!(WebView, WebViewInner, WebView, WebViewProps);

impl WebView {
    /// Create a Web View.
    pub fn new() -> Self {
        unsafe { Self::from_raw(webview_create()) }
    }

    /// Set an event handler for keyboard focus.
    pub fn set_on_focus_handler<F>(&self, handler: F)
    where
        F: Fn(&bool) + 'static,
    {
        self.inner(|inner| *inner.props.on_focus.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), WebViewInner, on_focus(bool));
        unsafe { webview_OnFocus(self.as_ptr(), listener) }
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
