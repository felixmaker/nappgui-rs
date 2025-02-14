use std::rc::Rc;

use nappgui_sys::{
    dctx_bitmap, dctx_image, draw_antialias, draw_clear, draw_matrixf, pixformat_t, T2Df,
};

use crate::util::macros::impl_ptr;

use super::{Color, Image};

/// Drawing context.
pub struct DCtx {
    pub(crate) inner: Rc<*mut nappgui_sys::DCtx>,
}

impl DCtx {
    impl_ptr!(nappgui_sys::DCtx);

    /// Create a memory context, in order to generate an image.
    ///
    /// # Remark
    /// When we finish drawing, we must call dctx_image to get the picture.
    pub fn bitmap(width: u32, height: u32, format: pixformat_t) -> Self {
        let ptr = unsafe { dctx_bitmap(width, height, format) };
        DCtx::new(ptr)
    }

    /// Get the result image after drawing in the context created with dctx_bitmap.
    pub fn image(self) -> Image {
        let ptr = unsafe { dctx_image(&mut self.as_ptr()) };
        Image::new(ptr)
    }

    /// Clears the entire context area, using a solid color.
    pub fn clear(&self, color: &Color) {
        unsafe { draw_clear(self.as_ptr(), color.inner) }
    }

    /// Set the context reference system (affine transformation).
    ///
    /// # Remark
    /// The origin of coordinates is in the upper left corner. The Y axis increases down.
    pub fn matrix(&self, t2d: &[T2Df]) {
        unsafe {
            draw_matrixf(self.as_ptr(), t2d.as_ptr());
        }
    }

    /// Set the reference system in Cartesian coordinates.
    pub fn matrix_cartesian(&self, t2d: &[T2Df]) {
        unsafe {
            draw_matrixf(self.as_ptr(), t2d.as_ptr());
        }
    }

    /// Enable or disable antialiasing.
    ///
    /// # Remark
    /// The antialias can change in each primitive. It is not necessary to establish a policy
    /// for the whole drawing. See Antialiasing.
    pub fn antialias(&self, enable: bool) {
        unsafe {
            draw_antialias(self.as_ptr(), enable as i8);
        }
    }
}
