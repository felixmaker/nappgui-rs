use crate::nappgui::*;

pub type FPtr_app_create = ::core::option::Option<unsafe extern "C" fn() -> *mut ::libc::c_void>;
pub type FPtr_app_update = ::core::option::Option<
    unsafe extern "C" fn(app: *mut ::libc::c_void, prtime: real64_t, ctime: real64_t),
>;

unsafe extern "C" {
    pub fn osmain_imp(
        argc: u32,
        argv: *mut *mut char_t,
        instance: *mut ::libc::c_void,
        lframe: real64_t,
        func_create: FPtr_app_create,
        func_update: FPtr_app_update,
        func_destroy: FPtr_destroy,
        options: *const char_t,
    );
}
