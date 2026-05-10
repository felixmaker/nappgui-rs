use nappgui_sys::{line_horizontal, line_length, line_vertical};

use crate::gui::define_object;

#[derive(Default)]
pub(crate) struct LineProps {}

define_object!(Line, LineInner, Line, LineProps);

impl Line {
    /// Create a horizontal separator.
    pub fn new_horizontal() -> Self {
        unsafe { Line::from_raw(line_horizontal()) }
    }

    /// Create a vertical separator.
    pub fn new_vertical() -> Self {
        unsafe { Line::from_raw(line_vertical()) }
    }

    /// Sets the natural length of the line. By default 100px.
    pub fn set_length(&self, length: f32) {
        unsafe {
            line_length(self.as_ptr(), length);
        }
    }
}
