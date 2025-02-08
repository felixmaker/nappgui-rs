use nappgui_sys::{
    color_bgr, color_blue, color_get_alpha, color_get_rgb, color_get_rgba, color_get_rgbaf,
    color_get_rgbf, color_gray, color_green, color_hsbf, color_html, color_red, color_rgb,
    color_rgba, color_rgbaf, color_set_alpha, color_to_hsbf, color_to_html,
};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/// The colors in Draw2D are encoded using a 32-bit integer with the four RGBA channels in Little-Endian: 
/// Red in byte 0, green in 1, blue in 2 and alpha (or transparency) in 3 
pub struct Color {
    pub(crate) inner: nappgui_sys::color_t,
}

impl Color {
    pub(crate) fn new(color: nappgui_sys::color_t) -> Self {
        Self { inner: color }
    }

    /// Create a color from the channels R (red), G (green) y B (blue).
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        let color = unsafe { color_rgb(r, g, b) };
        Self::new(color)
    }

    /// Create a color from the channels R (red), G (green), B (blue) and A (alpha).
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let color = unsafe { color_rgba(r, g, b, a) };
        Self::new(color)
    }

    /// Create a color from the normalized RGBA channels from 0 to 1.
    pub fn rgbaf(r: f32, g: f32, b: f32, a: f32) -> Self {
        let color = unsafe { color_rgbaf(r, g, b, a) };
        Self::new(color)
    }

    /// Creates a color (rgb) from its components Hue-Saturation-Brightness.
    pub fn hsbf(h: f32, s: f32, b: f32) -> Self {
        let color = unsafe { color_hsbf(h, s, b) };
        Self::new(color)
    }

    /// Create an RGB color using only the red channel.
    ///
    /// # Remarks
    /// Equivalent to color_rgb(r, 0, 0).
    pub fn red(r: u8) -> Self {
        let color = unsafe { color_red(r) };
        Self::new(color)
    }

    /// Create an RGB color using only the green channel.
    ///
    /// # Remarks
    /// Equivalent to color_rgb(0, g, 0).
    pub fn green(g: u8) -> Self {
        let color = unsafe { color_green(g) };
        Self::new(color)
    }

    /// Create an RGB color using only the blue channel.
    ///
    /// # Remarks
    /// Equivalent to color_rgb(0, 0, b).
    pub fn blue(b: u8) -> Self {
        let color = unsafe { color_blue(b) };
        Self::new(color)
    }

    /// Creates a gray RGB color from intensity value.
    ///
    /// # Remarks
    /// Equivalent to color_rgb(l, l, l).
    pub fn gray(l: u8) -> Self {
        let color = unsafe { color_gray(l) };
        Self::new(color)
    }

    /// Create a color from a 32-bit BGR value. Byte 0 corresponds to channel B,
    ///  1 to G and 2 to R. The highest order byte is ignored (set to 255).
    ///
    /// # Remarks
    /// This byte order is typical in Web colors.
    pub fn bgr(bgr: u32) -> Self {
        let color = unsafe { color_bgr(bgr) };
        Self::new(color)
    }

    /// Create a color from a string in HTML or CSS format.
    ///
    /// # Remarks
    /// The color transformed to RGB.
    pub fn html(html: &str) -> Self {
        let html = std::ffi::CString::new(html).unwrap();
        let color = unsafe { color_html(html.as_ptr()) };
        Color::new(color)
    }

    /// Convert a color (rgb) to HSB space (hue, saturation, brightness).
    pub fn to_hsbf(&self) -> (f32, f32, f32) {
        let mut h = 0f32;
        let mut s = 0f32;
        let mut b = 0f32;
        unsafe {
            color_to_hsbf(self.inner, &mut h, &mut s, &mut b);
        }
        (h, s, b)
    }

    /// Convert a color to the HTML or CSS format (#RRGGBB).
    pub fn to_html(&self) -> String {
        const MAX_LEN: u32 = 32;
        let mut html: Vec<i8> = Vec::with_capacity(MAX_LEN as usize);
        unsafe {
            color_to_html(self.inner, html.as_mut_ptr(), MAX_LEN);
            std::ffi::CStr::from_ptr(html.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Returns RGB color values.
    ///
    /// # Remarks
    /// In system or indexed colors, it makes effective the RGB value.
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        let mut r = 0u8;
        let mut g = 0u8;
        let mut b = 0u8;
        unsafe { color_get_rgb(self.inner, &mut r, &mut g, &mut b) }
        (r, g, b)
    }

    /// Returns RGB color values, normalized from 0 to 1.
    ///
    /// # Remarks
    /// In system or indexed colors, it makes effective the RGB value.
    pub fn get_rgbf(&self) -> (f32, f32, f32) {
        let mut r = 0f32;
        let mut g = 0f32;
        let mut b = 0f32;
        unsafe {
            color_get_rgbf(self.inner, &mut r, &mut g, &mut b);
        }
        (r, g, b)
    }

    /// Returns the RGBA values of the color.
    ///
    /// # Remarks
    /// In system or indexed colors, it makes effective the RGBA value.
    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        let mut r = 0u8;
        let mut g = 0u8;
        let mut b = 0u8;
        let mut a = 0u8;
        unsafe {
            color_get_rgba(self.inner, &mut r, &mut g, &mut b, &mut a);
        }
        (r, g, b, a)
    }

    /// Returns the RGBA values of the color, normalized from 0 to 1.
    ///
    /// # Remarks
    /// In system or indexed colors, it makes effective the RGBA value.
    pub fn get_rgbaf(&self) -> (f32, f32, f32, f32) {
        let mut r = 0f32;
        let mut g = 0f32;
        let mut b = 0f32;
        let mut a = 0f32;
        unsafe {
            color_get_rgbaf(self.inner, &mut r, &mut g, &mut b, &mut a);
        }
        (r, g, b, a)
    }

    /// Get the alpha (transparency) color component.
    ///
    /// # Remarks
    /// The alpha component. If it is equal 0 it means that the color is indexed
    /// (does not contain RGB values).
    pub fn get_alpha(&self) -> u8 {
        unsafe { color_get_alpha(self.inner) }
    }

    /// Changes the alpha (transparency) value of a color.
    ///
    /// # Remarks
    /// The new color, with the altered alpha component.
    pub fn set_alpha(&mut self, alpha: u8) {
        let color = unsafe { color_set_alpha(self.inner, alpha) };
        self.inner = color
    }
}
