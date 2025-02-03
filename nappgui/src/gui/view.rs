use nappgui_sys::{
    view_OnAcceptFocus, view_OnClick, view_OnDown, view_OnDrag, view_OnDraw, view_OnEnter,
    view_OnExit, view_OnFocus, view_OnKeyDown, view_OnKeyUp, view_OnMove, view_OnOverlay,
    view_OnResignFocus, view_OnScroll, view_OnSize, view_OnUp, view_OnWheel, view_allow_tab,
    view_content_size, view_create, view_custom, view_data_imp, view_get_data_imp, view_get_size,
    view_keybuf, view_native, view_point_scale, view_scroll, view_scroll_size, view_scroll_visible,
    view_scroll_x, view_scroll_y, view_size, view_update, view_viewport,
};

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

    /// Set an event handler to draw in the view.
    pub fn on_draw<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Sets an event handler to draw the overlay.
    pub fn on_overlay<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for resizing.
    pub fn on_size<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for mouse enter.
    pub fn on_enter<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handle for mouse exit.
    pub fn on_exit<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for mouse movement.
    pub fn on_move<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Sets an event handler for a mouse button down.
    pub fn on_down<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Sets an event handler for a mouse button up.
    pub fn on_up<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for mouse click.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for mouse drag.
    pub fn on_drag<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for mouse wheel.
    pub fn on_wheel<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for a keystroke.
    pub fn on_key_down<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for releasing a key.
    pub fn on_key_up<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Sets an event handler for keyboard focus.
    pub fn on_focus<F>(&self, handler: F)
    where
        F: FnMut(&mut View, bool) + 'static,
    {
        todo!();
    }

    /// Set a handler to avoid losing keyboard focus.
    pub fn on_accept_focus<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set a handler to prevent getting keyboard focus.
    pub fn on_resign_focus<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
    }

    /// Set an event handler for the scroll bars.
    pub fn on_scroll<F>(&self, handler: F)
    where
        F: FnMut(&mut View) + 'static,
    {
        todo!();
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

    /// Sets a keyboard buffer for synchronous or asynchronous query of key state.
    ///
    /// # Remarks
    /// It just keeps a reference to the buffer, which will need to be destroyed by the object that created it.
    /// See Keyboard buffer. The application will still be able to receive keyboard events through
    /// view_OnKeyDown and view_OnKeyUp.
    pub fn keybuf(&self, keybuf: &str) {
        todo!();
    }

    /// Gets the current size of the view.
    pub fn get_size(&self) -> (f32, f32) {
        todo!();
    }

    /// Set the size of the drawing area when scroll bars exist.
    ///
    /// # Remarks
    /// When creating a scroll view, this method indicates the entire drawing area. The control
    /// will use it to size and position the scroll bars.
    pub fn content_size(&self, width: f32, height: f32) {
        todo!()
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
        todo!();
    }

    /// Show or hide the scroll bars.
    pub fn scroll_visible(&self, horizontal: bool, vertical: bool) {
        unsafe {
            view_scroll_visible(self.inner, horizontal as i8, vertical as i8);
        }
    }

    /// Gets the dimensions of the visible area of the view.
    pub fn viewport(&self) -> (f32, f32) {
        todo!();
    }

    /// Gets the scaling of the point.
    pub fn point_scale(&self) -> f32 {
        todo!();
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
        unsafe {
            view_native(self.inner)
        }
    }
}
