#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub(crate) mod util;

/// Just as a building needs a strong foundation, any software project must be supported by robust 
/// and efficient pillars. For this purpose, the core library has been developed, which provides 
/// commonly used non-graphical utilities.
pub mod core;
/// The Draw2D library integrates all the functionality necessary to create two dimensions vector 
/// graphics. It depends directly on Geom2D and, as we will see later, drawing does not 
/// imply having a graphical user interface in the program.
pub mod draw_2d;
/// The Gui library allows you to create graphical user interfaces in a simple and intuitive way. 
/// Only available for desktop applications for obvious reasons, unlike the rest of libraries 
/// that can also be used in command line applications.
pub mod gui;
/// The OSApp library starts and manages the message cycle of a desktop application.
pub mod osapp;

/// Enums and types
pub mod prelude {
    /// State values.
    pub type GuiState = nappgui_sys::gui_state_t;
    /// Orientation.
    pub type GuiOrient = nappgui_sys::gui_orient_t;
    /// Cursors.
    pub type GuiCursor = nappgui_sys::gui_cursor_t;
    /// Result when changing the keyboard focus.
    pub type GuiFocus = nappgui_sys::gui_focus_t;
    /// Window creation attributes.
    pub type WindowFlag = nappgui_sys::window_flag_t;
    /// Alignment values.
    pub type Align = nappgui_sys::align_t;
    /// Style in typographic fonts. Multiple values can be combined with the OR operator ('|').
    pub type FStyle = nappgui_sys::fstyle_t;
    /// Pixel format in an image. Number of bits per pixel and color model.
    pub type PixFormat = nappgui_sys::pixformat_t;
    /// Keyboard codes.
    pub type Vkey = nappgui_sys::vkey_t;
    /// 2D affine transformation.
    pub type T2Df = nappgui_sys::T2Df;
    /// Represents a 2D size.
    pub type S2Df = nappgui_sys::S2Df;
    /// Represents a 2d vector or point.
    pub type V2Df = nappgui_sys::V2Df;
    /// Behavior of the divider in a SplitView.
    pub type SplitMode = nappgui_sys::split_mode_t;
}
