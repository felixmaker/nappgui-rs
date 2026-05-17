use nappgui_sys::{
    comwin_color, comwin_open_file, comwin_save_file, comwin_select_dir, listener_imp, osapp_menubar, window_OnClose,
    window_OnMoved, window_OnResize, window_clear_hotkeys, window_client_size, window_client_to_screen,
    window_control_frame, window_create, window_cursor, window_cycle_tabstop, window_defbutton, window_destroy,
    window_focus, window_focus_info, window_get_client_size, window_get_focus, window_get_maximize,
    window_get_minimize, window_get_origin, window_get_size, window_get_visible, window_hide, window_hotkey,
    window_maximize, window_minimize, window_modal, window_next_tabstop, window_origin, window_overlay, window_panel,
    window_previous_tabstop, window_show, window_stop_modal, window_title, window_update, S2Df, V2Df,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::rc::Rc;

use crate::draw_2d::{Color, Image};
use crate::gui::event::{PositionEvent, SizeEvent, WindowCloseEvent};
use crate::gui::{define_object, listener, Button, Callback, Control, Menu, Panel};
use crate::types::{
    Align, FocusInfo, GuiClose, GuiCursor, GuiFocus, GuiTab, KeyCode, ModifierKey, Point2D, Rect2D, Size2D, WindowFlags,
};

struct HotkeyContext {
    key: KeyCode,
    modifiers: ModifierKey,
    window: Window,
}

#[derive(Default)]
pub(crate) struct WindowProps {
    menu_bar: RefCell<Option<Menu>>,
    default_button: RefCell<Option<Button>>,
    panel: RefCell<Option<Panel>>,
    on_close: Callback<WindowCloseEvent, bool>,
    on_moved: Callback<PositionEvent>,
    on_resize: Callback<SizeEvent>,
    on_hotkey: RefCell<HashMap<(KeyCode, ModifierKey), Rc<dyn Fn() + 'static>>>,
    on_hotkey_context: RefCell<Vec<*mut HotkeyContext>>,
}

define_object!(Window, WindowInner, Window, WindowProps);

impl Drop for WindowProps {
    fn drop(&mut self) {
        for context in self.on_hotkey_context.borrow().iter() {
            unsafe {
                let _ = Box::from_raw(*context);
            };
        }
    }
}

impl Window {
    pub(crate) fn destroy(&self) {
        self.inner(|inner| {
            let mut window = inner.ptr.get();
            unsafe { window_destroy(&mut window) };
            inner.ptr.set(window);
        });
    }
}

impl Window {
    /// Create a new window.
    pub fn new() -> Self {
        unsafe { Self::from_raw(window_create(WindowFlags::default().to_window_flag_t() as u32)) }
    }

    /// Create a new window with a specific flag.
    pub fn new_with_flag(flag: WindowFlags) -> Self {
        unsafe { Self::from_raw(window_create(flag.to_window_flag_t() as u32)) }
    }

    /// Set an event handler for the window closing.
    pub fn set_on_close_handler<F>(&self, handler: F)
    where
        F: Fn(&WindowCloseEvent) -> bool + 'static,
    {
        self.inner(|inner| *inner.props.on_close.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), WindowInner, on_close(WindowCloseEvent));
        unsafe { window_OnClose(self.as_ptr(), listener) }
    }

    /// Set an event handler for moving the window on the desktop.
    pub fn set_on_moved_handler<F>(&self, handler: F)
    where
        F: Fn(&PositionEvent) + 'static,
    {
        self.inner(|inner| *inner.props.on_moved.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), WindowInner, on_moved(PositionEvent));
        unsafe { window_OnMoved(self.as_ptr(), listener) }
    }

    /// Set an event handler for window resizing.
    pub fn set_on_resize_handler<F>(&self, handler: F)
    where
        F: Fn(&SizeEvent) + 'static,
    {
        self.inner(|inner| *inner.props.on_resize.borrow_mut() = Some(Rc::new(handler)));
        let listener = listener!(self.as_ptr(), WindowInner, on_resize(SizeEvent));
        unsafe { window_OnResize(self.as_ptr(), listener) }
    }

    /// Associate the main panel with a window.
    ///
    /// # Remarks
    /// The size of the window will be adjusted based on the Natural sizing of the main panel.
    /// In order to aviod memory leak, the panel can only be set once.
    ///
    /// # Panics
    /// This method will panic if the panel has no layout in it.
    pub fn set_panel(&self, panel: Panel) {
        assert!(panel.layout(0).is_some(), "Panel has no layout in it.");

        // Check if the same panel has already been set.
        if Some(Some(panel.as_ptr())) == self.inner(|x| x.props.panel.borrow().as_ref().map(|x| x.as_ptr())) {
            return;
        }

        debug_assert_eq!(
            self.inner(|x| x.props.panel.borrow().is_none()),
            Some(true),
            "Panel has already been set."
        );

        unsafe { window_panel(self.as_ptr(), panel.as_ptr()) };
        self.inner(|inner| *inner.props.panel.borrow_mut() = Some(panel.clone()));
    }

    /// Set the text that will display the window in the title bar.
    pub fn set_title(&self, text: &str) {
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
    pub fn overlay(&self, parent: Self) {
        unsafe {
            window_overlay(self.as_ptr(), parent.as_ptr());
        }
    }

    /// Launch a window in modal mode.
    pub fn modal(&self, parent: Self) -> GuiClose {
        let value = unsafe { window_modal(self.as_ptr(), parent.as_ptr()) };
        GuiClose::from(value)
    }

    /// Ends the modal cycle of a window.
    pub fn stop_modal(&self, return_value: GuiClose) {
        unsafe {
            window_stop_modal(self.as_ptr(), return_value.into());
        }
    }

    /// Sets an action associated with pressing a key.
    pub fn set_on_hotkey_handler<F>(&self, key: KeyCode, modifiers: ModifierKey, handler: F)
    where
        F: Fn() + 'static,
    {
        let id = (key, modifiers);
        self.inner(|inner| inner.props.on_hotkey.borrow_mut().insert(id, Rc::new(handler)));

        extern "C" fn shim(obj: *mut std::ffi::c_void, _event: *mut nappgui_sys::Event) {
            let context = unsafe { &*(obj as *mut HotkeyContext) };
            let id = (context.key, context.modifiers);
            if let Some(Some(f)) = context
                .window
                .inner(|obj| obj.props.on_hotkey.borrow().get(&id).cloned())
            {
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));
            }
        }

        let context = Box::into_raw(Box::new(HotkeyContext {
            key,
            modifiers,
            window: self.clone(),
        }));

        self.inner(|inner| inner.props.on_hotkey_context.borrow_mut().push(context));

        let listener = unsafe { listener_imp(context as _, Some(shim)) };
        unsafe {
            window_hotkey(self.as_ptr(), key as _, modifiers as _, listener);
        }
    }

    /// Returns whether or not the window is visible.
    pub fn is_visible(&self) -> bool {
        unsafe { window_get_visible(self.as_ptr()) != 0 }
    }

    /// Removes all keyboard shortcuts associated with the window.
    pub fn clear_hotkeys(&self) {
        unsafe { window_clear_hotkeys(self.as_ptr()) }
    }

    /// Activate or deactivate the cyclic behavior of tabstops.
    pub fn cycle_tabstop(&self, cycle: bool) {
        let cycle = cycle as _;
        unsafe { window_cycle_tabstop(self.as_ptr(), cycle) }
    }

    /// Moves keyboard focus to the next control in the tab-list. It has the same effect as pressing \[TAB\].
    pub fn next_tabstop(&self) -> GuiFocus {
        let focus = unsafe { window_next_tabstop(self.as_ptr()) };
        GuiFocus::try_from(focus).unwrap()
    }

    /// Moves the keyboard focus to the previous control in the tab-list. This has the same effect as pressing \[SHIFT\]+\[TAB\].
    pub fn previous_tabstop(&self) -> GuiFocus {
        let focus = unsafe { window_previous_tabstop(self.as_ptr()) };
        GuiFocus::try_from(focus).unwrap()
    }

    /// Set keyboard focus to a specific control.
    pub fn set_focus<T>(&self, control: T) -> GuiFocus
    where
        T: AsRef<Control>,
    {
        let focus = unsafe { window_focus(self.as_ptr(), control.as_ref().as_ptr()) };
        GuiFocus::try_from(focus).unwrap()
    }

    /// Gets the control that keyboard focus has.
    pub fn focus(&self) -> Option<Control> {
        let control = unsafe { window_get_focus(self.as_ptr()) };
        if control.is_null() {
            None
        } else {
            Some(Control::from_raw(control))
        }
    }

    /// Gets additional information about a keyboard focus change operation.
    ///
    /// # Remarks
    /// Sometimes the decision to release keyboard focus for a control requires context information.
    /// For example, what action caused the change (press \[TAB\], click on another control) or
    /// what control will receive the focus.
    pub fn focus_info(&self) -> FocusInfo {
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
    pub fn update(&self) {
        unsafe { window_update(self.as_ptr()) }
    }

    /// Gets if the window is maximized.
    pub fn is_maximized(&self) -> bool {
        unsafe { window_get_maximize(self.as_ptr()) != 0 }
    }

    /// Maximizes the window.
    ///
    /// # Remarks
    /// This function will NOT take effect if the window has been created without ekWINDOW_RESIZE flag.
    pub fn maximize(&self) {
        unsafe { window_maximize(self.as_ptr()) }
    }

    /// Gets if the window is minimized.
    pub fn is_minimized(&self) -> bool {
        unsafe { window_get_minimize(self.as_ptr()) != 0 }
    }

    /// Minimizes the window.
    ///
    /// # Remarks
    /// This function will NOT take effect if the window has been created without ekWINDOW_RESIZE flag.
    pub fn minimize(&self) {
        unsafe { window_minimize(self.as_ptr()) }
    }

    /// Move the window to specific desktop coordinates.
    pub fn set_origin(&self, x: f32, y: f32) {
        unsafe {
            window_origin(self.as_ptr(), V2Df { x, y });
        }
    }

    /// Get the window position.
    pub fn origin(&self) -> Point2D {
        unsafe {
            let origin = window_get_origin(self.as_ptr());
            std::mem::transmute(origin)
        }
    }

    /// Get the total dimensions of the window.
    pub fn size(&self) -> Size2D {
        unsafe {
            let size = window_get_size(self.as_ptr());
            std::mem::transmute(size)
        }
    }

    /// Set the size of the client area of the window.
    ///
    /// # Remarks
    /// The final size will depend on the window frame and desktop theme settings. This measure only refers to the interior area.
    pub fn set_client_size(&self, width: f32, height: f32) {
        unsafe { window_client_size(self.as_ptr(), S2Df { width, height }) }
    }

    /// Get the dimensions of the client area of the window.
    pub fn client_size(&self) -> Size2D {
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
    pub fn control_frame<T>(&self, control: T) -> Rect2D
    where
        T: AsRef<Control>,
    {
        unsafe {
            let rect = window_control_frame(self.as_ptr(), control.as_ref().as_ptr());
            std::mem::transmute(rect)
        }
    }

    /// Transforms a point expressed in window coordinates to screen coordinates.
    pub fn client_to_screen(&self, x: f32, y: f32) -> Point2D {
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
    /// it must exist in the active layout.
    pub fn set_default_button(&self, button: Button) {
        self.inner(|inner| *inner.props.default_button.borrow_mut() = Some(button.clone()));
        unsafe { window_defbutton(self.as_ptr(), button.as_ptr()) }
    }

    /// Change the mouse cursor.
    ///
    /// # Remarks
    ///
    /// hot_x, hot_y indicate the "sensitive" point within the image, which will indicate the exact position of the mouse.
    pub fn set_cursor(&self, cursor: GuiCursor, image: &Image, hot_x: f32, hot_y: f32) {
        unsafe { window_cursor(self.as_ptr(), cursor as i32, image.as_ptr(), hot_x, hot_y) }
    }

    /// Set the general menu bar of the application.
    pub fn set_menubar(&self, menu: Menu) {
        self.inner(|inner| *inner.props.menu_bar.borrow_mut() = Some(menu.clone()));
        unsafe { osapp_menubar(menu.as_ptr(), self.as_ptr()) }
    }

    /// Launches the directory selection dialog.
    ///
    /// # Remarks
    /// It will be launched in modal. parent will remain locked until the dialog is accepted.
    pub fn launch_select_dir_dialog(&self, caption: &str, start_dir: &str) -> Option<String> {
        let caption = CString::new(caption).unwrap();
        let start_dir = CString::new(start_dir).unwrap();
        let dir = unsafe { comwin_select_dir(self.as_ptr(), caption.as_ptr(), start_dir.as_ptr()) };
        if dir.is_null() {
            return None;
        }
        let dir = unsafe { CStr::from_ptr(dir) };
        Some(dir.to_string_lossy().into_owned())
    }

    /// Launch the open file dialog.
    pub fn launch_open_file_dialog(
        &self,
        caption: &str,
        file_types: &[&str],
        start_dir: &str,
        filename: &str,
    ) -> Option<String> {
        let types: Box<[CString]> = file_types.iter().map(|x| CString::new(*x).unwrap()).collect();
        let mut types: Box<[*const std::ffi::c_char]> = types.iter().map(|x| x.as_ptr()).collect();
        let caption = CString::new(caption).unwrap();
        let start_dir = CString::new(start_dir).unwrap();
        let filename = CString::new(filename).unwrap();
        let file = unsafe {
            comwin_open_file(
                self.as_ptr(),
                caption.as_ptr(),
                types.as_mut_ptr(),
                types.len() as _,
                start_dir.as_ptr(),
                filename.as_ptr(),
            )
        };
        if file.is_null() {
            return None;
        }
        let file = unsafe { std::ffi::CStr::from_ptr(file) };
        Some(file.to_string_lossy().into_owned())
    }

    /// Launch the save file dialog.
    pub fn launch_save_file_dialog(
        &self,
        caption: &str,
        file_types: &[&str],
        start_dir: &str,
        filename: &str,
    ) -> Option<String> {
        let types: Box<[CString]> = file_types.iter().map(|x| CString::new(*x).unwrap()).collect();
        let mut types: Box<[*const std::ffi::c_char]> = types.iter().map(|x| x.as_ptr()).collect();
        let caption = CString::new(caption).unwrap();
        let start_dir = CString::new(start_dir).unwrap();
        let filename = CString::new(filename).unwrap();

        let file = unsafe {
            comwin_save_file(
                self.as_ptr(),
                caption.as_ptr(),
                types.as_mut_ptr(),
                types.len() as _,
                start_dir.as_ptr(),
                filename.as_ptr(),
            )
        };
        if file.is_null() {
            return None;
        }
        let file = unsafe { std::ffi::CStr::from_ptr(file) };
        Some(file.to_string_lossy().into_owned())
    }

    /// Launch the color selection dialog.
    pub fn launch_color_dialog<F>(
        &self,
        title: &str,
        x: f32,
        y: f32,
        halign: Align,
        valign: Align,
        current: Color,
        colors: &[Color],
        on_change: F,
    ) where
        F: FnOnce(&Color) + 'static,
    {
        let on_change: *mut Box<dyn FnOnce(&Color) + 'static> = Box::into_raw(Box::new(Box::new(on_change)));

        extern "C" fn shim(obj: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let on_change = unsafe { Box::from_raw(obj as *mut Box<dyn FnOnce(&Color) + 'static>) };
            let color =
                unsafe { *(nappgui_sys::event_params_imp(event, c"color_t".as_ptr()) as *const nappgui_sys::color_t) };
            let color = Color::new(color);
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| on_change(&color)));
        }

        let listener = unsafe { nappgui_sys::listener_imp(on_change as _, Some(shim)) };
        let title = CString::new(title).unwrap();
        let mut colors: Vec<u32> = colors.iter().map(|color| color.inner).collect();

        unsafe {
            comwin_color(
                self.as_ptr(),
                title.as_ptr(),
                x,
                y,
                halign as _,
                valign as _,
                current.inner,
                colors.as_mut_ptr(),
                colors.len() as _,
                listener,
            );
        }
    }
}
