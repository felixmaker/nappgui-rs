use std::rc::Rc;

use nappgui_sys::{
    pixbuf_cdata, pixbuf_convert, pixbuf_copy, pixbuf_create, pixbuf_data, pixbuf_destroy,
    pixbuf_dsize, pixbuf_format, pixbuf_format_bpp, pixbuf_get, pixbuf_height, pixbuf_set,
    pixbuf_size, pixbuf_trim, pixbuf_width
};

use crate::{prelude::*, util::macros::impl_ptr};
use super::Palette;

/// Pixbuf
pub struct Pixbuf {
    pub(crate) inner: Rc<*mut nappgui_sys::Pixbuf>,
}

impl Pixbuf {
    impl_ptr!(nappgui_sys::Pixbuf);

    /// Create a new pixel buffer.
    pub fn create(width: u32, height: u32, format: PixFormat) -> Self {
        let pixbuf = unsafe { pixbuf_create(width, height, format) };
        Self::new(pixbuf)
    }

    /// Crop a buffer pixel.
    ///
    /// # Remarks
    /// The function does not check that the limits are valid. You will get a segmentation error in such cases.
    pub fn trim(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let pixbuf = unsafe { pixbuf_trim(self.as_ptr(), x, y, width, height) };
        Self::new(pixbuf)
    }

    /// Change the format of a buffer pixel.
    pub fn convert(&self, palette: &Palette, oformat: PixFormat) -> Self {
        let pixbuf = unsafe { pixbuf_convert(self.as_ptr(), palette.as_ptr(), oformat) };
        Self::new(pixbuf)
    }

    /// Get the pixel format.
    pub fn format(&self) -> PixFormat {
        unsafe { pixbuf_format(self.as_ptr()) }
    }

    /// Get the width of the buffer.
    pub fn width(&self) -> u32 {
        unsafe { pixbuf_width(self.as_ptr()) }
    }

    /// Get the height of the buffer.       
    pub fn height(&self) -> u32 {
        unsafe { pixbuf_height(self.as_ptr()) }
    }

    /// Get the buffer size (in pixels).
    pub fn size(&self) -> u32 {
        unsafe { pixbuf_size(self.as_ptr()) }
    }

    /// Gets the buffer size (in bytes).
    pub fn dsize(&self) -> u32 {
        unsafe { pixbuf_dsize(self.as_ptr()) }
    }

    /// Gets a read-only pointer to the contents of the buffer.
    ///
    /// # Remarks
    /// Correctly manipulating the buffer requires knowing the Pixel formats and sometimes using the
    /// operators at the bit level. Use pixbuf_get to correctly read a pixel.
    pub fn cdata(&self) -> *const u8 {
        unsafe { pixbuf_cdata(self.as_ptr()) }
    }

    /// Gets a pointer to the contents of the buffer.
    ///
    /// # Remarks
    /// Correctly manipulating the buffer requires knowing the Pixel formats and sometimes using the
    /// operators at the bit level. Use pixbuf_get to correctly read a pixel.
    pub fn data(&mut self) -> *mut u8 {
        unsafe { pixbuf_data(self.as_ptr()) }
    }

    /// Gets bits per pixel based on format.
    pub fn format_bpp(format: PixFormat) -> u32 {
        unsafe { pixbuf_format_bpp(format) }
    }

    /// Get the value of a pixel.
    pub fn get(&self, x: u32, y: u32) -> u32 {
        unsafe { pixbuf_get(self.as_ptr(), x, y) }
    }

    /// Sets the value of a pixel.
    pub fn set(&mut self, x: u32, y: u32, value: u32) {
        unsafe { pixbuf_set(self.as_ptr(), x, y, value) }
    }
}

impl Clone for Pixbuf {
    fn clone(&self) -> Self {
        let pixbuf = unsafe { pixbuf_copy(self.as_ptr()) };
        Self::new(pixbuf)
    }
}

impl Drop for Pixbuf  {
    fn drop(&mut self) {
        unsafe { pixbuf_destroy(&mut self.as_ptr()) };
    }
}
