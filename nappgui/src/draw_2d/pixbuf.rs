use nappgui_sys::{
    pixbuf_cdata, pixbuf_convert, pixbuf_copy, pixbuf_create, pixbuf_data, pixbuf_destroy,
    pixbuf_dsize, pixbuf_format, pixbuf_format_bpp, pixbuf_get, pixbuf_height, pixbuf_set,
    pixbuf_size, pixbuf_trim, pixbuf_width,
};

use crate::util::macros::impl_i32_to_enum;

use super::Palette;

/// Pixel format in an image. Number of bits per pixel and color model.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixFormat {
    /// 1 bit per pixel. 2 colors, indexed.
    Index1 = 1,
    /// 2 bits per pixel. 4 colors, indexed.
    Index2 = 2,
    /// 4 bits per pixel. 16 colors, indexed.
    Index4 = 3,
    /// 8 bits per pixel. 256 colors, indexed.
    Index8 = 4,
    /// 8 bits per pixel in grayscale. 256 shades of gray.
    Gray8 = 5,
    /// 24 bits per RGB pixel. 8 bits per channel (red, green, blue). The lowest order byte corresponds to the red one and the highest one to the blue one.
    RGB24 = 6,
    /// 32 bits per pixel RGBA. 8 bits per channel (red, green, blue, alpha). The lowest order byte corresponds to the red one and the highest one to alpha (transparency).
    RGBA32 = 7,
    /// Represents the original format of the image. Only valid at image_pixels.
    Image = 8,
}

impl_i32_to_enum!(PixFormat, 1..=8);

/// Pixbuf
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
    pub fn create(width: u32, height: u32, format: PixFormat) -> Self {
        let pixbuf = unsafe { pixbuf_create(width, height, format as _) };
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
    pub fn convert(&self, palette: &Palette, oformat: PixFormat) -> Self {
        let pixbuf = unsafe { pixbuf_convert(self.inner, palette.inner, oformat as _) };
        Self::new(pixbuf)
    }

    /// Get the pixel format.
    pub fn format(&self) -> PixFormat {
        let result = unsafe { pixbuf_format(self.inner) };
        PixFormat::try_from(result).unwrap()
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
    pub fn format_bpp(format: PixFormat) -> u32 {
        unsafe { pixbuf_format_bpp(format as _) }
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

impl Drop for Pixbuf {
    fn drop(&mut self) {
        unsafe {
            pixbuf_destroy(&mut self.inner);
        }
    }
}
