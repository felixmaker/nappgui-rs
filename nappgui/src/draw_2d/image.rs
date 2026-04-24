use std::sync::Arc;

use nappgui_sys::{
    codec_t, image_codec, image_copy, image_destroy, image_format, image_frame_length, image_from_data,
    image_from_file, image_from_pixbuf, image_from_pixels, image_get_codec, image_height, image_num_frames,
    image_pixels, image_rotate, image_scale, image_to_file, image_trim, image_width, pixformat_t,
};

use super::{color::Color, palette::Palette, pixbuf::PixBuf};

/// The image type.
pub(crate) struct ImageInner {
    pub(crate) inner: *mut nappgui_sys::Image,
}

impl Drop for ImageInner {
    fn drop(&mut self) {
        unsafe { image_destroy(&mut self.inner) }
    }
}

/// The image type.
#[repr(transparent)]
pub struct Image {
    pub(crate) inner: Arc<ImageInner>,
}

/// The image trait.
impl Image {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Image) -> Self {
        assert!(!ptr.is_null());
        Self {
            inner: Arc::new(ImageInner { inner: ptr }),
        }
    }

    /// Create an image from an array of pixels.
    pub fn from_pixels(
        width: u32,
        height: u32,
        format: pixformat_t,
        data: &[u8],
        palette: Color,
        palsize: u32,
    ) -> Self {
        let image = unsafe { image_from_pixels(width, height, format, data.as_ptr(), &palette.inner, palsize) };
        unsafe { Image::from_raw(image) }
    }

    /// Create an image from a buffer pixel.
    pub fn from_pixbuf(pixbuf: &PixBuf, palette: &Palette) -> Self {
        let image = unsafe { image_from_pixbuf(pixbuf.inner, palette.inner) };
        unsafe { Image::from_raw(image) }
    }

    /// Create an image from a file on disk.
    pub fn from_file(path: &str) -> Option<Self> {
        let path = std::ffi::CString::new(path).unwrap();
        let mut error = nappgui_sys::_ferror_t_ekFUNDEF;
        let image = unsafe { image_from_file(path.as_ptr(), &mut error) };
        if image.is_null() {
            None
        } else {
            Some(unsafe { Image::from_raw(image) })
        }
    }

    /// Create an image from a buffer containing the encoded data.
    pub fn from_data(data: &[u8]) -> Self {
        let size = data.len();
        let image = unsafe { image_from_data(data.as_ptr(), size as u32) };
        unsafe { Image::from_raw(image) }
    }

    /// Save an image to disk, using the codec associated with it
    pub fn to_file(&self, path: &str) -> bool {
        let path = std::ffi::CString::new(path).unwrap();
        unsafe { image_to_file(self.as_ptr(), path.as_ptr(), std::ptr::null_mut()) != 0 }
    }

    /// Get the pixel format of the image.
    pub fn format(&self) -> pixformat_t {
        unsafe { image_format(self.as_ptr()) }
    }

    /// Get the width of the image in pixels.
    pub fn width(&self) -> u32 {
        unsafe { image_width(self.as_ptr()) }
    }

    /// Get the height of the image in pixels.
    pub fn height(&self) -> u32 {
        unsafe { image_height(self.as_ptr()) }
    }

    /// Get a buffer with the pixels that make up the decoded image.
    ///
    /// # Remarks
    /// If in pixformat we indicate ekFIMAGE it will return the buffer with the original format of the image.
    /// We can indicate ekRGB24, ekRGBA32 or ekGRAY8 if we need a specific format. Cannot use indexed formats.
    pub fn pixels(&self, format: pixformat_t) -> PixBuf {
        let pixbuf = unsafe { image_pixels(self.as_ptr(), format) };
        PixBuf::new(pixbuf)
    }

    /// Change the default codec associated with the image.
    ///
    /// # Remarks
    /// The change will take effect the next time we save or write the image. By default, the image retains
    /// the codec with which it was read. When we create it with image_from_pixels ekJPG codec is assigned
    /// as default. For images from 2d contexts dctx_image, the default codec is ekPNG. All codecs are supported
    /// by all graphical APIs, except ekGIF in some versions of Linux. Check the return value if it is imperative
    /// that your application export images in GIF.
    pub fn codec(&self, codec: codec_t) -> bool {
        unsafe { image_codec(self.as_ptr(), codec) != 0 }
    }

    /// Get the codec associated with the image.
    pub fn get_codec(&self) -> codec_t {
        unsafe { image_get_codec(self.as_ptr()) }
    }

    /// Get the number of sequences in animated images.
    ///
    /// # Remarks
    /// Only the gif format supports animations. For the rest 1 will always be returned.
    pub fn num_frames(&self) -> u32 {
        unsafe { image_num_frames(self.as_ptr()) }
    }

    /// Get the time of an animation sequence.
    ///
    /// # Remarks
    /// Only gif format supports animations.
    pub fn frame_length(&self, index: u32) -> f32 {
        unsafe { image_frame_length(self.as_ptr(), index) }
    }

    /// Create an image by cropping another image.
    pub fn trim(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let image = unsafe { image_trim(self.as_ptr(), x, y, width, height) };
        unsafe { Image::from_raw(image) }
    }

    /// Create a new image by rotating an existing one.
    pub fn rotate(&self, angle: f32, nsize: bool, background: Color) -> Self {
        let image = unsafe { image_rotate(self.as_ptr(), angle, nsize as _, background.inner, std::ptr::null_mut()) };
        unsafe { Image::from_raw(image) }
    }

    /// Create a copy of the image, with a new size.
    pub fn scale(&self, width: u32, height: u32) -> Self {
        let image = unsafe { image_scale(self.as_ptr(), width, height) };
        unsafe { Image::from_raw(image) }
    }

    /// Returns the raw pointer of Image object
    pub fn as_ptr(&self) -> *mut nappgui_sys::Image {
        self.inner.inner
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let image = unsafe { image_copy(self.as_ptr()) };
        unsafe { Image::from_raw(image) }
    }
}
