use std::rc::{Rc, Weak};

use nappgui_sys::{
    codec_t, image_codec, image_copy, image_destroy, image_format, image_frame_length,
    image_from_data, image_from_file, image_from_pixbuf, image_from_pixels, image_get_codec,
    image_height, image_num_frames, image_pixels, image_rotate, image_scale, image_to_file,
    image_trim, image_width, pixformat_t,
};

use super::{color::Color, palette::Palette, pixbuf::Pixbuf};

/// Image
pub struct Image {
    pub(crate) inner: Rc<ImageInner>,
}

/// The image inner type.
#[repr(transparent)]
pub struct ImageInner {
    pub(crate) inner: *mut nappgui_sys::Image,
}

/// The image trait.
pub trait ImageTrait {
    /// Returns the raw pointer of Image object
    fn as_ptr(&self) -> *mut nappgui_sys::Image;

    /// Save an image to disk, using the codec associated with it
    fn to_file(&self, path: &str) -> bool {
        let path = std::ffi::CString::new(path).unwrap();
        unsafe { image_to_file(self.as_ptr(), path.as_ptr(), std::ptr::null_mut()) != 0 }
    }

    /// Get the pixel format of the image.
    fn format(&self) -> pixformat_t {
        unsafe { image_format(self.as_ptr()) }
    }

    /// Get the width of the image in pixels.
    fn width(&self) -> u32 {
        unsafe { image_width(self.as_ptr()) }
    }

    /// Get the height of the image in pixels.
    fn height(&self) -> u32 {
        unsafe { image_height(self.as_ptr()) }
    }

    /// Get a buffer with the pixels that make up the decoded image.
    ///
    /// # Remarks
    /// If in pixformat we indicate ekFIMAGE it will return the buffer with the original format of the image.
    /// We can indicate ekRGB24, ekRGBA32 or ekGRAY8 if we need a specific format. Cannot use indexed formats.
    fn pixels(&self, format: pixformat_t) -> Pixbuf {
        let pixbuf = unsafe { image_pixels(self.as_ptr(), format) };
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
    fn codec(&self, codec: codec_t) -> bool {
        unsafe { image_codec(self.as_ptr(), codec) != 0 }
    }

    /// Get the codec associated with the image.
    fn get_codec(&self) -> codec_t {
        unsafe { image_get_codec(self.as_ptr()) }
    }

    /// Get the number of sequences in animated images.
    ///
    /// # Remarks
    /// Only the gif format supports animations. For the rest 1 will always be returned.
    fn num_frames(&self) -> u32 {
        unsafe { image_num_frames(self.as_ptr()) }
    }

    /// Get the time of an animation sequence.
    ///
    /// # Remarks
    /// Only gif format supports animations.
    fn frame_length(&self, index: u32) -> f32 {
        unsafe { image_frame_length(self.as_ptr(), index) }
    }

    /// Create an image by cropping another image.
    fn trim(&self, x: u32, y: u32, width: u32, height: u32) -> Image {
        let image = unsafe { image_trim(self.as_ptr(), x, y, width, height) };
        Image::from_raw(image)
    }

    /// Create a new image by rotating an existing one.
    fn rotate(&self, angle: f32, nsize: bool, background: Color) -> Image {
        let image = unsafe {
            image_rotate(
                self.as_ptr(),
                angle,
                nsize as _,
                background.inner,
                std::ptr::null_mut(),
            )
        };
        Image::from_raw(image)
    }

    /// Create a copy of the image, with a new size.
    fn scale(&self, width: u32, height: u32) -> Image {
        let image = unsafe { image_scale(self.as_ptr(), width, height) };
        Image::from_raw(image)
    }
}

impl Image {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::Image) -> Self {
        if ptr.is_null() {
            panic!("Image is NULL");
        }
        Image {
            inner: Rc::new(ImageInner { inner: ptr }),
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
        Image::from_raw(image)
    }

    /// Create an image from a buffer pixel.
    pub fn from_pixbuf(pixbuf: &Pixbuf, palette: &Palette) -> Self {
        let image = unsafe { image_from_pixbuf(pixbuf.inner, palette.inner) };
        Image::from_raw(image)
    }

    /// Create an image from a file on disk.
    pub fn from_file(path: &str) -> Option<Self> {
        let path = std::ffi::CString::new(path).unwrap();
        let mut error = nappgui_sys::_ferror_t_ekFUNDEF;
        let image = unsafe { image_from_file(path.as_ptr(), &mut error) };
        if image.is_null() {
            None
        } else {
            Some(Image::from_raw(image))
        }
    }

    /// Create an image from a buffer containing the encoded data.
    pub fn from_data(data: &[u8]) -> Self {
        let size = data.len();
        let image = unsafe { image_from_data(data.as_ptr(), size as u32) };
        Image::from_raw(image)
    }

    /// Get the weak reference of the window.
    pub fn as_weak(&self) -> Weak<ImageInner> {
        Rc::downgrade(&self.inner)
    }
}

impl ImageTrait for ImageInner {
    fn as_ptr(&self) -> *mut nappgui_sys::Image {
        self.inner
    }
}

/// Window Weak Reference
pub type WeakImage = Weak<ImageInner>;

impl ImageTrait for WeakImage {
    fn as_ptr(&self) -> *mut nappgui_sys::Image {
        let image = self.upgrade().unwrap();
        image.inner
    }
}

impl ImageTrait for Image {
    fn as_ptr(&self) -> *mut nappgui_sys::Image {
        self.inner.inner
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let image = unsafe { image_copy(self.as_ptr()) };
        Image::from_raw(image)
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { image_destroy(&mut self.as_ptr()) }
    }
}
