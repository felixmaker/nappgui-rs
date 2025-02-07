use nappgui_sys::{
    pixbuf_cdata, pixbuf_convert, pixbuf_copy, pixbuf_create, pixbuf_data, pixbuf_destroy,
    pixbuf_dsize, pixbuf_format, pixbuf_format_bpp, pixbuf_get, pixbuf_height, pixbuf_set,
    pixbuf_size, pixbuf_trim, pixbuf_width, pixformat_t,
};

use super::palette::Palette;

pub struct Pixbuf {
    pub(crate) inner: *mut nappgui_sys::Pixbuf,
}

impl Pixbuf {
    pub(crate) fn new(ptr: *mut nappgui_sys::Pixbuf) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new pixel buffer.
    pub fn create(width: u32, height: u32, format: pixformat_t) -> Self {
        let pixbuf = unsafe { pixbuf_create(width, height, format) };
        Self::new(pixbuf)
    }

    /// Crop a buffer pixel.
    ///
    /// # Remarks
    /// The function does not check that the limits are valid. You will get a segmentation error in such cases.
    pub fn trim(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let pixbuf = unsafe { pixbuf_trim(self.inner, x, y, width, height) };
        Self::new(pixbuf)
    }

    /// Change the format of a buffer pixel.
    pub fn convert(&self, palette: &Palette, oformat: pixformat_t) -> Self {
        let pixbuf = unsafe { pixbuf_convert(self.inner, palette.inner, oformat) };
        Self::new(pixbuf)
    }

    /// Destroy the buffer.
    pub fn destroy(mut self) {
        unsafe { pixbuf_destroy(&mut self.inner) }
    }

    /// Get the pixel format.
    pub fn format(&self) -> pixformat_t {
        unsafe { pixbuf_format(self.inner) }
    }

    /// Get the width of the buffer.
    pub fn width(&self) -> u32 {
        unsafe { pixbuf_width(self.inner) }
    }

    /// Get the height of the buffer.       
    pub fn height(&self) -> u32 {
        unsafe { pixbuf_height(self.inner) }
    }

    /// Get the buffer size (in pixels).
    pub fn size(&self) -> u32 {
        unsafe { pixbuf_size(self.inner) }
    }

    /// Gets the buffer size (in bytes).
    pub fn dsize(&self) -> u32 {
        unsafe { pixbuf_dsize(self.inner) }
    }

    /// Gets a read-only pointer to the contents of the buffer.
    ///
    /// # Remarks
    /// Correctly manipulating the buffer requires knowing the Pixel formats and sometimes using the
    /// operators at the bit level. Use pixbuf_get to correctly read a pixel.
    pub fn cdata(&self) -> *const u8 {
        unsafe { pixbuf_cdata(self.inner) }
    }

    /// Gets a pointer to the contents of the buffer.
    ///
    /// # Remarks
    /// Correctly manipulating the buffer requires knowing the Pixel formats and sometimes using the
    /// operators at the bit level. Use pixbuf_get to correctly read a pixel.
    pub fn data(&mut self) -> *mut u8 {
        unsafe { pixbuf_data(self.inner) }
    }

    /// Gets bits per pixel based on format.
    pub fn format_bpp(format: pixformat_t) -> u32 {
        unsafe { pixbuf_format_bpp(format) }
    }

    /// Get the value of a pixel.
    pub fn get(&self, x: u32, y: u32) -> u32 {
        unsafe { pixbuf_get(self.inner, x, y) }
    }

    /// Sets the value of a pixel.
    pub fn set(&mut self, x: u32, y: u32, value: u32) {
        unsafe { pixbuf_set(self.inner, x, y, value) }
    }
}

impl Clone for Pixbuf {
    fn clone(&self) -> Self {
        let pixbuf = unsafe { pixbuf_copy(self.inner) };
        Self::new(pixbuf)
    }
}
