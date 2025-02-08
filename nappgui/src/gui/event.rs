use nappgui_sys::{
    align_t, gui_close_t, gui_mouse_t, gui_orient_t, gui_scroll_t, gui_state_t, vkey_t,
};

use crate::{draw_2d::DCtx, util::helper::array_u32};

/// Parameters of the OnClick event of a button or OnSelect of a popup.
pub struct EvButton {
    /// Button or item index.
    pub index: u32,
    /// State.
    pub state: gui_state_t,
    /// Text.
    pub text: String,
}

impl EvButton {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvButton) -> EvButton {
        if ptr.is_null() {
            panic!("EvButton is null");
        }
        let evbutton = unsafe { &*ptr };
        EvButton {
            index: evbutton.index,
            state: evbutton.state,
            text: unsafe { std::ffi::CStr::from_ptr(evbutton.text) }
                .to_string_lossy()
                .into_owned(),
        }
    }
}

/// Parameters of the OnMoved event of a slider.
pub struct EvSlider {
    /// Normalized slider position (0, 1).
    pub pos: f32,
    /// Increase with respect to the previous position.
    pub incr: f32,
    /// Interval index (only for discrete ranges).
    pub step: u32,
}

impl EvSlider {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvSlider) -> EvSlider {
        if ptr.is_null() {
            panic!("EvSlider is null");
        }
        let evslider = unsafe { &*ptr };
        EvSlider {
            pos: evslider.pos,
            incr: evslider.incr,
            step: evslider.step,
        }
    }
}

/// Parameters of the OnChange event of the text boxes.
pub struct EvText {
    /// Text.
    pub text: String,
    /// Cursor position (caret).
    pub cpos: u32,
    /// Number of characters inserted or deleted.
    pub len: i32,
}

impl EvText {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvText) -> EvText {
        if ptr.is_null() {
            panic!("EvText is null");
        }
        let evtext = unsafe { &*ptr };
        EvText {
            text: unsafe { std::ffi::CStr::from_ptr(evtext.text) }
                .to_string_lossy()
                .into_owned(),
            cpos: evtext.cpos,
            len: evtext.len,
        }
    }
}

/// Result of the OnFilter event of the text boxes.
pub struct EvTextFilter {
    /// TRUE if the original control text should be changed.
    pub apply: bool,
    /// New control text, which is a revision (filter) of the original text.
    pub text: String,
    /// Cursor position (caret).
    pub cpos: u32,
}

impl EvTextFilter {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvTextFilter) -> EvTextFilter {
        if ptr.is_null() {
            panic!("EvTextFilter is null");
        }
        let evfilter = unsafe { &*ptr };
        EvTextFilter {
            apply: evfilter.apply != 0,
            text: unsafe { std::ffi::CStr::from_ptr(evfilter.text.as_ptr()) }
                .to_string_lossy()
                .into_owned(),
            cpos: evfilter.cpos,
        }
    }
}

/// OnDraw event parameters.
pub struct EvDraw {
    /// 2D drawing context.
    pub ctx: DCtx,
    /// X coordinate of the drawing area (viewport).
    pub x: f32,
    /// Y coordinate of the drawing area.
    pub y: f32,
    /// Width of the drawing area.
    pub width: f32,
    /// Height of the drawing area.
    pub height: f32,
}

impl EvDraw {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvDraw) -> EvDraw {
        if ptr.is_null() {
            panic!("EvDraw is null");
        }
        let evdraw = unsafe { &*ptr };
        EvDraw {
            ctx: DCtx::new(evdraw.ctx),
            x: evdraw.x,
            y: evdraw.y,
            width: evdraw.width,
            height: evdraw.height,
        }
    }
}

/// Mouse event parameters.
pub struct EvMouse {
    /// X coordinate of the pointer in the drawing area.
    pub x: f32,
    /// Y coordinate of the pointer in the drawing area.
    pub y: f32,
    /// X coordinate of the pointer on the control. Same as x if there are no scroll bars.
    pub lx: f32,
    /// Y coordinate of the pointer on the control. Same as y if there are no scroll bars.
    pub ly: f32,
    /// Active button.
    pub button: gui_mouse_t,
    /// Number of clicks.
    pub count: u32,
    /// Combination of values mkey_t. todo!
    pub modifiers: u32,
    /// Additional value for controls.
    pub tag: u32,
}

impl EvMouse {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvMouse) -> EvMouse {
        if ptr.is_null() {
            panic!("EvMouse is null");
        }
        let evmouse = unsafe { &*ptr };
        EvMouse {
            x: evmouse.x,
            y: evmouse.y,
            lx: evmouse.lx,
            ly: evmouse.ly,
            button: evmouse.button,
            count: evmouse.count,
            modifiers: evmouse.modifiers,
            tag: evmouse.tag,
        }
    }
}

/// OnWheel event parameters.
pub struct EvWheel {
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

impl EvWheel {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvWheel) -> EvWheel {
        if ptr.is_null() {
            panic!("EvWheel is null");
        }
        let evwheel = unsafe { &*ptr };
        EvWheel {
            x: evwheel.x,
            y: evwheel.y,
            dx: evwheel.dx,
            dy: evwheel.dy,
            dz: evwheel.dz,
        }
    }
}

/// Keyboard event parameters.
pub struct EvKey {
    /// Referenced key.
    pub key: vkey_t,
    /// Combination of values mkey_t. todo!
    pub modifiers: u32,
}

impl EvKey {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvKey) -> EvKey {
        if ptr.is_null() {
            panic!("EvKey is null");
        }
        let evkey = unsafe { &*ptr };
        EvKey {
            key: evkey.key,
            modifiers: evkey.modifiers,
        }
    }
}

/// Parameters of change of position events.
pub struct EvPos {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

impl EvPos {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvPos) -> EvPos {
        if ptr.is_null() {
            panic!("EvPos is null");
        }
        let evpos = unsafe { &*ptr };
        EvPos {
            x: evpos.x,
            y: evpos.y,
        }
    }
}

/// Resize event parameters.
pub struct EvSize {
    /// Width (size in x).
    pub width: f32,
    /// Height (size in y).
    pub height: f32,
}

impl EvSize {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvSize) -> EvSize {
        if ptr.is_null() {
            panic!("EvSize is null");
        }
        let evresize = unsafe { &*ptr };
        EvSize {
            width: evresize.width,
            height: evresize.height,
        }
    }
}

/// Window closing Event Parameters.
pub struct EvWinClose {
    /// Origin of the close.
    pub origin: gui_close_t,
}

impl EvWinClose {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvWinClose) -> EvWinClose {
        if ptr.is_null() {
            panic!("EvWinClose is null");
        }
        let evclose = unsafe { &*ptr };
        EvWinClose {
            origin: evclose.origin,
        }
    }
}

/// Menu event parameters.
pub struct EvMenu {
    /// Menu item index.
    pub index: u32,
    /// Pressed item status.
    pub state: gui_state_t,
    /// Pressed item text.
    pub text: String,
}

impl EvMenu {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvMenu) -> EvMenu {
        if ptr.is_null() {
            panic!("EvMenu is null");
        }
        let evmenu = unsafe { &*ptr };
        EvMenu {
            index: evmenu.index,
            state: evmenu.state,
            text: unsafe {
                std::ffi::CStr::from_ptr(evmenu.text)
                    .to_string_lossy()
                    .into_owned()
            },
        }
    }
}

/// Scroll event parameters.
pub struct EvScroll {
    /// Scroll bar orientation.
    pub orient: gui_orient_t,
    /// Scroll type.
    pub scroll: gui_scroll_t,
    /// Scroll position.
    pub cpos: f32,
}

impl EvScroll {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvScroll) -> EvScroll {
        if ptr.is_null() {
            panic!("EvScroll is null");
        }
        let evscroll = unsafe { &*ptr };
        EvScroll {
            orient: evscroll.orient,
            scroll: evscroll.scroll,
            cpos: evscroll.cpos,
        }
    }
}

/// Location of a cell in a table.
pub struct EvTbPos {
    /// Column index.
    pub col: u32,
    /// Row index.
    pub row: u32,
}

impl EvTbPos {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvTbPos) -> EvTbPos {
        if ptr.is_null() {
            panic!("EvTbPos is null");
        }
        let evtbpos = unsafe { &*ptr };
        EvTbPos {
            col: evtbpos.col,
            row: evtbpos.row,
        }
    }
}

/// Location of a row in a table.
pub struct EvTbRow {
    /// Selected or not.
    pub sel: bool,
    /// Row index.
    pub row: u32,
}

impl EvTbRow {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvTbRow) -> EvTbRow {
        if ptr.is_null() {
            panic!("EvTbRow is null");
        }
        let evtbrow = unsafe { &*ptr };
        EvTbRow {
            sel: evtbrow.sel != 0,
            row: evtbrow.row,
        }
    }
}

/// Group of cells in a table.
pub struct EvTbRect {
    /// Initial column index.
    pub stcol: u32,
    /// End column index.
    pub edcol: u32,
    /// Initial row index.
    pub strow: u32,
    /// End row index.
    pub edrow: u32,
}

impl EvTbRect {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvTbRect) -> EvTbRect {
        if ptr.is_null() {
            panic!("EvTbRect is null");
        }
        let evtbrect = unsafe { &*ptr };
        EvTbRect {
            stcol: evtbrect.stcol,
            edcol: evtbrect.edcol,
            strow: evtbrect.strow,
            edrow: evtbrect.edrow,
        }
    }
}

/// Selection in a table.
pub struct EvTbSel {
    /// Row indices.
    pub sel: Vec<u32>,
}

impl EvTbSel {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvTbSel) -> EvTbSel {
        if ptr.is_null() {
            panic!("EvTbSel is null");
        }
        let evtbsel = unsafe { &*ptr };
        EvTbSel {
            sel: array_u32(evtbsel.sel).unwrap(),
        }
    }
}

/// Data from a cell in a table.
pub struct EvTbCell {
    /// Cell text.
    pub text: String,
    /// Text alignment.
    pub align: align_t,
}

impl EvTbCell {
    pub(crate) fn from_ptr(ptr: *mut nappgui_sys::EvTbCell) -> EvTbCell {
        if ptr.is_null() {
            panic!("EvTbCell is null");
        }
        let evtbcell = unsafe { &*ptr };
        EvTbCell {
            text: unsafe {
                std::ffi::CStr::from_ptr(evtbcell.text)
                    .to_string_lossy()
                    .into_owned()
            },
            align: evtbcell.align,
        }
    }
}

macro_rules! event_params {
    ($type:ty) => {
        impl crate::core::event::NappGUIEventParams for $type {
            fn type_() -> &'static str {
                stringify!($type)
            }

            fn from_ptr(ptr: *mut std::ffi::c_void) -> Option<Self> {
                if ptr.is_null() {
                    return None;
                }

                Some(Self::from_ptr(ptr as _))
            }
        }
    };
}

macro_rules! event_result {
    ($type:ty) => {
        impl crate::core::event::NappGUIEventResult for $type {
            fn type_() -> &'static str {
                stringify!($type)
            }

            fn from_ptr(ptr: *mut std::ffi::c_void) -> Option<Self> {
                if ptr.is_null() {
                    return None;
                }

                Some(Self::from_ptr(ptr as _))
            }
        }
    };
}

event_params!(EvButton);
event_params!(EvSlider);
event_params!(EvText);
event_params!(EvDraw);
event_params!(EvMouse);
event_params!(EvWheel);
event_params!(EvKey);
event_params!(EvPos);
event_params!(EvSize);
event_params!(EvWinClose);
event_params!(EvMenu);
event_params!(EvScroll);
event_params!(EvTbPos);
event_params!(EvTbRow);
event_params!(EvTbRect);
event_params!(EvTbSel);
event_params!(EvTbCell);

event_result!(EvTextFilter);
