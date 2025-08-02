use nappgui_sys::{
    view_OnAcceptFocus, view_OnClick, view_OnDown, view_OnDrag, view_OnDraw, view_OnEnter,
    view_OnExit, view_OnFocus, view_OnKeyDown, view_OnKeyUp, view_OnMove, view_OnOverlay,
    view_OnResignFocus, view_OnScroll, view_OnSize, view_OnUp, view_OnWheel, view_allow_tab,
    view_content_size, view_create, view_custom, view_get_size, view_native, view_point_scale,
    view_scroll, view_scroll_size, view_scroll_visible, view_scroll_x, view_scroll_y, view_size,
    view_update, view_viewport, S2Df, V2Df,
};

use crate::{
    gui::{
        control::impl_control,
        event::{EvDraw, EvKey, EvMouse, EvScroll, EvSize},
    },
    util::macros::callback,
};

/// The view trait.
pub trait ViewTrait {
    /// Returns a raw pointer to the view object.
    fn as_ptr(&self) -> *mut nappgui_sys::View;

    /// Set the default view size.
    fn size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { view_size(self.as_ptr(), size) }
    }

    callback! {
        /// Set an event handler to draw in the view.
        on_draw(EvDraw) => view_OnDraw;

        /// Sets an event handler to draw the overlay.
        on_overlay(EvDraw) => view_OnOverlay;

        /// Set an event handler for resizing.
        on_size(EvSize) => view_OnSize;

        /// Set an event handler for mouse enter.
        on_enter(EvMouse) => view_OnEnter;

        /// Set an event handle for mouse exit.
         on_exit(EvMouse) => view_OnExit;

        /// Set an event handler for mouse movement.
         on_move(EvMouse) => view_OnMove;

        /// Sets an event handler for a mouse button down.
         on_down(EvMouse) => view_OnDown;

        /// Sets an event handler for a mouse button up.
         on_up(EvMouse) => view_OnUp;

        /// Set an event handler for mouse click.
         on_click(EvMouse) => view_OnClick;

        /// Set an event handler for mouse drag.
         on_drag(EvMouse) => view_OnDrag;

        /// Set an event handler for mouse wheel.
        on_wheel(EvMouse) => view_OnWheel;

        /// Set an event handler for a keystroke.
        on_key_down(EvKey) => view_OnKeyDown;

        /// Set an event handler for releasing a key.
        on_key_up(EvKey) => view_OnKeyUp;

        /// Sets an event handler for keyboard focus.
        on_focus(bool) => view_OnFocus;

        /// Set a handler to avoid losing keyboard focus.
        on_accept_focus() -> bool => view_OnAcceptFocus;

        /// Set a handler to prevent getting keyboard focus.
        on_resign_focus() -> bool => view_OnResignFocus;

        /// Set an event handler for the scroll bars.
        on_scroll(EvScroll) -> f32 => view_OnScroll;
    }

    /// Allows to capture the press of the \[TAB\] key.
    ///
    /// # Remarks
    /// If TRUE the pressing of \[TAB\] with the keyboard focus in the view will be captured as a KeyDown
    /// event and not as navigation between the controls. The call to this function will have no effect
    /// if there is no associated OnKeyDown handler. In general, you should not use this function.
    fn allow_tab(&self, allow: bool) {
        unsafe { view_allow_tab(self.as_ptr(), allow as _) }
    }

    /// Gets the current size of the view.
    fn get_size(&self) -> S2Df {
        let mut size = S2Df {
            width: 0.0,
            height: 0.0,
        };
        unsafe { view_get_size(self.as_ptr(), &mut size) };
        size
    }

    /// Set the size of the drawing area when scroll bars exist.
    ///
    /// # Remarks
    /// When creating a scroll view, this method indicates the entire drawing area. The control
    /// will use it to size and position the scroll bars.
    fn content_size(&self, size: S2Df, line: S2Df) {
        unsafe {
            view_content_size(self.as_ptr(), size, line);
        }
    }

    /// Move the horizontal scroll bar to the indicated position.
    fn scroll_horizontal(&self, pos: f32) {
        unsafe {
            view_scroll_x(self.as_ptr(), pos);
        }
    }

    /// Move the vertical scroll bar to the indicated position.
    fn scroll_vertical(&self, pos: f32) {
        unsafe {
            view_scroll_y(self.as_ptr(), pos);
        }
    }

    /// Gets the measurements of the scroll bars.
    fn scroll_size(&self) -> (f32, f32) {
        let mut height = 0f32;
        let mut width = 0f32;
        unsafe {
            view_scroll_size(self.as_ptr(), &mut width, &mut height);
        }
        (width, height)
    }

    /// Show or hide the scroll bars.
    fn scroll_visible(&self, horizontal: bool, vertical: bool) {
        unsafe {
            view_scroll_visible(self.as_ptr(), horizontal as _, vertical as _);
        }
    }

    /// Gets the dimensions of the visible area of the view.
    fn viewport(&self) -> (V2Df, S2Df) {
        let mut pos = V2Df { x: 0.0, y: 0.0 };
        let mut size = S2Df {
            width: 0.0,
            height: 0.0,
        };
        unsafe {
            view_viewport(self.as_ptr(), &mut pos, &mut size);
        }
        (pos, size)
    }

    /// Gets the scaling of the point.
    fn point_scale(&self) -> f32 {
        let mut scale = 0f32;
        unsafe {
            view_point_scale(self.as_ptr(), &mut scale);
        }
        scale
    }

    /// Send an order to the operating system that the view should be refreshed.
    fn update(&self) {
        unsafe {
            view_update(self.as_ptr());
        }
    }

    /// Gets a pointer to the native control.
    ///
    /// # Remarks
    /// Do not use this function if you do not know very well what you are doing.
    fn native(&self) -> *mut std::ffi::c_void {
        unsafe { view_native(self.as_ptr()) }
    }
}

/// The View controls or custom views are blank areas within the window that allow us
/// to implement our own components.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the window object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub(crate) inner: *mut nappgui_sys::View,
}

impl ViewTrait for View {
    fn as_ptr(&self) -> *mut nappgui_sys::View {
        self.inner
    }
}

impl View {
    /// Create a new custom view.
    pub fn new() -> Self {
        let view = unsafe { view_create() };
        Self { inner: view }
    }

    /// Create a new custom view with scrollbars.
    pub fn new_scroll() -> Self {
        let view = unsafe { view_scroll() };
        Self { inner: view }
    }

    /// Create a new view with all the options.
    pub fn new_custom(hscroll: bool, vscroll: bool) -> Self {
        let view = unsafe { view_custom(hscroll as _, vscroll as _) };
        Self { inner: view }
    }
}

impl_control!(View, guicontrol_view);
