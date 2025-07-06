use std::rc::Rc;

use crate::{
    draw_2d::Image,
    types::Scale,
    util::macros::{callback, pub_crate_ptr_ops},
};

use nappgui_sys::{
    imageview_OnClick, imageview_OnOverDraw, imageview_create, imageview_image, imageview_scale,
    imageview_size,
};

/// ImageView are specialized views in visualizing images and GIF animations.
pub struct ImageView {
    pub(crate) inner: Rc<*mut nappgui_sys::ImageView>,
}

impl ImageView {
    pub_crate_ptr_ops!(*mut nappgui_sys::ImageView);

    /// Create a image view.
    pub fn create() -> Self {
        let imageview = unsafe { imageview_create() };
        Self::from_raw(imageview)
    }

    /// Set the default control size.
    pub fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            imageview_size(self.as_ptr(), size);
        }
    }

    /// Set the scaling to apply to the image.
    pub fn scale(&self, scale: Scale) {
        unsafe {
            imageview_scale(self.as_ptr(), scale as _);
        }
    }

    /// Set the image to be displayed in the control.
    pub fn image(&self, image: &Image) {
        unsafe {
            imageview_image(self.as_ptr(), image.inner);
        }
    }

    callback! {
        /// Set a handle for the event click on the image.
        pub on_click(ImageView) => imageview_OnClick;

        /// Allows you to draw an overlay on the image when the mouse is over it.
        pub on_over_draw(ImageView) => imageview_OnOverDraw;
    }
}
