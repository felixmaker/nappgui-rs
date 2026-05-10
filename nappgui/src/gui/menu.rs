use std::cell::{Cell, RefCell};

use nappgui_sys::{
    menu_add_item, menu_count, menu_create, menu_del_item, menu_destroy, menu_ins_item, menu_is_menubar, menu_launch,
    menu_off_items, V2Df,
};

use crate::gui::{define_object, Window};

use super::MenuItem;

#[derive(Default)]
pub(crate) struct MenuProps {
    c_managed: Cell<bool>,
    items: RefCell<Vec<MenuItem>>,
}

define_object!(Menu, MenuInner, Menu, MenuProps);

// impl Drop for MenuInner {
//     fn drop(&mut self) {
//         if !self.c_managed.get() {
//             unsafe { menu_destroy(&mut self.as_ptr()) }
//         }
//     }
// }

impl Menu {
    pub(crate) fn set_c_managed(&self, managed: bool) {
        self.inner(|x| x.props.c_managed.set(managed));
    }

    /// Create a new menu.
    pub fn new() -> Self {
        unsafe { Self::from_raw(menu_create()) }
    }

    /// Add an item at the end of the menu.
    pub fn add_item(&self, item: MenuItem) {
        unsafe { menu_add_item(self.as_ptr(), item.as_ptr()) };
        self.inner(|inner| inner.props.items.borrow_mut().push(item));
    }

    /// Insert an item in an arbitrary position of the menu.
    pub fn insert_item(&self, index: u32, item: MenuItem) {
        unsafe { menu_ins_item(self.as_ptr(), index, item.as_ptr()) };
        self.inner(|inner| inner.props.items.borrow_mut().insert(index as _, item));
    }

    /// Remove an item from the menu.
    ///
    /// # Remark
    /// The element will be destroyed and cannot be reused. If has a submenu associated,
    /// it will also be destroyed recursively.
    pub fn delete_item(&self, index: u32) {
        unsafe { menu_del_item(self.as_ptr(), index) };
        self.inner(|inner| inner.props.items.borrow_mut().remove(index as _));
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
        self.inner(|inner| inner.props.items.borrow().get(index as usize).cloned())?
    }

    /// Returns TRUE if the menu is currently established as a menu bar.
    pub fn is_menubar(&self) -> bool {
        (unsafe { menu_is_menubar(self.as_ptr()) }) != 0
    }

    /// Returns the native implementation of the menu.
    pub fn native(&self) -> *mut () {
        (unsafe { nappgui_sys::menu_imp(self.as_ptr() as _) }) as _
    }
}
