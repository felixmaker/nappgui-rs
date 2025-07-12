use std::rc::Rc;

use crate::{
    draw_2d::Image,
    gui::event::EvMenu,
    types::{GuiState, KeyCode, ModifierKey},
    util::macros::{callback, pub_crate_ptr_ops},
};

use nappgui_sys::{
    menuitem_OnClick, menuitem_create, menuitem_enabled, menuitem_image, menuitem_key,
    menuitem_separator, menuitem_state, menuitem_submenu, menuitem_text, menuitem_visible,
};

use super::Menu;

/// Represents an option within a Menu. They will always have an associated action that will be executed when activated.
#[derive(Clone)]
pub struct MenuItem {
    pub(crate) inner: Rc<*mut nappgui_sys::MenuItem>,
}

impl MenuItem {
    pub_crate_ptr_ops!(*mut nappgui_sys::MenuItem);

    /// Create a new item for a menu.
    pub fn new() -> Self {
        let menu_item = unsafe { menuitem_create() };
        Self::from_raw(menu_item)
    }

    /// Create a new separator for a menu.
    pub fn new_separator() -> Self {
        let menu_item = unsafe { menuitem_separator() };
        Self::from_raw(menu_item)
    }

    callback! {
        /// Set an event handle for item click.
        pub on_click(EvMenu) => menuitem_OnClick
    }

    /// Enables or disables a menu item.
    pub fn enabled(&self, enabled: bool) {
        unsafe { menuitem_enabled(self.as_ptr(), enabled as _) };
    }

    /// Show or hide a menu item.
    pub fn visible(&self, visible: bool) {
        unsafe { menuitem_visible(self.as_ptr(), visible as _) };
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
    pub fn key(&self, key: KeyCode, modifiers: ModifierKey) {
        unsafe { menuitem_key(self.as_ptr(), key as _, modifiers as _) };
    }

    /// Assign a drop-down submenu when selecting the item.
    pub fn submenu(&self, menu: &mut Menu) {
        unsafe { menuitem_submenu(self.as_ptr(), &mut menu.as_ptr()) };
    }

    /// Set the status of the item, which will be reflected with a mark next to the text.
    pub fn state(&self, state: GuiState) {
        unsafe { menuitem_state(self.as_ptr(), state as _) };
    }
}
