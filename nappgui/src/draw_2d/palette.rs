use nappgui_sys::{
    palette_binary, palette_ccolors, palette_cga2, palette_colors, palette_create, palette_destroy,
    palette_ega4, palette_gray1, palette_gray2, palette_gray4, palette_gray8, palette_rgb8,
    palette_size,
};

use super::Color;

/// A palette is nothing more than an indexed list of colors (Figure 1), usually related to Pixel Buffer.
/// Its main utility is to save space in the images representation, since each pixel is encoded by an index
/// of 1, 2, 4 or 8 bits instead of the real color where 24 or 32 bits are necessary. For this reason, it is
/// usual to have palettes of 2, 4, 16 or 256 colors.
pub struct Palette {
    pub(crate) inner: *mut nappgui_sys::Palette,
}

impl Palette {
    pub(crate) fn new(ptr: *mut nappgui_sys::Palette) -> Self {
        if ptr.is_null() {
            panic!("Palette is null");
        }
        Self { inner: ptr }
    }

    /// Create a palette.
    pub fn create(size: u32) -> Self {
        Self::new(unsafe { palette_create(size) })
    }

    /// Create the 4-color (2-bit) palette of CGA cards.
    pub fn cga2(mode: bool, intense: bool) -> Self {
        Self::new(unsafe { palette_cga2(mode as i8, intense as i8) })
    }

    /// Create the default palette for EGA cards (16 colors, 4 bits).
    pub fn ega4() -> Self {
        Self::new(unsafe { palette_ega4() })
    }

    /// Create the default 8-bit RGB palette. Colors combine 8 tones of red, 8 green and 4 blue.
    pub fn rgb8() -> Self {
        Self::new(unsafe { palette_rgb8() })
    }

    /// Create a palette of 2 tones of gray (1 bit). Black (0) and white (1).
    pub fn gray1() -> Self {
        Self::new(unsafe { palette_gray1() })
    }

    /// Create a palette of 4 tones of gray (2 bit). Black (0), White (3).
    pub fn gray2() -> Self {
        Self::new(unsafe { palette_gray2() })
    }

    /// Create a palette of 16 tones of gray (4 bit). Black (0), White (15).
    pub fn gray4() -> Self {
        Self::new(unsafe { palette_gray4() })
    }

    /// Create a palette of 256 shades of gray (8 bit). Black (0), White (255).
    pub fn gray8() -> Self {
        Self::new(unsafe { palette_gray8() })
    }

    /// Create a two-color palette.
    pub fn binary(zero: Color, one: Color) -> Self {
        Self::new(unsafe { palette_binary(zero.inner, one.inner) })
    }

    /// Destroy the palette.
    pub fn destroy(mut self) {
        unsafe {
            palette_destroy(&mut self.inner);
        }
    }

    /// Returns the number of colors in the palette.
    pub fn size(&self) -> u32 {
        unsafe { palette_size(self.inner) }
    }

    /// Get the color list.
    pub fn colors(&self) -> &mut [Color] {
        let size = self.size();
        let ptr = unsafe { palette_colors(self.inner) };
        unsafe { std::slice::from_raw_parts_mut(ptr as _, size as usize) }
    }

    /// Get the color list.
    pub fn ccolors(&self) -> &[Color] {
        let size = self.size();
        let ptr = unsafe { palette_ccolors(self.inner) };
        unsafe { std::slice::from_raw_parts(ptr as _, size as usize) }
    }
}
