macro_rules! impl_i32_to_enum {
    ($type: ty, $range: expr) => {
        impl TryFrom<i32> for $type {
            type Error = crate::error::NappguiError;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if !($range).contains(&value) {
                    return Err(crate::error::NappguiError::Internal(
                        crate::error::NappguiErrorKind::UndefinedEnumTransmute,
                    ));
                } else {
                    Ok(unsafe { std::mem::transmute(value) })
                }
            }
        }
    };
}

/// Alignment values.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Align {
    /// Alignment to the left/upper margin.
    Left = 1,
    /// Centered alignment.
    Center = 2,
    /// Alignment to the right/lower margin.
    Right = 3,
    /// Justification or expansion of content.
    Justify = 4,
}

impl_i32_to_enum!(Align, 1..=4);

/// State values.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiState {
    /// Off
    Off = 0,
    /// On
    On = 1,
    /// Medium/undetermined.
    Mixed = 2,
}

impl_i32_to_enum!(GuiState, 0..=2);

/// Font style.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FontStyle {
    /// Normal font, no style. Also called Regular, indicated in pixels.
    Normal = 0,
    /// Bold font.
    Bold = 1,
    /// Italic font.
    Italic = 2,
    /// Crossed out font.
    StrikeOut = 4,
    /// Underlined font.
    Underlined = 8,
    /// Subscript.
    Subscript = 16,
    /// Superscript.
    Superscript = 32,
    /// Font sizes will be indicated in pixels.
    Points = 64,
    /// Font sizes will refer to cell height and not character height.
    Cell = 128,
}

/// Trait for converting a type into a font style.
pub trait IntoFontStyle {
    /// Convert into a font style.
    fn into_font_style(self) -> u32;
}

impl IntoFontStyle for FontStyle {
    fn into_font_style(self) -> u32 {
        self as u32
    }
}

impl<T> IntoFontStyle for T
where
    T: IntoIterator<Item = FontStyle>,
{
    fn into_font_style(self) -> u32 {
        let mut result = 0u32;
        let mut seen = HashSet::new();

        for font in self {
            if seen.insert(font) {
                result += font as u32;
            }
        }
        result
    }
}

#[cfg(test)]
mod font_style_test {
    use super::*;

    #[test]
    fn test_font_style() {
        let font_style = FontStyle::Bold;
        let font_style = font_style.into_font_style();
        assert_eq!(font_style, 1);
    }

    #[test]
    fn test_font_style_iter() {
        let font_style = vec![FontStyle::Bold, FontStyle::Italic];
        let font_style = font_style.into_font_style();
        assert_eq!(font_style, 3);
    }

    #[test]
    fn test_font_style_iter2() {
        let font_style = vec![FontStyle::Bold, FontStyle::Bold, FontStyle::Italic, FontStyle::Points];
        let font_style = font_style.into_font_style();
        assert_eq!(font_style, 67);
    }

    #[test]
    fn test_font_style_iter3() {
        let font_style = vec![];
        let font_style = font_style.into_font_style();
        assert_eq!(font_style, 0);
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

impl_i32_to_enum!(GuiOrient, 1..=2);

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
    /// Pressing the \[TAB\] key.
    Key = 1,
    /// Pressing \[SHIFT\]+\[TAB\].
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
    /// Control that has received the focus. TODO!
    pub fn next(&self) -> Option<Control> {
        if self.next.is_null() {
            None
        } else {
            Some(Control::from_raw(self.next))
        }
    }
}

/// The window flag.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WindowFlag {
    /// The window draws an outer border.
    Edge = 1,
    /// The window has a title bar.
    Title = 2,
    /// The window shows the maximize button.
    Maximize = 4,
    /// The window shows the minimize button.
    Minimize = 8,
    /// The window shows the close button.
    Close = 16,
    /// The window has resizable borders.
    Resizable = 32,
    /// The window will process the pressing of the \[RETURN\] key as a possible closing event, sending the message OnClose.
    Return = 64,
    /// The window will process the pressing of the \[ESC\] key as a possible closing event, sending the message OnClose.
    Escape = 128,
    /// Avoids hiding a modal window when the modal cycle has finished. See Modal windows.
    OffScreen = 256,
    /// The window is a modal window.
    ModalNoHide = 512,
}

/// Trait for converting a type into a window flag.
pub trait IntoWindowFlag {
    /// Convert into a window flag.
    fn into_window_flag(self) -> u32;
}

impl IntoWindowFlag for WindowFlag {
    fn into_window_flag(self) -> u32 {
        self as u32
    }
}

impl<T> IntoWindowFlag for T
where
    T: IntoIterator<Item = WindowFlag>,
{
    fn into_window_flag(self) -> u32 {
        let mut result = 0u32;
        let mut seen = HashSet::new();

        for font in self {
            if seen.insert(font) {
                result += font as u32;
            }
        }
        result
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

impl_i32_to_enum!(KeyCode, 0..=111);

/// Modifier keys.
#[allow(missing_docs)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ModifierKey {
    None = 0,
    Shift = 1,
    Control = 2,
    Alt = 4,
    Command = 8,
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

/// Reason for closing a window.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiClose {
    /// The \[ESC\] key has been pressed (cancel).
    Cancel,
    /// The \[ENTER\] key has been pressed (accept).
    Accept,
    /// The close button \[X\] has been pressed in the title bar.
    Close,
    /// The main window has been clicked (only received by overlay windows).
    Deactivate,
    /// Custem close reason,
    Custom(u32),
}

impl From<u32> for GuiClose {
    fn from(value: u32) -> Self {
        match value {
            1 => GuiClose::Cancel,
            2 => GuiClose::Accept,
            3 => GuiClose::Close,
            4 => GuiClose::Deactivate,
            v => GuiClose::Custom(v),
        }
    }
}

impl From<GuiClose> for u32 {
    fn from(value: GuiClose) -> Self {
        match value {
            GuiClose::Cancel => 1,
            GuiClose::Accept => 2,
            GuiClose::Close => 3,
            GuiClose::Deactivate => 4,
            GuiClose::Custom(v) => v,
        }
    }
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
pub struct Trans2D {
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

/// Pixel format in an image. Number of bits per pixel and color model.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixFormat {
    /// 1 bit per pixel. 2 colors, indexed.
    Index1 = 1,
    /// 2 bits per pixel. 4 colors, indexed.
    Index2 = 2,
    /// 4 bits per pixel. 16 colors, indexed.
    Index4 = 3,
    /// 8 bits per pixel. 256 colors, indexed.
    Index8 = 4,
    /// 8 bits per pixel in grayscale. 256 shades of gray.
    Gray8 = 5,
    /// 24 bits per RGB pixel. 8 bits per channel (red, green, blue). The lowest order byte corresponds to the red one and the highest one to the blue one.
    RGB24 = 6,
    /// 32 bits per pixel RGBA. 8 bits per channel (red, green, blue, alpha). The lowest order byte corresponds to the red one and the highest one to alpha (transparency).
    RGBA32 = 7,
    /// Represents the original format of the image. Only valid at image_pixels.
    Image = 8,
}

impl_i32_to_enum!(PixFormat, 1..=8);

/// Mouse buttons.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiMouse {
    /// Left.
    Left = 1,
    /// Right.
    Right = 2,
    /// Center.
    Middle = 3,
}

impl_i32_to_enum!(GuiMouse, 1..=3);

/// Types of scroll.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuiScroll {
    /// Jump to start.
    Begin = 1,
    /// Skip to the end.
    End = 2,
    /// Jump one step (or line) to the left or up.
    StepLeft = 3,
    /// Jump one step (or line) to the right or down.
    StepRight = 4,
    /// Jump a page to the left or up.
    PageLeft = 5,
    /// Jump a page to the right or down.
    PageRight = 6,
    /// Jump to the thumb position.
    Thumb = 7,
}

impl_i32_to_enum!(GuiScroll, 1..=7);

/// Event type.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EventType {
    // /// Redirection of Asserts.
    // CoreAssert = 256,
    // /// A file detected while browsing a directory. hfile_dir_loop.
    // CoreFile = 257,
    // /// Entry in a sub-directory while we go through a directory. hfile_dir_loop.
    // CoreEntry = 258,
    // /// Exit of a sub-directory.
    // CoreExit = 259,
    /// Click on a Label control.
    Label = 1024,
    /// Click on a Button control.
    Button = 1025,
    /// The selection of a PopUp control has been changed.
    PopUp = 1026,
    /// The selection of a control has been changed ListBox.
    ListBox = 1027,
    /// You are moving an Slider control.
    Slider = 1028,
    /// Click on a UpDown control.
    UpDown = 1029,
    /// The text of a Edit or Combo control is being edited.
    TextFilter = 1030,
    /// You have finished editing the text of a Edit or Combo control.
    TextChange = 1031,
    /// Ask a control if it wants to give up keyboard focus.
    FocusResign = 1032,
    /// Ask a control if it wants to accept keyboard focus.
    FocusAccept = 1033,
    /// A control has received or lost keyboard focus.
    Focus = 1034,
    /// Click on a menu.
    Menu = 1035,
    /// The view content must be redrawn.
    Draw = 1036,
    /// There is draw the overlay layer.
    Overlay = 1037,
    /// The size of a view has changed.
    Resize = 1038,
    /// The mouse has entered the view area.
    Enter = 1039,
    /// The mouse has left the view area.
    Exit = 1040,
    /// The mouse is moving on the view surface.
    Move = 1041,
    /// A mouse button was pressed.
    Down = 1042,
    /// A mouse button has been released.
    Up = 1043,
    /// Click on a view.
    Click = 1044,
    /// Dragging is being done over.
    Drag = 1045,
    /// Mouse wheel has moved.
    Wheel = 1046,
    /// A key has been pressed.
    KeyDown = 1047,
    /// A key has been released.
    KeyUp = 1048,
    /// The scroll bars are being manipulated.
    Scroll = 1049,
    /// The window is moving across the desktop.
    WindowMoved = 1050,
    /// The window is being resized.
    WindowSizing = 1051,
    /// The window has been resized.
    WindowSize = 1052,
    /// The window has been closed.
    WindowClose = 1053,
    /// An update color of comwin_color.
    Color = 1054,
    /// Desktop theme has changed.
    Theme = 1055,
    /// An object linked to a layout has been edited. Notifications and calculated fields.
    ObjectChange = 1056,
    /// A table needs to know the number of rows.
    TableNRows = 1057,
    /// A table will begin to draw the visible part of the data.
    TableBegin = 1058,
    /// A table has finished drawing.
    TableEnd = 1059,
    /// A table needs the data of a cell.
    TableCell = 1060,
    /// The selected rows in a table have changed.
    TableSelected = 1061,
    /// Click on a table header.
    TableHeadClick = 1062,
    /// Click on a table row.
    TableRowClick = 1063,
}

impl_i32_to_enum!(EventType, 1024..=1063);

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The type of a control.
pub enum GuiType {
    /// Label.
    Label = 0,
    /// Button.
    Button = 1,
    /// Popup.
    Popup = 2,
    /// EditBox.
    EditBox = 3,
    /// ComboBox.
    ComboBox = 4,
    /// Slider.
    Slider = 5,
    /// UpDown.
    UpDown = 6,
    /// Progress.
    Progress = 7,

    /// TextView.
    TextView = 8,
    /// WebView.
    WebView = 9,
    /// TreeView.
    TreeView = 10,
    /// BoxView.
    BoxView = 11,
    /// SplitView.
    SplitView = 12,
    /// CustomView.
    CustomView = 13,

    /// Panel.
    Panel = 14,
    /// Line.
    Line = 15,
    /// Header.
    Header = 16,

    /// Window.
    Window = 17,
    /// Toolbar.
    Toolbar = 18,
}

impl_i32_to_enum!(GuiType, 0..=18);

use std::collections::HashSet;

pub use crate::gui::event::*;
use crate::gui::Control;

/// The button style.
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum ButtonStyle {
    Push = 0,
    Check2 = 1,
    Check3 = 2,
    Radio = 3,
    Flat = 4,
    Flatgle = 5,
}

/// The ellipsis(trimming) style.
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Ellipsis {
    None = 1,
    Begin,
    Middle,
    End,
    Mline,
}
