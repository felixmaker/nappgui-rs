#![doc = include_str!("../README.md")]

pub(crate) mod util;

pub mod core;
pub mod draw_2d;
pub mod gui;
pub mod osapp;

pub mod prelude {
    pub type GuiState = nappgui_sys::gui_state_t;
    pub type GuiOrient = nappgui_sys::gui_orient_t;
    pub type GuiCursor = nappgui_sys::gui_cursor_t;
    pub type GuiFocus = nappgui_sys::gui_focus_t;

    pub type WindowFlag = nappgui_sys::window_flag_t;

    pub type Align = nappgui_sys::align_t;
    pub type FStyle = nappgui_sys::fstyle_t;
    pub type PixFormat = nappgui_sys::pixformat_t;
    pub type Vkey = nappgui_sys::vkey_t;

    pub type T2Df = nappgui_sys::T2Df;
    pub type S2Df = nappgui_sys::S2Df;
    pub type V2Df = nappgui_sys::V2Df;
}
