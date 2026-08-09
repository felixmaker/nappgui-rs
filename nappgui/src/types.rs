macro_rules! define_enum {
    (
        $(
            $(#[doc = $enum_doc:expr])*
            pub enum $name:ident {
                $(
                    $(#[doc = $variant_doc:expr])*
                    $variant:ident = $sys_const:expr
                ),* $(,)?
            }
        )*
    ) => {
        $( // Outer loop over every enum provided
            $(#[doc = $enum_doc])*
            #[repr(i32)]
            #[allow(missing_docs)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum $name {
                $(
                    $(#[doc = $variant_doc])*
                    $variant = $sys_const as i32,
                )*
            }

            impl TryFrom<i32> for $name {
                type Error = crate::error::NappguiError;

                fn try_from(value: i32) -> Result<Self, Self::Error> {
                    match value {
                        $(
                            v if v == ($sys_const as i32) => Ok($name::$variant),
                        )*
                        _ => Err(crate::error::NappguiError::Internal(
                            crate::error::NappguiErrorKind::UndefinedEnumTransmute,
                        )),
                    }
                }
            }

            impl From<$name> for i32 {
                #[inline]
                fn from(value: $name) -> Self {
                    value as i32
                }
            }

            impl From<$name> for u32 {
                #[inline]
                fn from(value: $name) -> Self {
                    value as u32
                }
            }
        )*
    };
}

define_enum! {
    /// Alignment values.
    pub enum Align {
        /// Alignment to the left/upper margin.
        Left = nappgui_sys::_align_t_ekLEFT,
        /// Centered alignment.
        Center = nappgui_sys::_align_t_ekCENTER,
        /// Alignment to the right/lower margin.
        Right = nappgui_sys::_align_t_ekRIGHT,
        /// Justification or expansion of content.
        Justify = nappgui_sys::_align_t_ekJUSTIFY,
    }

    /// State values.
    pub enum GuiState {
        /// Off
        Off = nappgui_sys::_gui_state_t_ekGUI_OFF,
        /// On
        On = nappgui_sys::_gui_state_t_ekGUI_ON,
        /// Medium/undetermined.
        Mixed = nappgui_sys::_gui_state_t_ekGUI_MIXED,
    }

    /// Ways of scaling an image.
    pub enum Scale {
        /// The image will be displayed centered in the control frame, without scaling. It may not be fully displayed.
        None = nappgui_sys::_gui_scale_t_ekGUI_SCALE_NONE,
        /// The image will be scaled to fit the control frame. Its aspect ratio may change.
        Auto = nappgui_sys::_gui_scale_t_ekGUI_SCALE_AUTO,
        /// The image will be scaled to fit the control frame, but without changing its aspect ratio. If one of the dimensions (width/height) is smaller than the size of the control, it will be centered.
        Aspect = nappgui_sys::_gui_scale_t_ekGUI_SCALE_ASPECT,
        /// Same as above, but does not increase the original size, only reduces it if necessary. If any of the dimensions (width/height) is less than the size of the control, it will be centered.
        AspectDW = nappgui_sys::_gui_scale_t_ekGUI_SCALE_ASPECTDW,
        /// The size of the control (e.g. ImageView) will be adjusted by default to the size of the image.
        Adjust = nappgui_sys::_gui_scale_t_ekGUI_SCALE_ADJUST,
    }

    /// Orientation.
    pub enum GuiOrient {
        /// Horizontal.
        Horizontal = nappgui_sys::_gui_orient_t_ekGUI_HORIZONTAL,
        /// Vertical.
        Vertical = nappgui_sys::_gui_orient_t_ekGUI_VERTICAL,
    }


    /// Behavior of the divider in a SplitView.
    pub enum SplitMode {
        /// The divider maintains its left/right or up/down proportional position.
        Normal = nappgui_sys::_split_mode_t_ekSPLIT_NORMAL,
        /// The left/upper child maintains its size.
        Fixed0 = nappgui_sys::_split_mode_t_ekSPLIT_FIXED0,
        /// The right/lower child maintains its size.
        Fixed1 = nappgui_sys::_split_mode_t_ekSPLIT_FIXED1,
    }


    /// Action that has motivated the change of keyboard focus.
    pub enum GuiTab {
        /// Pressing the \[TAB\] key.
        Key = nappgui_sys::_gui_tab_t_ekGUI_TAB_KEY,
        /// Pressing \[SHIFT\]+\[TAB\].
        BackKey = nappgui_sys::_gui_tab_t_ekGUI_TAB_BACKKEY,
        /// Call to window_next_tabstop.
        Next = nappgui_sys::_gui_tab_t_ekGUI_TAB_NEXT,
        /// Call to window_previous_tabstop.
        Prev = nappgui_sys::_gui_tab_t_ekGUI_TAB_PREV,
        /// Call to window_focus.
        Move = nappgui_sys::_gui_tab_t_ekGUI_TAB_MOVE,
        /// Click on another control.
        Click = nappgui_sys::_gui_tab_t_ekGUI_TAB_CLICK,
    }


    /// Keyboard codes.
    pub enum KeyCode {
        Undefined = nappgui_sys::_key_t_ekKEY_UNDEF,
        KeyA = nappgui_sys::_key_t_ekKEY_A,
        KeyS = nappgui_sys::_key_t_ekKEY_S,
        KeyD = nappgui_sys::_key_t_ekKEY_D,
        KeyF = nappgui_sys::_key_t_ekKEY_F,
        KeyH = nappgui_sys::_key_t_ekKEY_H,
        KeyG = nappgui_sys::_key_t_ekKEY_G,
        KeyZ = nappgui_sys::_key_t_ekKEY_Z,
        KeyX = nappgui_sys::_key_t_ekKEY_X,
        KeyC = nappgui_sys::_key_t_ekKEY_C,
        KeyV = nappgui_sys::_key_t_ekKEY_V,
        Backslash = nappgui_sys::_key_t_ekKEY_BSLASH,
        KeyB = nappgui_sys::_key_t_ekKEY_B,
        KeyQ = nappgui_sys::_key_t_ekKEY_Q,
        KeyW = nappgui_sys::_key_t_ekKEY_W,
        KeyE = nappgui_sys::_key_t_ekKEY_E,
        KeyR = nappgui_sys::_key_t_ekKEY_R,
        KeyY = nappgui_sys::_key_t_ekKEY_Y,
        KeyT = nappgui_sys::_key_t_ekKEY_T,
        Digit1 = nappgui_sys::_key_t_ekKEY_1,
        Digit2 = nappgui_sys::_key_t_ekKEY_2,
        Digit3 = nappgui_sys::_key_t_ekKEY_3,
        Digit4 = nappgui_sys::_key_t_ekKEY_4,
        Digit6 = nappgui_sys::_key_t_ekKEY_6,
        Digit5 = nappgui_sys::_key_t_ekKEY_5,
        Digit9 = nappgui_sys::_key_t_ekKEY_9,
        Digit7 = nappgui_sys::_key_t_ekKEY_7,
        Digit8 = nappgui_sys::_key_t_ekKEY_8,
        Digit0 = nappgui_sys::_key_t_ekKEY_0,
        RCurlyBrace = nappgui_sys::_key_t_ekKEY_RCURLY,
        KeyO = nappgui_sys::_key_t_ekKEY_O,
        KeyU = nappgui_sys::_key_t_ekKEY_U,
        LCurlyBrace = nappgui_sys::_key_t_ekKEY_LCURLY,
        KeyI = nappgui_sys::_key_t_ekKEY_I,
        KeyP = nappgui_sys::_key_t_ekKEY_P,
        Enter = nappgui_sys::_key_t_ekKEY_RETURN,
        KeyL = nappgui_sys::_key_t_ekKEY_L,
        KeyJ = nappgui_sys::_key_t_ekKEY_J,
        Semicolon = nappgui_sys::_key_t_ekKEY_SEMICOLON,
        KeyK = nappgui_sys::_key_t_ekKEY_K,
        QuestionMark = nappgui_sys::_key_t_ekKEY_QUEST,
        Comma = nappgui_sys::_key_t_ekKEY_COMMA,
        Minus = nappgui_sys::_key_t_ekKEY_MINUS,
        KeyN = nappgui_sys::_key_t_ekKEY_N,
        KeyM = nappgui_sys::_key_t_ekKEY_M,
        Period = nappgui_sys::_key_t_ekKEY_PERIOD,
        Tab = nappgui_sys::_key_t_ekKEY_TAB,
        Space = nappgui_sys::_key_t_ekKEY_SPACE,
        GreaterThanLessThan = nappgui_sys::_key_t_ekKEY_GTLT,
        Back = nappgui_sys::_key_t_ekKEY_BACK,
        Escape = nappgui_sys::_key_t_ekKEY_ESCAPE,
        F17 = nappgui_sys::_key_t_ekKEY_F17,
        NumpadDecimal = nappgui_sys::_key_t_ekKEY_NUMDECIMAL,
        NumpadMultiply = nappgui_sys::_key_t_ekKEY_NUMMULT,
        NumpadAdd = nappgui_sys::_key_t_ekKEY_NUMADD,
        NumLock = nappgui_sys::_key_t_ekKEY_NUMLOCK,
        NumpadDivide = nappgui_sys::_key_t_ekKEY_NUMDIV,
        NumpadEnter = nappgui_sys::_key_t_ekKEY_NUMRET,
        NumpadMinus = nappgui_sys::_key_t_ekKEY_NUMMINUS,
        F18 = nappgui_sys::_key_t_ekKEY_F18,
        F19 = nappgui_sys::_key_t_ekKEY_F19,
        NumpadEqual = nappgui_sys::_key_t_ekKEY_NUMEQUAL,
        Numpad0 = nappgui_sys::_key_t_ekKEY_NUM0,
        Numpad1 = nappgui_sys::_key_t_ekKEY_NUM1,
        Numpad2 = nappgui_sys::_key_t_ekKEY_NUM2,
        Numpad3 = nappgui_sys::_key_t_ekKEY_NUM3,
        Numpad4 = nappgui_sys::_key_t_ekKEY_NUM4,
        Numpad5 = nappgui_sys::_key_t_ekKEY_NUM5,
        Numpad6 = nappgui_sys::_key_t_ekKEY_NUM6,
        Numpad7 = nappgui_sys::_key_t_ekKEY_NUM7,
        Numpad8 = nappgui_sys::_key_t_ekKEY_NUM8,
        Numpad9 = nappgui_sys::_key_t_ekKEY_NUM9,
        F5 = nappgui_sys::_key_t_ekKEY_F5,
        F6 = nappgui_sys::_key_t_ekKEY_F6,
        F7 = nappgui_sys::_key_t_ekKEY_F7,
        F3 = nappgui_sys::_key_t_ekKEY_F3,
        F8 = nappgui_sys::_key_t_ekKEY_F8,
        F9 = nappgui_sys::_key_t_ekKEY_F9,
        F11 = nappgui_sys::_key_t_ekKEY_F11,
        F13 = nappgui_sys::_key_t_ekKEY_F13,
        F16 = nappgui_sys::_key_t_ekKEY_F16,
        F14 = nappgui_sys::_key_t_ekKEY_F14,
        F10 = nappgui_sys::_key_t_ekKEY_F10,
        F12 = nappgui_sys::_key_t_ekKEY_F12,
        F15 = nappgui_sys::_key_t_ekKEY_F15,
        PageUp = nappgui_sys::_key_t_ekKEY_PAGEUP,
        Home = nappgui_sys::_key_t_ekKEY_HOME,
        Delete = nappgui_sys::_key_t_ekKEY_SUPR,
        F4 = nappgui_sys::_key_t_ekKEY_F4,
        PageDown = nappgui_sys::_key_t_ekKEY_PAGEDOWN,
        F2 = nappgui_sys::_key_t_ekKEY_F2,
        End = nappgui_sys::_key_t_ekKEY_END,
        F1 = nappgui_sys::_key_t_ekKEY_F1,
        Left = nappgui_sys::_key_t_ekKEY_LEFT,
        Right = nappgui_sys::_key_t_ekKEY_RIGHT,
        Down = nappgui_sys::_key_t_ekKEY_DOWN,
        Up = nappgui_sys::_key_t_ekKEY_UP,
        ShiftLeft = nappgui_sys::_key_t_ekKEY_LSHIFT,
        ShiftRight = nappgui_sys::_key_t_ekKEY_RSHIFT,
        ControlLeft = nappgui_sys::_key_t_ekKEY_LCTRL,
        ControlRight = nappgui_sys::_key_t_ekKEY_RCTRL,
        AltLeft = nappgui_sys::_key_t_ekKEY_LALT,
        AltRight = nappgui_sys::_key_t_ekKEY_RALT,
        Insert = nappgui_sys::_key_t_ekKEY_INSERT,
        Exclamation = nappgui_sys::_key_t_ekKEY_EXCLAM,
        Menu = nappgui_sys::_key_t_ekKEY_MENU,
        SuperLeft = nappgui_sys::_key_t_ekKEY_LWIN,
        SuperRight = nappgui_sys::_key_t_ekKEY_RWIN,
        CapsLock = nappgui_sys::_key_t_ekKEY_CAPS,
        Tilde = nappgui_sys::_key_t_ekKEY_TILDE,
        Backquote = nappgui_sys::_key_t_ekKEY_GRAVE,
        Add = nappgui_sys::_key_t_ekKEY_PLUS,
    }

    /// Result when changing the keyboard focus.
    pub enum GuiFocus {
        /// Focus has changed to the specified control.
        Changed = nappgui_sys::_gui_focus_t_ekGUI_FOCUS_CHANGED,
        /// Focus has not moved, it remains on the same control.
        Keep = nappgui_sys::_gui_focus_t_ekGUI_FOCUS_KEEP,
        /// Target control not found, hidden, or disabled.
        NoNext = nappgui_sys::_gui_focus_t_ekGUI_FOCUS_NO_NEXT,
        /// The current control does not allow changing focus.
        NoResign = nappgui_sys::_gui_focus_t_ekGUI_FOCUS_NO_RESIGN,
        /// The new control does not accept focus.
        NoAccept = nappgui_sys::_gui_focus_t_ekGUI_FOCUS_NO_ACCEPT,
    }


    /// Cursors.
    pub enum GuiCursor {
        /// Arrow (default).
        Arrow = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_ARROW,
        /// Hand.
        Hand = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_HAND,
        /// Vertical bar (text editing).
        VerticalBar = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_IBEAM,
        /// Cross.
        Cross = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_CROSS,
        /// Horizontal resize (left-right).
        HorizontalResize = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_SIZEWE,
        /// Vertical resize (top-bottom).
        VerticalResize = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_SIZENS,
        /// Created from an image.
        User = nappgui_sys::_gui_cursor_t_ekGUI_CURSOR_USER,
    }


    /// Pixel format in an image. Number of bits per pixel and color model.
    pub enum PixFormat {
        /// 1 bit per pixel. 2 colors, indexed.
        Index1 = nappgui_sys::_pixformat_t_ekINDEX1,
        /// 2 bits per pixel. 4 colors, indexed.
        Index2 = nappgui_sys::_pixformat_t_ekINDEX2,
        /// 4 bits per pixel. 16 colors, indexed.
        Index4 = nappgui_sys::_pixformat_t_ekINDEX4,
        /// 8 bits per pixel. 256 colors, indexed.
        Index8 = nappgui_sys::_pixformat_t_ekINDEX8,
        /// 8 bits per pixel in grayscale. 256 shades of gray.
        Gray8 = nappgui_sys::_pixformat_t_ekGRAY8,
        /// 24 bits per RGB pixel. 8 bits per channel (red, green, blue). The lowest order byte corresponds to the red one and the highest one to the blue one.
        RGB24 = nappgui_sys::_pixformat_t_ekRGB24,
        /// 32 bits per pixel RGBA. 8 bits per channel (red, green, blue, alpha). The lowest order byte corresponds to the red one and the highest one to alpha (transparency).
        RGBA32 = nappgui_sys::_pixformat_t_ekRGBA32,
        /// Represents the original format of the image. Only valid at image_pixels.
        Image = nappgui_sys::_pixformat_t_ekFIMAGE,
    }


    /// Mouse buttons.
    pub enum GuiMouse {
        /// Left.
        Left = nappgui_sys::_gui_mouse_t_ekGUI_MOUSE_LEFT,
        /// Right.
        Right = nappgui_sys::_gui_mouse_t_ekGUI_MOUSE_RIGHT,
        /// Center.
        Middle = nappgui_sys::_gui_mouse_t_ekGUI_MOUSE_MIDDLE,
    }


    /// Types of scroll.
    pub enum GuiScroll {
        /// Jump to start.
        Begin = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_BEGIN,
        /// Skip to the end.
        End = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_END,
        /// Jump one step (or line) to the left or up.
        StepLeft = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_STEP_LEFT,
        /// Jump one step (or line) to the right or down.
        StepRight = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_STEP_RIGHT,
        /// Jump a page to the left or up.
        PageLeft = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_PAGE_LEFT,
        /// Jump a page to the right or down.
        PageRight = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_PAGE_RIGHT,
        /// Jump to the thumb position.
        Thumb = nappgui_sys::_gui_scroll_t_ekGUI_SCROLL_THUMB,
    }


    /// Event type.
    pub enum EventType {
        /// Click on a Label control.
        Label = nappgui_sys::_gui_event_t_ekGUI_EVENT_LABEL,
        /// Click on a Button control.
        Button = nappgui_sys::_gui_event_t_ekGUI_EVENT_BUTTON,
        /// The selection of a PopUp control has been changed.
        PopUp = nappgui_sys::_gui_event_t_ekGUI_EVENT_POPUP,
        /// The selection of a Tabs control has been changed.
        Tabs = nappgui_sys::_gui_event_t_ekGUI_EVENT_TABS,
        /// The selection of a control has been changed ListBox.
        ListBox = nappgui_sys::_gui_event_t_ekGUI_EVENT_LISTBOX,
        /// You are moving an Slider control.
        Slider = nappgui_sys::_gui_event_t_ekGUI_EVENT_SLIDER,
        /// Click on a UpDown control.
        UpDown = nappgui_sys::_gui_event_t_ekGUI_EVENT_UPDOWN,
        /// The text of a Edit or Combo control is being edited.
        TextFilter = nappgui_sys::_gui_event_t_ekGUI_EVENT_TXTFILTER,
        /// You have finished editing the text of a Edit or Combo control.
        TextChange = nappgui_sys::_gui_event_t_ekGUI_EVENT_TXTCHANGE,
        /// Ask a control if it wants to give up keyboard focus.
        FocusResign = nappgui_sys::_gui_event_t_ekGUI_EVENT_FOCUS_RESIGN,
        /// Ask a control if it wants to accept keyboard focus.
        FocusAccept = nappgui_sys::_gui_event_t_ekGUI_EVENT_FOCUS_ACCEPT,
        /// A control has received or lost keyboard focus.
        Focus = nappgui_sys::_gui_event_t_ekGUI_EVENT_FOCUS,
        /// Click on a menu.
        Menu = nappgui_sys::_gui_event_t_ekGUI_EVENT_MENU,
        /// The view content must be redrawn.
        Draw = nappgui_sys::_gui_event_t_ekGUI_EVENT_DRAW,
        /// There is draw the overlay layer.
        Overlay = nappgui_sys::_gui_event_t_ekGUI_EVENT_OVERLAY,
        /// The size of a view has changed.
        Resize = nappgui_sys::_gui_event_t_ekGUI_EVENT_RESIZE,
        /// The mouse has entered the view area.
        Enter = nappgui_sys::_gui_event_t_ekGUI_EVENT_ENTER,
        /// The mouse has left the view area.
        Exit = nappgui_sys::_gui_event_t_ekGUI_EVENT_EXIT,
        /// The mouse is moving on the view surface.
        Move = nappgui_sys::_gui_event_t_ekGUI_EVENT_MOVED,
        /// A mouse button was pressed.
        Down = nappgui_sys::_gui_event_t_ekGUI_EVENT_DOWN,
        /// A mouse button has been released.
        Up = nappgui_sys::_gui_event_t_ekGUI_EVENT_UP,
        /// Click on a view.
        Click = nappgui_sys::_gui_event_t_ekGUI_EVENT_CLICK,
        /// Dragging is being done over.
        Drag = nappgui_sys::_gui_event_t_ekGUI_EVENT_DRAG,
        /// Mouse wheel has moved.
        Wheel = nappgui_sys::_gui_event_t_ekGUI_EVENT_WHEEL,
        /// A key has been pressed.
        KeyDown = nappgui_sys::_gui_event_t_ekGUI_EVENT_KEYDOWN,
        /// A key has been released.
        KeyUp = nappgui_sys::_gui_event_t_ekGUI_EVENT_KEYUP,
        /// The scroll bars are being manipulated.
        Scroll = nappgui_sys::_gui_event_t_ekGUI_EVENT_SCROLL,
        /// The window is moving across the desktop.
        WindowMoved = nappgui_sys::_gui_event_t_ekGUI_EVENT_WND_MOVED,
        /// The window is being resized.
        WindowSizing = nappgui_sys::_gui_event_t_ekGUI_EVENT_WND_SIZING,
        /// The window has been resized.
        WindowSize = nappgui_sys::_gui_event_t_ekGUI_EVENT_WND_SIZE,
        /// The window has been closed.
        WindowClose = nappgui_sys::_gui_event_t_ekGUI_EVENT_WND_CLOSE,
        /// An update color of comwin_color.
        Color = nappgui_sys::_gui_event_t_ekGUI_EVENT_COLOR,
        /// Desktop theme has changed.
        Theme = nappgui_sys::_gui_event_t_ekGUI_EVENT_THEME,
        /// An object linked to a layout has been edited. Notifications and calculated fields.
        ObjectChange = nappgui_sys::_gui_event_t_ekGUI_EVENT_OBJCHANGE,
        /// A table needs to know the number of rows.
        TableNRows = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_NROWS,
        /// A table will begin to draw the visible part of the data.
        TableBegin = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_BEGIN,
        /// A table has finished drawing.
        TableEnd = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_END,
        /// A table needs the data of a cell.
        TableCell = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_CELL,
        /// The selected rows in a table have changed.
        TableSelected = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_SEL,
        /// Click on a table header.
        TableHeadClick = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_HEADCLICK,
        /// Click on a table row.
        TableRowClick = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_ROWCLICK,
        /// A table tree needs the number of children for a node.
        TableNChildren = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_NCHILDREN,
        /// A table tree needs configuration information for a node.
        TableNodeInfo = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_NODEINFO,
        /// A table tree node has been expanded or collapsed.
        TableExpand = nappgui_sys::_gui_event_t_ekGUI_EVENT_TBL_EXPAND,
        /// Application is processing an idle state.
        Idle = nappgui_sys::_gui_event_t_ekGUI_EVENT_IDLE,
    }

    /// The type of a control.
    pub enum GuiType {
        Button = nappgui_sys::_gui_type_t_ekGUI_TYPE_BUTTON,
        Popup = nappgui_sys::_gui_type_t_ekGUI_TYPE_POPUP,
        EditBox = nappgui_sys::_gui_type_t_ekGUI_TYPE_EDITBOX,
        ComboBox = nappgui_sys::_gui_type_t_ekGUI_TYPE_COMBOBOX,
        TabList = nappgui_sys::_gui_type_t_ekGUI_TYPE_TABLIST,
        Slider = nappgui_sys::_gui_type_t_ekGUI_TYPE_SLIDER,
        UpDown = nappgui_sys::_gui_type_t_ekGUI_TYPE_UPDOWN,
        Progress = nappgui_sys::_gui_type_t_ekGUI_TYPE_PROGRESS,
        TextView = nappgui_sys::_gui_type_t_ekGUI_TYPE_TEXTVIEW,
        WebView = nappgui_sys::_gui_type_t_ekGUI_TYPE_WEBVIEW,
        SplitView = nappgui_sys::_gui_type_t_ekGUI_TYPE_SPLITVIEW,
        CustomView = nappgui_sys::_gui_type_t_ekGUI_TYPE_CUSTOMVIEW,
        Panel = nappgui_sys::_gui_type_t_ekGUI_TYPE_PANEL,
        Line = nappgui_sys::_gui_type_t_ekGUI_TYPE_LINE,
        Window = nappgui_sys::_gui_type_t_ekGUI_TYPE_WINDOW,
    }

    /// The button style.
    pub enum ButtonStyle {
        Push = nappgui_sys::_button_flag_t_ekBUTTON_PUSH,
        Check2 = nappgui_sys::_button_flag_t_ekBUTTON_CHECK2,
        Check3 =nappgui_sys::_button_flag_t_ekBUTTON_CHECK3,
        Radio = nappgui_sys::_button_flag_t_ekBUTTON_RADIO,
        Flat = nappgui_sys::_button_flag_t_ekBUTTON_FLAT,
        Flatgle = nappgui_sys::_button_flag_t_ekBUTTON_FLATGLE,
        Other = nappgui_sys::_button_flag_t_ekBUTTON_TYPE,
    }

    /// The ellipsis(trimming) style.
    pub enum Ellipsis {
        None = nappgui_sys::_ellipsis_t_ekELLIPNONE,
        Begin = nappgui_sys::_ellipsis_t_ekELLIPBEGIN,
        Middle = nappgui_sys::_ellipsis_t_ekELLIPMIDDLE,
        End = nappgui_sys::_ellipsis_t_ekELLIPEND,
        Multiline = nappgui_sys::_ellipsis_t_ekELLIPMLINE,
    }

    /// The close reason.
    pub enum GuiClose {
        /// The \[ESC\] key has been pressed (cancel).
        Cancel = nappgui_sys::_gui_close_t_ekGUI_CLOSE_ESC,
        /// The \[ENTER\] key has been pressed (accept).
        Accept = nappgui_sys::_gui_close_t_ekGUI_CLOSE_INTRO,
        /// The close button \[X\] has been pressed in the title bar.
        Close = nappgui_sys::_gui_close_t_ekGUI_CLOSE_BUTTON,
        /// The main window has been clicked (only received by overlay windows).
        Deactivate = nappgui_sys::_gui_close_t_ekGUI_CLOSE_DEACT,
    }
}

use bitflags::bitflags;

bitflags! {
    /// Window flags.
    pub struct WindowFlags: u32 {
        /// The window draws an outer border.
        const Edge = nappgui_sys::_window_flag_t_ekWINDOW_EDGE as u32;
        /// The window has a title bar.
        const Title = nappgui_sys::_window_flag_t_ekWINDOW_TITLE as u32;
        /// The window shows the maximize button.
        const Maximize = nappgui_sys::_window_flag_t_ekWINDOW_MAX as u32;
        /// The window shows the minimize button.
        const Minimize = nappgui_sys::_window_flag_t_ekWINDOW_MIN as u32;
        /// The window shows the close button.
        const Close = nappgui_sys::_window_flag_t_ekWINDOW_CLOSE as u32;
        /// The window has resizable borders.
        const Resizable = nappgui_sys::_window_flag_t_ekWINDOW_RESIZE as u32;
        /// The window will process the pressing of the \[RETURN\] key as a possible closing event, sending the message OnClose.
        const Return = nappgui_sys::_window_flag_t_ekWINDOW_RETURN as u32;
        /// The window will process the pressing of the \[ESC\] key as a possible closing event, sending the message OnClose.
        const Escape = nappgui_sys::_window_flag_t_ekWINDOW_ESC as u32;
        /// Avoids hiding a modal window when the modal cycle has finished. See Modal windows.
        const OffScreen = nappgui_sys::_window_flag_t_ekWINDOW_OFFSCREEN as u32;
        /// The window is a modal window.
        const ModalNoHide = nappgui_sys::_window_flag_t_ekWINDOW_MODAL_NOHIDE as u32;
    }

    /// Modifier keys.
    pub struct ModifierKey: u32 {
        /// None.
        const None = nappgui_sys::_mkey_t_ekMKEY_NONE as u32;
        /// Shift key.
        const Shift = nappgui_sys::_mkey_t_ekMKEY_SHIFT as u32;
        /// Control key.
        const Control = nappgui_sys::_mkey_t_ekMKEY_CONTROL as u32;
        /// Alt key.
        const Alt = nappgui_sys::_mkey_t_ekMKEY_ALT as u32;
        /// Command key.
        const Command = nappgui_sys::_mkey_t_ekMKEY_COMMAND as u32;
    }

    /// Font style.
    pub struct FontStyle: u32 {
        /// Normal font, no style. Also called Regular, indicated in pixels.
        const Normal = nappgui_sys::_fstyle_t_ekFNORMAL as u32;
        /// Bold font.
        const Bold = nappgui_sys::_fstyle_t_ekFBOLD as u32;
        /// Italic font.
        const Italic = nappgui_sys::_fstyle_t_ekFITALIC as u32;
        /// Crossed out font.
        const StrikeOut = nappgui_sys::_fstyle_t_ekFSTRIKEOUT as u32;
        /// Underlined font.
        const Underlined = nappgui_sys::_fstyle_t_ekFUNDERLINE as u32;
        /// Subscript.
        const Subscript = nappgui_sys::_fstyle_t_ekFSUBSCRIPT as u32;
        /// Superscript.
        const Superscript = nappgui_sys::_fstyle_t_ekFSUPSCRIPT as u32;
        /// Font sizes will be indicated in pixels.
        const Points = nappgui_sys::_fstyle_t_ekFPOINTS as u32;
        /// Font sizes will refer to cell height and not character height.
        const Cell = nappgui_sys::_fstyle_t_ekFCELL as u32;
    }
}
