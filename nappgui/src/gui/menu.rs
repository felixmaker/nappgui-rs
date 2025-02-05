use nappgui_sys::{
    menu_create, menu_destroy, menu_get_item, menu_hide, menu_item, menu_launch, menu_off_items,
    menu_size, V2Df,
};

use super::MenuItem;

pub struct Menu {
    pub(crate) inner: *mut nappgui_sys::Menu,
}

impl Menu {
    pub(crate) fn new(ptr: *mut nappgui_sys::Menu) -> Self {
        if ptr.is_null() {
            panic!("ptr is null");
        }
        Self { inner: ptr }
    }

    /// Create a new menu.
    pub fn create() -> Self {
        let menu = unsafe { menu_create() };
        Self::new(menu)
    }

    /// Destroy a menu and its entire hierarchy.
    pub fn destroy(mut self) {
        unsafe { menu_destroy(&mut self.inner) };
    }

    /// Launch a menu as secondary or PopUp.
    pub fn launch(&self, x: f32, y: f32) {
        let position = V2Df { x, y };
        unsafe { menu_launch(self.inner, position) };
    }

    /// Hides a secondary PopUp menu.
    pub fn hide(&self) {
        unsafe { menu_hide(self.inner) };
    }

    /// Add an item to the menu.
    pub fn item(&self, item: &MenuItem) {
        unsafe { menu_item(self.inner, item.inner) };
    }

    /// Set status ekGUI_OFF for all menu items.
    pub fn off_items(&self) {
        unsafe { menu_off_items(self.inner) };
    }

    /// Get an item from the menu.
    pub fn get_item(&self, index: u32) -> Option<MenuItem> {
        let item = unsafe { menu_get_item(self.inner, index) };
        if item.is_null() {
            None
        } else {
            Some(MenuItem::new(item))
        }
    }

    /// Gets the number of items.
    pub fn size(&self) -> u32 {
        unsafe { menu_size(self.inner) }
    }
}
