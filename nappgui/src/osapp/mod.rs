use std::ffi::{c_void, CString};

use nappgui_sys::{osapp_finish, osapp_open_url, osmain_imp};

use crate::gui::GLOBAL_OBJECTS;

/// Application handler.
pub trait AppHandler {
    /// Create the application. Controls should be created in this function.
    fn create() -> Self;
    /// Destroy the application.
    fn destroy(&mut self) {}
    /// Update the application.
    fn update(&mut self, _prtime: f64, _ctime: f64) {}
}

/// Start a desktop application.
pub fn osmain<T>()
where
    T: AppHandler,
{
    extern "C" fn on_create<T>() -> *mut c_void
    where
        T: AppHandler,
    {
        let app = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let app = T::create();
            Box::into_raw(Box::new(app)) as *mut c_void
        }));

        match app {
            Ok(app) => Box::into_raw(Box::new(app)) as *mut c_void,
            Err(_) => {
                std::process::exit(1) // Quit the application if create fails.
            }
        }
    }

    extern "C" fn on_destory<T>(obj: *mut *mut c_void)
    where
        T: AppHandler,
    {
        let app = unsafe { *obj as *mut T };
        if app.is_null() {
            return;
        }

        let mut app = unsafe { Box::from_raw(app) };
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            app.destroy();
            GLOBAL_OBJECTS.with_borrow_mut(|objs| {
                objs.clear();
            });
        }));
    }

    extern "C" fn on_update<T>(obj: *mut c_void, prtime: f64, ctime: f64)
    where
        T: AppHandler,
    {
        let app = obj as *mut T;
        if app.is_null() {
            return;
        }
        unsafe {
            (*app).update(prtime, ctime);
        }
    }

    unsafe {
        osmain_imp(
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0.0,
            Some(on_create::<T>),
            Some(on_update::<T>),
            Some(on_destory::<T>),
            std::ptr::null(),
        )
    }
}

/// End a desktop application, destroying the message cycle and the application object.
pub fn finish() -> bool {
    unsafe {
        osapp_finish();
    }
    true
}

/// Open an Internet address using the default operating system browser.
pub fn open_url(url: &str) {
    let url = CString::new(url).unwrap();
    unsafe {
        osapp_open_url(url.as_ptr());
    }
}
