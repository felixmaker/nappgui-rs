use nappgui_sys::{
    window_OnClose, window_OnMoved, window_OnResize, window_clear_hotkeys, window_client_to_screen,
    window_control_frame, window_create, window_cursor, window_cycle_tabstop, window_defbutton,
    window_destroy, window_focus, window_get_client_size, window_get_focus, window_get_origin,
    window_get_size, window_hide, window_hotkey, window_is_visible, window_modal,
    window_next_tabstop, window_origin, window_overlay, window_panel, window_previous_tabstop,
    window_show, window_size, window_stop_modal, window_title, window_update, R2Df, S2Df, V2Df,
};
use std::ffi::CString;
use std::rc::Rc;

use crate::core::event::Event;
use crate::draw_2d::Image;
use crate::prelude::{GuiCursor, GuiFocus, WindowFlag};
use crate::util::macros::{callback, impl_ptr, listener};

use super::*;

/// Window objects are the highest-level containers within the user interface.
pub struct Window {
    pub(crate) inner: Rc<*mut nappgui_sys::Window>,
}

impl Window {
    impl_ptr!(nappgui_sys::Window);

    /// Create a new window.
    pub fn create(flags: WindowFlag) -> Self {
        let window = unsafe { window_create(flags as u32) };
        Self::new(window)
    }

    /// Destroy the window and all its contents.
    ///
    /// # Remarks
    /// Panels, layouts and components will be recursively destroyed.
    pub unsafe fn destroy(self) {
        unsafe {
            let window = self.as_ptr().cast();
            window_destroy(window);
        }
    }

    /// Associate the main panel with a window.
    ///
    /// # Remarks
    /// The size of the window will be adjusted based on the Natural sizing of the main panel.
    pub fn panel(&self, panel: &Panel) {
        unsafe { window_panel(self.as_ptr(), panel.as_ptr()) }
    }

    callback! {
        /// Set an event handler for the window closing.
        pub on_close(Window) => window_OnClose;

        /// Set an event handler for moving the window on the desktop.
        pub on_moved(Window) => window_OnMoved;

        /// Set an event handler for window resizing.
        pub on_resize(Window) => window_OnResize;
    }

    /// Set the text that will display the window in the title bar.
    pub fn title(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { window_title(self.as_ptr(), text.as_ptr()) }
    }

    /// Show the window. By default windows are created hidden. You have to show them explicitly.
    pub fn show(&self) {
        unsafe { window_show(self.as_ptr()) }
    }

    /// Hide the window.
    pub fn hide(&self) {
        unsafe { window_hide(self.as_ptr()) }
    }

    /// Launch an overlay window.
    pub fn overlay(&self, parent: &Window) {
        unsafe {
            window_overlay(self.as_ptr(), parent.as_ptr());
        }
    }

    /// Launch a window in modal mode.
    pub fn modal(&self, parent: &Window) {
        unsafe {
            window_modal(self.as_ptr(), parent.as_ptr());
        }
    }

    /// Ends the modal cycle of a window.
    pub fn stop_modal(&self, return_value: u32) {
        unsafe {
            window_stop_modal(self.as_ptr(), return_value);
        }
    }

    /// Returns whether or not the window is visible.
    pub fn is_visible(&self) -> bool {
        unsafe { window_is_visible(self.as_ptr()) != 0 }
    }

    /// Sets an action associated with pressing a key.
    pub fn hotkey<F>(&self, key: nappgui_sys::_key_t, modifiers: u32, handler: F)
    where
        F: FnMut(&mut Window, &Event) + 'static,
    {
        let listener = listener!(self.as_ptr(), handler, Window);
        unsafe {
            window_hotkey(self.as_ptr(), key, modifiers, listener);
        }
    }

    /// Removes all keyboard shortcuts associated with the window.
    pub fn clear_hotkeys(&self) {
        unsafe { window_clear_hotkeys(self.as_ptr()) }
    }

    /// Activate or deactivate the cyclic behavior of tabstops.
    pub fn cycle_tabstop(&self, cycle: bool) {
        let cycle = cycle as i8;
        unsafe { window_cycle_tabstop(self.as_ptr(), cycle) }
    }

    /// Moves keyboard focus to the next control in the tab-list. It has the same effect as pressing \[TAB\].
    pub fn next_tabstop(&self) -> GuiFocus {
        unsafe { window_next_tabstop(self.as_ptr()) }
    }

    /// Moves the keyboard focus to the previous control in the tab-list. This has the same effect as pressing \[SHIFT\]+\[TAB\].
    pub fn previous_tabstop(&self) -> GuiFocus {
        unsafe { window_previous_tabstop(self.as_ptr()) }
    }

    /// Set keyboard focus to a specific control.
    pub fn focus(&self, control: &Control) -> GuiFocus {
        unsafe { window_focus(self.as_ptr(), control.as_ptr()) }
    }

    /// Gets the control that keyboard focus has.
    pub fn get_focus(&self) -> Control {
        let control = unsafe { window_get_focus(self.as_ptr()) };
        Control::new(control)
    }

    /// Gets additional information about a keyboard focus change operation.
    ///
    /// # Remarks
    /// Sometimes the decision to release keyboard focus for a control requires context information.
    /// For example, what action caused the change (press \[TAB\], click on another control) or
    /// what control will receive the focus.
    pub fn focus_info(&self) {
        unimplemented!()
    }

    /// Recalculate the position and size of the controls after modifying any Layout.
    pub fn update(&self) {
        unsafe { window_update(self.as_ptr()) }
    }

    /// Move the window to specific desktop coordinates.
    pub fn origin(&self, origin: V2Df) {
        unsafe {
            window_origin(self.as_ptr(), origin);
        }
    }

    /// Set the size of the client area of the window.
    ///
    /// # Remarks
    /// The final size will depend on the window frame and desktop theme settings. This measure only refers to the interior area.
    pub fn size(&self, size: S2Df) {
        unsafe {
            window_size(self.as_ptr(), size);
        }
    }

    /// Get the window position.
    pub fn get_origin(&self) -> V2Df {
        unsafe { window_get_origin(self.as_ptr()) }
    }

    /// Get the total dimensions of the window.
    pub fn get_size(&self) -> S2Df {
        unsafe { window_get_size(self.as_ptr()) }
    }

    /// Get the dimensions of the client area of the window.
    pub fn get_client_size(&self) -> S2Df {
        unsafe { window_get_client_size(self.as_ptr()) }
    }

    /// Gets the position and size of a control in window coordinates.
    ///
    /// # Remarks
    ///
    /// control must belong to the window, be active and visible. The point (0,0) corresponds to the upper left vertex of the client area of the window.
    pub fn control_frame(&self, control: &Control) -> R2Df {
        unsafe { window_control_frame(self.as_ptr(), control.as_ptr()) }
    }

    /// Transforms a point expressed in window coordinates to screen coordinates.
    pub fn client_to_screen(&self, point: V2Df) -> V2Df {
        unsafe { window_client_to_screen(self.as_ptr(), point) }
    }

    /// Set the default window button. It will be activated when pressed \[Intro\].
    ///
    /// # Remarks
    ///
    /// This function disables the possible previous default button. For the new button to be set,
    /// it must exist in the active layout, which requires this function to be called after window_panel
    pub fn defbutton(&self, button: &Button) {
        unsafe {
            window_defbutton(self.as_ptr(), button.as_ptr());
        }
    }

    /// Change the mouse cursor.
    ///
    /// # Remarks
    ///
    /// hot_x, hot_y indicate the "sensitive" point within the image, which will indicate the exact position of the mouse.
    pub fn cursor(&self, cursor: GuiCursor, image: &Image, hot_x: f32, hot_y: f32) {
        unsafe { window_cursor(self.as_ptr(), cursor, image.as_ptr(), hot_x, hot_y) }
    }
}
