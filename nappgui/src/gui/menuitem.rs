use std::{
    cell::RefCell,
    ffi::CStr,
    ptr::NonNull,
    rc::{Rc, Weak},
};

use crate::{
    draw_2d::Image,
    gui::{event::EvMenu, global_get, global_record, Menu},
    types::{GuiState, KeyCode, ModifierKey},
    util::macros::listener,
};

use nappgui_sys::{
    menuitem_OnClick, menuitem_create, menuitem_enabled, menuitem_get_enabled, menuitem_get_image,
    menuitem_get_separator, menuitem_get_state, menuitem_get_text, menuitem_get_visible, menuitem_image, menuitem_key,
    menuitem_separator, menuitem_state, menuitem_submenu, menuitem_text, menuitem_visible,
};

pub(crate) struct MenuItemInner {
    ptr: NonNull<nappgui_sys::MenuItem>,
    on_click: RefCell<Option<Rc<dyn Fn(&EvMenu) + 'static>>>,
    submenu: RefCell<Option<Menu>>,
}

impl MenuItemInner {
    pub(crate) fn from_raw(ptr: *mut nappgui_sys::MenuItem) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Null pointer passed to MenuItemInner::from_raw"),
            on_click: RefCell::new(None),
            submenu: RefCell::new(None),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::MenuItem {
        self.ptr.as_ptr()
    }
}

/// The menu item control.
///
/// # Remarks
/// If the object is not attached to a menu, it will cause a memory leak.
#[repr(transparent)]
#[derive(Clone)]
pub struct MenuItem(Weak<MenuItemInner>);

impl MenuItem {
    pub(crate) unsafe fn from_raw(ptr: *mut nappgui_sys::MenuItem) -> Self {
        let object = global_record(ptr as _, MenuItemInner::from_raw(ptr));
        Self(Rc::downgrade(&object))
    }

    pub(crate) fn as_ptr(&self) -> *mut nappgui_sys::MenuItem {
        self.0.upgrade().map(|inner| inner.as_ptr()).unwrap()
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

    /// Set an event handle for item click.
    pub fn on_click<F>(&self, callback: F)
    where
        F: Fn(&EvMenu) + 'static,
    {
        self.0
            .upgrade()
            .map(|inner| *inner.on_click.borrow_mut() = Some(Rc::new(callback)));

        let listener = listener!(self.as_ptr(), MenuItemInner, on_click(EvMenu));
        unsafe { menuitem_OnClick(self.as_ptr(), listener) };
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
    pub fn set_submenu(&self, menu: Menu) {
        unsafe { menuitem_submenu(self.as_ptr(), &mut menu.as_ptr()) };
        menu.set_c_managed(true); // Avoid double leak from C
        self.0.upgrade().map(|inner| *inner.submenu.borrow_mut() = Some(menu));
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
        self.0.upgrade().and_then(|inner| inner.submenu.borrow().clone())
    }
}
