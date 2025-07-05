use std::rc::Rc;

use crate::{core::event::Event, draw_2d::Image, types::Scale, util::macros::pub_crate_ptr_ops};

use nappgui_sys::{
    imageview_OnClick, imageview_OnOverDraw, imageview_create, imageview_image, imageview_scale,
    imageview_size, listener_imp,
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

    /// Set a handle for the event click on the image.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(&mut ImageView, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (
                Box<dyn FnMut(&mut ImageView, &Event)>,
                *mut nappgui_sys::ImageView,
            );
            let f = &mut *(*data).0;
            let mut obj = ImageView::new((*data).1);
            let ev = Event::new(event as _);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &ev)));
        }

        let cb: Box<dyn FnMut(&mut ImageView, &Event)> = Box::new(handler);

        let data: *mut (
            Box<dyn FnMut(&mut ImageView, &Event)>,
            *mut nappgui_sys::ImageView,
        ) = Box::into_raw(Box::new((cb, self.as_ptr())));

        unsafe {
            imageview_OnClick(
                self.as_ptr(),
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Allows you to draw an overlay on the image when the mouse is over it.
    pub fn on_over_draw<F>(&self, handler: F)
    where
        F: FnMut(&mut ImageView, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (
                Box<dyn FnMut(&mut ImageView, &Event)>,
                *mut nappgui_sys::ImageView,
            );
            let f = &mut *(*data).0;
            let mut obj = ImageView::new((*data).1);
            let ev = Event::new(event as _);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &ev)));
        }

        let cb: Box<dyn FnMut(&mut ImageView, &Event)> = Box::new(handler);

        let data: *mut (
            Box<dyn FnMut(&mut ImageView, &Event)>,
            *mut nappgui_sys::ImageView,
        ) = Box::into_raw(Box::new((cb, self.as_ptr())));

        unsafe {
            imageview_OnOverDraw(
                self.as_ptr(),
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }
}
