use std::ffi::{CStr, CString};

use nappgui_sys::{
    font_ascent, font_copy, font_create, font_descent, font_destroy, font_equals,
    font_exists_family, font_extents, font_family, font_height, font_installed_families,
    font_installed_monospace, font_is_monospace, font_leading, font_mini_size, font_monospace,
    font_regular_size, font_size, font_small_size, font_style, font_system, font_width,
    font_with_style, font_with_width, font_with_xscale, font_xscale,
};

use crate::types::FontStyle;

/// Represents a typographic family, size and style with which the texts will be drawn.
pub struct Font {
    pub(crate) inner: *mut nappgui_sys::Font,
}

impl Font {
    pub(crate) fn new(ptr: *mut nappgui_sys::Font) -> Self {
        if ptr.is_null() {
            panic!("Font is NULL");
        }

        Font { inner: ptr }
    }

    /// Create a font.
    pub fn create(family: &str, size: f32, style: FontStyle) -> Self {
        let family = CString::new(family).unwrap();
        let font = unsafe { font_create(family.as_ptr(), size, style.to_fstyle_t() as _) };
        Font::new(font)
    }

    /// Create a font, with the system's default family.
    pub fn system(size: f32, style: FontStyle) -> Self {
        let font = unsafe { font_system(size, style.to_fstyle_t() as _) };
        Font::new(font)
    }

    /// Create a font, with the system's default monospace family.
    pub fn monospace(size: f32, style: FontStyle) -> Self {
        let font = unsafe { font_monospace(size, style.to_fstyle_t() as _) };
        Font::new(font)
    }

    /// Create a copy of an existing font, changing the style.
    pub fn with_style(&self, style: FontStyle) -> Self {
        let font = unsafe { font_with_style(self.inner, style.to_fstyle_t() as _) };
        Font::new(font)
    }

    /// Creates a copy of an existing font, changing the average width of the character.
    pub fn with_width(&self, width: f32) -> Self {
        let font = unsafe { font_with_width(self.inner, width) };
        Font::new(font)
    }

    /// Creates a copy of an existing font, changing the x-scaling of the text.
    pub fn with_xscale(&self, xscale: f32) -> Self {
        let font = unsafe { font_with_xscale(self.inner, xscale) };
        Font::new(font)
    }

    /// Compare two sources. They are considered equal if they have the same family, size and style.
    pub fn equals(&self, other: &Font) -> bool {
        unsafe { font_equals(self.inner, other.inner) != 0 }
    }

    /// Gets the default font size for interface controls.
    pub fn regular_size() -> f32 {
        unsafe { font_regular_size() }
    }

    /// Gets the small default font size for interface controls.
    pub fn small_size() -> f32 {
        unsafe { font_small_size() }
    }

    /// Gets the mini default font size for interface controls.
    pub fn mini_size() -> f32 {
        unsafe { font_mini_size() }
    }

    /// Gets the font family.
    pub fn family(&self) -> String {
        let family = unsafe { font_family(self.inner) };
        let family = unsafe { std::ffi::CStr::from_ptr(family) };
        family.to_string_lossy().into_owned()
    }

    /// Gets the font size.
    pub fn size(&self) -> f32 {
        unsafe { font_size(self.inner) }
    }

    /// Gets the height of the cell or line of text with this font.
    pub fn height(&self) -> f32 {
        unsafe { font_height(self.inner) }
    }

    /// Gets the average width of the character.
    pub fn width(&self) -> f32 {
        unsafe { font_width(self.inner) }
    }

    /// Gets the x scaling of the text.
    pub fn xscale(&self) -> f32 {
        unsafe { font_xscale(self.inner) }
    }

    /// Obtains the measurement of the font above the baseline.
    pub fn ascent(&self) -> f32 {
        unsafe { font_ascent(self.inner) }
    }

    /// Obtains the measurement of the font below the baseline.
    pub fn descent(&self) -> f32 {
        unsafe { font_descent(self.inner) }
    }

    /// Gets the margin between the character size and the line height.
    pub fn leading(&self) -> f32 {
        unsafe { font_leading(self.inner) }
    }

    /// Checks if a font is monospaced or not.
    pub fn is_monospace(&self) -> bool {
        unsafe { font_is_monospace(self.inner) != 0 }
    }

    /// Gets the style of the font.
    pub fn style(&self) -> u32 {
        unsafe { font_style(self.inner) }
    }

    /// Gets the size in pixels of a text string, based on the font.
    pub fn extents(&self, text: &str, refwidth: f32) -> (f32, f32) {
        let text = CString::new(text).unwrap();
        let mut width = 0f32;
        let mut height = 0f32;

        unsafe {
            font_extents(self.inner, text.as_ptr(), refwidth, &mut width, &mut height);
        };

        (width, height)
    }

    /// Checks if a font family is installed on the operating system.
    pub fn exists_family(family: &str) -> bool {
        let family = CString::new(family).unwrap();
        unsafe { font_exists_family(family.as_ptr()) != 0 }
    }

    /// Gets a list of the names of all font families installed on the operating system
    pub fn installed_families() -> Vec<String> {
        let mut families = Vec::new();
        unsafe {
            let ptr = font_installed_families();
            if ptr.is_null() {
                return families;
            }
            let families_arr = *ptr;
            let content = families_arr.content;
            let elem = (*content).elem;
            for i in 0..families_arr.size {
                let family = *elem[i as usize];
                let family = &family.data[0..family.size as usize];
                let family = CStr::from_ptr(family.as_ptr());
                families.push(family.to_string_lossy().to_string());
            }
        }

        families
    }

    /// Gets a list of the names of all monospaced families installed on the operating system.
    pub fn installed_monospace() -> Vec<String> {
        let mut families = Vec::new();
        unsafe {
            let ptr = font_installed_monospace();
            if ptr.is_null() {
                return families;
            }
            let families_arr = *ptr;
            let content = families_arr.content;
            let elem = (*content).elem;
            for i in 0..families_arr.size {
                let family = *elem[i as usize];
                let family = &family.data[0..family.size as usize];
                let family = CStr::from_ptr(family.as_ptr());
                families.push(family.to_string_lossy().to_string());
            }
        }

        families
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        let font = unsafe { font_copy(self.inner) };
        Font::new(font)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { font_destroy(&mut self.inner) };
    }
}
