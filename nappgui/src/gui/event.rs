use std::ffi::{CStr, CString};

use nappgui_sys::{
    EvButton, EvDraw, EvKey, EvMenu, EvMouse, EvPos, EvScroll, EvSize, EvSlider, EvTbCell, EvTbPos, EvTbRect, EvTbRow,
    EvTbSel, EvText, EvTextFilter, EvWheel, EvWinClose,
};

use crate::{
    core::{event::NappGUIEventResult, NappGUIEventParams},
    draw_2d::{DCtx, Image},
    types::{Align, GuiClose, GuiMouse, GuiOrient, GuiScroll, GuiState, KeyCode},
};

/// Parameters of the OnClick event of a button or OnSelect of a popup.
pub struct ButtonEvent {
    /// Button or item index.
    pub index: u32,
    /// State.
    pub state: GuiState,
    /// Text.
    pub text: String,
}

impl NappGUIEventParams for ButtonEvent {
    type CType = EvButton;
    const TYPE: &'static CStr = c"EvButton";

    fn from(event: &Self::CType) -> ButtonEvent {
        ButtonEvent {
            index: event.index as _,
            state: GuiState::try_from(event.state).unwrap(),
            text: unsafe { CStr::from_ptr(event.text) }.to_string_lossy().into_owned(),
        }
    }
}

/// Parameters of the OnMoved event of a slider.
pub struct SliderEvent {
    /// Normalized slider position (0, 1).
    pub position: f32,
    /// Increase with respect to the previous position.
    pub increase: f32,
    /// Interval index (only for discrete ranges).
    pub step: u32,
}

impl NappGUIEventParams for SliderEvent {
    type CType = EvSlider;
    const TYPE: &'static CStr = c"EvSlider";

    fn from(event: &Self::CType) -> SliderEvent {
        SliderEvent {
            position: event.pos,
            increase: event.incr,
            step: event.step as _,
        }
    }
}

/// Parameters of the OnChange event of the text boxes.
pub struct TextEvent {
    /// Text.
    pub text: String,
    /// Cursor position (caret).
    pub current_position: u32,
    /// Number of characters inserted or deleted.
    pub length: i32,
}

impl NappGUIEventParams for TextEvent {
    type CType = EvText;
    const TYPE: &'static CStr = c"EvText";

    fn from(event: &Self::CType) -> TextEvent {
        TextEvent {
            text: unsafe { CStr::from_ptr(event.text) }.to_string_lossy().into_owned(),
            current_position: event.cpos as _,
            length: event.len,
        }
    }
}

/// OnDraw event parameters.
pub struct DrawEvent {
    /// 2D drawing context.
    pub context: DCtx,
    /// X coordinate of the drawing area (viewport).
    pub x: f32,
    /// Y coordinate of the drawing area.
    pub y: f32,
    /// Width of the drawing area.
    pub width: f32,
    /// Height of the drawing area.
    pub height: f32,
}

impl NappGUIEventParams for DrawEvent {
    type CType = EvDraw;
    const TYPE: &'static CStr = c"EvDraw";

    fn from(event: &Self::CType) -> DrawEvent {
        DrawEvent {
            context: DCtx::new(event.ctx),
            x: event.x,
            y: event.y,
            width: event.width,
            height: event.height,
        }
    }
}

/// Mouse event parameters.
pub struct MouseEvent {
    /// X coordinate of the pointer in the drawing area.
    pub x: f32,
    /// Y coordinate of the pointer in the drawing area.
    pub y: f32,
    /// X coordinate of the pointer on the control. Same as x if there are no scroll bars.
    pub lx: f32,
    /// Y coordinate of the pointer on the control. Same as y if there are no scroll bars.
    pub ly: f32,
    /// Active button.
    pub button: GuiMouse,
    /// Number of clicks.
    pub count: u32,
    /// Combination of values mkey_t. todo!
    pub modifiers: u32,
    /// Additional value for controls.
    pub tag: u32,
}

impl NappGUIEventParams for MouseEvent {
    type CType = EvMouse;
    const TYPE: &'static CStr = c"EvMouse";

    fn from(event: &Self::CType) -> MouseEvent {
        MouseEvent {
            x: event.x,
            y: event.y,
            lx: event.lx,
            ly: event.ly,
            button: GuiMouse::try_from(event.button).unwrap(),
            count: event.count,
            modifiers: event.modifiers,
            tag: event.tag,
        }
    }
}

/// OnWheel event parameters.
pub struct WheelEvent {
    /// Pointer x coordinate.
    pub x: f32,
    /// Pointer y coordinate.
    pub y: f32,
    /// Increase in x of the wheel or trackpad.
    pub dx: f32,
    /// Increase in y of the wheel or trackpad.
    pub dy: f32,
    /// Increase in z of the wheel or trackpad.
    pub dz: f32,
}

impl NappGUIEventParams for WheelEvent {
    type CType = EvWheel;
    const TYPE: &'static CStr = c"EvWheel";

    fn from(event: &Self::CType) -> WheelEvent {
        WheelEvent {
            x: event.x,
            y: event.y,
            dx: event.dx,
            dy: event.dy,
            dz: event.dz,
        }
    }
}

/// Keyboard event parameters.
pub struct KeyEvent {
    /// Referenced key.
    pub key: KeyCode,
    /// Combination of values mkey_t. todo!
    pub modifiers: u32,
}

impl NappGUIEventParams for KeyEvent {
    type CType = EvKey;
    const TYPE: &'static CStr = c"EvKey";

    fn from(event: &Self::CType) -> KeyEvent {
        KeyEvent {
            key: KeyCode::try_from(event.key).unwrap(),
            modifiers: event.modifiers,
        }
    }
}

/// Parameters of change of position events.
pub struct PositionEvent {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

impl NappGUIEventParams for PositionEvent {
    type CType = EvPos;
    const TYPE: &'static CStr = c"EvPos";

    fn from(event: &Self::CType) -> PositionEvent {
        PositionEvent { x: event.x, y: event.y }
    }
}

/// Resize event parameters.
pub struct SizeEvent {
    /// Width (size in x).
    pub width: f32,
    /// Height (size in y).
    pub height: f32,
}

impl NappGUIEventParams for SizeEvent {
    type CType = EvSize;
    const TYPE: &'static CStr = c"EvSize";

    fn from(event: &Self::CType) -> SizeEvent {
        SizeEvent {
            width: event.width,
            height: event.height,
        }
    }
}

/// Window closing Event Parameters.
pub struct WindowCloseEvent {
    /// Origin of the close.
    pub origin: GuiClose,
}

impl NappGUIEventParams for WindowCloseEvent {
    type CType = EvWinClose;
    const TYPE: &'static CStr = c"EvWinClose";

    fn from(event: &Self::CType) -> WindowCloseEvent {
        WindowCloseEvent {
            origin: GuiClose::try_from(event.origin as u32).unwrap(),
        }
    }
}

/// Menu event parameters.
pub struct MenuEvent {
    /// Menu item index.
    pub index: u32,
    /// Pressed item status.
    pub state: GuiState,
    /// Pressed item text.
    pub text: String,
}

impl NappGUIEventParams for MenuEvent {
    type CType = EvMenu;
    const TYPE: &'static CStr = c"EvMenu";

    fn from(event: &Self::CType) -> MenuEvent {
        MenuEvent {
            index: event.index,
            state: GuiState::try_from(event.state).unwrap(),
            text: unsafe { CStr::from_ptr(event.text).to_string_lossy().into_owned() },
        }
    }
}

/// Scroll event parameters.
pub struct ScrollEvent {
    /// Scroll bar orientation.
    pub orient: GuiOrient,
    /// Scroll type.
    pub scroll: GuiScroll,
    /// Scroll position.
    pub cpos: f32,
}

impl NappGUIEventParams for ScrollEvent {
    type CType = EvScroll;
    const TYPE: &'static CStr = c"EvScroll";

    fn from(event: &Self::CType) -> ScrollEvent {
        ScrollEvent {
            orient: GuiOrient::try_from(event.orient).unwrap(),
            scroll: GuiScroll::try_from(event.scroll).unwrap(),
            cpos: event.cpos,
        }
    }
}

/// Location of a cell in a table.
pub struct TablePositionEvent {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

impl NappGUIEventParams for TablePositionEvent {
    type CType = EvTbPos;
    const TYPE: &'static CStr = c"EvTbPos";

    fn from(event: &Self::CType) -> TablePositionEvent {
        TablePositionEvent {
            x: event.col as f32,
            y: event.row as f32,
        }
    }
}

/// Location of a row in a table.
pub struct TablebRowEvent {
    /// Selected or not.
    pub is_selected: bool,
    /// Row index.
    pub row: u32,
}

impl NappGUIEventParams for TablebRowEvent {
    type CType = EvTbRow;
    const TYPE: &'static CStr = c"EvTbRow";

    fn from(event: &Self::CType) -> TablebRowEvent {
        TablebRowEvent {
            is_selected: event.sel != 0,
            row: event.row as _,
        }
    }
}

/// Group of cells in a table.
pub struct TablebRectEvent {
    /// Initial column index.
    pub start_col: u32,
    /// End column index.
    pub end_col: u32,
    /// Initial row index.
    pub start_row: u32,
    /// End row index.
    pub end_row: u32,
}

impl NappGUIEventParams for TablebRectEvent {
    type CType = EvTbRect;
    const TYPE: &'static CStr = c"EvTbRect";

    fn from(event: &Self::CType) -> TablebRectEvent {
        TablebRectEvent {
            start_col: event.stcol as _,
            end_col: event.edcol as _,
            start_row: event.strow as _,
            end_row: event.edrow as _,
        }
    }
}

/// Selection in a table.
pub struct TableSelectEvent {
    /// Row indices.
    pub select: Vec<u32>,
}

impl NappGUIEventParams for TableSelectEvent {
    type CType = EvTbSel;
    const TYPE: &'static CStr = c"EvTbSel";

    fn from(event: &Self::CType) -> TableSelectEvent {
        fn array_usize(array: *mut nappgui_sys::ArrStuint32_t) -> Option<Vec<u32>> {
            let array = unsafe { array.as_ref() }?;
            let content = unsafe { array.content.as_ref() }?;
            Some(content.elem[..array.size as usize].to_vec())
        }
        TableSelectEvent {
            select: array_usize(event.sel).unwrap(),
        }
    }
}

/// Data from a cell in a table.
pub struct TableCellEvent {
    /// Cell text.
    pub text: String,
    /// Text alignment.
    pub align: Align,
    /// Icon.
    pub icon: Image,
}

impl NappGUIEventParams for TableCellEvent {
    type CType = EvTbCell;
    const TYPE: &'static CStr = c"EvTbCell";

    fn from(event: &Self::CType) -> TableCellEvent {
        TableCellEvent {
            text: unsafe { CStr::from_ptr(event.text).to_string_lossy().into_owned() },
            align: Align::try_from(event.align).unwrap(),
            icon: unsafe { Image::from_raw_cloned(event.icon) },
        }
    }
}

impl NappGUIEventResult for TableCellEvent {
    type CType = EvTbCell;
    const TYPE: &'static CStr = c"EvTbCell";

    fn to(&self) -> Self::CType {
        let text = CString::new(self.text.as_str()).unwrap();
        let icon = self.icon.clone();
        nappgui_sys::EvTbCell {
            text: text.as_ptr(),
            align: self.align as _,
            icon: icon.as_ptr(),
        }
    }
}

/// Implement for core types.
impl NappGUIEventParams for bool {
    type CType = bool;
    const TYPE: &'static CStr = c"bool_t";

    fn from(event: &Self::CType) -> bool {
        *event
    }
}

/// Result of the OnFilter event of the text boxes.
pub struct TextFilterEvent {
    /// TRUE if the original control text should be changed.
    pub apply: bool,
    /// New control text, which is a revision (filter) of the original text. len <= 4096usize!
    pub text: String,
    /// Cursor position (caret).
    pub current_position: u32,
}

impl NappGUIEventResult for TextFilterEvent {
    type CType = EvTextFilter;
    const TYPE: &'static CStr = c"EvTextFilter";

    fn to(&self) -> Self::CType {
        use std::ffi::c_char;
        fn c_char_array<const N: usize>(text: &str) -> [c_char; N] {
            let mut result = [0 as c_char; N];

            let length = text
                .char_indices()
                .map(|(idx, _)| idx)
                .take_while(|&idx| idx <= N)
                .last()
                .unwrap_or(0);

            let count = if text.len() <= N { text.len() } else { length };

            unsafe {
                std::ptr::copy_nonoverlapping(text.as_ptr() as *const c_char, result.as_mut_ptr(), count);
            }

            result
        }

        EvTextFilter {
            apply: self.apply as _,
            text: c_char_array(&self.text),
            cpos: self.current_position as _,
        }
    }
}

/// Implement for core types.
impl NappGUIEventResult for bool {
    type CType = bool;

    const TYPE: &'static CStr = c"bool_t";

    fn to(&self) -> Self::CType {
        *self
    }
}
impl NappGUIEventResult for f32 {
    type CType = f32;

    const TYPE: &'static CStr = c"real32_t";

    fn to(&self) -> Self::CType {
        *self
    }
}

impl NappGUIEventResult for u32 {
    type CType = u32;

    const TYPE: &'static CStr = c"uint32_t";

    fn to(&self) -> Self::CType {
        *self
    }
}

/// The params of table on_data handler.
pub enum TableDataParams {
    /// Column index.
    TableNCols,
    /// Row index.
    TableCell(TablePositionEvent),
}

/// The result of table on_data handler.
pub enum TableDataResult {
    /// The text of the cell.
    TableNCols(u32),
    /// The align of the cell.
    TableCell(TableCellEvent),
}
