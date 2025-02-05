use crate::{core::event::Event, draw_2d::Image};

use nappgui_sys::{
    gui_state_t, listener_imp, menuitem_OnClick, menuitem_create, menuitem_enabled, menuitem_image,
    menuitem_key, menuitem_separator, menuitem_state, menuitem_submenu, menuitem_text,
    menuitem_visible, vkey_t,
};

use super::Menu;

pub struct MenuItem {
    pub(crate) inner: *mut nappgui_sys::MenuItem,
}

impl MenuItem {
    pub(crate) fn new(ptr: *mut nappgui_sys::MenuItem) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new item for a menu.
    pub fn create() -> Self {
        let menu_item = unsafe { menuitem_create() };
        Self::new(menu_item)
    }

    /// Create a new separator for a menu.
    pub fn separator() -> Self {
        let menu_item = unsafe { menuitem_separator() };
        Self::new(menu_item)
    }

    /// Set an event handle for item click.
    pub fn on_click<F>(&self, handler: F)
    where
        F: FnMut(&mut MenuItem, &Event) + 'static,
    {
        unsafe extern "C" fn shim(data: *mut std::ffi::c_void, event: *mut nappgui_sys::Event) {
            let data = data as *mut (
                Box<dyn FnMut(&mut MenuItem, &Event)>,
                *mut nappgui_sys::MenuItem,
            );
            let f = &mut *(*data).0;
            let mut obj = MenuItem::new((*data).1);
            let ev = Event::new(event as _);
            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut obj, &ev)));
        }

        let cb: Box<dyn FnMut(&mut MenuItem, &Event)> = Box::new(handler);

        let data: *mut (
            Box<dyn FnMut(&mut MenuItem, &Event)>,
            *mut nappgui_sys::MenuItem,
        ) = Box::into_raw(Box::new((cb, self.inner)));

        unsafe {
            menuitem_OnClick(
                self.inner,
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Enables or disables a menu item.
    pub fn enabled(&self, enabled: bool) {
        unsafe { menuitem_enabled(self.inner, enabled as i8) };
    }

    /// Show or hide a menu item.
    pub fn visible(&self, visible: bool) {
        unsafe { menuitem_visible(self.inner, visible as i8) };
    }

    /// Set the item text.
    pub fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { menuitem_text(self.inner, text.as_ptr()) };
    }

    /// Set the icon that will display the item.
    pub fn image(&self, image: &Image) {
        unsafe { menuitem_image(self.inner, image.inner) };
    }

    /// Set a keyboard shortcut to select the menu item.
    pub fn key(&self, key: vkey_t, modifiers: u32) {
        unsafe { menuitem_key(self.inner, key, modifiers) };
    }

    /// Assign a drop-down submenu when selecting the item.
    pub fn submenu(&self, menu: &mut Menu) {
        unsafe { menuitem_submenu(self.inner, &mut menu.inner) };
    }

    /// Set the status of the item, which will be reflected with a mark next to the text.
    pub fn state(&self, state: gui_state_t) {
        unsafe { menuitem_state(self.inner, state) };
    }
}
