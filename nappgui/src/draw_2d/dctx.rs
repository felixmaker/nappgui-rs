use nappgui_sys::{dctx_bitmap, dctx_image, draw_antialias, draw_clear, draw_matrixf};

use crate::types::{PixFormat, Trans2D};

use super::{Color, ImageBuf};

/// Drawing context.
pub struct DCtx {
    pub(crate) inner: *mut nappgui_sys::DCtx,
}

impl DCtx {
    pub(crate) fn new(ptr: *mut nappgui_sys::DCtx) -> Self {
        if ptr.is_null() {
            panic!("DCtx is NULL");
        }
        Self { inner: ptr }
    }

    /// Create a memory context, in order to generate an image.
    ///
    /// # Remark
    /// When we finish drawing, we must call dctx_image to get the picture.
    pub fn bitmap(width: u32, height: u32, format: PixFormat) -> Self {
        let ptr = unsafe { dctx_bitmap(width, height, format as _) };
        DCtx::new(ptr)
    }

    /// Get the result image after drawing in the context created with dctx_bitmap.
    pub fn image(mut self) -> ImageBuf {
        let ptr = unsafe { dctx_image(&mut self.inner) };
        ImageBuf::from_raw(ptr)
    }

    /// Clears the entire context area, using a solid color.
    pub fn clear(&self, color: Color) {
        unsafe { draw_clear(self.inner, color.inner) }
    }

    /// Set the context reference system (affine transformation).
    ///
    /// # Remark
    /// The origin of coordinates is in the upper left corner. The Y axis increases down.
    pub fn matrix(&self, t2d: &[Trans2D]) {
        unsafe {
            draw_matrixf(self.inner, t2d.as_ptr() as _);
        }
    }

    /// Set the reference system in Cartesian coordinates.
    pub fn matrix_cartesian(&self, t2d: &[Trans2D]) {
        unsafe {
            draw_matrixf(self.inner, t2d.as_ptr() as _);
        }
    }

    /// Enable or disable antialiasing.
    ///
    /// # Remark
    /// The antialias can change in each primitive. It is not necessary to establish a policy
    /// for the whole drawing. See Antialiasing.
    pub fn antialias(&self, enable: bool) {
        unsafe {
            draw_antialias(self.inner, enable as _);
        }
    }
}
