use std::ffi::{c_void, CString};

use nappgui_sys::{osapp_finish, osapp_menubar, osapp_open_url, osmain_imp};

use crate::gui::{MenuTrait, WindowTrait};

/// Application handler.
pub trait AppHandler {
    /// Create the application. Controls should be created in this function.
    fn create() -> Self;
    /// Destroy the application.
    fn destroy(&mut self) {}
}

/// Start a desktop application.
pub fn osmain<T>()
where
    T: AppHandler,
{
    unsafe extern "C" fn on_create<T>() -> *mut c_void
    where
        T: AppHandler,
    {
        let app = T::create();
        Box::into_raw(Box::new(app)) as *mut c_void
    }

    unsafe extern "C" fn on_destory<T>(_obj: *mut *mut c_void)
    where
        T: AppHandler,
    {
        let mut app = Box::from_raw(*_obj as *mut T);
        app.destroy();
    }

    unsafe {
        osmain_imp(
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0.0,
            Some(on_create::<T>),
            None,
            Some(on_destory::<T>),
            std::ptr::null(),
        );
    }
}

/// End a desktop application, destroying the message cycle and the application object.
pub fn finish() -> bool {
    unsafe {
        osapp_finish();
    }
    true
}

/// Set the general menu bar of the application.
pub fn menubar<M, T>(menu: &M, win: &T)
where
    M: MenuTrait,
    T: WindowTrait,
{
    unsafe {
        osapp_menubar(menu.as_ptr(), win.as_ptr());
    }
}

/// Open an Internet address using the default operating system browser.
pub fn open_url(url: &str) {
    let url = CString::new(url).unwrap();
    unsafe {
        osapp_open_url(url.as_ptr());
    }
}
