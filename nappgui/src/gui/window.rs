use nappgui_sys::{
    gui_focus_t, listener_imp, window_OnClose, window_OnMoved, window_OnResize,
    window_clear_hotkeys, window_client_to_screen, window_control_frame, window_create,
    window_cursor, window_cycle_tabstop, window_defbutton, window_destroy, window_flag_t,
    window_focus, window_focus_info, window_get_client_size, window_get_focus, window_get_origin,
    window_get_size, window_hide, window_hotkey, window_imp, window_is_visible, window_modal,
    window_next_tabstop, window_origin, window_overlay, window_panel, window_previous_tabstop,
    window_show, window_size, window_stop_modal, window_title, window_update,
};
use std::ffi::{c_void, CString};

use super::control::Control;
use super::panel::Panel;

/// Window objects are the highest-level containers within the user interface
pub struct Window {
    pub(crate) inner: *mut nappgui_sys::Window,
}

impl Window {
    pub(crate) fn new(ptr: *mut nappgui_sys::Window) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new window.
    pub fn create(flags: window_flag_t) -> Self {
        let window = unsafe { window_create(flags as u32) };
        Self::new(window)
    }

    /// Destroy the window and all its contents.
    ///
    /// # Remarks
    /// Panels, layouts and components will be recursively destroyed.
    pub fn destroy(self) {
        unsafe {
            let window = self.inner.cast();
            window_destroy(window);
        }
    }

    /// Associate the main panel with a window.
    ///
    /// # Remarks
    /// The size of the window will be adjusted based on the Natural sizing of the main panel.
    pub fn panel(&self, panel: &Panel) {
        unsafe { window_panel(self.inner, panel.inner) }
    }

    pub fn on_close<F>(&self, handler: F)
    where
        F: FnMut(&mut Window) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window);
            let f = &mut *(*data).0;
            let mut window = Window { inner: (*data).1 };
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut window)));
        }

        let cb: Box<dyn FnMut(&mut Window)> = Box::new(handler);

        let data: *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window) =
            Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            window_OnClose(self.inner, listener_imp(data as *mut c_void, Some(shim)));
        }
    }

    /// Set an event handler for moving the window on the desktop.
    pub fn on_moved<F>(&self, handler: F)
    where
        F: FnMut(&mut Window) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window);
            let f = &mut *(*data).0;
            let mut window = Window { inner: (*data).1 };
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut window)));
        }

        let cb: Box<dyn FnMut(&mut Window)> = Box::new(handler);

        let data: *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window) =
            Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            window_OnMoved(self.inner, listener_imp(data as *mut c_void, Some(shim)));
        }
    }

    /// Set an event handler for window resizing.
    pub fn on_resize<F>(&self, handler: F)
    where
        F: FnMut(&mut Window) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window);
            let f = &mut *(*data).0;
            let mut window = Window { inner: (*data).1 };
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut window)));
        }

        let cb: Box<dyn FnMut(&mut Window)> = Box::new(handler);

        let data: *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window) =
            Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            window_OnResize(self.inner, listener_imp(data as *mut c_void, Some(shim)));
        }
    }

    /// Set the text that will display the window in the title bar.
    pub fn title(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { window_title(self.inner, text.as_ptr()) }
    }

    /// Show the window. By default windows are created hidden. You have to show them explicitly.
    pub fn show(&self) {
        unsafe { window_show(self.inner) }
    }

    /// Hide the window.
    pub fn hide(&self) {
        unsafe { window_hide(self.inner) }
    }

    /// Launch an overlay window.
    pub fn overlay(&self, parent: &Window) {
        unsafe {
            window_overlay(self.inner, parent.inner);
        }
    }

    /// Launch a window in modal mode.
    pub fn modal(&self, parent: &Window) {
        unsafe {
            window_modal(self.inner, parent.inner);
        }
    }

    /// Ends the modal cycle of a window.
    pub fn stop_modal(&self, return_value: u32) {
        unsafe {
            window_stop_modal(self.inner, return_value);
        }
    }

    /// Returns whether or not the window is visible.
    pub fn is_visible(&self) -> bool {
        unsafe { window_is_visible(self.inner) != 0 }
    }

    /// Sets an action associated with pressing a key.
    pub fn hotkey<F>(&self, key: i32, modifiers: u32)
    where
        F: FnMut(&mut Window) + 'static,
    {
        // unsafe extern "C" fn shim(data: *mut c_void, event: *mut nappgui_sys::Event) {
        //     let data = data as *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window);
        //     let f = &mut *(*data).0;
        //     let mut window = Window { inner: (*data).1 };
        //     let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut window)));
        // }

        // let cb: Box<dyn FnMut(&mut Window)> = Box::new(handler);

        // let data: *mut (Box<dyn FnMut(&mut Window)>, *mut nappgui_sys::Window) =
        //     Box::into_raw(Box::new((cb, self.inner)));
        todo!()
        // unsafe { window_hotkey(data, key, modifiers, Some(shim)) }
    }

    /// Removes all keyboard shortcuts associated with the window.
    pub fn clear_hotkeys(&self) {
        unsafe { window_clear_hotkeys(self.inner) }
    }

    /// Activate or deactivate the cyclic behavior of tabstops.
    pub fn cycle_tabstop(&self, cycle: bool) {
        let cycle = cycle as i8;
        unsafe { window_cycle_tabstop(self.inner, cycle) }
    }

    /// Moves keyboard focus to the next control in the tab-list. It has the same effect as pressing [TAB].
    pub fn next_tabstop(&self) -> gui_focus_t {
        unsafe { window_next_tabstop(self.inner) }
    }

    /// Moves the keyboard focus to the previous control in the tab-list. This has the same effect as pressing [SHIFT]+[TAB].
    pub fn previous_tabstop(&self) -> gui_focus_t {
        unsafe { window_previous_tabstop(self.inner) }
    }

    /// Set keyboard focus to a specific control.
    pub fn focus(&self, control: &Control) -> gui_focus_t {
        unsafe { window_focus(self.inner, control.inner) }
    }

    /// Gets the control that keyboard focus has.
    pub fn get_focus(&self) -> Control {
        let control = unsafe { window_get_focus(self.inner) };
        Control::new(control)
    }

    /// Gets additional information about a keyboard focus change operation.
    ///
    /// # Remarks
    /// Sometimes the decision to release keyboard focus for a control requires context information.
    /// For example, what action caused the change (press [TAB], click on another control) or
    /// what control will receive the focus.
    pub fn focus_info(&self) {
        todo!()
    }

    /// Recalculate the position and size of the controls after modifying any Layout.
    pub fn update(&self) {
        unsafe { window_update(self.inner) }
    }

    /// Move the window to specific desktop coordinates.
    pub fn origin(&self, x: i32, y: i32) {
        todo!()
    }

    /// Set the size of the client area of the window.
    ///
    /// # Remarks
    /// The final size will depend on the window frame and desktop theme settings. This measure only refers to the interior area.
    pub fn size(&self, width: u32, height: u32) {
        todo!()
    }

    /// Get the window position.
    pub fn get_origin(&self) -> (i32, i32) {
        todo!()
    }

    /// Get the total dimensions of the window.
    pub fn get_size(&self) -> (u32, u32) {
        todo!()
    }

    /// Get the dimensions of the client area of the window.
    pub fn get_client_size(&self) -> (u32, u32) {
        todo!()
    }

    /// Gets the position and size of a control in window coordinates.
    ///
    /// # Remarks
    ///
    /// control must belong to the window, be active and visible. The point (0,0) corresponds to the upper left vertex of the client area of the window.
    pub fn control_frame(&self) -> (i32, i32, u32, u32) {
        todo!()
    }

    /// Transforms a point expressed in window coordinates to screen coordinates.
    pub fn client_to_screen(&self, x: i32, y: i32) -> (i32, i32) {
        todo!()
    }

    /// Set the default window button. It will be activated when pressed [Intro].
    ///
    /// # Remarks
    ///
    /// This function disables the possible previous default button. For the new button to be set,
    /// it must exist in the active layout, which requires this function to be called after window_panel
    pub fn defbutton(&self) {
        todo!()
    }

    /// Change the mouse cursor.
    ///
    /// # Remarks
    ///
    /// hot_x, hot_y indicate the "sensitive" point within the image, which will indicate the exact position of the mouse.
    pub fn cursor(&self) {
        todo!()
    }
}
