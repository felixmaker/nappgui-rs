use std::ffi::{CStr, CString};

/// String objects contain dynamically reserved UTF-8 character strings.
#[repr(transparent)]
pub struct NappguiString {
    pub(crate) inner: *mut nappgui_sys::String,
}

impl NappguiString {
    /// Create a String from a UTF-8-encoded C string.
    pub fn new(text: &str) -> Self {
        let text = CString::new(text).unwrap();
        let inner = unsafe { nappgui_sys::str_c(text.as_ptr()) };
        Self { inner }
    }

    /// Returns the inner C string in format UTF-8 contained in the String.
    pub fn as_cstr(&self) -> &CStr {
        let cstr = unsafe { nappgui_sys::tc(self.inner) };
        assert!(!cstr.is_null());
        unsafe { CStr::from_ptr(cstr) }
    }

    /// Returns the Rust String.
    pub fn to_string(&self) -> String {
        self.as_cstr().to_string_lossy().into_owned()
    }

    /// Returns the size in bytes of a string.
    ///
    /// # Remarks
    /// In UTF-8 strings the number of bytes is not the same as the characters. str_nchars.
    pub fn len(&self) -> u32 {
        unsafe { nappgui_sys::str_len(self.inner) }
    }

    /// Returns the number of characters of a string object.
    ///
    /// # Remarks
    /// In UTF-8 strings the number of bytes is not the same as the characters.
    pub fn nchars(&self) -> u32 {
        unsafe { nappgui_sys::str_nchars(self.inner) }
    }

    /// Check if a string is empty (str->data[0] == '\0').
    pub fn is_empty(&self) -> bool {
        unsafe { nappgui_sys::str_empty(self.inner) != 0 }
    }
}

impl PartialEq for NappguiString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { nappgui_sys::str_scmp(self.inner, other.inner) == 0 }
    }
}

impl Eq for NappguiString {}

impl Clone for NappguiString {
    fn clone(&self) -> Self {
        let inner = unsafe { nappgui_sys::str_copy(self.inner) };
        Self { inner }
    }
}

impl Drop for NappguiString {
    fn drop(&mut self) {
        unsafe {
            nappgui_sys::str_destroy(&mut self.inner);
        }
    }
}
