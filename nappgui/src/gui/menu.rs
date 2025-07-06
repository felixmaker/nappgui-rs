use std::rc::Rc;

use nappgui_sys::{
    menu_add_item, menu_count, menu_create, menu_del_item, menu_destroy, menu_get_item,
    menu_ins_item, menu_is_menubar, menu_launch, menu_off_items, V2Df,
};

use crate::util::macros::pub_crate_ptr_ops;

use super::{MenuItem, Window};

/// A Menu is a type of control that integrates a series of options, also called items or Menuitems.
/// Each of them consists of a short text, optionally an icon and optionally also a keyboard shortcut,
/// such as the classic Ctrl+C/Ctrl+V to copy and paste. Additionally, an item can house a submenu
/// forming a hierarchy with different levels of depth. In Products you have an application that uses
/// menus and in Hello dynamic Menu! an example of adding or eliminating items at runtime.
pub struct Menu {
    pub(crate) inner: Rc<*mut nappgui_sys::Menu>,
}

impl Menu {
    pub_crate_ptr_ops!(*mut nappgui_sys::Menu);

    /// Create a new menu.
    pub fn create() -> Self {
        let menu = unsafe { menu_create() };
        Self::from_raw(menu)
    }

    /// Add an item at the end of the menu.
    pub fn add_item(&self, item: &MenuItem) {
        unsafe { menu_add_item(self.as_ptr(), item.as_ptr()) };
    }

    /// Insert an item in an arbitrary position of the menu.
    pub fn insert_item(&self, index: usize, item: &MenuItem) {
        unsafe { menu_ins_item(self.as_ptr(), index as _, item.as_ptr()) };
    }

    /// Remove an item from the menu.
    ///
    /// # Remark
    /// The element will be destroyed and cannot be reused. If has a submenu associated,
    /// it will also be destroyed recursively.
    pub fn delete_item(&self, index: usize) {
        unsafe { menu_del_item(self.as_ptr(), index as _) };
    }

    /// Launch a menu as secondary or PopUp.
    pub fn launch(&self, window: &Window, x: f32, y: f32) {
        let position = V2Df { x, y };
        unsafe { menu_launch(self.as_ptr(), window.as_ptr(), position) };
    }

    /// Set status ekGUI_OFF for all menu items.
    pub fn off_items(&self) {
        unsafe { menu_off_items(self.as_ptr()) };
    }

    /// Get the number of items.
    pub fn count(&self) -> usize {
        unsafe { menu_count(self.as_ptr()) as _ }
    }

    /// Get an item from the menu.
    pub fn get_item(&self, index: usize) -> Option<MenuItem> {
        let item = unsafe { menu_get_item(self.as_ptr(), index as _) };
        if item.is_null() {
            None
        } else {
            Some(MenuItem::from_raw(item))
        }
    }

    /// Returns TRUE if the menu is currently established as a menu bar.
    pub fn is_menubar(&self) -> bool {
        (unsafe { menu_is_menubar(self.as_ptr()) }) != 0
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        unsafe { menu_destroy(&mut self.as_ptr()) };
    }
}
