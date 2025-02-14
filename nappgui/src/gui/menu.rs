use std::rc::Rc;

use nappgui_sys::{
    menu_create, menu_destroy, menu_get_item, menu_hide, menu_item, menu_launch, menu_off_items,
    menu_size, V2Df,
};

use crate::util::macros::impl_ptr;

use super::MenuItem;

/// A Menu is a type of control that integrates a series of options, also called items or MenuItems. 
/// Each of them consists of a short text, optionally an icon and optionally also a keyboard shortcut, 
/// such as the classic Ctrl+C/Ctrl+V for copy and paste. Additionally, an item can host a submenu, 
/// forming a hierarchy with different levels of depth. In Products you have an example application 
/// that uses menus.
pub struct Menu {
    pub(crate) inner: Rc<*mut nappgui_sys::Menu>,
}

impl Menu {
    impl_ptr!(nappgui_sys::Menu);

    /// Create a new menu.
    pub fn create() -> Self {
        let menu = unsafe { menu_create() };
        Self::new(menu)
    }

    /// Destroy a menu and its entire hierarchy.
    pub fn destroy(self) {
        unsafe { menu_destroy(&mut self.as_ptr()) };
    }

    /// Launch a menu as secondary or PopUp.
    pub fn launch(&self, x: f32, y: f32) {
        let position = V2Df { x, y };
        unsafe { menu_launch(self.as_ptr(), position) };
    }

    /// Hides a secondary PopUp menu.
    pub fn hide(&self) {
        unsafe { menu_hide(self.as_ptr()) };
    }

    /// Add an item to the menu.
    pub fn item(&self, item: &MenuItem) {
        unsafe { menu_item(self.as_ptr(), item.as_ptr()) };
    }

    /// Set status ekGUI_OFF for all menu items.
    pub fn off_items(&self) {
        unsafe { menu_off_items(self.as_ptr()) };
    }

    /// Get an item from the menu.
    pub fn get_item(&self, index: u32) -> Option<MenuItem> {
        let item = unsafe { menu_get_item(self.as_ptr(), index) };
        if item.is_null() {
            None
        } else {
            Some(MenuItem::new(item))
        }
    }

    /// Gets the number of items.
    pub fn size(&self) -> u32 {
        unsafe { menu_size(self.as_ptr()) }
    }
}
