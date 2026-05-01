use std::{ffi::CStr, ptr::NonNull};

use crate::{
    draw_2d::Image,
    gui::{event::EvMenu, Menu},
    types::{GuiState, KeyCode, ModifierKey},
    util::macros::callback,
};

use nappgui_sys::{
    menuitem_OnClick, menuitem_create, menuitem_enabled, menuitem_get_enabled, menuitem_get_image,
    menuitem_get_separator, menuitem_get_state, menuitem_get_submenu, menuitem_get_text, menuitem_get_visible,
    menuitem_image, menuitem_key, menuitem_separator, menuitem_state, menuitem_submenu, menuitem_text,
    menuitem_visible,
};

/// Represents an option within a Menu. They will always have an associated action that will be executed when activated.
///
/// # Remark
/// This type is managed by nappgui itself. Rust does not have its ownership. When the menu object is dropped, all
/// components assciated with it will be automatically released.
#[repr(transparent)]
#[derive(Clone)]
pub struct MenuItem(NonNull<nappgui_sys::MenuItem>);

impl MenuItem {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::MenuItem) -> Self {
        Self(NonNull::new(ptr).expect("Null pointer passed to MenuItem::from_raw"))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::MenuItem {
        self.0.as_ptr()
    }

    /// Create a new item for a menu.
    pub fn new(text: &str) -> Self {
        let menu_item = unsafe { Self::from_raw(menuitem_create()) };
        menu_item.set_text(text);
        menu_item
    }

    /// Create a new separator for a menu.
    pub fn new_separator() -> Self {
        let menu_item = unsafe { menuitem_separator() };
        unsafe { Self::from_raw(menu_item) }
    }

    callback! {
        /// Set an event handle for item click.
        pub on_click(EvMenu) => menuitem_OnClick
    }

    /// Enables or disables a menu item.
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { menuitem_enabled(self.as_ptr(), enabled as _) };
    }

    /// Show or hide a menu item.
    pub fn set_visible(&self, visible: bool) {
        unsafe { menuitem_visible(self.as_ptr(), visible as _) };
    }

    /// Set the item text.
    pub fn set_text(&self, text: &str) {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { menuitem_text(self.as_ptr(), text.as_ptr()) };
    }

    /// Set the icon that will display the item.
    pub fn set_image(&self, image: &Image) {
        unsafe { menuitem_image(self.as_ptr(), image.as_ptr()) };
    }

    /// Set a keyboard shortcut to select the menu item.
    pub fn set_key(&self, key: KeyCode, modifiers: ModifierKey) {
        unsafe { menuitem_key(self.as_ptr(), key as _, modifiers as _) };
    }

    /// Assign a drop-down submenu when selecting the item.
    pub fn set_submenu(&self, menu: &Menu) {
        unsafe { menuitem_submenu(self.as_ptr(), &mut menu.as_ptr()) };
    }

    /// Set the status of the item, which will be reflected with a mark next to the text.
    pub fn set_state(&self, state: GuiState) {
        unsafe { menuitem_state(self.as_ptr(), state as _) };
    }

    /// Get the current text of an item.
    pub fn text(&self) -> String {
        let text = unsafe { menuitem_get_text(self.as_ptr()) };
        unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() }
    }

    /// Get the current icon of an item.
    pub fn image(&self) -> Option<Image> {
        let image = unsafe { menuitem_get_image(self.as_ptr()) };
        if image.is_null() {
            None
        } else {
            Some(unsafe { Image::from_raw_cloned(image) })
        }
    }

    /// Get if an item is a separator.
    pub fn is_separator(&self) -> bool {
        (unsafe { menuitem_get_separator(self.as_ptr()) } != 0)
    }

    /// Get if an item is enabled or not.
    pub fn is_enabled(&self) -> bool {
        (unsafe { menuitem_get_enabled(self.as_ptr()) } != 0)
    }

    /// Get if an item is visible or not.
    pub fn is_visible(&self) -> bool {
        (unsafe { menuitem_get_visible(self.as_ptr()) } != 0)
    }

    /// Gets the state of an item.
    pub fn state(&self) -> GuiState {
        let state = unsafe { menuitem_get_state(self.as_ptr()) };
        GuiState::try_from(state).unwrap()
    }

    /// Gets the submenu associated with item.
    pub fn submenu(&self) -> Option<Menu> {
        let submenu = unsafe { menuitem_get_submenu(self.as_ptr()) };
        if submenu.is_null() {
            None
        } else {
            Some(unsafe { Menu::from_raw(submenu) })
        }
    }
}
