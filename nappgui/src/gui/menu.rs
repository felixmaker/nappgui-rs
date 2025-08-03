use std::rc::{Rc, Weak};

use nappgui_sys::{
    menu_add_item, menu_count, menu_create, menu_del_item, menu_destroy, menu_get_item,
    menu_ins_item, menu_is_menubar, menu_launch, menu_off_items, V2Df,
};

use crate::gui::{window::WindowTrait, MenuItemTrait};

use super::MenuItem;

/// The menu trait.
pub trait MenuTrait {
    /// Returns a raw pointer to the menu object.
    fn as_ptr(&self) -> *mut nappgui_sys::Menu;

    /// Add an item at the end of the menu.
    fn add_item<T>(&self, item: T)
    where
        T: MenuItemTrait,
    {
        unsafe { menu_add_item(self.as_ptr(), item.as_ptr()) };
    }

    /// Insert an item in an arbitrary position of the menu.
    fn insert_item<T>(&self, index: u32, item: T)
    where
        T: MenuItemTrait,
    {
        unsafe { menu_ins_item(self.as_ptr(), index, item.as_ptr()) };
    }

    /// Remove an item from the menu.
    ///
    /// # Remark
    /// The element will be destroyed and cannot be reused. If has a submenu associated,
    /// it will also be destroyed recursively.
    fn delete_item(&self, index: u32) {
        unsafe { menu_del_item(self.as_ptr(), index) };
    }

    /// Launch a menu as secondary or PopUp.
    fn launch<T>(&self, window: T, x: f32, y: f32)
    where
        T: WindowTrait,
    {
        let position = V2Df { x, y };
        unsafe { menu_launch(self.as_ptr(), window.as_ptr(), position) };
    }

    /// Set status ekGUI_OFF for all menu items.
    fn off_items(&self) {
        unsafe { menu_off_items(self.as_ptr()) };
    }

    /// Get the number of items.
    fn count(&self) -> usize {
        unsafe { menu_count(self.as_ptr()) as _ }
    }

    /// Get an item from the menu.
    fn get_item(&self, index: u32) -> Option<MenuItem> {
        let item = unsafe { menu_get_item(self.as_ptr(), index) };
        if item.is_null() {
            None
        } else {
            Some(MenuItem { inner: item })
        }
    }

    /// Returns TRUE if the menu is currently established as a menu bar.
    fn is_menubar(&self) -> bool {
        (unsafe { menu_is_menubar(self.as_ptr()) }) != 0
    }
}

/// The inner type of menu.
#[doc(hidden)]
#[repr(transparent)]
pub struct MenuInner {
    pub(crate) inner: *mut nappgui_sys::Menu,
}

impl MenuTrait for MenuInner {
    fn as_ptr(&self) -> *mut nappgui_sys::Menu {
        self.inner
    }
}

/// A Menu is a type of control that integrates a series of options, also called items or Menuitems.
/// Each of them consists of a short text, optionally an icon and optionally also a keyboard shortcut,
/// such as the classic Ctrl+C/Ctrl+V to copy and paste. Additionally, an item can house a submenu
/// forming a hierarchy with different levels of depth. In Products you have an application that uses
/// menus and in Hello dynamic Menu! an example of adding or eliminating items at runtime.
pub struct Menu {
    pub(crate) inner: Rc<MenuInner>,
}

impl Menu {
    /// Create a new menu.
    pub fn new() -> Self {
        let menu = unsafe { menu_create() };
        Self {
            inner: Rc::new(MenuInner { inner: menu }),
        }
    }

    /// Get the weak reference of the menu.
    pub fn as_weak(&self) -> Weak<MenuInner> {
        Rc::downgrade(&self.inner)
    }
}

impl MenuTrait for Menu {
    fn as_ptr(&self) -> *mut nappgui_sys::Menu {
        self.inner.as_ptr()
    }
}

impl MenuTrait for Weak<MenuInner> {
    fn as_ptr(&self) -> *mut nappgui_sys::Menu {
        let menu = self.upgrade().unwrap();
        menu.inner
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) == 0 {
            unsafe { menu_destroy(&mut self.as_ptr()) };
        }
    }
}
