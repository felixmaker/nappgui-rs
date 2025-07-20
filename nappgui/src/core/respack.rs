/// Package of resources that will be loaded together. Use ResId to access a specific resource. Resources.
pub struct ResPack {
    pub(crate) inner: *mut nappgui_sys::ResPack,
    pub(crate) len: usize,
}

impl ResPack {
    /// Returns the inner pointer of the resource package.
    pub fn as_ptr(&self) -> *mut nappgui_sys::ResPack {
        self.inner
    }

    /// Creates a embedded resource package.
    pub fn new_embedded(name: &str) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        let pack = unsafe { nappgui_sys::respack_embedded(name.as_ptr()) };
        Self {
            inner: pack,
            len: 0,
        }
    }

    /// Add a message to the resource package.
    pub fn add_message(&mut self, message: &str) -> usize {
        let message = std::ffi::CString::new(message).unwrap();
        unsafe { nappgui_sys::respack_add_msg(self.as_ptr(), message.as_ptr()) }
        self.len += 1;
        self.len
    }

    /// Add a byte array to the resource package. Used for embedding images.
    pub fn add_bytes(&mut self, bytes: &[u8]) -> usize {
        unsafe {
            nappgui_sys::respack_add_cdata(self.as_ptr(), 1, bytes.as_ptr(), bytes.len() as u32)
        }
        self.len += 1;
        self.len
    }

    /// Add a file to the resource package. Used for other file.
    pub fn add_file(&mut self, bytes: &[u8]) -> usize {
        unsafe {
            nappgui_sys::respack_add_cdata(self.as_ptr(), 2, bytes.as_ptr(), bytes.len() as u32)
        }
        self.len += 1;
        self.len
    }
}

#[doc(hidden)]
/// Used for macros to generate code only depending on nappgui
pub type ResPackPtr = *mut nappgui_sys::ResPack;
