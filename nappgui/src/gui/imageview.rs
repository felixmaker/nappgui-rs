use std::sync::Arc;

use crate::{draw_2d::Image, types::Scale, util::macros::callback};

use nappgui_sys::{
    imageview_OnClick, imageview_OnOverDraw, imageview_create, imageview_image, imageview_scale, imageview_size,
};

pub(crate) struct ImageViewInner {
    inner: *mut nappgui_sys::ImageView,
}

/// ImageView are specialized views in visualizing images and GIF animations.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone)]
pub struct ImageView {
    pub(crate) inner: Arc<ImageViewInner>,
}

impl ImageView {
    /// Create a image view.
    pub fn new(width: f32, height: f32) -> Self {
        let imageview = unsafe { imageview_create() };
        assert!(!imageview.is_null());
        let imageview = ImageView {
            inner: Arc::new(ImageViewInner { inner: imageview }),
        };
        imageview.set_size(width, height);
        imageview
    }

    /// Set the default control size.
    pub fn set_size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            imageview_size(self.as_ptr(), size);
        }
    }

    /// Set the scaling to apply to the image.
    pub fn set_scale(&self, scale: Scale) {
        unsafe {
            imageview_scale(self.as_ptr(), scale as _);
        }
    }

    /// Set the image to be displayed in the control.
    pub fn set_image(&self, image: &Image) {
        unsafe {
            imageview_image(self.as_ptr(), image.as_ptr());
        }
    }

    callback! {
        /// Set a handle for the event click on the image.
        pub on_click() => imageview_OnClick;

        /// Allows you to draw an overlay on the image when the mouse is over it.
        pub on_over_draw() => imageview_OnOverDraw;
    }

    /// Returns a raw pointer to the image view object.
    pub fn as_ptr(&self) -> *mut nappgui_sys::ImageView {
        self.inner.inner
    }
}
