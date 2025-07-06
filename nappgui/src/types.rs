use crate::{gui::GuiControl, util::macros::impl_i32_to_enum};

/// Alignment values.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Align {
    /// Alignment to the left margin.
    Left = 1,
    /// Alignment to the upper margin.
    Top = 2,
    /// Centered alignment.
    Center = 3,
    /// Alignment to the right margin.
    Right = 4,
    /// Alignment to the lower margin.
    Bottom = 5,
    /// Justification or expansion of content.
    Justify = 6,
}

/// State values.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiState {
    /// On
    On = 1,
    /// Off
    Off = 2,
    /// Medium/undetermined.
    Mixed = 3,
}

impl_i32_to_enum!(GuiState, 1..=3);

/// Style in typographic fonts.
pub struct FontStyle {
    /// Normal font, no style. Also called Regular.
    pub is_normal: bool,
    /// Bold font.
    pub is_bold: bool,
    /// Italic font.
    pub is_italic: bool,
    /// Crossed out font.
    pub is_strike_out: bool,
    /// Underlined font.
    pub is_underlined: bool,
    /// Subscript.
    pub is_subscript: bool,
    /// Superscript.
    pub is_superscript: bool,
    /// Font sizes will be indicated in pixels.
    pub is_pixels: bool,
    /// Font sizes will be indicated in points.
    pub is_points: bool,
    /// Font sizes will refer to cell height and not character height.
    pub is_cell: bool,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self {
            is_normal: true,
            is_bold: false,
            is_italic: false,
            is_strike_out: false,
            is_underlined: false,
            is_subscript: false,
            is_superscript: false,
            is_pixels: false,
            is_points: false,
            is_cell: false,
        }
    }
}

impl FontStyle {
    pub(crate) fn to_fstyle_t(&self) -> i32 {
        let mut style = nappgui_sys::_fstyle_t_ekFNORMAL;

        if self.is_bold {
            style |= nappgui_sys::_fstyle_t_ekFBOLD;
        }
        if self.is_italic {
            style |= nappgui_sys::_fstyle_t_ekFITALIC;
        }
        if self.is_strike_out {
            style |= nappgui_sys::_fstyle_t_ekFSTRIKEOUT;
        }
        if self.is_underlined {
            style |= nappgui_sys::_fstyle_t_ekFUNDERLINE;
        }
        if self.is_subscript {
            style |= nappgui_sys::_fstyle_t_ekFSUBSCRIPT;
        }
        if self.is_superscript {
            style |= nappgui_sys::_fstyle_t_ekFSUPSCRIPT;
        }
        if self.is_pixels {
            style |= nappgui_sys::_fstyle_t_ekFPIXELS;
        }
        if self.is_points {
            style |= nappgui_sys::_fstyle_t_ekFPOINTS;
        }
        if self.is_cell {
            style |= nappgui_sys::_fstyle_t_ekFCELL;
        }
        style
    }
}

/// Ways of scaling an image.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Scale {
    /// The image will be displayed centered in the control frame, without scaling. It may not be fully displayed.
    None = 1,
    /// The image will be scaled to fit the control frame. Its aspect ratio may change.
    Auto = 2,
    /// The image will be scaled to fit the control frame, but without changing its aspect ratio. If one of the dimensions (width/height) is smaller than the size of the control, it will be centered.
    Aspect = 3,
    /// Same as above, but does not increase the original size, only reduces it if necessary. If any of the dimensions (width/height) is less than the size of the control, it will be centered.
    AspectDW = 4,
    /// The size of the control (e.g. ImageView) will be adjusted by default to the size of the image.
    Adjust = 5,
}

/// Orientation.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiOrient {
    /// Horizontal.
    Horizontal = 1,
    /// Vertical.
    Vertical = 2,
}

/// Behavior of the divider in a SplitView.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SplitMode {
    /// The divider maintains its left/right or up/down proportional position.
    Normal = 1,
    /// The left/upper child maintains its size.
    Fixed0 = 2,
    /// The right/lower child maintains its size.
    Fixed1 = 3,
}

/// Action that has motivated the change of keyboard focus.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiTab {
    /// Pressing the [TAB] key.
    Key = 1,
    /// Pressing [SHIFT]+[TAB].
    BackKey = 2,
    /// Call to window_next_tabstop.
    Next = 3,
    /// Call to window_previous_tabstop.
    Prev = 4,
    /// Call to window_focus.
    Move = 5,
    /// Click on another control.
    Click = 6,
}

impl_i32_to_enum!(GuiTab, 1..=6);

/// Information about changing the keyboard focus.
pub struct FocusInfo {
    /// Action that has motivated the change of keyboard focus.
    pub action: GuiTab,
    pub(crate) next: *mut nappgui_sys::GuiControl,
}

impl FocusInfo {
    /// Control that has received the focus.
    pub fn next<T>(&self) -> Option<T>
    where
        T: GuiControl,
    {
        T::from_control_ptr(self.next)
    }
}

/// Window Flag
pub struct WindowFlags {
    /// The window draws an outer border.
    pub has_outer_border: bool,
    /// The window has a title bar.
    pub has_title_bar: bool,
    /// The window shows the maximize button.
    pub has_maximize_button: bool,
    /// The window shows the minimize button.
    pub has_minimize_button: bool,
    /// The window shows the close button.
    pub has_close_button: bool,
    /// The window has resizable borders.
    pub has_resizable_borders: bool,
    /// The window will process the pressing of the [RETURN] key as a possible closing event, sending the message OnClose.
    pub process_return_key: bool,
    /// The window will process the pressing of the [ESC] key as a possible closing event, sending the message OnClose.
    pub process_escape_key: bool,
    /// Avoids hiding a modal window when the modal cycle has finished. See Modal windows.
    pub avoid_hiding_modal: bool,
}

impl Default for WindowFlags {
    fn default() -> Self {
        Self {
            has_outer_border: false,
            has_title_bar: true,
            has_maximize_button: false,
            has_minimize_button: true,
            has_close_button: true,
            has_resizable_borders: true,
            process_return_key: false,
            process_escape_key: false,
            avoid_hiding_modal: false,
        }
    }
}

/// Keyboard codes.
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyCode {
    Undefined = 0,
    KeyA = 1,
    KeyS = 2,
    KeyD = 3,
    KeyF = 4,
    KeyH = 5,
    KeyG = 6,
    KeyZ = 7,
    KeyX = 8,
    KeyC = 9,
    KeyV = 10,
    Backslash = 11,
    KeyB = 12,
    KeyQ = 13,
    KeyW = 14,
    KeyE = 15,
    KeyR = 16,
    KeyY = 17,
    KeyT = 18,
    Digit1 = 19,
    Digit2 = 20,
    Digit3 = 21,
    Digit4 = 22,
    Digit6 = 23,
    Digit5 = 24,
    Digit9 = 25,
    Digit7 = 26,
    Digit8 = 27,
    Digit0 = 28,
    RCURLY = 29,
    KeyO = 30,
    KeyU = 31,
    LeftCurlyBrace = 32,
    KeyI = 33,
    KeyP = 34,
    Enter = 35,
    KeyL = 36,
    KeyJ = 37,
    Semicolon = 38,
    KeyK = 39,
    QuestionMark = 40,
    Comma = 41,
    Minus = 42,
    KeyN = 43,
    KeyM = 44,
    Period = 45,
    Tab = 46,
    Space = 47,
    GreaterThanLessThan = 48,
    Back = 49,
    Escape = 50,
    F17 = 51,
    NumpadDecimal = 52,
    NumpadMultiply = 53,
    NumpadAdd = 54,
    NumLock = 55,
    NumpadDivide = 56,
    NumpadEnter = 57,
    NumpadMinus = 58,
    F18 = 59,
    F19 = 60,
    NumpadEqual = 61,
    Numpad0 = 62,
    Numpad1 = 63,
    Numpad2 = 64,
    Numpad3 = 65,
    Numpad4 = 66,
    Numpad5 = 67,
    Numpad6 = 68,
    Numpad7 = 69,
    Numpad8 = 70,
    Numpad9 = 71,
    F5 = 72,
    F6 = 73,
    F7 = 74,
    F3 = 75,
    F8 = 76,
    F9 = 77,
    F11 = 78,
    F13 = 79,
    F16 = 80,
    F14 = 81,
    F10 = 82,
    F12 = 83,
    F15 = 84,
    PageUp = 85,
    Home = 86,
    Delete = 87,
    F4 = 88,
    PageDown = 89,
    F2 = 90,
    End = 91,
    F1 = 92,
    Left = 93,
    Right = 94,
    Down = 95,
    Up = 96,
    ShiftLeft = 97,
    ShiftRight = 98,
    ControlLeft = 99,
    ControlRight = 100,
    AltLeft = 101,
    AltRight = 102,
    Insert = 103,
    Exclamation = 104,
    Menu = 105,
    SuperLeft = 106,
    SuperRight = 107,
    CapsLock = 108,
    Tilde = 109,
    Backquote = 110,
    Add = 111,
}

impl_i32_to_enum!(KeyCode, 1..=111);

/// Modifier keys.
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Modifiers {
    None = 1,
    Shift = 2,
    Control = 3,
    Alt = 4,
    Command = 5,
}

/// Result when changing the keyboard focus.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiFocus {
    /// Focus has changed to the specified control.
    Changed = 1,
    /// Focus has not moved, it remains on the same control.
    Keep = 2,
    /// Target control not found, hidden, or disabled.
    NoNext = 3,
    /// The current control does not allow changing focus.
    NoResign = 4,
    /// The new control does not accept focus.
    NoAccept = 5,
}

impl_i32_to_enum!(GuiFocus, 1..=5);

impl WindowFlags {
    pub(crate) fn to_window_flag_t(&self) -> u32 {
        let mut result = 0;

        if self.has_outer_border {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_EDGE;
        }

        if self.has_title_bar {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_TITLE;
        }

        if self.has_maximize_button {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_MAX;
        }

        if self.has_minimize_button {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_MIN;
        }

        if self.has_close_button {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_CLOSE;
        }

        if self.has_resizable_borders {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_RESIZE;
        }

        if self.process_return_key {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_RETURN;
        }

        if self.process_escape_key {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_ESC;
        }

        if self.avoid_hiding_modal {
            result |= nappgui_sys::_window_flag_t_ekWINDOW_MODAL_NOHIDE;
        }

        result as u32
    }
}

/// Reason for closing a window.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiClose {
    /// The [ESC] key has been pressed (cancel).
    Cancel = 1,
    /// The [ENTER] key has been pressed (accept).
    Accept = 2,
    /// The close button [X] has been pressed in the title bar.
    Close = 3,
    /// The main window has been clicked (only received by overlay windows).
    Deactivate = 4,
}

/// Cursors.
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiCursor {
    /// Arrow (default).
    Arrow = 1,
    /// Hand.
    Hand = 2,
    /// Vertical bar (text editing).
    VerticalBar = 3,
    /// Cross.
    Cross = 4,
    /// Horizontal resize (left-right).
    HorizontalResize = 5,
    /// Vertical resize (top-bottom).
    VerticalResize = 6,
    /// Created from an image.
    User = 7,
}

/// Represents a 2d vector or point.
#[repr(C)]
pub struct Vector2D {
    /// Coordinate x.
    pub x: f32,
    /// Coordinate y.
    pub y: f32,
}

/// 2d affine transformation.
#[repr(C)]
pub struct Transform2D {
    /// Component i of the linear transformation.
    pub i: Vector2D,
    /// Component j of the linear transformation.
    pub j: Vector2D,
    /// Position.
    pub position: Vector2D,
}

/// Represents a 2d size.
#[repr(C)]
pub struct Size2D {
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

/// 2d rectangle.
#[repr(C)]
pub struct Rect2D {
    /// Position.
    pub position: Vector2D,
    /// Size.
    pub size: Size2D,
}
