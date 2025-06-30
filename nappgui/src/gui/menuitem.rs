use std::rc::Rc;

use crate::{
    core::event::Event,
    draw_2d::Image,
    prelude::{GuiState, Vkey},
    util::macros::pub_crate_ptr_ops,
};

use nappgui_sys::{
    listener_imp, menuitem_OnClick, menuitem_create, menuitem_enabled, menuitem_image,
    menuitem_key, menuitem_separator, menuitem_state, menuitem_submenu, menuitem_text,
    menuitem_visible,
};

use super::Menu;

/// Represents an option within a Menu. They will always have an associated action that will be executed when activated.
pub struct MenuItem {
    pub(crate) inner: Rc<*mut nappgui_sys::MenuItem>,
}

impl MenuItem {
    pub_crate_ptr_ops!(*mut nappgui_sys::MenuItem, Rc<*mut nappgui_sys::MenuItem>);

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
        ) = Box::into_raw(Box::new((cb, self.as_ptr())));

        unsafe {
            menuitem_OnClick(
                self.as_ptr(),
                listener_imp(data as *mut std::ffi::c_void, Some(shim)),
            );
        }
    }

    /// Enables or disables a menu item.
    pub fn enabled(&self, enabled: bool) {
        unsafe { menuitem_enabled(self.as_ptr(), enabled as i8) };
    }

    /// Show or hide a menu item.
    pub fn visible(&self, visible: bool) {
        unsafe { menuitem_visible(self.as_ptr(), visible as i8) };
    }

    /// Set the item text.
    pub fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { menuitem_text(self.as_ptr(), text.as_ptr()) };
    }

    /// Set the icon that will display the item.
    pub fn image(&self, image: &Image) {
        unsafe { menuitem_image(self.as_ptr(), image.inner) };
    }

    /// Set a keyboard shortcut to select the menu item.
    pub fn key(&self, key: Vkey, modifiers: u32) {
        unsafe { menuitem_key(self.as_ptr(), key, modifiers) };
    }

    /// Assign a drop-down submenu when selecting the item.
    pub fn submenu(&self, menu: &mut Menu) {
        unsafe { menuitem_submenu(self.as_ptr(), &mut menu.as_ptr()) };
    }

    /// Set the status of the item, which will be reflected with a mark next to the text.
    pub fn state(&self, state: GuiState) {
        unsafe { menuitem_state(self.as_ptr(), state) };
    }
}
