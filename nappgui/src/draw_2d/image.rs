use nappgui_sys::{
    image_codec, image_copy, image_data_imp, image_destroy, image_format, image_frame_length,
    image_from_data, image_from_file, image_from_pixbuf, image_from_pixels, image_from_resource,
    image_get_codec, image_get_data_imp, image_height, image_native, image_num_frames,
    image_pixels, image_read, image_rotate, image_scale, image_to_file, image_trim, image_width,
    image_write, pixformat_t,
};

use super::{color::Color};

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
        palette: &Color,
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
}
