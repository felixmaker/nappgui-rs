use nappgui_sys::{
    window_OnClose, window_OnMoved, window_OnResize, window_clear_hotkeys, window_client_to_screen,
    window_control_frame, window_create, window_cursor, window_cycle_tabstop, window_defbutton,
    window_destroy, window_focus, window_focus_info, window_get_client_size, window_get_focus,
    window_get_origin, window_get_size, window_hide, window_hotkey, window_is_visible,
    window_modal, window_next_tabstop, window_origin, window_overlay, window_panel,
    window_previous_tabstop, window_show, window_size, window_stop_modal, window_title,
    window_update, S2Df, V2Df,
};
use std::ffi::CString;
use std::rc::{Rc, Weak};

use crate::draw_2d::ImageTrait;
use crate::gui::event::{EvPos, EvSize, EvWinClose};
use crate::gui::{ButtonTrait, ControlTrait, PanelTrait};
use crate::types::{
    FocusInfo, GuiClose, GuiCursor, GuiFocus, GuiTab, KeyCode, ModifierKey, Point2D, Rect2D,
    Size2D, WindowFlags,
};
use crate::util::macros::{callback, listener};

/// Window objects are the highest-level containers within the user interface.
pub struct Window {
    pub(crate) inner: Rc<WindowInner>,
}

impl Window {
    /// Create a new window.
    pub fn new(flag: WindowFlags) -> Self {
        let result = flag.to_window_flag_t();
        let window = unsafe { window_create(result as u32) };
        Window {
            inner: Rc::new(WindowInner { inner: window }),
        }
    }

    /// Get the weak reference of the window.
    pub fn as_weak(&self) -> Weak<WindowInner> {
        Rc::downgrade(&self.inner)
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new(WindowFlags::default())
    }
}

impl Drop for Window {
    /// Destroy the window and all its contents.
    ///
    /// # Remarks
    /// Panels, layouts and components will be recursively destroyed.
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) == 1 {
            unsafe { window_destroy(&mut self.as_ptr()) }
        }
    }
}

impl WindowTrait for Window {
    fn as_ptr(&self) -> *mut nappgui_sys::Window {
        self.inner.inner
    }
}

/// Window Weak Reference
pub type WeakWindow = Weak<WindowInner>;

impl WindowTrait for WeakWindow {
    fn as_ptr(&self) -> *mut nappgui_sys::Window {
        let window = self.upgrade().unwrap();
        window.inner
    }
}

/// Wrappers to Window object.
#[repr(transparent)]
pub struct WindowInner {
    pub(crate) inner: *mut nappgui_sys::Window,
}

/// The Window Trait
pub trait WindowTrait {
    /// Returns the raw pointer of Window object
    fn as_ptr(&self) -> *mut nappgui_sys::Window;

    /// Associate the main panel with a window.
    ///
    /// # Remarks
    /// The size of the window will be adjusted based on the Natural sizing of the main panel.
    fn panel<T>(&self, panel: T)
    where
        T: PanelTrait,
    {
        unsafe { window_panel(self.as_ptr(), panel.as_ptr()) }
    }

    callback! {
        /// Set an event handler for the window closing.
        on_close(EvWinClose) -> bool => window_OnClose;

        /// Set an event handler for moving the window on the desktop.
        on_moved(EvPos) => window_OnMoved;

        /// Set an event handler for window resizing.
        on_resize(EvSize) => window_OnResize;
    }

    /// Set the text that will display the window in the title bar.
    fn title(&self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { window_title(self.as_ptr(), text.as_ptr()) }
    }

    /// Show the window. By default windows are created hidden. You have to show them explicitly.
    fn show(&self) {
        unsafe { window_show(self.as_ptr()) }
    }

    /// Hide the window.
    fn hide(&self) {
        unsafe { window_hide(self.as_ptr()) }
    }

    /// Launch an overlay window.
    fn overlay<T>(&self, parent: &T)
    where
        T: WindowTrait,
    {
        unsafe {
            window_overlay(self.as_ptr(), parent.as_ptr());
        }
    }

    /// Launch a window in modal mode.
    fn modal<T>(&self, parent: &T) -> GuiClose
    where
        T: WindowTrait,
    {
        let value = unsafe { window_modal(self.as_ptr(), parent.as_ptr()) };
        GuiClose::try_from(value as i32).unwrap()
    }

    /// Ends the modal cycle of a window.
    fn stop_modal(&self, return_value: GuiClose) {
        unsafe {
            window_stop_modal(self.as_ptr(), return_value as u32);
        }
    }

    /// Returns whether or not the window is visible.
    fn is_visible(&self) -> bool {
        unsafe { window_is_visible(self.as_ptr()) != 0 }
    }

    /// Sets an action associated with pressing a key.
    fn hotkey<F>(&self, key: KeyCode, modifiers: ModifierKey, handler: F)
    where
        F: FnMut() + 'static,
    {
        let listener = listener!(handler, ());
        unsafe {
            window_hotkey(self.as_ptr(), key as _, modifiers as _, listener);
        }
    }

    /// Removes all keyboard shortcuts associated with the window.
    fn clear_hotkeys(&self) {
        unsafe { window_clear_hotkeys(self.as_ptr()) }
    }

    /// Activate or deactivate the cyclic behavior of tabstops.
    fn cycle_tabstop(&self, cycle: bool) {
        let cycle = cycle as _;
        unsafe { window_cycle_tabstop(self.as_ptr(), cycle) }
    }

    /// Moves keyboard focus to the next control in the tab-list. It has the same effect as pressing \[TAB\].
    fn next_tabstop(&self) -> GuiFocus {
        let focus = unsafe { window_next_tabstop(self.as_ptr()) };
        GuiFocus::try_from(focus).unwrap()
    }

    /// Moves the keyboard focus to the previous control in the tab-list. This has the same effect as pressing \[SHIFT\]+\[TAB\].
    fn previous_tabstop(&self) -> GuiFocus {
        let focus = unsafe { window_previous_tabstop(self.as_ptr()) };
        GuiFocus::try_from(focus).unwrap()
    }

    /// Set keyboard focus to a specific control.
    fn focus<T>(&self, control: &T) -> GuiFocus
    where
        T: ControlTrait,
    {
        let focus = unsafe { window_focus(self.as_ptr(), control.as_control_ptr()) };
        GuiFocus::try_from(focus).unwrap()
    }

    /// Gets the control that keyboard focus has.
    fn get_focus<T>(&self) -> Option<T>
    where
        T: ControlTrait,
    {
        let control = unsafe { window_get_focus(self.as_ptr()) };
        T::from_control_ptr(control)
    }

    /// Gets additional information about a keyboard focus change operation.
    ///
    /// # Remarks
    /// Sometimes the decision to release keyboard focus for a control requires context information.
    /// For example, what action caused the change (press \[TAB\], click on another control) or
    /// what control will receive the focus.
    fn focus_info(&self) -> FocusInfo {
        let mut raw_info = nappgui_sys::FocusInfo {
            action: nappgui_sys::_gui_tab_t_ekGUI_TAB_KEY,
            next: std::ptr::null_mut() as _,
        };

        unsafe { window_focus_info(self.as_ptr(), &mut raw_info) };

        FocusInfo {
            action: GuiTab::try_from(raw_info.action).unwrap(),
            next: raw_info.next,
        }
    }

    /// Recalculate the position and size of the controls after modifying any Layout.
    fn update(&self) {
        unsafe { window_update(self.as_ptr()) }
    }

    /// Move the window to specific desktop coordinates.
    fn origin(&self, x: f32, y: f32) {
        unsafe {
            window_origin(self.as_ptr(), V2Df { x, y });
        }
    }

    /// Set the size of the client area of the window.
    ///
    /// # Remarks
    /// The final size will depend on the window frame and desktop theme settings. This measure only refers to the interior area.
    fn size(&self, width: f32, height: f32) {
        unsafe {
            window_size(self.as_ptr(), S2Df { width, height });
        }
    }

    /// Get the window position.
    fn get_origin(&self) -> Point2D {
        unsafe {
            let origin = window_get_origin(self.as_ptr());
            std::mem::transmute(origin)
        }
    }

    /// Get the total dimensions of the window.
    fn get_size(&self) -> Size2D {
        unsafe {
            let size = window_get_size(self.as_ptr());
            std::mem::transmute(size)
        }
    }

    /// Get the dimensions of the client area of the window.
    fn get_client_size(&self) -> Size2D {
        unsafe {
            let size = window_get_client_size(self.as_ptr());
            std::mem::transmute(size)
        }
    }

    /// Gets the position and size of a control in window coordinates.
    ///
    /// # Remarks
    ///
    /// control must belong to the window, be active and visible. The point (0,0) corresponds to the upper left vertex of the client area of the window.
    fn control_frame<T>(&self, control: &T) -> Rect2D
    where
        T: ControlTrait,
    {
        unsafe {
            let rect = window_control_frame(self.as_ptr(), control.as_control_ptr());
            std::mem::transmute(rect)
        }
    }

    /// Transforms a point expressed in window coordinates to screen coordinates.
    fn client_to_screen(&self, x: f32, y: f32) -> Point2D {
        unsafe {
            let pos = window_client_to_screen(self.as_ptr(), V2Df { x, y });
            std::mem::transmute(pos)
        }
    }

    /// Set the default window button. It will be activated when pressed \[Intro\].
    ///
    /// # Remarks
    ///
    /// This function disables the possible previous default button. For the new button to be set,
    /// it must exist in the active layout, which requires this function to be called after window_panel
    fn defbutton<T>(&self, button: &T)
    where
        T: ButtonTrait,
    {
        unsafe {
            window_defbutton(self.as_ptr(), button.as_ptr());
        }
    }

    /// Change the mouse cursor.
    ///
    /// # Remarks
    ///
    /// hot_x, hot_y indicate the "sensitive" point within the image, which will indicate the exact position of the mouse.
    fn cursor<T>(&self, cursor: GuiCursor, image: &T, hot_x: f32, hot_y: f32)
    where
        T: ImageTrait,
    {
        unsafe { window_cursor(self.as_ptr(), cursor as i32, image.as_ptr(), hot_x, hot_y) }
    }
}
