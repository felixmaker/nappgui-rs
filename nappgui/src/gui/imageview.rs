use crate::{draw_2d::ImageTrait, gui::impl_layout, types::Scale, util::macros::callback};

use nappgui_sys::{
    imageview_OnClick, imageview_OnOverDraw, imageview_create, imageview_image, imageview_scale,
    imageview_size,
};

/// The image view trait.
pub trait ImageViewTrait {
    /// Returns a raw pointer to the image view object.
    fn as_ptr(&self) -> *mut nappgui_sys::ImageView;

    /// Set the default control size.
    fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            imageview_size(self.as_ptr(), size);
        }
    }

    /// Set the scaling to apply to the image.
    fn scale(&self, scale: Scale) {
        unsafe {
            imageview_scale(self.as_ptr(), scale as _);
        }
    }

    /// Set the image to be displayed in the control.
    fn image<T>(&self, image: &T)
    where
        T: ImageTrait,
    {
        unsafe {
            imageview_image(self.as_ptr(), image.as_ptr());
        }
    }

    callback! {
        /// Set a handle for the event click on the image.
         on_click() => imageview_OnClick;

        /// Allows you to draw an overlay on the image when the mouse is over it.
         on_over_draw() => imageview_OnOverDraw;
    }
}

/// ImageView are specialized views in visualizing images and GIF animations.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ImageView {
    pub(crate) inner: *mut nappgui_sys::ImageView,
}

impl ImageViewTrait for ImageView {
    fn as_ptr(&self) -> *mut nappgui_sys::ImageView {
        self.inner
    }
}

impl ImageView {
    /// Create a image view.
    pub fn new(width: f32, height: f32) -> Self {
        let imageview = unsafe { imageview_create() };
        let imageview = Self { inner: imageview };
        imageview.size(width, height);
        imageview
    }
}

impl_layout!(ImageView, ImageViewTrait, layout_imageview);
