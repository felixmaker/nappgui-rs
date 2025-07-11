use nappgui_sys::{
    codec_t, image_codec, image_copy, image_destroy, image_format, image_frame_length,
    image_from_data, image_from_file, image_from_pixbuf, image_from_pixels, image_get_codec,
    image_height, image_num_frames, image_pixels, image_rotate, image_scale, image_to_file,
    image_trim, image_width, pixformat_t,
};

use super::{color::Color, palette::Palette, pixbuf::Pixbuf};

/// Image
pub struct Image {
    pub(crate) inner: *mut nappgui_sys::Image,
}

impl Image {
    pub(crate) fn new(ptr: *mut nappgui_sys::Image) -> Self {
        if ptr.is_null() {
            panic!("Image is NULL");
        }

        Image { inner: ptr }
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
        let image = unsafe {
            image_from_pixels(
                width,
                height,
                format,
                data.as_ptr(),
                &palette.inner,
                palsize,
            )
        };
        Image::new(image)
    }

    /// Create an image from a buffer pixel.
    pub fn from_pixbuf(pixbuf: &Pixbuf, palette: &Palette) -> Self {
        let image = unsafe { image_from_pixbuf(pixbuf.inner, palette.inner) };
        Image::new(image)
    }

    /// Create an image from a file on disk.
    pub fn from_file(path: &str) -> Option<Self> {
        let path = std::ffi::CString::new(path).unwrap();
        let mut error = nappgui_sys::_ferror_t_ekFUNDEF;
        let image = unsafe { image_from_file(path.as_ptr(), &mut error) };
        if image.is_null() {
            None
        } else {
            Some(Image::new(image))
        }
    }

    /// Create an image from a buffer containing the encoded data.
    pub fn from_data(data: &[u8]) -> Self {
        let size = data.len();
        let image = unsafe { image_from_data(data.as_ptr(), size as u32) };
        Image::new(image)
    }

    /// Create an image by cropping another image.
    pub fn trim(&self, x: u32, y: u32, width: u32, height: u32) -> Image {
        let image = unsafe { image_trim(self.inner, x, y, width, height) };
        Image::new(image)
    }

    /// Create a new image by rotating an existing one.
    pub fn rotate(&self, angle: f32, nsize: bool, background: Color) -> Image {
        let image = unsafe {
            image_rotate(
                self.inner,
                angle,
                nsize as _,
                background.inner,
                std::ptr::null_mut(),
            )
        };
        Image::new(image)
    }

    /// Create a copy of the image, with a new size.
    pub fn scale(&self, width: u32, height: u32) -> Image {
        let image = unsafe { image_scale(self.inner, width, height) };
        Image::new(image)
    }

    /// Save an image to disk, using the codec associated with it
    pub fn to_file(&self, path: &str) -> bool {
        let path = std::ffi::CString::new(path).unwrap();
        unsafe { image_to_file(self.inner, path.as_ptr(), std::ptr::null_mut()) != 0 }
    }

    /// Get the pixel format of the image.
    pub fn format(&self) -> pixformat_t {
        unsafe { image_format(self.inner) }
    }

    /// Get the width of the image in pixels.
    pub fn width(&self) -> u32 {
        unsafe { image_width(self.inner) }
    }

    /// Get the height of the image in pixels.
    pub fn height(&self) -> u32 {
        unsafe { image_height(self.inner) }
    }

    /// Get a buffer with the pixels that make up the decoded image.
    ///
    /// # Remarks
    /// If in pixformat we indicate ekFIMAGE it will return the buffer with the original format of the image.
    /// We can indicate ekRGB24, ekRGBA32 or ekGRAY8 if we need a specific format. Cannot use indexed formats.
    pub fn pixels(&self, format: pixformat_t) -> Pixbuf {
        let pixbuf = unsafe { image_pixels(self.inner, format) };
        Pixbuf::new(pixbuf)
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
        unsafe { image_codec(self.inner, codec) != 0 }
    }

    /// Get the codec associated with the image.
    pub fn get_codec(&self) -> codec_t {
        unsafe { image_get_codec(self.inner) }
    }

    /// Get the number of sequences in animated images.
    ///
    /// # Remarks
    /// Only the gif format supports animations. For the rest 1 will always be returned.
    pub fn num_frames(&self) -> u32 {
        unsafe { image_num_frames(self.inner) }
    }

    /// Get the time of an animation sequence.
    ///
    /// # Remarks
    /// Only gif format supports animations.
    pub fn frame_length(&self, index: u32) -> f32 {
        unsafe { image_frame_length(self.inner, index) }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let image = unsafe { image_copy(self.inner) };
        Image::new(image)
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { image_destroy(&mut self.inner) }
    }
}
