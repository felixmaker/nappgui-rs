use crate::{
    draw_2d::ImageTrait,
    gui::{event::EvMenu, MenuTrait},
    types::{GuiState, KeyCode, ModifierKey},
    util::macros::callback,
};

use nappgui_sys::{
    menuitem_OnClick, menuitem_create, menuitem_enabled, menuitem_image, menuitem_key,
    menuitem_separator, menuitem_state, menuitem_submenu, menuitem_text, menuitem_visible,
};

/// The menu item trait.
pub trait MenuItemTrait {
    /// Returns a raw pointer to the menu item object.
    fn as_ptr(&self) -> *mut nappgui_sys::MenuItem;

    callback! {
        /// Set an event handle for item click.
        on_click(EvMenu) => menuitem_OnClick
    }

    /// Enables or disables a menu item.
    fn enabled(&self, enabled: bool) {
        unsafe { menuitem_enabled(self.as_ptr(), enabled as _) };
    }

    /// Show or hide a menu item.
    fn visible(&self, visible: bool) {
        unsafe { menuitem_visible(self.as_ptr(), visible as _) };
    }

    /// Set the item text.
    fn text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { menuitem_text(self.as_ptr(), text.as_ptr()) };
    }

    /// Set the icon that will display the item.
    fn image<T>(&self, image: &T)
    where
        T: ImageTrait,
    {
        unsafe { menuitem_image(self.as_ptr(), image.as_ptr()) };
    }

    /// Set a keyboard shortcut to select the menu item.
    fn key(&self, key: KeyCode, modifiers: ModifierKey) {
        unsafe { menuitem_key(self.as_ptr(), key as _, modifiers as _) };
    }

    /// Assign a drop-down submenu when selecting the item.
    fn submenu<T>(&self, menu: &mut T)
    where
        T: MenuTrait,
    {
        unsafe { menuitem_submenu(self.as_ptr(), &mut menu.as_ptr()) };
    }

    /// Set the status of the item, which will be reflected with a mark next to the text.
    fn state(&self, state: GuiState) {
        unsafe { menuitem_state(self.as_ptr(), state as _) };
    }
}

/// Represents an option within a Menu. They will always have an associated action that will be executed when activated.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the menu object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct MenuItem {
    pub(crate) inner: *mut nappgui_sys::MenuItem,
}

impl MenuItem {
    /// Create a new item for a menu.
    pub fn new() -> Self {
        let menu_item = unsafe { menuitem_create() };
        Self { inner: menu_item }
    }

    /// Create a new separator for a menu.
    pub fn new_separator() -> Self {
        let menu_item = unsafe { menuitem_separator() };
        Self { inner: menu_item }
    }
}

impl MenuItemTrait for MenuItem {
    fn as_ptr(&self) -> *mut nappgui_sys::MenuItem {
        self.inner
    }
}
