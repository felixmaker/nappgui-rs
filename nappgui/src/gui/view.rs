use nappgui_sys::{
    view_OnAcceptFocus, view_OnClick, view_OnDown, view_OnDrag, view_OnDraw, view_OnEnter,
    view_OnExit, view_OnFocus, view_OnKeyDown, view_OnKeyUp, view_OnMove, view_OnOverlay,
    view_OnResignFocus, view_OnScroll, view_OnSize, view_OnUp, view_OnWheel, view_allow_tab,
    view_content_size, view_create, view_custom, view_get_size, view_native, view_point_scale,
    view_scroll, view_scroll_size, view_scroll_visible, view_scroll_x, view_scroll_y, view_size,
    view_update, view_viewport, S2Df, V2Df,
};

use crate::callback;

/// The View controls or custom views are blank areas within the window that allow us 
/// to implement our own components. 
pub struct View {
    pub(crate) inner: *mut nappgui_sys::View,
}

impl View {
    pub(crate) fn new(ptr: *mut nappgui_sys::View) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new custom view.
    pub fn create() -> Self {
        let view = unsafe { view_create() };
        Self::new(view)
    }

    /// Create a new custom view with scrollbars.
    pub fn scroll() -> Self {
        let view = unsafe { view_scroll() };
        Self::new(view)
    }

    /// Create a new view with all the options.
    pub fn custom(hscroll: bool, vscroll: bool) -> Self {
        let view = unsafe { view_custom(hscroll as i8, vscroll as i8) };
        Self::new(view)
    }

    /// Set the default view size.
    pub fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { view_size(self.inner, size) }
    }

    callback! {
        /// Set an event handler to draw in the view.
        pub on_draw(View) => view_OnDraw;

        /// Sets an event handler to draw the overlay.
        pub on_overlay(View) => view_OnOverlay;

        /// Set an event handler for resizing.
        pub on_size(View) => view_OnSize;

        /// Set an event handler for mouse enter.
        pub on_enter(View) => view_OnEnter;

        /// Set an event handle for mouse exit.
        pub on_exit(View) => view_OnExit;

        /// Set an event handler for mouse movement.
        pub on_move(View) => view_OnMove;

        /// Sets an event handler for a mouse button down.
        pub on_down(View) => view_OnDown;

        /// Sets an event handler for a mouse button up.
        pub on_up(View) => view_OnUp;

        /// Set an event handler for mouse click.
        pub on_click(View) => view_OnClick;

        /// Set an event handler for mouse drag.
        pub on_drag(View) => view_OnDrag;

        /// Set an event handler for mouse wheel.
        pub on_wheel(View) => view_OnWheel;

        /// Set an event handler for a keystroke.
        pub on_key_down(View) => view_OnKeyDown;

        /// Set an event handler for releasing a key.
        pub on_key_up(View) => view_OnKeyUp;

        /// Sets an event handler for keyboard focus.
        pub on_focus(View) => view_OnFocus;

        /// Set a handler to avoid losing keyboard focus.
        pub on_accept_focus(View) => view_OnAcceptFocus;

        /// Set a handler to prevent getting keyboard focus.
        pub on_resign_focus(View) => view_OnResignFocus;

        /// Set an event handler for the scroll bars.
        pub on_scroll(View) => view_OnScroll;
    }

    /// Allows to capture the press of the [TAB] key.
    ///
    /// # Remarks
    /// If TRUE the pressing of [TAB] with the keyboard focus in the view will be captured as a KeyDown
    /// event and not as navigation between the controls. The call to this function will have no effect
    /// if there is no associated OnKeyDown handler. In general, you should not use this function.
    pub fn allow_tab(&self, allow: bool) {
        unsafe { view_allow_tab(self.inner, allow as i8) }
    }

    /// Gets the current size of the view.
    pub fn get_size(&self) -> S2Df {
        let mut size = S2Df {
            width: 0.0,
            height: 0.0,
        };
        unsafe { view_get_size(self.inner, &mut size) };
        size
    }

    /// Set the size of the drawing area when scroll bars exist.
    ///
    /// # Remarks
    /// When creating a scroll view, this method indicates the entire drawing area. The control
    /// will use it to size and position the scroll bars.
    pub fn content_size(&self, size: S2Df, line: S2Df) {
        unsafe {
            view_content_size(self.inner, size, line);
        }
    }

    /// Move the horizontal scroll bar to the indicated position.
    pub fn scroll_x(&self, pos: f32) {
        unsafe {
            view_scroll_x(self.inner, pos);
        }
    }

    /// Move the vertical scroll bar to the indicated position.
    pub fn scroll_y(&self, pos: f32) {
        unsafe {
            view_scroll_y(self.inner, pos);
        }
    }

    /// Gets the measurements of the scroll bars.
    pub fn scroll_size(&self) -> (f32, f32) {
        let mut height = 0f32;
        let mut width = 0f32;
        unsafe {
            view_scroll_size(self.inner, &mut width, &mut height);
        }
        (width, height)
    }

    /// Show or hide the scroll bars.
    pub fn scroll_visible(&self, horizontal: bool, vertical: bool) {
        unsafe {
            view_scroll_visible(self.inner, horizontal as i8, vertical as i8);
        }
    }

    /// Gets the dimensions of the visible area of the view.
    pub fn viewport(&self) -> (V2Df, S2Df) {
        let mut pos = V2Df { x: 0.0, y: 0.0 };
        let mut size = S2Df {
            width: 0.0,
            height: 0.0,
        };
        unsafe {
            view_viewport(self.inner, &mut pos, &mut size);
        }
        (pos, size)
    }

    /// Gets the scaling of the point.
    pub fn point_scale(&self) -> f32 {
        let mut scale = 0f32;
        unsafe {
            view_point_scale(self.inner, &mut scale);
        }
        scale
    }

    /// Send an order to the operating system that the view should be refreshed.
    pub fn update(&self) {
        unsafe {
            view_update(self.inner);
        }
    }

    /// Gets a pointer to the native control.
    ///
    /// # Remarks
    /// Do not use this function if you do not know very well what you are doing.
    pub fn native(&self) -> *mut std::ffi::c_void {
        unsafe { view_native(self.inner) }
    }
}
