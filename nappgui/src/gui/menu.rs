use std::{
    cell::Cell,
    rc::{Rc, Weak},
};

use nappgui_sys::{
    menu_add_item, menu_count, menu_create, menu_del_item, menu_destroy, menu_get_item, menu_ins_item, menu_is_menubar,
    menu_launch, menu_off_items, V2Df,
};

use crate::gui::{global_exists, global_get, global_move_ownership, global_record, Window};

use super::MenuItem;

pub(crate) struct MenuInner {
    ptr: *mut nappgui_sys::Menu,
    c_managed: Cell<bool>,
}

impl MenuInner {
    /// Creates a `MenuInner` from a raw pointer.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Menu) -> Self {
        assert!(!ptr.is_null());
        Self {
            ptr,
            c_managed: Cell::new(false),
        }
    }

    /// Returns the underlying raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Menu {
        self.ptr
    }
}

impl Drop for MenuInner {
    fn drop(&mut self) {
        if !self.c_managed.get() {
            unsafe { menu_destroy(&mut self.as_ptr()) }
        }
    }
}

/// A Menu is a type of control that integrates a series of options, also called items or Menuitems.
/// Each of them consists of a short text, optionally an icon and optionally also a keyboard shortcut,
/// such as the classic Ctrl+C/Ctrl+V to copy and paste. Additionally, an item can house a submenu
/// forming a hierarchy with different levels of depth. In Products you have an application that uses
/// menus and in Hello dynamic Menu! an example of adding or eliminating items at runtime.
#[repr(transparent)]
#[derive(Clone)]
pub struct Menu(Weak<MenuInner>);

impl Menu {
    /// Creates a `Menu` from a raw pointer.
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::Menu) -> Self {
        assert!(!ptr.is_null());
        if !global_exists(ptr as _) {
            let menu = global_record(ptr as _, MenuInner::from_raw(ptr));
            return Self(Rc::downgrade(&menu));
        }

        if let Some(menu) = global_get(ptr as _) {
            return Self(Rc::downgrade(&menu));
        }

        panic!("Menu object has been destroyed already.");
    }

    /// Returns the underlying raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::Menu {
        self.0.upgrade().map(|x| x.as_ptr()).unwrap()
    }

    pub(crate) fn set_c_managed(&self, managed: bool) {
        self.0.upgrade().map(|x| x.c_managed.set(managed));
    }

    /// Create a new menu.
    pub fn new() -> Self {
        unsafe { Self::from_raw(menu_create()) }
    }

    /// Add an item at the end of the menu.
    pub fn add_item(&self, item: MenuItem) {
        unsafe { menu_add_item(self.as_ptr(), item.as_ptr()) };
        global_move_ownership(item.as_ptr() as _, self.as_ptr() as _);
    }

    /// Insert an item in an arbitrary position of the menu.
    pub fn insert_item(&self, index: u32, item: MenuItem) {
        unsafe { menu_ins_item(self.as_ptr(), index, item.as_ptr()) };
        global_move_ownership(item.as_ptr() as _, self.as_ptr() as _);
    }

    /// Remove an item from the menu.
    ///
    /// # Remark
    /// The element will be destroyed and cannot be reused. If has a submenu associated,
    /// it will also be destroyed recursively.
    pub fn delete_item(&self, index: u32) {
        unsafe { menu_del_item(self.as_ptr(), index) };
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
    pub fn get_item(&self, index: u32) -> Option<MenuItem> {
        let item = unsafe { menu_get_item(self.as_ptr(), index) };
        if item.is_null() {
            None
        } else {
            Some(unsafe { MenuItem::from_raw(item) })
        }
    }

    /// Returns TRUE if the menu is currently established as a menu bar.
    pub fn is_menubar(&self) -> bool {
        (unsafe { menu_is_menubar(self.as_ptr()) }) != 0
    }

    /// Returns the native implementation of the menu.
    pub fn native(&self) -> *mut () {
        (unsafe { nappgui_sys::menu_imp(self.0.as_ptr() as _) }) as _
    }
}
