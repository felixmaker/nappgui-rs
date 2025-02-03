use nappgui_sys::{dctx_bitmap, pixformat_t};

pub struct DCtx {
    pub(crate) inner: *mut nappgui_sys::DCtx,
}

impl DCtx {
    pub(crate) fn new(ptr: *mut nappgui_sys::DCtx) -> Self {
        if ptr.is_null() {
            panic!("DCtx is NULL");
        }
        Self { inner: ptr }
    }

    /// Create a memory context, in order to generate an image.
    ///
    /// # Remark
    /// When we finish drawing, we must call dctx_image to get the picture.
    pub fn bitmap(width: u32, height: u32, format: pixformat_t) -> Self {
        let ptr = unsafe { dctx_bitmap(width, height, format) };
        DCtx::new(ptr)
    }
}
