pub(crate) mod color;
pub(crate) mod dctx;
pub(crate) mod font;
pub(crate) mod image;
pub(crate) mod palette;
pub(crate) mod pixbuf;

pub use {color::Color, dctx::DCtx, font::Font, image::*, palette::Palette, pixbuf::PixBuf};

/// Represents a 2d vector or point.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point2D {
    /// Coordinate x.
    pub x: f32,
    /// Coordinate y.
    pub y: f32,
}

/// 2d affine transformation.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Transformation2D {
    /// Component i of the linear transformation.
    pub i: Point2D,
    /// Component j of the linear transformation.
    pub j: Point2D,
    /// Position.
    pub position: Point2D,
}

/// Represents a 2d size.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Size2D {
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

/// 2d rectangle.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect2D {
    /// Position.
    pub position: Point2D,
    /// Size.
    pub size: Size2D,
}
