use crate::nappgui::*;

pub const _dtype_t_ekDTYPE_BOOL: _dtype_t = 0;
pub const _dtype_t_ekDTYPE_INT: _dtype_t = 1;
pub const _dtype_t_ekDTYPE_REAL: _dtype_t = 2;
pub const _dtype_t_ekDTYPE_ENUM: _dtype_t = 3;
pub const _dtype_t_ekDTYPE_STRING: _dtype_t = 4;
pub const _dtype_t_ekDTYPE_STRUCT: _dtype_t = 5;
pub const _dtype_t_ekDTYPE_BINARY: _dtype_t = 6;
pub const _dtype_t_ekDTYPE_CONTAINER: _dtype_t = 7;
pub const _dtype_t_ekDTYPE_UNKNOWN: _dtype_t = 8;
pub type _dtype_t = ::libc::c_int;
pub use self::_dtype_t as dtype_t;
pub const _bindset_t_ekBINDSET_OK: _bindset_t = 0;
pub const _bindset_t_ekBINDSET_UNCHANGED: _bindset_t = 1;
pub const _bindset_t_ekBINDSET_NOT_ALLOWED: _bindset_t = 2;
pub type _bindset_t = ::libc::c_int;
pub use self::_bindset_t as bindset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _dbind_t {
    _unused: [u8; 0],
}
pub type DBind = _dbind_t;
pub type FPtr_str_create =
    ::core::option::Option<unsafe extern "C" fn(str_: *const char_t) -> *mut ::libc::c_void>;
pub type FPtr_str_get =
    ::core::option::Option<unsafe extern "C" fn(arg1: *const ::libc::c_void) -> *const char_t>;
pub type FPtr_container_create =
    ::core::option::Option<unsafe extern "C" fn(type_: *const char_t, esize: u16) -> *mut byte_t>;
pub type FPtr_container_size =
    ::core::option::Option<unsafe extern "C" fn(cont: *const byte_t) -> u32>;
pub type FPtr_container_get = ::core::option::Option<
    unsafe extern "C" fn(
        cont: *mut byte_t,
        pos: u32,
        type_: *const char_t,
        esize: u16,
    ) -> *mut byte_t,
>;
pub type FPtr_container_insert = ::core::option::Option<
    unsafe extern "C" fn(
        cont: *mut byte_t,
        pos: u32,
        n: u32,
        type_: *const char_t,
        esize: u16,
    ) -> *mut byte_t,
>;
pub type FPtr_container_delete = ::core::option::Option<
    unsafe extern "C" fn(cont: *mut byte_t, pos: u32, type_: *const char_t, esize: u16),
>;
pub type FPtr_container_destroy =
    ::core::option::Option<unsafe extern "C" fn(cont: *mut *mut byte_t, type_: *const char_t)>;
pub type FPtr_from_data = ::core::option::Option<
    unsafe extern "C" fn(data: *const byte_t, size: u32) -> *mut ::libc::c_void,
>;
unsafe extern "C" {
    pub fn respack_embedded(name: *const char_t) -> *mut ResPack;
}
unsafe extern "C" {
    pub fn respack_packed(name: *const char_t, locale: *const char_t) -> *mut ResPack;
}
unsafe extern "C" {
    pub fn respack_add_msg(pack: *mut ResPack, msg: *const char_t);
}
unsafe extern "C" {
    pub fn respack_add_cdata(pack: *mut ResPack, type_: u32, data: *const byte_t, data_size: u32);
}
unsafe extern "C" {
    pub fn respack_object_imp(
        pack: *const ResPack,
        id: ResId,
        func_create: FPtr_from_data,
        func_destroy: FPtr_destroy,
    ) -> *mut ::libc::c_void;
}
unsafe extern "C" {
    pub fn respack_aobj_imp(
        packs: *const ArrPtResPack,
        id: ResId,
        func_create: FPtr_from_data,
        func_destroy: FPtr_destroy,
        is_resid: *mut bool_t,
    ) -> *mut ::libc::c_void;
}
unsafe extern "C" {
    pub fn respack_atext(
        packs: *const ArrPtResPack,
        id: ResId,
        is_resid: *mut bool_t,
    ) -> *const char_t;
}
unsafe extern "C" {
    pub fn respack_afile(
        packs: *const ArrPtResPack,
        id: ResId,
        size: *mut u32,
        is_resid: *mut bool_t,
    ) -> *const byte_t;
}