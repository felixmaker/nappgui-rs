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

/// Nappgui Error types
pub mod error;
/// Wrapper inner types
pub mod types;

/// Enums and types
pub mod prelude {
    pub use crate::core::*;
    pub use crate::draw_2d::*;
    pub use crate::error::*;
    pub use crate::gui::*;
    pub use crate::types::*;
}

/// Embed resources
pub use nappgui_macros::include_resource;
