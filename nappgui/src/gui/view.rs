use std::{
    cell::RefCell,
    ffi::CString,
    ptr::NonNull,
    rc::{Rc, Weak},
};

use nappgui_sys::{
    view_OnAcceptFocus, view_OnClick, view_OnDown, view_OnDrag, view_OnDraw, view_OnEnter, view_OnExit, view_OnFocus,
    view_OnKeyDown, view_OnKeyUp, view_OnMove, view_OnOverlay, view_OnResignFocus, view_OnScroll, view_OnSize,
    view_OnUp, view_OnWheel, view_allow_tab, view_content_size, view_create, view_custom, view_get_size, view_native,
    view_point_scale, view_scroll, view_scroll_size, view_scroll_visible, view_scroll_x, view_scroll_y, view_size,
    view_tooltip, view_update, view_viewport, S2Df, V2Df,
};

use crate::{
    gui::{
        event::{EvDraw, EvKey, EvMouse, EvScroll, EvSize},
        global_get, global_record,
    },
    util::macros::listener,
};

pub(crate) struct ViewInner {
    ptr: NonNull<nappgui_sys::View>,
    on_draw: RefCell<Option<Rc<dyn Fn(&EvDraw) + 'static>>>,
    on_overlay: RefCell<Option<Rc<dyn Fn(&EvDraw) + 'static>>>,
    on_size: RefCell<Option<Rc<dyn Fn(&EvSize) + 'static>>>,
    on_enter: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_exit: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_move: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_down: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_up: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_click: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_drag: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_wheel: RefCell<Option<Rc<dyn Fn(&EvMouse) + 'static>>>,
    on_key_down: RefCell<Option<Rc<dyn Fn(&EvKey) + 'static>>>,
    on_key_up: RefCell<Option<Rc<dyn Fn(&EvKey) + 'static>>>,
    on_focus: RefCell<Option<Rc<dyn Fn(&bool) + 'static>>>,
    on_accept_focus: RefCell<Option<Rc<dyn Fn() -> bool + 'static>>>,
    on_resign_focus: RefCell<Option<Rc<dyn Fn() -> bool + 'static>>>,
    on_scroll: RefCell<Option<Rc<dyn Fn(&EvScroll) -> f32 + 'static>>>,
}

impl ViewInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::View) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to ViewInner::from_raw"),
            on_draw: RefCell::new(None),
            on_overlay: RefCell::new(None),
            on_size: RefCell::new(None),
            on_enter: RefCell::new(None),
            on_exit: RefCell::new(None),
            on_move: RefCell::new(None),
            on_down: RefCell::new(None),
            on_up: RefCell::new(None),
            on_click: RefCell::new(None),
            on_drag: RefCell::new(None),
            on_wheel: RefCell::new(None),
            on_key_down: RefCell::new(None),
            on_key_up: RefCell::new(None),
            on_focus: RefCell::new(None),
            on_accept_focus: RefCell::new(None),
            on_resign_focus: RefCell::new(None),
            on_scroll: RefCell::new(None),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::View {
        self.ptr.as_ptr()
    }
}

/// The view control.
///
/// # Remarks
/// If the object is not attached to a window, it will cause a memory leak.
#[repr(transparent)]
pub struct View(Weak<ViewInner>);

impl View {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::View) -> Self {
        let object = global_record(ptr as _, ViewInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) unsafe fn from_ptr(ptr: *mut nappgui_sys::View) -> Self {
        let object = global_get(ptr as _).unwrap();
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::View {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
    }

    /// Create a new custom view.
    pub fn new() -> Self {
        unsafe { View::from_raw(view_create()) }
    }

    /// Create a new custom view with scrollbars.
    pub fn new_scroll() -> Self {
        unsafe { View::from_raw(view_scroll()) }
    }

    /// Create a new view with all the options.
    pub fn new_custom(hscroll: bool, vscroll: bool) -> Self {
        unsafe { View::from_raw(view_custom(hscroll as _, vscroll as _)) }
    }

    /// Set the default view size.
    pub fn set_size(&self, width: f32, height: f32) {
        let size = nappgui_sys::S2Df { width, height };
        unsafe { view_size(self.as_ptr(), size) }
    }

    /// Set an event handler to draw in the view.
    pub fn set_on_draw_handler<F>(&self, handler: F)
    where
        F: Fn(&EvDraw) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_draw.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_draw(EvDraw));
        unsafe { view_OnDraw(self.as_ptr(), listener) }
    }

    /// Set an event handler to draw the overlay.
    pub fn set_on_overlay_handler<F>(&self, handler: F)
    where
        F: Fn(&EvDraw) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_overlay.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_overlay(EvDraw));
        unsafe { view_OnOverlay(self.as_ptr(), listener) }
    }

    /// Set an event handler for resizing.
    pub fn set_on_size_handler<F>(&self, handler: F)
    where
        F: Fn(&EvSize) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_size.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_size(EvSize));
        unsafe { view_OnSize(self.as_ptr(), listener) }
    }

    /// Set an event handler for mouse enter.
    pub fn set_on_enter_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_enter.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_enter(EvMouse));
        unsafe { view_OnEnter(self.as_ptr(), listener) }
    }

    /// Set an event handler for mouse exit.
    pub fn set_on_exit_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_exit.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_exit(EvMouse));
        unsafe { view_OnExit(self.as_ptr(), listener) }
    }

    /// Set an event handler for mouse movement.
    pub fn set_on_move_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_move.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_move(EvMouse));
        unsafe { view_OnMove(self.as_ptr(), listener) }
    }

    /// Set an event handler for a mouse button down.
    pub fn set_on_down_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_down.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_down(EvMouse));
        unsafe { view_OnDown(self.as_ptr(), listener) }
    }

    /// Set an event handler for a mouse button up.
    pub fn set_on_up_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_up.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_up(EvMouse));
        unsafe { view_OnUp(self.as_ptr(), listener) }
    }

    /// Set an event handler for mouse click.
    pub fn set_on_click_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_click.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_click(EvMouse));
        unsafe { view_OnClick(self.as_ptr(), listener) }
    }

    /// Set an event handler for mouse drag.
    pub fn set_on_drag_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_drag.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_drag(EvMouse));
        unsafe { view_OnDrag(self.as_ptr(), listener) }
    }

    /// Set an event handler for mouse wheel.
    pub fn set_on_wheel_handler<F>(&self, handler: F)
    where
        F: Fn(&EvMouse) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_wheel.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_wheel(EvMouse));
        unsafe { view_OnWheel(self.as_ptr(), listener) }
    }

    /// Set an event handler for a keystroke.
    pub fn set_on_key_down_handler<F>(&self, handler: F)
    where
        F: Fn(&EvKey) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_key_down.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_key_down(EvKey));
        unsafe { view_OnKeyDown(self.as_ptr(), listener) }
    }

    /// Set an event handler for releasing a key.
    pub fn set_on_key_up_handler<F>(&self, handler: F)
    where
        F: Fn(&EvKey) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_key_up.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_key_up(EvKey));
        unsafe { view_OnKeyUp(self.as_ptr(), listener) }
    }

    /// Set an event handler for keyboard focus.
    pub fn set_on_focus_handler<F>(&self, handler: F)
    where
        F: Fn(&bool) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_focus.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_focus(bool));
        unsafe { view_OnFocus(self.as_ptr(), listener) }
    }

    /// Set a handler to avoid losing keyboard focus.
    pub fn set_on_accept_focus_handler<F>(&self, handler: F)
    where
        F: Fn() -> bool + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_accept_focus.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_accept_focus());
        unsafe { view_OnAcceptFocus(self.as_ptr(), listener) }
    }

    /// Set a handler to prevent getting keyboard focus.
    pub fn set_on_resign_focus_handler<F>(&self, handler: F)
    where
        F: Fn() -> bool + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_resign_focus.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_resign_focus());
        unsafe { view_OnResignFocus(self.as_ptr(), listener) }
    }

    /// Set an event handler for the scroll bars.
    pub fn set_on_scroll_handler<F>(&self, handler: F)
    where
        F: Fn(&EvScroll) -> f32 + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_scroll.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), ViewInner, on_scroll(EvScroll));
        unsafe { view_OnScroll(self.as_ptr(), listener) }
    }

    /// Sets a tooltip for the view. It is a small explanatory text that will appear when the mouse is over the control.
    pub fn set_tooltip(&self, tooltip: &str) {
        let tooltip = CString::new(tooltip).unwrap();
        unsafe { view_tooltip(self.as_ptr(), tooltip.as_ptr() as _) };
    }

    /// Allows to capture the press of the \[TAB\] key.
    ///
    /// # Remarks
    /// If TRUE the pressing of \[TAB\] with the keyboard focus in the view will be captured as a KeyDown
    /// event and not as navigation between the controls. The call to this function will have no effect
    /// if there is no associated OnKeyDown handler. In general, you should not use this function.
    pub fn set_allow_tab(&self, allow: bool) {
        unsafe { view_allow_tab(self.as_ptr(), allow as _) }
    }

    /// Gets the current size of the view.
    pub fn content_size(&self) -> S2Df {
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
    pub fn set_content_size(&self, size: S2Df, line: S2Df) {
        unsafe {
            view_content_size(self.as_ptr(), size, line);
        }
    }

    /// Move the horizontal scroll bar to the indicated position.
    pub fn set_scroll_horizontal(&self, pos: f32) {
        unsafe {
            view_scroll_x(self.as_ptr(), pos);
        }
    }

    /// Move the vertical scroll bar to the indicated position.
    pub fn set_scroll_vertical(&self, pos: f32) {
        unsafe {
            view_scroll_y(self.as_ptr(), pos);
        }
    }

    /// Gets the measurements of the scroll bars.
    pub fn scroll_size(&self) -> (f32, f32) {
        let mut height = 0f32;
        let mut width = 0f32;
        unsafe {
            view_scroll_size(self.as_ptr(), &mut width, &mut height);
        }
        (width, height)
    }

    /// Show or hide the scroll bars.
    pub fn set_scroll_visible(&self, horizontal: bool, vertical: bool) {
        unsafe {
            view_scroll_visible(self.as_ptr(), horizontal as _, vertical as _);
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
            view_viewport(self.as_ptr(), &mut pos, &mut size);
        }
        (pos, size)
    }

    /// Gets the scaling of the point.
    pub fn point_scale(&self) -> f32 {
        let mut scale = 0f32;
        unsafe {
            view_point_scale(self.as_ptr(), &mut scale);
        }
        scale
    }

    /// Send an order to the operating system that the view should be refreshed.
    pub fn update(&self) {
        unsafe {
            view_update(self.as_ptr());
        }
    }

    /// Gets a pointer to the native control.
    ///
    /// # Returns
    /// HWND in Windows, GtkWidget in Linux and NSView in macOS.
    ///
    /// # Remarks
    /// Do not use this function if you do not know very well what you are doing.
    pub fn native(&self) -> *mut () {
        unsafe { view_native(self.as_ptr()) as _ }
    }
}
