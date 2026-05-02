use std::{
    cell::RefCell,
    ptr::NonNull,
    rc::{Rc, Weak},
};

use crate::{
    draw_2d::Image,
    gui::{global_get, global_record},
    types::Scale,
    util::macros::listener,
};

use nappgui_sys::{
    imageview_OnClick, imageview_OnOverDraw, imageview_create, imageview_get_image, imageview_image, imageview_scale,
    imageview_size,
};

pub(crate) struct ImageViewInner {
    ptr: NonNull<nappgui_sys::ImageView>,
    on_click: RefCell<Option<Rc<dyn Fn() + 'static>>>,
    on_over_draw: RefCell<Option<Rc<dyn Fn() + 'static>>>,
}

impl ImageViewInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::ImageView) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to ImageViewInner::from_raw"),
            on_click: RefCell::new(None),
            on_over_draw: RefCell::new(None),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::ImageView {
        self.ptr.as_ptr()
    }
}

/// The image view control.
///
/// # Remark
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct ImageView(Weak<ImageViewInner>);

impl ImageView {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::ImageView) -> Self {
        let object = global_record(ptr as _, ImageViewInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::ImageView) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::ImageView {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a image view.
    pub fn new(width: f32, height: f32) -> Self {
        let imageview = unsafe { imageview_create() };
        let imageview = unsafe { ImageView::from_raw(imageview) };
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

    /// Set a handle for the event click on the image.
    pub fn set_on_click_handler<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_click.borrow_mut() = Some(Rc::new(callback)));

        let listener = listener!(self.as_ptr(), ImageViewInner, on_click());
        unsafe { imageview_OnClick(self.as_ptr(), listener) };
    }

    /// Allows you to draw an overlay on the image when the mouse is over it.
    pub fn set_on_over_draw_handler<F>(&self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_over_draw.borrow_mut() = Some(Rc::new(callback)));
        let listener = listener!(self.as_ptr(), ImageViewInner, on_over_draw());
        unsafe { imageview_OnOverDraw(self.as_ptr(), listener) };
    }

    /// Gets the image.
    pub fn image(&self) -> Option<Image> {
        let image = unsafe { imageview_get_image(self.as_ptr()) };
        if image.is_null() {
            None
        } else {
            Some(unsafe { Image::from_raw_cloned(image) })
        }
    }
}
