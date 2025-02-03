use nappgui_sys::{regex_create, regex_destroy, regex_match};

pub struct RegEx {
    pub(crate) inner: *mut nappgui_sys::RegEx,
}

impl RegEx {
    pub(crate) fn new(ptr: *mut nappgui_sys::RegEx) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a regular expression from a pattern.
    pub fn create(pattern: &str) -> Self {
        let pattern = std::ffi::CString::new(pattern).unwrap();
        let regex = unsafe { regex_create(pattern.as_ptr()) };
        Self::new(regex)
    }

    /// Check if a string matches the search pattern.
    pub fn matches(&self, text: &str) -> bool {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { regex_match(self.inner, text.as_ptr()) != 0 }
    }
}

impl Drop for RegEx {
    fn drop(&mut self) {
        unsafe { regex_destroy(&mut self.inner) };
    }
}
