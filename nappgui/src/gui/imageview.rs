use std::rc::Rc;

use crate::{ draw_2d::Image, util::macros::{callback, impl_ptr}};

use nappgui_sys::{
    gui_scale_t, imageview_OnClick, imageview_OnOverDraw, imageview_create, imageview_image,
    imageview_scale, imageview_size,
};

/// ImageView are specialized views in visualizing images and GIF animations.
pub struct ImageView {
    pub(crate) inner: Rc<*mut nappgui_sys::ImageView>,
}

impl ImageView {
    impl_ptr!(nappgui_sys::ImageView);

    /// Create a image view.
    pub fn create() -> Self {
        let imageview = unsafe { imageview_create() };
        Self::new(imageview)
    }

    /// Set the default control size.
    pub fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe {
            imageview_size(self.as_ptr(), size);
        }
    }

    /// Set the scaling to apply to the image.
    pub fn scale(&self, scale: gui_scale_t) {
        unsafe {
            imageview_scale(self.as_ptr(), scale);
        }
    }

    /// Set the image to be displayed in the control.
    pub fn image(&self, image: &Image) {
        unsafe {
            imageview_image(self.as_ptr(), image.as_ptr());
        }
    }

    callback! {
        /// Set a handle for the event change of the image.
        pub on_click(ImageView) => imageview_OnClick;
        /// Allows you to draw an overlay on the image when the mouse is over it.
        pub on_over_draw(ImageView) => imageview_OnOverDraw;
    }
}
